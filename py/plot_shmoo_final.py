#!/usr/bin/env python3
"""Final figure: aggregate shmoo plots for four representative SRAMs."""

import argparse
import json
from pathlib import Path

import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
import numpy as np
from matplotlib import rcParams
from matplotlib.colors import ListedColormap

rcParams["font.family"] = "Arial"

SRAMS = [
    (7,  "sram22_256x32m4w8"),
    (12, "sram22_512x64m4w8"),
    (15, "sram22_1024x32m8w8"),
    (20, "sram22_4096x32m8w8"),
]

PASS_CODE = 1
FAIL_CODE = 0
CMAP = ListedColormap(["#d62728", "#2ca02c"])  # 0=fail (red), 1=pass (green)
LEGEND_HANDLES = [
    mpatches.Patch(color="#2ca02c", label="Pass"),
    mpatches.Patch(color="#d62728", label="Fail"),
]

RESULT_CODE = {"IntfTimeout": 0, "BistTimeout": 0, "BistFail": 0, "Pass": 1}


def load(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def build_agg_grid(data: dict):
    """Return (grid, freqs, vdds) aggregate pass/fail grid, trimmed."""
    points = data["points"]
    if not points:
        return None, None, None

    freqs = sorted({p["clock_freq_hz"] for p in points})
    vdds  = sorted({p["vdd_set_v"]     for p in points})
    freq_idx = {f: i for i, f in enumerate(freqs)}
    vdd_idx  = {v: i for i, v in enumerate(vdds)}

    tests = sorted({p["test"] for p in points})
    # per-test grids, then take minimum (pass only if all pass)
    grids = []
    for test in tests:
        g = np.zeros((len(vdds), len(freqs)), dtype=int)
        for p in points:
            if p["test"] == test:
                g[vdd_idx[p["vdd_set_v"]], freq_idx[p["clock_freq_hz"]]] = RESULT_CODE[p["result"]]
        grids.append(g)

    agg = grids[0].copy()
    for g in grids[1:]:
        np.minimum(agg, g, out=agg)

    return agg, freqs, vdds


def render(ax, grid, freqs, vdds, title):
    ax.set_title(title, fontsize=22, fontweight="bold")
    if grid is None or grid.size == 0:
        ax.text(0.5, 0.5, "No data", ha="center", va="center", transform=ax.transAxes)
        return

    ax.imshow(grid, origin="lower", cmap=CMAP, vmin=0, vmax=1,
              aspect="auto", interpolation="nearest")

    # Show every other tick (data is on a 5 MHz / 0.05 V grid → stride 2 = 10 MHz / 0.1 V)
    x_ticks = range(0, len(freqs), 2)
    y_ticks = range(0, len(vdds), 2)

    ax.set_xticks(list(x_ticks))
    ax.set_xticklabels([f"{freqs[i] / 1e6:.0f}" for i in x_ticks],
                       rotation=45, ha="right", rotation_mode="anchor", fontsize=18)
    ax.set_yticks(list(y_ticks))
    ax.set_yticklabels([f"{vdds[i]:.2f}" for i in y_ticks], fontsize=18)
    ax.set_xlabel("Frequency (MHz)", fontsize=20, labelpad=2)
    ax.set_ylabel("VDD (V)", fontsize=20, labelpad=2)
    ax.legend(handles=LEGEND_HANDLES, fontsize=18, loc="lower right")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("dir", help="Directory containing sram*_shmoo.json files")
    parser.add_argument("--out", help="Output file (e.g. final.svg). Interactive if omitted.")
    args = parser.parse_args()

    datadir = Path(args.dir)
    fig, axes = plt.subplots(2, 2, figsize=(10, 8), squeeze=False)


    for idx, (sram_id, name) in enumerate(SRAMS):
        path = datadir / f"sram{sram_id}_shmoo.json"
        ax = axes[idx // 2][idx % 2]
        if not path.exists():
            ax.set_title(name, fontsize=9)
            ax.text(0.5, 0.5, f"Missing: {path.name}", ha="center", va="center",
                    transform=ax.transAxes, color="red")
            continue
        data = load(path)
        grid, freqs, vdds = build_agg_grid(data)
        render(ax, grid, freqs, vdds, name)

    fig.tight_layout()

    if args.out:
        fig.savefig(args.out, bbox_inches="tight")
        print(f"Saved to {args.out}")
    else:
        plt.show()


if __name__ == "__main__":
    main()
