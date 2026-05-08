#!/usr/bin/env python3
"""Plot shmoo plots from sram*_shmoo.json files produced by shmootest.rs."""

import argparse
import json
from pathlib import Path

import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import ListedColormap

PASS_COLOR = "#2ca02c"
SRAM_FAIL_COLOR = "#d62728"
BIST_FAIL_COLOR = "#ff7f0e"
INTF_FAIL_COLOR = "#aaaaaa"

# Maps result string → integer code used in the grid
RESULT_CODE = {"IntfFail": 0, "BistFail": 1, "SramFail": 2, "Pass": 3}
CMAP = ListedColormap([INTF_FAIL_COLOR, BIST_FAIL_COLOR, SRAM_FAIL_COLOR, PASS_COLOR])

# (depth, width) for each SRAM ID, from rs/src/tests.rs SRAM_SIZES
SRAM_SIZES = [
    (64, 24), (64, 32), (128, 16), (128, 24), (128, 32),
    (256, 8), (256, 16), (256, 32), (256, 64), (256, 128),
    (512, 8), (512, 32), (512, 64), (512, 128),
    (1024, 8), (1024, 32), (1024, 64),
    (2048, 8), (2048, 32),
    (4096, 8), (4096, 32),
    (8192, 32),
]


def load_shmoo(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def build_grid(data: dict):
    """Return (grid, freqs, vdds) or (None, None, None) if no points exist.

    grid[vdd_row, freq_col]: 0=IntfFail, 1=BistFail, 2=SramFail, 3=Pass
    Axes are ordered low→high for both VDD and frequency.
    All-IntfFail rows/cols are trimmed from each edge, keeping at most one.
    """
    points = data["points"]
    if not points:
        return None, None, None

    freqs = sorted({p["clock_freq_hz"] for p in points})
    vdds = sorted({p["vdd_set_v"] for p in points})
    freq_idx = {f: i for i, f in enumerate(freqs)}
    vdd_idx = {v: i for i, v in enumerate(vdds)}

    intf = RESULT_CODE["IntfFail"]
    grid = np.full((len(vdds), len(freqs)), intf, dtype=int)
    for p in points:
        grid[vdd_idx[p["vdd_set_v"]], freq_idx[p["clock_freq_hz"]]] = RESULT_CODE[p["result"]]

    # Trim all-IntfFail rows/cols from each edge, leaving at most one.
    active_rows = [i for i in range(grid.shape[0]) if np.any(grid[i] != intf)]
    active_cols = [j for j in range(grid.shape[1]) if np.any(grid[:, j] != intf)]
    if active_rows and active_cols:
        r0 = max(0, min(active_rows) - 1)
        r1 = min(grid.shape[0], max(active_rows) + 2)
        c0 = max(0, min(active_cols) - 1)
        c1 = min(grid.shape[1], max(active_cols) + 2)
        grid = grid[r0:r1, c0:c1]
        vdds = vdds[r0:r1]
        freqs = freqs[c0:c1]

    return grid, freqs, vdds


def plot_shmoo(data: dict, ax: plt.Axes) -> None:
    sram_id = data["sram_id"]
    grid, freqs, vdds = build_grid(data)

    if sram_id < len(SRAM_SIZES):
        depth, width = SRAM_SIZES[sram_id]
        size_str = f"{depth}×{width}b"
    else:
        size_str = "?"
    ax.set_title(f"SRAM {sram_id} ({size_str})", fontsize=9)

    if grid is None:
        ax.text(0.5, 0.5, "No data", ha="center", va="center", transform=ax.transAxes)
        return

    ax.imshow(
        grid,
        origin="lower",
        cmap=CMAP,
        vmin=0,
        vmax=3,
        aspect="auto",
        interpolation="nearest",
    )

    ax.set_xticks(range(len(freqs)))
    ax.set_xticklabels(
        [f"{f / 1e6:.0f}" for f in freqs], rotation=45, ha="right", fontsize=7
    )
    ax.set_yticks(range(len(vdds)))
    ax.set_yticklabels([f"{v:.2f}" for v in vdds], fontsize=7)
    ax.set_xlabel("Clock (MHz)", fontsize=8)
    ax.set_ylabel("VDD (V)", fontsize=8)

    legend_handles = [
        mpatches.Patch(color=PASS_COLOR, label="Pass"),
        mpatches.Patch(color=SRAM_FAIL_COLOR, label="SRAM fail"),
        mpatches.Patch(color=BIST_FAIL_COLOR, label="BIST fail"),
        mpatches.Patch(color=INTF_FAIL_COLOR, label="Intf fail"),
    ]
    ax.legend(handles=legend_handles, fontsize=6, loc="lower right")


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Plot shmoo plots from sram*_shmoo.json files."
    )
    parser.add_argument("dir", help="Directory containing sram*_shmoo.json files")
    parser.add_argument(
        "--out", help="Output file (e.g. shmoo.png). Shows interactively if omitted."
    )
    args = parser.parse_args()

    def sram_id_from_path(p: Path) -> int:
        import re
        m = re.search(r"sram(\d+)_shmoo", p.name)
        return int(m.group(1)) if m else -1

    paths = sorted(Path(args.dir).glob("sram*_shmoo.json"), key=sram_id_from_path)
    if not paths:
        print(f"No sram*_shmoo.json files found in {args.dir}")
        return

    n = len(paths)
    ncols = min(4, n)
    nrows = (n + ncols - 1) // ncols

    fig, axes = plt.subplots(
        nrows, ncols, figsize=(4 * ncols, 4 * nrows + 0.5), squeeze=False
    )
    fig.suptitle("SRAM Shmoo Plots", fontsize=13, fontweight="bold", y=1.0)

    for i, path in enumerate(paths):
        plot_shmoo(load_shmoo(path), axes[i // ncols][i % ncols])

    for i in range(n, nrows * ncols):
        axes[i // ncols][i % ncols].set_visible(False)

    fig.tight_layout()

    if args.out:
        fig.savefig(args.out, bbox_inches="tight")
        print(f"Saved to {args.out}")
    else:
        plt.show()


if __name__ == "__main__":
    main()
