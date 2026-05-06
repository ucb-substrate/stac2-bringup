package edu.berkeley.cs.stac2.bringup

import chisel3._
import chisel3.util._

import org.chipsalliance.cde.config.Config
import org.chipsalliance.diplomacy.lazymodule._
import freechips.rocketchip.devices.tilelink.{CLINTKey, BootROMLocated}
import freechips.rocketchip.devices.debug.DebugModuleKey
import freechips.rocketchip.devices.tilelink.PLICKey
import freechips.rocketchip.prci.ResetWrangler
import freechips.rocketchip.subsystem.SystemBusKey
import freechips.rocketchip.subsystem.ControlBusKey
import freechips.rocketchip.subsystem.PeripheryBusKey
import freechips.rocketchip.subsystem.MemoryBusKey
import freechips.rocketchip.subsystem.FrontBusKey
import freechips.rocketchip.subsystem.SubsystemBankedCoherenceKey
import freechips.rocketchip.subsystem.CoherenceManagerWrapper
import freechips.rocketchip.subsystem.{SBUS, PBUS}
import freechips.rocketchip.diplomacy.AddressSet
import org.chipsalliance.cde.config.Parameters
import freechips.rocketchip.prci.{ClockGroup, ClockSinkNode, ClockGroupSourceNode}
import freechips.rocketchip.prci.ClockGroupSourceParameters
import freechips.rocketchip.diplomacy.BindingScope
import freechips.rocketchip.util.DontTouch
import testchipip.serdes.DecoupledExternalSyncPhitIO
import testchipip.serdes.SerialTLKey
import testchipip.soc.OffchipBusKey
import freechips.rocketchip.subsystem.ExtMem
import org.chipsalliance.cde.config.Field
import freechips.rocketchip.subsystem.MemoryPortParams
import freechips.rocketchip.subsystem.BaseSubsystem
import freechips.rocketchip.diplomacy.MemoryDevice
import freechips.rocketchip.subsystem.MBUS
import freechips.rocketchip.tilelink.TLManagerNode
import freechips.rocketchip.tilelink.TLSlavePortParameters
import freechips.rocketchip.tilelink.TLSlaveParameters
import freechips.rocketchip.diplomacy.RegionType
import freechips.rocketchip.diplomacy.TransferSizes
import freechips.rocketchip.tilelink.TLBuffer
import freechips.rocketchip.tilelink.TLSourceShrinker
import freechips.rocketchip.tilelink._
import freechips.rocketchip.diplomacy.DisableMonitors
import freechips.rocketchip.prci.ClockGroupAggregateNode
import testchipip.serdes.DecoupledInternalSyncPhitIO

object Stac2Bringup {
  val freqMHz: Int = 50
}


// Similar to ExtMem but instantiates a TL mem port
case object ExtTLMem extends Field[Option[MemoryPortParams]](None)

/** Adds a port to the system intended to master an TL DRAM controller. */
trait CanHaveMasterTLMemPort { this: BaseSubsystem =>

  require(!(p(ExtTLMem).nonEmpty && p(ExtMem).nonEmpty),
    "Can only have 1 backing memory port. Use ExtTLMem for a TL memory port or ExtMem for an AXI memory port.")

  private val memPortParamsOpt = p(ExtTLMem)
  private val portName = "tl_mem"
  private val device = new MemoryDevice
  private val idBits = memPortParamsOpt.map(_.master.idBits).getOrElse(1)
  private val mbus = tlBusWrapperLocationMap.lift(MBUS).getOrElse(locateTLBusWrapper(SBUS))

  val memTLNode = TLManagerNode(memPortParamsOpt.map({ case MemoryPortParams(memPortParams, nMemoryChannels, _) =>
    Seq.tabulate(nMemoryChannels) { channel =>
      val base = AddressSet.misaligned(memPortParams.base, memPortParams.size)
      val filter = AddressSet(channel * mbus.blockBytes, ~((nMemoryChannels-1) * mbus.blockBytes))

     TLSlavePortParameters.v1(
       managers = Seq(TLSlaveParameters.v1(
         address            = base.flatMap(_.intersect(filter)),
         resources          = device.reg,
         regionType         = RegionType.UNCACHED, // cacheable
         executable         = true,
         supportsGet        = TransferSizes(1, mbus.blockBytes),
         supportsPutFull    = TransferSizes(1, mbus.blockBytes),
         supportsPutPartial = TransferSizes(1, mbus.blockBytes))),
         beatBytes = memPortParams.beatBytes)
    }
  }).toList.flatten)

  // disable inwards monitors from node since the class with this trait (i.e. DigitalTop)
  // doesn't provide an implicit clock to those monitors
  mbus.coupleTo(s"memory_controller_port_named_$portName") {
    (DisableMonitors { implicit p => memTLNode :*= TLBuffer() }
      :*= TLSourceShrinker(1 << idBits)
      :*= TLWidthWidget(mbus.beatBytes)
      :*= _)
  }

  val mem_tl = InModuleBody { memTLNode.makeIOs() }
}

class NoCoresConfig extends Config(
  new Config((site, here, up) => {
    case CLINTKey => None
    case BootROMLocated(_) => Nil
    case SubsystemBankedCoherenceKey => up(SubsystemBankedCoherenceKey, site).copy(coherenceManager = CoherenceManagerWrapper.broadcastManager)
    case DebugModuleKey => None
    case PLICKey => None
    case SystemBusKey => up(SystemBusKey).copy(errorDevice = None)
    case ControlBusKey => up(ControlBusKey).copy(errorDevice = None)
    case PeripheryBusKey => up(PeripheryBusKey).copy(errorDevice = None)
    case MemoryBusKey => up(MemoryBusKey).copy(errorDevice = None)
    case FrontBusKey => up(FrontBusKey).copy(errorDevice = None)
  }) ++
  new edu.berkeley.cs.chippy.clocking.WithNoTileClockGaters ++
  new edu.berkeley.cs.chippy.clocking.WithNoTileResetSetters ++ 
  new freechips.rocketchip.system.BaseConfig)

class Stac2BringupConfig extends Config(
    //=============================
  // Setup the SerialTL side on the bringup device
  //=============================
  new testchipip.serdes.old.WithSerialTL(Seq(testchipip.serdes.old.SerialTLParams(
    manager = Some(testchipip.serdes.old.SerialTLManagerParams(
      memParams = Seq(testchipip.serdes.old.ManagerRAMParams(                            // Bringup platform can access all memory from 0 to DRAM_BASE
        address = BigInt("00000000", 16),
        size    = BigInt("80000000", 16)
      ))
    )),
    client = Some(testchipip.serdes.old.SerialTLClientParams()),                                        // Allow chip to access this device's memory (DRAM)
    phyParams = testchipip.serdes.old.ExternalSyncSerialParams(width = 1) // chip provides the clock
  ))) ++

  //============================
  // Setup bus topology on the bringup system
  //============================
  new testchipip.soc.WithOffchipBusClient(SBUS,                                // offchip bus hangs off the SBUS
    blockRange = AddressSet.misaligned(0x80000000L, (BigInt(1) << 30) * 4)) ++ // offchip bus should not see the main memory of the testchip, since that can be accessed directly
  new testchipip.soc.WithOffchipBus ++                                         // offchip bus

  //=============================
  // Set up memory on the bringup system
  //=============================
  new freechips.rocketchip.subsystem.WithExtMemSize((1 << 30) * 4L) ++         // match what the chip believes the max size should be
  new Config((site, here, up) => {
    case ExtMem   => None // disable AXI backing memory
    case ExtTLMem => up(ExtMem, site) // enable TL backing memory
  }) ++

  //=============================
  // Generate the TSI-over-UART side of the bringup system
  //=============================
  new testchipip.tsi.WithUARTTSIClient(initBaudRate = BigInt(115200)) ++       // nonstandard baud rate to improve performance

  //=============================
  // Set up clocks/buses of the bringup system
  //=============================
  new edu.berkeley.cs.chippy.clocking.WithUniformBusFrequencies(Stac2Bringup.freqMHz) ++   // run all buses of this system at 75 MHz
  new Config((site, here, up) => { case OffchipBusKey =>
    up(OffchipBusKey, site)
      .copy(dtsFrequency = Some(BigInt(Stac2Bringup.freqMHz * 1_000_000L)))
  }) ++
  new edu.berkeley.cs.chippy.clocking.WithNoSubsystemClockIO ++ 
  new freechips.rocketchip.subsystem.WithCoherentBusTopology ++
  new freechips.rocketchip.subsystem.WithDontDriveBusClocksFromSBus ++  /** leave the bus clocks undriven by sbus */
  new freechips.rocketchip.subsystem.WithClockGateModel ++              /** add default EICG_wrapper clock gate model */
  new edu.berkeley.cs.chippy.clocking.WithClockGroupsCombinedByName(("uncore",        /** create a "uncore" clock group tieing all the bus clocks together */
    Seq("sbus", "mbus", "pbus", "fbus", "cbus", "obus", "implicit", "clock_tap"),
    Seq("tile"))) ++


  // Base is the no-cores config
  new NoCoresConfig)

class Stac2BringupSystem(implicit p: Parameters)
    extends edu.berkeley.cs.chippy.ChippySystem
    with testchipip.soc.CanHaveSubsystemInjectors // Enables the subsystem injector API
    with testchipip.soc.CanHaveSwitchableOffchipBus // Enables optional off-chip-bus with interface-switch
    with testchipip.serdes.old.CanHavePeripheryTLSerial
    with testchipip.tsi.CanHavePeripheryUARTTSI
    with edu.berkeley.cs.chippy.clocking.HasChippyPRCI
    with CanHaveMasterTLMemPort {
    val pbus = locateTLBusWrapper(SBUS)
    val stacController = LazyModule(new StacController(pbus.beatBytes))
    stacController.clockNode := pbus.fixedClockNode
    pbus.coupleTo("resetReg") { stacController.node := TLFragmenter(pbus.beatBytes, pbus.blockBytes) := _ }
    val ctl = InModuleBody {
      val ctl = IO(new StacControllerIO())
      ctl <> stacController.module.io
      ctl
    }
}

class LedPattern(counterMax: Int = 25_000_000) extends Module {
  val io = IO(new Bundle {
    val led_0 = Output(Bool())
    val led_1 = Output(Bool())
    val led_2 = Output(Bool())
    val led_3 = Output(Bool())
    val led_4 = Output(Bool())
    val led_5 = Output(Bool())
    val led_6 = Output(Bool())
    val led_7 = Output(Bool())
  })

  // Slow down animation
  val counter = RegInit(0.U(log2Ceil(counterMax).W))
  val tick = counter === (counterMax - 1).U
  counter := Mux(tick, 0.U, counter + 1.U)

  // Current LED position and direction
  val pos = RegInit(0.U(3.W))
  val dirRight = RegInit(true.B)

  when(tick) {
    when(dirRight) {
      when(pos === 7.U) {
        dirRight := false.B
        pos := 6.U
      }.otherwise {
        pos := pos + 1.U
      }
    }.otherwise {
      when(pos === 0.U) {
        dirRight := true.B
        pos := 1.U
      }.otherwise {
        pos := pos - 1.U
      }
    }
  }

  // Tail: current LED + two trailing LEDs
  val leds = Wire(Vec(8, Bool()))
  for (i <- 0 until 8) {
    val dist = Mux(pos > i.U, pos - i.U, i.U - pos)
    leds(i) := dist <= 2.U
  }

  io.led_0 := leds(0)
  io.led_1 := leds(1)
  io.led_2 := leds(2)
  io.led_3 := leds(3)
  io.led_4 := leds(4)
  io.led_5 := leds(5)
  io.led_6 := leds(6)
  io.led_7 := leds(7)
}

class Stac2BringupTop(driveClk: Boolean = true)(implicit p: Parameters) extends LazyModule with BindingScope {
  val system = LazyModule(new Stac2BringupSystem)
  val pllSourceNode = ClockGroupSourceNode(
    Seq(ClockGroupSourceParameters())
  )
  val dutClock = ClockSinkNode(freqMHz = Stac2Bringup.freqMHz)
  val dutWrangler = LazyModule(new ResetWrangler())
  val dutGroup = ClockGroup()
  dutClock := dutWrangler.node := dutGroup := pllSourceNode
  val dutSourceNode = ClockGroupSourceNode(
    Seq(ClockGroupSourceParameters())
  )
  system.chiptopClockGroupsNode := dutSourceNode

  override lazy val module = new Stac2BringupTopImpl
  class Stac2BringupTopImpl extends LazyRawModuleImp(this) with DontTouch {
    val io = IO(new Bundle {
      val clock = Input(Clock())
      val reset = Input(AsyncReset())
      val ctl = new StacControllerIO()
    })

    io.ctl <> system.ctl

    if (!driveClk) {
      // Tri-state the clock output so an external generator can drive the chip
      // clock without fighting the FPGA. Set driveClk=true to restore normal
      // FPGA-driven clock behaviour.
      val clkBufT = Module(new OBUFT)
      clkBufT.io.I := system.ctl.clk
      clkBufT.io.T := true.B
      io.ctl.clk := clkBufT.io.O
    }

    val rstBuf = Module(new IBUF)
    rstBuf.io.I := io.reset.asBool

    val clkBuf = Module(new IBUFG)
    clkBuf.io.I := io.clock
    val powerOnReset = PowerOnResetFPGAOnly(clkBuf.io.O)

    val pll = Module(new mmcm)
    pll.io.clk_in1 := clkBuf.io.O
    pll.io.reset := (!rstBuf.io.O) || powerOnReset

    pllSourceNode.out.foreach { case (bundle, edge) =>
      bundle.member.data.foreach { b =>
        b.clock := pll.io.clk_out1
        b.reset := (!rstBuf.io.O) || !pll.io.locked
      }
    }

    dutSourceNode.out.foreach { case (bundle, edge) =>
      bundle.member.data.foreach { b =>
        b := dutClock.in.head._1
      }
    }

    // Tie off interupts and chip ID
    system.module.interrupts := DontCare

    // TODO: Get correct serial TL directionality.
    val serial_tl = IO(chiselTypeOf(system.old_serial_tls(0).getWrappedValue))
    serial_tl <> system.old_serial_tls(0)

    val uart = IO(chiselTypeOf(system.uart_tsi.get.uart))
    uart <> system.uart_tsi.get.uart

    system.mem_tl := DontCare

    val led_0 = IO(Output(Bool()))
    val led_1 = IO(Output(Bool()))
    val led_2 = IO(Output(Bool()))
    val led_3 = IO(Output(Bool()))
    val led_4 = IO(Output(Bool()))
    val led_5 = IO(Output(Bool()))
    val led_6 = IO(Output(Bool()))
    val led_7 = IO(Output(Bool()))
    val pattern = withClockAndReset(dutClock.in.head._1.clock, dutClock.in.head._1.reset) {
      Module(new LedPattern()) 
    }
    led_0 := pattern.io.led_0
    led_1 := pattern.io.led_1
    led_2 := pattern.io.led_2
    led_3 := pattern.io.led_3
    led_4 := pattern.io.led_4
    led_5 := pattern.io.led_5
    led_6 := pattern.io.led_6
    led_7 := pattern.io.led_7

    val reset_btn = IO(Input(Bool()))
    io.ctl.reset := !(reset_btn || system.ctl.reset)

    // led_0 := system0.uart_tsi.get.tsi2tl_state(0)
    // led_1 := system0.uart_tsi.get.tsi2tl_state(1)
    // led_2 := system0.uart_tsi.get.tsi2tl_state(2)
    // led_3 := system0.uart_tsi.get.tsi2tl_state(3)
    // led_4 := reset_u_0_chip_rst || reset_u_1_chip_rst
  }
}
