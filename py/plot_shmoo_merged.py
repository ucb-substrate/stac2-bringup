#!/usr/bin/env python3
"""Merged shmoo plot combining shmoo_vmin (low-freq) and shmoo_all_patterns (high-freq).

The x-axis is broken at the boundary between the two frequency ranges. The y-axis
(voltage) spans the union of voltage points across both datasets; voltage points
that are absent from shmoo_all_patterns are rendered as failures on the high-freq
side.
"""

import argparse
import json
import re
from pathlib import Path

import matplotlib.patches as mpatches
import matplotlib.pyplot as plt
import numpy as np
from matplotlib import rcParams
from matplotlib.colors import ListedColormap
from matplotlib.ticker import FixedLocator, MultipleLocator

rcParams["font.family"] = "Arial"

# Publication-scale font sizes.
FS_SUPTITLE = 20
FS_TITLE = 17
FS_LABEL = 15
FS_TICK = 13
FS_LEGEND = 13

PASS = 1
FAIL = 0
CMAP = ListedColormap(["#d62728", "#2ca02c"])  # 0=fail (red), 1=pass (green)
LEGEND_HANDLES = [
    mpatches.Patch(color="#2ca02c", label="Pass"),
    mpatches.Patch(color="#d62728", label="Fail"),
]

# (depth, width) per SRAM id, mirrored from rs/src/tests.rs SRAM_SIZES.
SRAM_SIZES = [
    (64, 24), (64, 32), (128, 16), (128, 24), (128, 32),
    (256, 8), (256, 16), (256, 32), (256, 64), (256, 128),
    (512, 8), (512, 32), (512, 64), (512, 128),
    (1024, 8), (1024, 32), (1024, 64),
    (2048, 8), (2048, 32),
    (4096, 8), (4096, 32),
    (8192, 32),
]


def load(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def _sram_label(sram_id: int) -> str:
    if 0 <= sram_id < len(SRAM_SIZES):
        depth, width = SRAM_SIZES[sram_id]
        return f"SRAM {sram_id} ({depth}×{width}b)"
    return f"SRAM {sram_id}"


def _sid_from_path(p: Path) -> int:
    m = re.search(r"sram(\d+)_shmoo", p.name)
    return int(m.group(1)) if m else -1


def _fmt_mhz(hz: float) -> str:
    mhz = hz / 1e6
    return f"{mhz:.0f}" if mhz >= 10 else f"{mhz:.2f}"


def _qv(v: float) -> float:
    """Canonicalize a voltage to the 50 mV sweep step.

    The two sweeps record voltages with different float drift (vmin tends to
    accumulate 0.05 V steps producing values like 1.4500000000000002, while
    all_patterns uses clean values like 1.45). Without rounding, set-union
    treats these as distinct rows and renders the un-populated one as a red
    band.
    """
    return round(v * 20) / 20


def _qf(f: float) -> float:
    """Canonicalize a frequency to 1 Hz."""
    return round(f)


def _aggregate(points, freqs, vdds):
    """Per-test minimum aggregation: cell passes only if every test passes there.

    Cells with no data (e.g. voltages absent from the source dataset) stay FAIL.
    Voltages and frequencies are canonicalized to the sweep grid to avoid
    floating-point drift between datasets.
    """
    freq_idx = {f: i for i, f in enumerate(freqs)}
    vdd_idx = {v: i for i, v in enumerate(vdds)}
    tests = sorted({p["test"] for p in points})
    if not tests or not freqs or not vdds:
        return np.full((len(vdds), len(freqs)), FAIL, dtype=int)
    grids = []
    for test in tests:
        g = np.full((len(vdds), len(freqs)), FAIL, dtype=int)
        for p in points:
            if p["test"] != test:
                continue
            v = _qv(p["vdd_set_v"])
            f = _qf(p["clock_freq_hz"])
            if v in vdd_idx and f in freq_idx:
                g[vdd_idx[v], freq_idx[f]] = PASS if p["result"] == "Pass" else FAIL
        grids.append(g)
    agg = grids[0].copy()
    for g in grids[1:]:
        np.minimum(agg, g, out=agg)
    return agg


def build_merged(vmin_data: dict, all_data: dict):
    """Return (vmin_grid, all_grid, vmin_freqs, all_freqs, vdds).

    The two grids share the same row order (union of voltages). Voltages not
    present in `all_data` produce all-FAIL rows on the high-freq side.
    """
    vmin_points = vmin_data.get("points", [])
    all_points = all_data.get("points", [])

    vmin_freqs = sorted({_qf(p["clock_freq_hz"]) for p in vmin_points})
    all_freqs = sorted({_qf(p["clock_freq_hz"]) for p in all_points})
    vdds = sorted(
        {_qv(p["vdd_set_v"]) for p in vmin_points}
        | {_qv(p["vdd_set_v"]) for p in all_points}
    )

    vmin_grid = _aggregate(vmin_points, vmin_freqs, vdds)
    all_grid = _aggregate(all_points, all_freqs, vdds)
    return vmin_grid, all_grid, vmin_freqs, all_freqs, vdds


def _render_pair(fig, gs_pair, vmin_grid, all_grid, vmin_freqs, all_freqs,
                 vdds, show_ylabel=True):
    ax_l = fig.add_subplot(gs_pair[0])
    ax_r = fig.add_subplot(gs_pair[1], sharey=ax_l)

    # pcolormesh places cells at their actual MHz/V coordinates, so when the
    # gridspec widths are set proportional to the MHz span (in main()), the
    # per-MHz scale is the same on both sides.
    vmin_mhz = np.asarray(vmin_freqs) / 1e6
    all_mhz = np.asarray(all_freqs) / 1e6
    vdd_y = np.asarray(vdds)
    ax_l.pcolormesh(vmin_mhz, vdd_y, vmin_grid, cmap=CMAP, vmin=0, vmax=1,
                    shading="nearest")
    ax_r.pcolormesh(all_mhz, vdd_y, all_grid, cmap=CMAP, vmin=0, vmax=1,
                    shading="nearest")

    ax_l.xaxis.set_major_locator(MultipleLocator(1))
    ax_r.xaxis.set_major_locator(MultipleLocator(20))
    ax_l.tick_params(axis="x", labelsize=FS_TICK)
    ax_r.tick_params(axis="x", labelsize=FS_TICK)

    ax_l.yaxis.set_major_locator(MultipleLocator(0.1))
    ax_l.tick_params(axis="y", labelsize=FS_TICK)
    ax_r.tick_params(left=False, labelleft=False)

    # Hide the inner spines and draw small diagonal slashes to mark the break.
    ax_l.spines["right"].set_visible(False)
    ax_r.spines["left"].set_visible(False)
    d = 0.012
    kw = dict(transform=ax_l.transAxes, color="k", clip_on=False, lw=1)
    ax_l.plot((1 - d, 1 + d), (-d, +d), **kw)
    ax_l.plot((1 - d, 1 + d), (1 - d, 1 + d), **kw)
    kw["transform"] = ax_r.transAxes
    ax_r.plot((-d, +d), (-d, +d), **kw)
    ax_r.plot((-d, +d), (1 - d, 1 + d), **kw)

    # Dashed vertical lines at the inner edges of each panel to flag the
    # discontinuity inside the plot area itself.
    ax_l.axvline(ax_l.get_xlim()[1], color="0.25", lw=0.8, ls="--")
    ax_r.axvline(ax_r.get_xlim()[0], color="0.25", lw=0.8, ls="--")

    # Single xlabel centered under the much wider all_patterns panel.
    ax_r.set_xlabel("Freq (MHz)", fontsize=FS_LABEL)
    if show_ylabel:
        ax_l.set_ylabel("VDD (V)", fontsize=FS_LABEL)

    ax_r.legend(handles=LEGEND_HANDLES, fontsize=FS_LEGEND, loc="lower right",
                framealpha=0.9)

    return ax_l, ax_r


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vmin-dir", default="data/shmoo_vmin",
                        help="Directory with low-freq vmin sweep JSONs.")
    parser.add_argument("--all-dir", default="data/shmoo_all_patterns",
                        help="Directory with high-freq all-pattern sweep JSONs.")
    parser.add_argument("--out", help="Output file (.png/.svg/.pdf). Interactive if omitted.")
    parser.add_argument("--sram-cols", type=int, default=2,
                        help="Number of SRAM cells per row (default 2).")
    parser.add_argument("--sram-id", type=int, default=None,
                        help="Plot a single SRAM id instead of all.")
    parser.add_argument("--sram-ids", type=str, default=None,
                        help="Comma-separated SRAM ids to plot (e.g. '7,12,15,20').")
    args = parser.parse_args()

    vmin_dir = Path(args.vmin_dir)
    all_dir = Path(args.all_dir)
    vmin_ids = {_sid_from_path(p): p for p in vmin_dir.glob("sram*_shmoo.json")}
    all_ids = {_sid_from_path(p): p for p in all_dir.glob("sram*_shmoo.json")}
    # SRAM 21 (the largest macro) does not work on this die — skip it.
    sram_ids = sorted(s for s in (set(vmin_ids) & set(all_ids)) if s < 21)
    if args.sram_ids is not None:
        wanted = [int(s) for s in args.sram_ids.split(",")]
        sram_ids = [s for s in wanted if s in set(sram_ids)]
    elif args.sram_id is not None:
        sram_ids = [s for s in sram_ids if s == args.sram_id]
    if not sram_ids:
        print(f"No matching SRAM JSON files in both {vmin_dir} and {all_dir}.")
        return

    ncols = max(1, min(args.sram_cols, len(sram_ids)))
    nrows = (len(sram_ids) + ncols - 1) // ncols

    # Allocate fixed inch budgets so layout is consistent for a single SRAM and
    # for the full 22-SRAM grid. title_h reserves vertical room for the suptitle
    # plus per-SRAM titles; bot_h leaves room for x-axis labels below the last
    # row.
    cell_h = 3.5
    title_h = 1.0
    bot_h = 1.0
    fig_h = cell_h * nrows + title_h + bot_h
    fig_w = 7.0 * ncols
    fig = plt.figure(figsize=(fig_w, fig_h))
    fig.suptitle("STAC2 SRAM Shmoo: vmin (low-freq) + all-pattern (high-freq)",
                 fontsize=FS_SUPTITLE, fontweight="bold",
                 y=1 - 0.3 * title_h / fig_h)

    outer = fig.add_gridspec(
        nrows, ncols,
        hspace=0.40, wspace=0.10,
        top=1 - title_h / fig_h,
        bottom=bot_h / fig_h,
        left=0.06, right=0.98,
    )

    rendered = []
    for i, sid in enumerate(sram_ids):
        r, c = divmod(i, ncols)
        vmin_data = load(vmin_ids[sid])
        all_data = load(all_ids[sid])
        vg, ag, vf, af, vs = build_merged(vmin_data, all_data)
        # Width ratios are proportional to MHz span (uniform x-axis scale).
        # The vmin span (4 MHz) is stretched 5x relative to all_patterns so the
        # vmin plateau — where reducing freq no longer reduces required VDD —
        # is clearly visible. The MHz numbers still scale linearly within each
        # panel; only the two panels have different MHz-per-inch.
        vmin_span = (max(vf) - min(vf)) / 1e6 if vf else 1.0
        all_span = (max(af) - min(af)) / 1e6 if af else 1.0
        vmin_stretch = 5.0
        inner = outer[r, c].subgridspec(
            1, 2,
            wspace=0.015,
            width_ratios=[vmin_span * vmin_stretch, all_span],
        )
        ax_l, ax_r = _render_pair(fig, inner, vg, ag, vf, af, vs,
                                  show_ylabel=(c == 0))
        rendered.append((sid, ax_l, ax_r))

    # Place per-SRAM titles centered across each broken-axis pair after axes
    # are positioned. Using fig.text in figure coordinates avoids the ax.title /
    # suptitle collision that occurs when both live near y=1.0.
    fig.canvas.draw()
    for sid, ax_l, ax_r in rendered:
        bl = ax_l.get_position()
        br = ax_r.get_position()
        # Center the title across the *combined* panel pair (left edge of ax_l
        # to right edge of ax_r), not across the inner boundary — the panels
        # have very different widths now.
        fig.text((bl.x0 + br.x1) / 2, bl.y1 + 0.005,
                 _sram_label(sid),
                 ha="center", va="bottom",
                 fontsize=FS_TITLE, fontweight="bold")

    if args.out:
        fig.savefig(args.out, dpi=300)
        print(f"Saved to {args.out}")
    else:
        plt.show()


if __name__ == "__main__":
    main()
