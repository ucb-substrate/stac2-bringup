### This file is a general .xdc for the Nexys Video Rev. A
### To use it in a project:
### - uncomment the lines corresponding to used pins
### - rename the used ports (in each line, after get_ports) according to the top level signal names in the project


# Clock Signal
set_property -dict { PACKAGE_PIN R4    IOSTANDARD LVCMOS33 } [get_ports { io_clock }]; #IO_L13P_T2_MRCC_34 Sch=sysclk


# LEDs
set_property -dict { PACKAGE_PIN T14   IOSTANDARD LVCMOS25 } [get_ports { led_0 }]; #IO_L15P_T2_DQS_13 Sch=led[0]
set_property -dict { PACKAGE_PIN T15   IOSTANDARD LVCMOS25 } [get_ports { led_1 }]; #IO_L15N_T2_DQS_13 Sch=led[1]
set_property -dict { PACKAGE_PIN T16   IOSTANDARD LVCMOS25 } [get_ports { led_2 }]; #IO_L17P_T2_13 Sch=led[2]
set_property -dict { PACKAGE_PIN U16   IOSTANDARD LVCMOS25 } [get_ports { led_3 }]; #IO_L17N_T2_13 Sch=led[3]
set_property -dict { PACKAGE_PIN V15   IOSTANDARD LVCMOS25 } [get_ports { led_4 }]; #IO_L14N_T2_SRCC_13 Sch=led[4]
set_property -dict { PACKAGE_PIN W16   IOSTANDARD LVCMOS25 } [get_ports { led_5 }]; #IO_L16N_T2_13 Sch=led[5]
set_property -dict { PACKAGE_PIN W15   IOSTANDARD LVCMOS25 } [get_ports { led_6 }]; #IO_L16P_T2_13 Sch=led[6]
set_property -dict { PACKAGE_PIN Y13   IOSTANDARD LVCMOS25 } [get_ports { led_7 }]; #IO_L5P_T0_13 Sch=led[7]


## Buttons
# set_property -dict { PACKAGE_PIN B22 IOSTANDARD LVCMOS12 } [get_ports { btnc }]; #IO_L20N_T3_16 Sch=btnc
# set_property -dict { PACKAGE_PIN D22 IOSTANDARD LVCMOS12 } [get_ports { btnd }]; #IO_L22N_T3_16 Sch=btnd
set_property -dict { PACKAGE_PIN C22 IOSTANDARD LVCMOS12 } [get_ports { reset_u_0_btn }]; #IO_L20P_T3_16 Sch=btnl
set_property -dict { PACKAGE_PIN D14 IOSTANDARD LVCMOS12 } [get_ports { reset_u_1_btn }]; #IO_L6P_T0_16 Sch=btnr
# set_property -dict { PACKAGE_PIN F15 IOSTANDARD LVCMOS12 } [get_ports { btnu }]; #IO_0_16 Sch=btnu
set_property -dict { PACKAGE_PIN G4  IOSTANDARD LVCMOS15 } [get_ports { io_reset }]; #IO_L12N_T1_MRCC_35 Sch=cpu_resetn


## Pmod header JB
set_property -dict { PACKAGE_PIN V9    IOSTANDARD LVCMOS33 } [get_ports { uart_1_ctsn }]; #IO_L21P_T3_DQS_34 Sch=jb_p[1]
set_property -dict { PACKAGE_PIN V8    IOSTANDARD LVCMOS33 } [get_ports { uart_1_rxd }]; #IO_L21N_T3_DQS_34 Sch=jb_n[1]
set_property -dict { PACKAGE_PIN V7    IOSTANDARD LVCMOS33 } [get_ports { uart_1_txd }]; #IO_L19P_T3_34 Sch=jb_p[2]
set_property -dict { PACKAGE_PIN W7    IOSTANDARD LVCMOS33 } [get_ports { uart_1_rtsn }]; #IO_L19N_T3_VREF_34 Sch=jb_n[2]


# UART
set_property -dict { PACKAGE_PIN AA19  IOSTANDARD LVCMOS33 } [get_ports { uart_0_txd }]; #IO_L15P_T2_DQS_RDWR_B_14 Sch=uart_rx_out
set_property -dict { PACKAGE_PIN V18   IOSTANDARD LVCMOS33 } [get_ports { uart_0_rxd }]; #IO_L14P_T2_SRCC_14 Sch=uart_tx_in


# FMC
set_property -dict { PACKAGE_PIN K19   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_clock_out }]; #IO_L13N_T2_MRCC_15 Sch=fmc_la00_cc_n
set_property -dict { PACKAGE_PIN K18   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_clock_out }]; #IO_L13P_T2_MRCC_15 Sch=fmc_la00_cc_p
set_property -dict { PACKAGE_PIN L18   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_out_ready }]; #IO_L16N_T2_A27_15 Sch=fmc_la_n[02]
set_property -dict { PACKAGE_PIN M18   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_out_valid }]; #IO_L16P_T2_A28_15 Sch=fmc_la_p[02]
set_property -dict { PACKAGE_PIN N19   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_in_bits_phit }]; #IO_L17N_T2_A25_15 Sch=fmc_la_n[03]
set_property -dict { PACKAGE_PIN N18   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_out_bits_phit }]; #IO_L17P_T2_A26_15 Sch=fmc_la_p[03]
set_property -dict { PACKAGE_PIN M20   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_in_ready }]; #IO_L18N_T2_A23_15 Sch=fmc_la_n[04]
set_property -dict { PACKAGE_PIN N20   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_0_in_valid }]; #IO_L18P_T2_A24_15 Sch=fmc_la_p[04]
set_property -dict { PACKAGE_PIN L13   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_in_bits_phit }]; #IO_L20N_T3_A19_15 Sch=fmc_la_n[07]
set_property -dict { PACKAGE_PIN M13   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_out_bits_phit }]; #IO_L20P_T3_A20_15 Sch=fmc_la_p[07]
set_property -dict { PACKAGE_PIN M16   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_out_ready }]; #IO_L24N_T3_RS0_15 Sch=fmc_la_n[08]
set_property -dict { PACKAGE_PIN M15   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_out_valid }]; #IO_L24P_T3_RS1_15 Sch=fmc_la_p[08]
set_property -dict { PACKAGE_PIN L20   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_in_ready }]; #IO_L14N_T2_SRCC_15 Sch=fmc_la_n[12]
set_property -dict { PACKAGE_PIN L19   IOSTANDARD LVCMOS12 } [get_ports { serial_tl_1_in_valid }]; #IO_L14P_T2_SRCC_15 Sch=fmc_la_p[12]
set_property -dict { PACKAGE_PIN A19   IOSTANDARD LVCMOS12 } [get_ports { reset_u_1_chip_rst }]; #IO_L17N_T2_16 Sch=fmc_la_n[19]
set_property -dict { PACKAGE_PIN A18   IOSTANDARD LVCMOS12 } [get_ports { reset_u_0_chip_rst }]; #IO_L17P_T2_16 Sch=fmc_la_p[19]

# UART-TSI Connections
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {uart_0_rxd}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {uart_0_txd}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {uart_1_rxd}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {uart_1_txd}]]]

# Serial-TL Chip 0
set_property CLOCK_DEDICATED_ROUTE {FALSE} [get_nets [get_ports {serial_tl_0_clock_out}]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_0_out_valid}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_0_out_ready}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_0_in_valid}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_0_in_ready}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_0_out_bits_phit}]]]
#set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_0_out_bits_phit[1]}]]]
#set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_0_out_bits_phit[2]}]]]
#set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_0_out_bits_phit[3]}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_0_in_bits_phit}]]]
#set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_0_in_bits_phit[1]}]]]
#set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_0_in_bits_phit[2]}]]]
#set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_0_in_bits_phit[3]}]]]


# Serial-TL Chip 1
set_property CLOCK_DEDICATED_ROUTE {FALSE} [get_nets [get_ports {serial_tl_1_clock_out}]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_1_in_valid}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_1_in_ready}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_1_out_valid}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_1_out_ready}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanin -flat -startpoints_only [get_ports {serial_tl_1_in_bits_phit}]]]
set_property IOB {TRUE} [ get_cells -of_objects [ all_fanout -flat -endpoints_only [get_ports {serial_tl_1_out_bits_phit}]]]


## Configuration options, can be used for all designs
set_property CONFIG_VOLTAGE 3.3 [current_design]
set_property CFGBVS VCCO [current_design]
