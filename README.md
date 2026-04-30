# STAC2 Bringup

## Setup

Requirements:
- [openFPGALoader](https://trabucayre.github.io/openFPGALoader/guide/install.html)
- [Rust](https://rust-lang.org/tools/install/)
- [uv](https://docs.astral.sh/uv/getting-started/installation/#installation-methods)
- [evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md)

Ensure that the STAC board and Arty100T FPGA are connected via PMODs and that the FPGA has been flashed with the correct bitstream:

```bash
openFPGALoader -b arty_a7_100t --write-flash --verify --reset Stac2BringupTop.bit
```

Connect the UART of the board and FPGA to a host computer and note their respective serial ports.
Update the config in `Stac.toml` with these serial ports.

Start an `excvr` repl from the root of this repo:

```bash
excvr
```

In the `irust` repl, run the following commands:

```rs
:dep stac2 = { path = "rs" }
use stac2::*;
let mut l = BringupState::new();
```

Try initializing the chip:

```rs
l.init_chip();
```

You should see the following output. If you don't, rerun `l.init_chip()`.

```bash
[bebe host] Waiting for DUT...
[bebe host] DUT found!
[bebe host] Trying to nock...
47 4F 42 45 41 52 53 21                           GOBEARS!

[bebe host] Connected to DUT!
()
```

You should then be able to write and read scratchpad memory on chip:

```rs
l.tsi_intf().write(SCRATCHPAD_BASE_ADDR, 0xdeadbeef);
l.tsi_intf().read(SCRATCHPAD_BASE_ADDR)
```

Available tests can be found in `rs/tests.rs`. To run the MATS+ test on SRAM 0:

```rs
l.mats_plus_tsi_test_sram(0)
```


## Development

### Organization

- `fpga/` - FPGA collateral and scripts.

- `py/` - A Python environment for bringup protocols such as `bebe_host` and `pyuartsi`.

- `rs/` - A Rust crate with code for exercising the chip.

- `scala/` A Scala package with code for generating the FPGA bitstream.

- `Stac.toml` - Bringup configuration.

### Writing chip tests

Available chip registers are located in `rs/memory.rs`. Available FPGA registers are located in `rs/tsi.rs`.
Chip tests should be declared in `rs/tests.rs` as methods on `BringupState`. Reference `BringupState::march_cm_rand_tsi_test_sram_all`.

### Updating the FPGA bitstream

To modify FPGA MMIO registers, update `scala/Controller.scala`.

To update top level RTL (e.g. top level IOs), update `scala/Stac2Bringup.scala`.
If you modify top level IOs, you will also have to modify `fpga/stac2_bringup.xdc`.
If you modify clocks, you will have to modify `fpga/stac2_bringup.sdc`.

To regenerate the bitstream, run the following from the `scala/` directory with `vivado` on PATH:

```bash
./mill test.testOnly "*" -- -z bitstream
```

The generated bitstream will be located at `scala/build/Stac2Bringup_should_generate_Arty100T_bitstream/obj/Stac2Bringup.bit`.
If it will be used repeatedly, copy it to the `fpga/` folder. Use a new name unless the bitstream is a strict feature superset of the existing `Stac2Bringup.bit`.
