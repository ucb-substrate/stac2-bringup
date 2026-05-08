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
FAIL_COLOR = "#d62728"
ERR_COLOR = "#aaaaaa"


def load_shmoo(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def build_grid(data: dict):
    """Return (grid, freqs, vdds) or (None, None, None) if no Ok points exist.

    grid[vdd_row, freq_col]: -1=init error, 0=fail, 1=pass
    Axes are ordered low→high for both VDD and frequency.
    """
    points = data["points"]
    ok_pts = [p["Ok"] for p in points if "Ok" in p]
    if not ok_pts:
        return None, None, None

    freqs = sorted({p["clock_freq_hz"] for p in ok_pts})
    vdds = sorted({p["vdd_set_v"] for p in ok_pts})
    freq_idx = {f: i for i, f in enumerate(freqs)}
    vdd_idx = {v: i for i, v in enumerate(vdds)}

    grid = np.full((len(vdds), len(freqs)), -1, dtype=int)
    for p in points:
        if "Ok" in p:
            pt = p["Ok"]
            grid[vdd_idx[pt["vdd_set_v"]], freq_idx[pt["clock_freq_hz"]]] = (
                1 if pt["pass"] else 0
            )

    return grid, freqs, vdds


def plot_shmoo(data: dict, ax: plt.Axes) -> None:
    sram_id = data["sram_id"]
    grid, freqs, vdds = build_grid(data)

    ax.set_title(f"SRAM {sram_id}", fontsize=9)

    if grid is None:
        ax.text(0.5, 0.5, "No data", ha="center", va="center", transform=ax.transAxes)
        return

    cmap = ListedColormap([ERR_COLOR, FAIL_COLOR, PASS_COLOR])
    ax.imshow(
        grid + 1,
        origin="lower",
        cmap=cmap,
        vmin=0,
        vmax=2,
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


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Plot shmoo plots from sram*_shmoo.json files."
    )
    parser.add_argument("dir", help="Directory containing sram*_shmoo.json files")
    parser.add_argument(
        "--out", help="Output file (e.g. shmoo.png). Shows interactively if omitted."
    )
    args = parser.parse_args()

    paths = sorted(Path(args.dir).glob("sram*_shmoo.json"))
    if not paths:
        print(f"No sram*_shmoo.json files found in {args.dir}")
        return

    n = len(paths)
    ncols = min(4, n)
    nrows = (n + ncols - 1) // ncols

    fig, axes = plt.subplots(
        nrows, ncols, figsize=(4 * ncols, 4 * nrows), squeeze=False
    )

    for i, path in enumerate(paths):
        plot_shmoo(load_shmoo(path), axes[i // ncols][i % ncols])

    for i in range(n, nrows * ncols):
        axes[i // ncols][i % ncols].set_visible(False)

    legend_handles = [
        mpatches.Patch(color=PASS_COLOR, label="Pass"),
        mpatches.Patch(color=FAIL_COLOR, label="Fail"),
        mpatches.Patch(color=ERR_COLOR, label="Init error"),
    ]
    fig.legend(handles=legend_handles, loc="lower right", fontsize=9)
    fig.suptitle("SRAM Shmoo Plots", fontsize=13, fontweight="bold")
    fig.tight_layout()

    if args.out:
        fig.savefig(args.out, dpi=150, bbox_inches="tight")
        print(f"Saved to {args.out}")
    else:
        plt.show()


if __name__ == "__main__":
    main()
