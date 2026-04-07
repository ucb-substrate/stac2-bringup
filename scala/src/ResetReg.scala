package edu.berkeley.cs.kodiak.bringup

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

case class ResetRegParams(
    address: BigInt = 0x90000000L
)

class ResetReg(params: ResetRegParams, beatBytes: Int)(implicit
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
  val device = new SimpleDevice("reset_reg", Seq("examples,reset_reg"))
  val node = TLRegisterNode(
    Seq(AddressSet(params.address, 256 - 1)),
    device,
    "reg/control",
    beatBytes = beatBytes
  )

  override lazy val module = new ResetRegImpl 
  class ResetRegImpl extends Impl {
    val reset_out = IO(Output(Bool()))

    withClockAndReset(clock, reset) {
      val resetReg = RegInit(false.B)
      reset_out := resetReg || reset.asBool

      node.regmap(
        0x0 -> Seq(toRegFieldRw(resetReg, "resetReg")),
      )
    }
  }
}
