package edu.berkeley.cs.stac2.bringup

import chisel3._
import chisel3.util._

object ProgrammableBistParams {
  def elementTableLength: Int = 8
  def operationsPerElement: Int = 8
  def patternTableLength: Int = 8
  def maxRowAddrWidth: Int = 11
  def maxColAddrWidth: Int = 3
  def dataWidth: Int = 128
  def randAddrWidth: Int = 14
}

object OperationType extends ChiselEnum {
  val read = Value(0.U(2.W))
  val write = Value(1.U(2.W))
  val rand = Value(2.U(2.W))
}

object FlipType extends ChiselEnum {
  val flipped = Value(0.U(1.W))
  val unflipped = Value(1.U(1.W))
}

object Direction extends ChiselEnum {
  val up = Value(0.U(2.W))
  val down = Value(1.U(2.W))
  val rand = Value(2.U(2.W))
}

object ElementType extends ChiselEnum {
  val waitOp = Value(0.U(1.W))
  val rwOp = Value(1.U(1.W))
}

object Dimension extends ChiselEnum {
  val row = Value(0.U(1.W))
  val col = Value(1.U(1.W))
}

class Operation extends Bundle {
  val operationType = OperationType()
  // Randomize data?
  val randData = Bool()
  // Randomize mask?
  val randMask = Bool()

  // Data pattern address if data is not randomized.
  val dataPatternIdx = UInt(
    log2Ceil(ProgrammableBistParams.patternTableLength).W
  )
  // Mask pattern address if mask is not randomized.
  val maskPatternIdx = UInt(
    log2Ceil(ProgrammableBistParams.patternTableLength).W
  )

  // Bitwise flip data?
  val flipped = FlipType()
}

class OperationElement extends Bundle {
  val operations =
    Vec(ProgrammableBistParams.operationsPerElement, new Operation())
  // Number of operations minus 1
  val maxIdx = UInt(log2Ceil(ProgrammableBistParams.operationsPerElement).W)
  val dir = Direction()
  // Number of random addresses to try.
  // Only used if `dir` is set to `rand`.
  val numAddrs = UInt(ProgrammableBistParams.randAddrWidth.W)
}

class WaitElement extends Bundle {
  val cyclesToWait = UInt(ProgrammableBistParams.randAddrWidth.W)
}

class Element extends Bundle {
  val operationElement = new OperationElement()
  val waitElement = new WaitElement()
  val elementType = ElementType()
}
