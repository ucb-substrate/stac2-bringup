package edu.berkeley.cs.stac2.bringup

import chisel3._
import chisel3.experimental.BundleLiterals._
import chisel3.experimental.VecLiterals._
import chisel3.simulator.ChiselSim
import org.scalatest.funspec.AnyFunSpec

class BistSpec extends AnyFunSpec with ChiselSim {
  private type BistElement = edu.berkeley.cs.stac2.bringup.Element

  private def op(
      operationType: OperationType.Type,
      dataPatternIdx: Int,
      maskPatternIdx: Int,
      flipData: Boolean
  ): Operation =
    (new Operation).Lit(
      _.operationType -> operationType,
      _.randData -> false.B,
      _.randMask -> false.B,
      _.dataPatternIdx -> dataPatternIdx.U,
      _.maskPatternIdx -> maskPatternIdx.U,
      _.flipped -> (if (flipData) FlipType.flipped else FlipType.unflipped)
    )

  private def zeroOp: Operation =
    op(OperationType.read, 0, 0, flipData = false)

  private def waitElement(cyclesToWait: Int = 0): WaitElement =
    (new WaitElement).Lit(
      _.cyclesToWait -> cyclesToWait.U
    )

  private def opElement(
      operations: Vec[Operation],
      maxIdx: Int,
      dir: Direction.Type = Direction.up,
      numAddrs: Int = 0
  ): OperationElement =
    (new OperationElement).Lit(
      _.operations -> operations,
      _.maxIdx -> maxIdx.U,
      _.dir -> dir,
      _.numAddrs -> numAddrs.U
    )

  private def element(
      operationElement: OperationElement,
      waitElement: WaitElement,
      elementType: ElementType.Type = ElementType.rwOp
  ): BistElement =
    (new BistElement).Lit(
      _.operationElement -> operationElement,
      _.waitElement -> waitElement,
      _.elementType -> elementType
    )

  private def elementWithOps(
      ops: Seq[Operation],
      dir: Direction.Type = Direction.up,
      maxIdx: Option[Int] = None
  ): BistElement = {
    require(ops.nonEmpty)
    require(ops.length <= ProgrammableBistParams.operationsPerElement)

    val paddedOps = ops.padTo(
      ProgrammableBistParams.operationsPerElement,
      zeroOp
    )

    element(
      opElement(
        Vec.Lit(paddedOps: _*),
        maxIdx = maxIdx.getOrElse(ops.length - 1),
        dir = dir
      ),
      waitElement()
    )
  }

  private def zeroElement: BistElement =
    element(
      opElement(
        Vec.Lit(
          Seq.fill(ProgrammableBistParams.operationsPerElement)(
            zeroOp
          ): _*
        ),
        maxIdx = 0
      ),
      waitElement(),
      ElementType.waitOp
    )

  private def elementSequence(elements: Seq[BistElement]): Vec[BistElement] = {
    require(elements.length <= ProgrammableBistParams.elementTableLength)
    Vec.Lit(
      elements
        .padTo(ProgrammableBistParams.elementTableLength, zeroElement): _*
    )
  }

  private def printElementSequenceWords(
      elementSequence: Vec[BistElement]
  ): Unit = {
    val words = elementSequence.asTypeOf(Vec(16, UInt(64.W)))
    for (i <- 0 until 16) {
      println(f"0x${words(i).asUInt.litValue}%x")
    }
  }

  describe("Bist") {
    it("should print basic BIST element table hex") {
      class ElementTest extends Module {
        val elements = Seq(
          elementWithOps(
            Seq(op(OperationType.write, 0, 1, flipData = false)),
            Direction.up
          ),
          elementWithOps(
            Seq(
              op(OperationType.read, 0, 1, flipData = false),
              op(OperationType.write, 0, 1, flipData = true)
            ),
            Direction.up
          ),
          elementWithOps(
            Seq(
              op(OperationType.read, 0, 1, flipData = true),
              op(OperationType.write, 0, 1, flipData = false)
            ),
            Direction.down
          ),
          elementWithOps(
            Seq(op(OperationType.read, 0, 1, flipData = false)),
            Direction.up
          )
        )

        printElementSequenceWords(elementSequence(elements))
      }
      simulate(new ElementTest) { _ => }
    }

    it("should print copied march BIST element sequence hex") {
      class ElementSequenceTest extends Module {
        val readOp = op(OperationType.read, 3, 0, flipData = false)
        val writeOp = op(OperationType.write, 3, 1, flipData = false)
        val readFlippedOp = op(OperationType.read, 3, 0, flipData = true)
        val writeFlippedOp = op(OperationType.write, 3, 1, flipData = true)
        val march = elementWithOps(
          Seq(
            writeOp,
            readOp,
            writeFlippedOp,
            readFlippedOp,
            readOp,
            readOp,
            readOp,
            readOp
          )
        )

        printElementSequenceWords(
          elementSequence(
            Seq(march, march, march, march)
          )
        )
      }
      simulate(new ElementSequenceTest) { _ => }
    }
  }

}
