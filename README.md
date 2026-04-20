# STAC2 Bringup

## Setup

Ensure that the STAC board and Arty100T FPGA are connected via PMODs and that the FPGA has been flashed with the correct bitstream:

```bash
openFPGALoader -b arty_a7_100t --write-flash --verify --reset Stac2BringupTop.bit
```

Connect the UART of the board and FPGA to a host computer and note their respective serial ports.
Update the config in `Stac.toml` with these serial ports.

Requirements:
- [Rust](https://rust-lang.org/tools/install/)
- [uv](https://docs.astral.sh/uv/getting-started/installation/#installation-methods)

Install `irust`:

```bash
cargo install irust
```

Start an `irust` repl from the root of this repo:

```bash
irust
```

In the `irust` repl, run the following commands:

```rs
:add --path rs
use stac2::*;
let mut l = BringupState::new();
```

Try initializing the chip:

```rs
l.init_chip();
```

The blue UART light on the STAC PCB should light up if the chip is functioning correctly.

You should then be able to write and read scratchpad memory on chip:

```rs
l.bebe_write(SCRATCHPAD_BASE_ADDR, 0xdeadbeef, 8);
l.bebe_read(SCRATCHPAD_BASE_ADDR, 8)
```

This will cause the blue light to turn off, which is expected.
The custom boot ROM spams UART with `A`s until the first operation is transmitted.
