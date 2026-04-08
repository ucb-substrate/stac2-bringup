package edu.berkeley.cs.stac2.bringup

import chisel3._
import chisel3.util.HasBlackBoxInline

class IBUF extends BlackBox {
  val io = IO(new Bundle {
    val O = Output(Bool())
    val I = Input(Bool())
  })
}

class IBUFG extends BlackBox {
  val io = IO(new Bundle {
    val O = Output(Clock())
    val I = Input(Clock())
  })
}

class mmcm extends BlackBox {
  val io = IO(new Bundle {
    val clk_in1 = Input(Clock())
    val clk_out1 = Output(Clock())
    val reset = Input(Bool())
    val locked = Output(Bool())
  })
}

class PowerOnResetFPGAOnly extends BlackBox with HasBlackBoxInline {
  val io = IO(new Bundle {
    val clock = Input(Clock())
    val power_on_reset = Output(Bool())
  })

  setInline(
    s"PowerOnResetFPGAOnly.v",
    s"""(* keep_hierarchy = "yes" *)
       |module PowerOnResetFPGAOnly(
       |  input wire clock,
       |  (* dont_touch = "true" *) output reg power_on_reset
       |);
       |  initial begin
       |    power_on_reset <= 1'b1;
       |  end
       |  always @(posedge clock) begin
       |    power_on_reset <= 1'b0;
       |  end
       |endmodule
       |""".stripMargin
  )
}

object PowerOnResetFPGAOnly {
  def apply(clk: Clock, name: String): Bool = {
    val por = Module(new PowerOnResetFPGAOnly())
    por.suggestName(name)
    por.io.clock := clk
    por.io.power_on_reset
  }
  def apply(clk: Clock): Bool = apply(clk, "fpga_power_on")
}
