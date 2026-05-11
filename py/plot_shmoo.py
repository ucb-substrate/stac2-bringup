#!/usr/bin/env python3
"""Plot shmoo plots from sram*_shmoo.json files produced by shmootest.rs."""

import argparse
import json
import re
from pathlib import Path

import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
import numpy as np
from matplotlib.colors import ListedColormap

# Maps result string → integer code used in the grid (0=worst, 3=best)
RESULT_CODE = {"IntfFail": 0, "BistFail": 1, "SramFail": 2, "Pass": 3}

# Colors ordered by result code: 0=IntfFail, 1=BistFail, 2=SramFail, 3=Pass
_COLORS = ["#d62728", "#ff7f0e", "#ffdd57", "#2ca02c"]
CMAP = ListedColormap(_COLORS)

LEGEND_HANDLES = [
    mpatches.Patch(color=_COLORS[3], label="Pass"),
    mpatches.Patch(color=_COLORS[2], label="BIST failure"),
    mpatches.Patch(color=_COLORS[1], label="BIST timeout"),
    mpatches.Patch(color=_COLORS[0], label="Intf timeout"),
]

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


def _trim_bounds(agg_grid):
    """Return (r0, r1, c0, c1) trim bounds based on aggregate grid."""
    intf = RESULT_CODE["IntfFail"]
    active_rows = [i for i in range(agg_grid.shape[0]) if np.any(agg_grid[i] != intf)]
    active_cols = [j for j in range(agg_grid.shape[1]) if np.any(agg_grid[:, j] != intf)]
    if not active_rows or not active_cols:
        return 0, agg_grid.shape[0], 0, agg_grid.shape[1]
    r0 = max(0, min(active_rows) - 1)
    r1 = min(agg_grid.shape[0], max(active_rows) + 2)
    c0 = max(0, min(active_cols) - 1)
    c1 = min(agg_grid.shape[1], max(active_cols) + 2)
    return r0, r1, c0, c1


def build_grids(data: dict) -> tuple[list[str], dict, list, list]:
    """Build per-test grids and an aggregate grid, all sharing the same trim window.

    Returns (test_names, grids, freqs, vdds) where grids maps test name (and "All")
    to a 2-D numpy array. test_names lists the individual tests in sorted order.
    """
    points = data["points"]
    if not points:
        return [], {}, [], []

    tests = sorted({p["test"] for p in points})
    freqs = sorted({p["clock_freq_hz"] for p in points})
    vdds = sorted({p["vdd_set_v"] for p in points})
    freq_idx = {f: i for i, f in enumerate(freqs)}
    vdd_idx = {v: i for i, v in enumerate(vdds)}
    intf = RESULT_CODE["IntfFail"]

    grids = {}
    for test in tests:
        g = np.full((len(vdds), len(freqs)), intf, dtype=int)
        for p in points:
            if p["test"] == test:
                g[vdd_idx[p["vdd_set_v"]], freq_idx[p["clock_freq_hz"]]] = RESULT_CODE[p["result"]]
        grids[test] = g

    # Aggregate: minimum result code across all tests (Pass only if all pass).
    agg = np.full((len(vdds), len(freqs)), RESULT_CODE["Pass"], dtype=int)
    for g in grids.values():
        np.minimum(agg, g, out=agg)
    grids["All"] = agg

    # Trim all grids using the same window derived from the aggregate.
    r0, r1, c0, c1 = _trim_bounds(agg)
    trimmed = {name: g[r0:r1, c0:c1] for name, g in grids.items()}
    return tests, trimmed, freqs[c0:c1], vdds[r0:r1]


def build_single_grid(data: dict) -> tuple:
    """Build a single grid (no per-test split) for backward-compatible single-test data."""
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

    r0, r1, c0, c1 = _trim_bounds(grid)
    return grid[r0:r1, c0:c1], freqs[c0:c1], vdds[r0:r1]


def _render_grid(grid, freqs, vdds, title: str, ax: plt.Axes) -> None:
    ax.set_title(title, fontsize=8)
    if grid is None or grid.size == 0:
        ax.text(0.5, 0.5, "No data", ha="center", va="center", transform=ax.transAxes)
        ax.set_visible(True)
        return

    ax.imshow(grid, origin="lower", cmap=CMAP, vmin=0, vmax=3,
              aspect="auto", interpolation="nearest")
    ax.set_xticks(range(len(freqs)))
    ax.set_xticklabels([f"{f / 1e6:.0f}" for f in freqs],
                       rotation=45, ha="right", fontsize=6)
    ax.set_yticks(range(len(vdds)))
    ax.set_yticklabels([f"{v:.2f}" for v in vdds], fontsize=6)
    ax.set_xlabel("Clock (MHz)", fontsize=7)
    ax.set_ylabel("VDD (V)", fontsize=7)
    ax.legend(handles=LEGEND_HANDLES, fontsize=5, loc="lower right")


def _sram_label(sram_id: int) -> str:
    if sram_id < len(SRAM_SIZES):
        depth, width = SRAM_SIZES[sram_id]
        return f"SRAM {sram_id} ({depth}×{width}b)"
    return f"SRAM {sram_id}"


def _sram_id_from_path(p: Path) -> int:
    m = re.search(r"sram(\d+)_shmoo", p.name)
    return int(m.group(1)) if m else -1


def _has_per_test(data: dict) -> bool:
    return any("test" in p for p in data.get("points", []))


def main() -> None:
    parser = argparse.ArgumentParser(
        description="Plot shmoo plots from sram*_shmoo.json files."
    )
    parser.add_argument("dir", help="Directory containing sram*_shmoo.json files")
    parser.add_argument(
        "--out", help="Output file (e.g. shmoo.svg). Shows interactively if omitted."
    )
    args = parser.parse_args()

    paths = sorted(Path(args.dir).glob("sram*_shmoo.json"), key=_sram_id_from_path)
    if not paths:
        print(f"No sram*_shmoo.json files found in {args.dir}")
        return

    datasets = [load_shmoo(p) for p in paths]
    per_test = _has_per_test(datasets[0]) if datasets else False

    if per_test:
        # Determine column layout: individual tests (sorted) + "All"
        all_tests = sorted({p["test"] for d in datasets for p in d["points"]})
        col_names = all_tests + ["All"]
        ncols = len(col_names)
        nrows = len(datasets)

        fig, axes = plt.subplots(
            nrows, ncols,
            figsize=(3 * ncols, 3 * nrows + 0.5),
            squeeze=False,
        )
        fig.suptitle("SRAM Shmoo Plots", fontsize=13, fontweight="bold", y=1.0)

        for row, data in enumerate(datasets):
            sram_id = data["sram_id"]
            label = _sram_label(sram_id)
            tests, grids, freqs, vdds = build_grids(data)
            for col, name in enumerate(col_names):
                grid = grids.get(name)
                _render_grid(grid, freqs, vdds, f"{label}\n{name}", axes[row][col])

    else:
        # Legacy: single result per (vdd, freq) point, no "test" field
        ncols = min(4, len(datasets))
        nrows = (len(datasets) + ncols - 1) // ncols

        fig, axes = plt.subplots(
            nrows, ncols,
            figsize=(4 * ncols, 4 * nrows + 0.5),
            squeeze=False,
        )
        fig.suptitle("SRAM Shmoo Plots", fontsize=13, fontweight="bold", y=1.0)

        for i, data in enumerate(datasets):
            sram_id = data["sram_id"]
            grid, freqs, vdds = build_single_grid(data)
            _render_grid(grid, freqs, vdds, _sram_label(sram_id),
                         axes[i // ncols][i % ncols])

        for i in range(len(datasets), nrows * ncols):
            axes[i // ncols][i % ncols].set_visible(False)

    fig.tight_layout()

    if args.out:
        fig.savefig(args.out, bbox_inches="tight")
        print(f"Saved to {args.out}")
    else:
        plt.show()


if __name__ == "__main__":
    main()
