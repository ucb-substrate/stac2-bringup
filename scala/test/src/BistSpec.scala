package edu.berkeley.cs.stac2.bringup

import chisel3._
import chisel3.util._
import chisel3.experimental.BundleLiterals._
import chisel3.experimental.VecLiterals._
import org.scalatest.funspec.AnyFunSpec
import chisel3.simulator.ChiselSim

class BistSpec extends AnyFunSpec with ChiselSim {
  describe("Bist") {
    it("should print basic BIST element table hex") {
      class ElementTest extends Module {
        def op(
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
            _.flipped -> (if (flipData) FlipType.flipped
                          else FlipType.unflipped)
          )

        def zeroOp: Operation =
          op(OperationType.read, 0, 0, flipData = false)

        def zeroWaitElement: WaitElement =
          (new WaitElement).Lit(
            _.cyclesToWait -> 0.U
          )

        def opElement(
            ops: Seq[Operation],
            dir: Direction.Type
        ): edu.berkeley.cs.stac2.bringup.Element = {
          require(ops.nonEmpty)
          require(ops.length <= ProgrammableBistParams.operationsPerElement)

          val paddedOps = ops.padTo(
            ProgrammableBistParams.operationsPerElement,
            zeroOp
          )

          (new edu.berkeley.cs.stac2.bringup.Element).Lit(
            _.operationElement -> (new OperationElement).Lit(
              _.operations -> Vec.Lit(paddedOps: _*),
              _.maxIdx -> (ops.length - 1).U,
              _.dir -> dir,
              _.numAddrs -> 0.U
            ),
            _.waitElement -> zeroWaitElement,
            _.elementType -> ElementType.rwOp
          )
        }

        def zeroElement: edu.berkeley.cs.stac2.bringup.Element =
          (new edu.berkeley.cs.stac2.bringup.Element).Lit(
            _.operationElement -> (new OperationElement).Lit(
              _.operations -> Vec.Lit(
                Seq.fill(ProgrammableBistParams.operationsPerElement)(
                  zeroOp
                ): _*
              ),
              _.maxIdx -> 0.U,
              _.dir -> Direction.up,
              _.numAddrs -> 0.U
            ),
            _.waitElement -> zeroWaitElement,
            _.elementType -> ElementType.waitOp
          )

        val elements = Seq(
          opElement(
            Seq(op(OperationType.write, 0, 1, flipData = false)),
            Direction.up
          ),
          opElement(
            Seq(
              op(OperationType.read, 0, 1, flipData = false),
              op(OperationType.write, 0, 1, flipData = true)
            ),
            Direction.up
          ),
          opElement(
            Seq(
              op(OperationType.read, 0, 1, flipData = true),
              op(OperationType.write, 0, 1, flipData = false)
            ),
            Direction.down
          ),
          opElement(
            Seq(op(OperationType.read, 0, 1, flipData = false)),
            Direction.up
          )
        ).padTo(
          ProgrammableBistParams.elementTableLength,
          zeroElement
        )

        val elementTable = Vec.Lit(elements: _*).asTypeOf(Vec(16, UInt(64.W)))
        for (i <- 0 until 16) {
          println(
            f"0x${elementTable(i).asUInt.litValue}%x"
          )
        }
      }
      simulate(new ElementTest) { c => }
    }
  }

}
