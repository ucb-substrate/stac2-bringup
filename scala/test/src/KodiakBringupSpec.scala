package edu.berkeley.cs.kodiak.bringup

import chisel3._
import org.scalatest.funspec.AnyFunSpec
import circt.stage.ChiselStage
import org.chipsalliance.diplomacy.lazymodule.LazyModule

class KodiakBringupSpec extends AnyFunSpec {
  describe("KodiakBringup") {
    it("should generate valid System Verilog") {
      val targetDir =
        Utils.buildRoot / "KodiakBringup_should_generate_valid_System_Verilog"
      implicit val p = new KodiakBringupConfig
      ChiselStage.emitSystemVerilogFile(
        LazyModule(new KodiakBringupTop).module,
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
            artifactsDir / s"KodiakBringup.${extension}",
            contents()
          )
      }
    }

    it("should generate a NexysVideo bitstream") {
      val targetDir =
        Utils.buildRoot / "KodiakBringup_should_generate_a_NexysVideo_bistream"
      implicit val p = new KodiakBringupConfig
      Utils.genBitstream(targetDir, LazyModule(new KodiakBringupTop).module)
    }
  }

}

class ForceResetSpec extends AnyFunSpec {
  describe("ForceReset") {
    it("should generate a NexysVideo bitstream") {
      val targetDir =
        Utils.buildRoot / "ForceReset_should_generate_a_NexysVideo_bistream"
      Utils.genBitstream(targetDir, new ForceReset)
    }
  }
}
