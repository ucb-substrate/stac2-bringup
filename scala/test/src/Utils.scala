package edu.berkeley.cs.stac2.bringup

import chisel3._
import os.Path
import circt.stage.ChiselStage
import java.nio.file.Paths
import testchipip.dram.SimDRAM
import chisel3.stage.DesignAnnotation
import org.chipsalliance.diplomacy.lazymodule._
import org.chipsalliance.cde.config.Parameters

object Utils {
  val root = Path(
    Paths.get(sys.env("MILL_TEST_RESOURCE_DIR")).toAbsolutePath
  ) / os.up / os.up
  val buildRoot = root / "build"

  def writeSourceFilesList(path: Path, sourceFiles: Seq[Path]) = {
    os.makeDir.all(path / os.up)
    os.write.over(path, sourceFiles.map(_.toString).mkString("\n"))
  }

  def writeGenBitstreamTcl(
      path: Path,
      objDir: Path,
      sourceFilesList: Path,
      topModule: String,
      incDirs: Seq[Path] = Seq.empty
  ) = {
    os.makeDir.all(path / os.up)
    os.write.over(
      path,
      s"""set name {arty-a7-100}
set part_fpga {xc7a100ticsg324-1L}
set part_board {digilentinc.com:arty-a7-100:part0:1.1}
set bootrom_inst {rom}

set wrkdir ${objDir.toString}
set ipdir [file join $$wrkdir ip]

# Create an in-memory project
create_project -part $$part_fpga -force $topModule

set_param messaging.defaultLimit 1000000

# Set the board part, target language, default library, and IP directory
# paths for the current project
set_property -dict [list \\
  BOARD_PART $$part_board \\
  TARGET_LANGUAGE {Verilog} \\
  DEFAULT_LIB {xil_defaultlib} \\
  ] [current_project]

if {[get_filesets -quiet sources_1] eq ""} {
  create_fileset -srcset sources_1
}
set obj [current_fileset]

# Add verilog files from manifest
set fp [open ${sourceFilesList.toString} r]
set files [lsearch -not -exact -all -inline [split [read $$fp] "\n"] {}]
set relative_files {}
foreach path $$files {
  if {[string match {/*} $$path]} {
    lappend relative_files $$path
  } elseif {![string match {#*} $$path]} {
    lappend relative_files [file join [file dirname $$vsrc_manifest] $$path]
  }
}
add_files -norecurse -fileset $$obj {*}$$relative_files
close $$fp

# Helper function that recursively includes files given a directory and a
# pattern/suffix extensions
proc recglob { basedir pattern } {
  set dirlist [glob -nocomplain -directory $$basedir -type d *]
  set findlist [glob -nocomplain -directory $$basedir $$pattern]
  foreach dir $$dirlist {
    set reclist [recglob $$dir $$pattern]
    set findlist [concat $$findlist $$reclist]
  }
  return $$findlist
}

# Helper function to find all subdirectories containing ".vh" files
proc findincludedir { basedir pattern } {
  set vhfiles [recglob $$basedir $$pattern]
  set vhdirs {}
  foreach match $$vhfiles {
    lappend vhdirs [file dir $$match]
  }
  set uniquevhdirs [lsort -unique $$vhdirs]
  return $$uniquevhdirs
}

# Create the directory for IPs
file mkdir $$ipdir

# Update the IP catalog
update_ip_catalog -rebuild

create_ip -name clk_wiz -vendor xilinx.com -library ip -version 6.0 -module_name \\
 mmcm -dir $$ipdir -force
set_property -dict [list \\
 CONFIG.CLKOUT1_REQUESTED_OUT_FREQ {50.0} \\
] [get_ips mmcm]

# AR 58526 <http://www.xilinx.com/support/answers/58526.html>
set xci_files [get_files -all {*.xci}]
foreach xci_file $$xci_files {
  set_property GENERATE_SYNTH_CHECKPOINT {false} -quiet $$xci_file
}

# Get a list of IPs in the current design
set obj [get_ips]

# Generate target data for the included IPs in the design
generate_target all $$obj

# Export the IP user files
export_ip_user_files -of_objects $$obj -no_script -force

# Get the list of active source and constraint files
set obj [current_fileset]

#Xilinx bug workaround
#scrape IP tree for directories containing .vh files
#[get_property include_dirs] misses all IP core subdirectory includes if user has specified -dir flag in create_ip
set property_include_dirs [get_property include_dirs $$obj]

# Include generated files for the IPs in the design
set ip_include_dirs [concat $$property_include_dirs [findincludedir $$ipdir "*.vh"]]

# Synthesis
synth_design -top $topModule -flatten_hierarchy rebuilt${incDirs
          .map(incDir => s" -include_dirs ${incDir.toString}")
          .mkString}
write_checkpoint -force [file join $$wrkdir post_synth]
report_timing -file synth_timing.rpt 
report_utilization -file synth_utilization.rpt

# Optmization
opt_design -directive Explore
write_checkpoint -force [file join $$wrkdir post_opt]

# Placement
place_design -directive Explore
phys_opt_design -directive Explore
power_opt_design
write_checkpoint -force [file join $$wrkdir post_place]

# Routing
route_design -directive Explore
phys_opt_design -directive Explore
write_checkpoint -force [file join $$wrkdir post_route]
report_timing -file route_timing.rpt 
report_utilization -file route_utilization.rpt

# Write bitstream
write_bitstream -force [file join $$wrkdir "$topModule.bit"]
write_sdf -force [file join $$wrkdir "$topModule.sdf"]
write_verilog -mode timesim -force [file join $$wrkdir "$topModule.v"]

# Write reports
set rptdir [file join $$wrkdir report]
file mkdir $$rptdir
report_datasheet -file [file join $$rptdir datasheet.txt]
set rptutil [file join $$rptdir utilization.txt]
report_utilization -hierarchical -file $$rptutil
report_clock_utilization -file $$rptutil -append
report_ram_utilization -file $$rptutil -append -detail
report_timing_summary -file [file join $$rptdir timing.txt] -max_paths 10
report_high_fanout_nets -file [file join $$rptdir fanout.txt] -timing -load_types -max_nets 25

# Run DRC
report_drc -file [file join $$rptdir drc.txt]

# Report details of the IO banks in the design
report_io -file [file join $$rptdir io.txt]

# Report a table of all clocks in the design
report_clocks -file [file join $$rptdir clocks.txt]

# Fail loudly if timing not met
#
# We would ideally elevate critical warning Route 35-39 to an error, but it is
# currently not being emitted with our flow for some reason.
# https://forums.xilinx.com/t5/Implementation/Making-timing-violations-fatal-to-the-Vivado-build/m-p/716957#M15979
set timing_slack [get_property SLACK [get_timing_paths]]
if {$$timing_slack < 0} {
  puts "Failed to meet timing by $$timing_slack, see [file join $$rptdir timing.txt]"
  exit 1
}
"""
    )
  }

  /** Finds source files within a given source directory with the given file
    * extensions.
    */
  def getSourceFiles(
      sourceDir: Path,
      fileExtensions: Seq[String] = Seq(".v", ".sv", ".cc", ".vams")
  ): Seq[Path] = {
    os
      .walk(sourceDir)
      .filter(os.isFile)
      .filter(path => fileExtensions.exists(ext => path.last.endsWith(ext)))
  }

  def genBitstream(
      workDir: Path,
      module: => RawModule
  ) = {
    os.makeDir.all(workDir)
    val sourceDir = workDir / "src"
    val objDir = workDir / "obj"
    val artifactsDir = workDir / "artifacts"
    os.makeDir.all(objDir)
    os.makeDir.all(artifactsDir)

    os.remove.all(sourceDir)
    val design = ChiselStage
      .emitSystemVerilogFile(
        module,
        args = Array(
          "--target-dir",
          sourceDir.toString
        )
      )
      .collectFirst { case a: DesignAnnotation[_] => a.design }
      .get
    freechips.rocketchip.util.ElaborationArtefacts.files.foreach {
      case (extension, contents) =>
        os.write.over(artifactsDir / s"KodiakBringup.${extension}", contents())
    }

    val sourceFiles = getSourceFiles(sourceDir) ++ Seq(
      root / os.up / "fpga" / "stac2_bringup.xdc",
      root / os.up / "fpga" / "stac2_bringup.sdc"
    )
    val sourceFilesList = objDir / "sourceFiles.F"
    val genBistreamTcl = objDir / "gen_bitstream.tcl"

    writeSourceFilesList(sourceFilesList, sourceFiles)

    writeGenBitstreamTcl(
      genBistreamTcl,
      objDir,
      sourceFilesList,
      "Stac2BringupTop",
      incDirs = os.walk(sourceDir).filter(os.isDir) ++ Seq(sourceDir)
    )

    os.proc(
      "vivado",
      "-nojournal",
      "-mode",
      "batch",
      "-source",
      genBistreamTcl
    ).call(stdout = os.Inherit, stderr = os.Inherit, cwd = objDir)
  }
}
