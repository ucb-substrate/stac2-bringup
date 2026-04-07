# STAC2 Bringup

## Setup

First, ensure that the config in `Stac.toml` is up to date.

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

```
:add --path rs
use stac2::*;
```

You should now be able to run bringup commands such as `bebe_write`, `bebe_read`, `tsi_write`, and `tsi_read`.
