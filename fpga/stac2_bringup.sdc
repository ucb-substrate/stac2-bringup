# ------------------------- Base Clocks --------------------
create_clock -name io_clock -period 20.0 [get_ports {io_clock}]
set_input_jitter io_clock 0.5
create_clock -name serial_tl_clock -period 10.0 [get_ports {serial_tl_clock_in}]
set_input_jitter  serial_tl_clock 0.5
create_clock -name io_ctl_clk -period 20.0 [get_ports {io_ctl_clk}]
set_input_jitter io_ctl_clk 0.5
# ------------------------- Clock Groups -------------------
set_clock_groups -asynchronous \
  -group [list [get_clocks -of_objects [get_pins { \
      pll/clk_out1 \
    }]]] \
  -group [list [get_clocks -of_objects [get_ports { \
      io_ctl_clk \
    }]]] \
  -group [get_clocks {serial_tl_clock}] 
# ------------------------- False Paths --------------------
set_false_path -through [get_pins {powerOnReset_fpga_power_on/power_on_reset}]
# ------------------------- IO Timings ---------------------
