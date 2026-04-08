package edu.berkeley.cs.stac2.bringup

import chisel3._
import org.scalatest.funspec.AnyFunSpec
import circt.stage.ChiselStage
import org.chipsalliance.diplomacy.lazymodule.LazyModule

class Stac2BringupSpec extends AnyFunSpec {
  describe("Stac2Bringup") {
    it("should generate valid System Verilog") {
      val targetDir =
        Utils.buildRoot / "Stac2Bringup_should_generate_valid_System_Verilog"
      implicit val p = new Stac2BringupConfig
      ChiselStage.emitSystemVerilogFile(
        LazyModule(new Stac2BringupTop).module,
        args = Array(
          "--target-dir",
          (targetDir / "src").toString()
        )
      )

      val artifactsDir = targetDir / "artifacts"
      os.makeDir.all(artifactsDir)
      freechips.rocketchip.util.ElaborationArtefacts.files.foreach {
        case (extension, contents) =>
          os.write.over(
            artifactsDir / s"Stac2Bringup.${extension}",
            contents()
          )
      }
    }

    it("should generate a Arty100T bitstream") {
      val targetDir =
        Utils.buildRoot / "Stac2Bringup_should_generate_a_Arty100T_bistream"
      implicit val p = new Stac2BringupConfig
      Utils.genBitstream(targetDir, LazyModule(new Stac2BringupTop).module)
    }
  }

}
