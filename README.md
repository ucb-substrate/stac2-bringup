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
- [evcxr](https://github.com/evcxr/evcxr/blob/main/evcxr_repl/README.md)

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
l.bebe_write(SCRATCHPAD_BASE_ADDR, 0xdeadbeef, 8);
l.bebe_read(SCRATCHPAD_BASE_ADDR, 8)
```

This will cause the blue light to turn off, which is expected.
The custom boot ROM spams UART with `A`s until the first operation is transmitted.
