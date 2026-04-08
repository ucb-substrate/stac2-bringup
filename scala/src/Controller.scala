package edu.berkeley.cs.stac2.bringup

import chisel3._
import chisel3.util._
import org.chipsalliance.cde.config.{Parameters, Field, Config}
import edu.berkeley.cs.chippy._
import freechips.rocketchip.regmapper.{RegField, RegWriteFn, RegFieldDesc}
import freechips.rocketchip.tilelink._
import freechips.rocketchip.prci._
import freechips.rocketchip.diplomacy.{SimpleDevice, AddressSet}
import org.chipsalliance.diplomacy._
import org.chipsalliance.diplomacy.lazymodule._

import scala.collection.mutable.LinkedHashMap

class StacControllerIO extends Bundle {
  val sramExtEn = Output(Bool())
  val sramScanMode = Output(Bool())
  val sramEn = Output(Bool())
  val sramScanIn = Output(Bool())
  val sramScanEn = Output(Bool())
  val sramBistEn = Output(Bool())
  val sramBistStart = Output(Bool())
  val pllSel = Output(Bool())
  val pllScanEn = Output(Bool())
  val pllScanRstn = Output(Bool())
  val pllScanClk = Output(Bool())
  val pllScanIn = Output(Bool())
  val pllArstb = Output(Bool())
  val sramScanOut = Input(Bool())
  val sramBistDone = Input(Bool())
  val pllScanOut = Input(Bool())
  val reset = Output(Bool())
  val clk = Output(Bool())
}

class StacController(
    beatBytes: Int,
    address: BigInt = 0x90000000L,
    halfClkDivRatioDefault: Int = 125
)(implicit
    p: Parameters
) extends ClockSinkDomain(ClockSinkParameters())(p) {
  def toRegFieldRw[T <: Data](r: T, name: String): RegField = {
    RegField(
      r.getWidth,
      r.asUInt,
      RegWriteFn((valid, data) => {
        when(valid) {
          r := data.asTypeOf(r)
        }
        true.B
      }),
      Some(RegFieldDesc(name, ""))
    )
  }
  def toRegFieldR[T <: Data](r: T, name: String): RegField = {
    RegField.r(r.getWidth, r.asUInt, RegFieldDesc(name, ""))
  }

  val device =
    new SimpleDevice("stac_controller", Seq("ucbbar,stac_controller"))
  val node = TLRegisterNode(
    Seq(AddressSet(address, 256 - 1)),
    device,
    "reg/control",
    beatBytes = beatBytes
  )

  override lazy val module = new StacControllerImpl
  class StacControllerImpl extends Impl {
    val io = IO(new StacControllerIO())

    withClockAndReset(clock, reset) {

      val sramExtEn = RegInit(false.B)
      val sramScanMode = RegInit(false.B)
      val sramEn = RegInit(false.B)
      val sramBistEn = RegInit(false.B)
      val sramBistStart = RegInit(false.B)
      val pllSel = RegInit(false.B)
      val pllScanRstn = RegInit(false.B)
      val pllArstb = RegInit(false.B)
      val halfClkDivRatio = RegInit(halfClkDivRatioDefault.U(32.W))
      val clkEn = RegInit(false.B)
      val divClk = RegInit(false.B)
      val cycles = RegInit(0.U(32.W))
      val resetReg = RegInit(false.B)

      io.sramScanIn := true.B
      io.sramScanEn := false.B
      io.pllScanEn := false.B
      io.pllScanClk := false.B
      io.pllScanIn := true.B
      io.reset := reset.asBool || resetReg

      when(clkEn) {
        io.clk := divClk
      }.otherwise {
        io.clk := false.B
      }

      when(halfClkDivRatio === 0.U || cycles >= halfClkDivRatio - 1.U) {
        cycles := 0.U
        divClk := ~divClk
      }.otherwise {
        cycles := cycles + 1.U
      }

      val regs = Seq(
        toRegFieldRw(sramExtEn, "sramExtEn"),
        toRegFieldRw(sramScanMode, "sramScanMode"),
        toRegFieldRw(sramEn, "sramEn"),
        toRegFieldRw(sramBistEn, "sramBistEn"),
        toRegFieldRw(sramBistStart, "sramBistStart"),
        toRegFieldRw(pllSel, "pllSel"),
        toRegFieldRw(pllScanRstn, "pllScanRstn"),
        toRegFieldRw(pllArstb, "pllArstb"),
        toRegFieldRw(halfClkDivRatio, "halfClkDivRatio"),
        toRegFieldRw(clkEn, "clkEn"),
        toRegFieldRw(divClk, "divClk"),
        toRegFieldRw(cycles, "cycles"),
        toRegFieldRw(resetReg, "resetReg"),
        toRegFieldR(io.sramBistDone, "sramBistDone")
      )

      node.regmap(
        regs
          .scanLeft(0)((acc, reg) => acc + ((reg.width - 1) / 64 + 1) * 8)
          .zip(regs.map(Seq(_))): _*
      )
    }
  }
}
