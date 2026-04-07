open_hw_manager
connect_hw_server -url localhost:3121
set dev_id 210276B9FE31B
# set dev_id 210276BA0073B
current_hw_target [get_hw_targets */xilinx_tcf/Digilent/$dev_id]
set_property PARAM.FREQUENCY 15000000 [get_hw_targets */xilinx_tcf/Digilent/$dev_id]
open_hw_target
set_property PROGRAM.FILE [lindex $argv 0] [lindex [get_hw_devices] 0]
program_hw_devices [lindex [get_hw_devices] 0]
