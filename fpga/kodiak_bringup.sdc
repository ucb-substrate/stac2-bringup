# ------------------------- Base Clocks --------------------
create_clock -name io_clock -period 10.0 [get_ports {io_clock}]
set_input_jitter io_clock 0.5
create_clock -name serial_tl_0_clock -period 40.0 [get_ports {serial_tl_0_clock_out}]
set_input_jitter  serial_tl_0_clock 0.5
create_clock -name serial_tl_1_clock -period 40.0 [get_ports {serial_tl_1_clock_out}]
set_input_jitter  serial_tl_1_clock 0.5
# ------------------------- Clock Groups -------------------
set_clock_groups -asynchronous \
  -group [list [get_clocks { \
      clk_pll_i \
    }] [get_clocks -of_objects [get_pins { \
      mig/island/blackbox/ui_clk \
    }]]] \
  -group [list [get_clocks -of_objects [get_pins { \
      harnessSysPLLNode/clk_out2 \
      harnessSysPLLNode/clk_out3 \
    }]]] \
  -group [list [get_clocks -of_objects [get_pins { \
      harnessSysPLLNode/clk_out1 \
    }]]] \
  -group [list [get_clocks -of_objects [get_ports { \
      serial_tl_0_clock_out \
      serial_tl_1_clock_out \
    }]]]
# ------------------------- False Paths --------------------
set_false_path -through [get_pins {powerOnReset_fpga_power_on/power_on_reset}]
# ------------------------- IO Timings ---------------------
