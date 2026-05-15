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

# (depth, width, mask_width, mux_ratio) per SRAM id, mirrored from
# rs/src/tests.rs SRAM_SIZES.
SRAM_SIZES = [
    (64, 24, 8, 4), (64, 32, 8, 4), (128, 16, 8, 4), (128, 24, 8, 4),
    (128, 32, 8, 4),
    (256, 8, 1, 8), (256, 16, 8, 8), (256, 32, 8, 4), (256, 64, 8, 4),
    (256, 128, 8, 4),
    (512, 8, 1, 8), (512, 32, 8, 4), (512, 64, 8, 4), (512, 128, 8, 4),
    (1024, 8, 1, 8), (1024, 32, 8, 8), (1024, 64, 8, 4),
    (2048, 8, 1, 8), (2048, 32, 8, 8),
    (4096, 8, 1, 8), (4096, 32, 8, 8),
    (8192, 32, 8, 8),
]


def load(path: Path) -> dict:
    with open(path) as f:
        return json.load(f)


def _sram_label(sram_id: int) -> str:
    if 0 <= sram_id < len(SRAM_SIZES):
        depth, width, mask, mux = SRAM_SIZES[sram_id]
        return f"sram22_{depth}x{width}m{mux}w{mask}"
    return f"sram{sram_id}"


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

    Returns (grid, tested) where `tested[v, f]` is True only if every test
    recorded a result for that (vdd, freq) cell. `grid[v, f]` is FAIL by
    default so untested cells appear failed if the caller doesn't mask them.

    Voltages and frequencies are canonicalized to the sweep grid to avoid
    floating-point drift between datasets.
    """
    freq_idx = {f: i for i, f in enumerate(freqs)}
    vdd_idx = {v: i for i, v in enumerate(vdds)}
    tests = sorted({p["test"] for p in points})
    shape = (len(vdds), len(freqs))
    grid = np.full(shape, FAIL, dtype=int)
    tested = np.zeros(shape, dtype=bool)
    if not tests or not freqs or not vdds:
        return grid, tested
    per_test_grids = []
    per_test_tested = []
    for test in tests:
        g = np.full(shape, FAIL, dtype=int)
        t = np.zeros(shape, dtype=bool)
        for p in points:
            if p["test"] != test:
                continue
            v = _qv(p["vdd_set_v"])
            f = _qf(p["clock_freq_hz"])
            if v in vdd_idx and f in freq_idx:
                g[vdd_idx[v], freq_idx[f]] = PASS if p["result"] == "Pass" else FAIL
                t[vdd_idx[v], freq_idx[f]] = True
        per_test_grids.append(g)
        per_test_tested.append(t)
    grid = per_test_grids[0].copy()
    for g in per_test_grids[1:]:
        np.minimum(grid, g, out=grid)
    tested = per_test_tested[0].copy()
    for t in per_test_tested[1:]:
        tested &= t
    return grid, tested


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

    vmin_grid, vmin_tested = _aggregate(vmin_points, vmin_freqs, vdds)
    all_grid, all_tested = _aggregate(all_points, all_freqs, vdds)

    # Mask untested cells *only* on rows where the row itself was tested at
    # some other freq. This way:
    #   - incomplete sweeps (e.g. vmin's 1.19 MHz column stopped at 1.45 V)
    #     render as no-data, not as red, since those vdds were tested at other
    #     freqs in the same panel;
    #   - voltages that are absent from the whole panel (e.g. 0.8 V never
    #     appears in shmoo_all_patterns) keep showing FAIL, per the original
    #     "missing voltage = fail" requirement for the all_patterns side.
    vmin_row_tested = vmin_tested.any(axis=1)
    all_row_tested = all_tested.any(axis=1)
    vmin_mask = (~vmin_tested) & vmin_row_tested[:, None]
    all_mask = (~all_tested) & all_row_tested[:, None]
    vmin_grid = np.ma.array(vmin_grid, mask=vmin_mask)
    all_grid = np.ma.array(all_grid, mask=all_mask)
    return vmin_grid, all_grid, vmin_freqs, all_freqs, vdds


def _cell_edges(centers, fallback_half=0.5):
    """Return N+1 cell edges from N centers, suitable for pcolormesh(shading='flat').

    Handles the single-center case (used for vmin, which has only one freq in
    this dataset) by extending +/- fallback_half around the lone center.
    """
    centers = np.asarray(centers, dtype=float)
    if centers.size == 0:
        return np.array([0.0, 1.0])
    if centers.size == 1:
        return np.array([centers[0] - fallback_half, centers[0] + fallback_half])
    mid = (centers[1:] + centers[:-1]) / 2.0
    left = centers[0] - (mid[0] - centers[0])
    right = centers[-1] + (centers[-1] - mid[-1])
    return np.concatenate([[left], mid, [right]])


def _render_pair(fig, gs_pair, vmin_grid, all_grid, vmin_freqs, all_freqs,
                 vdds, show_ylabel=True):
    ax_l = fig.add_subplot(gs_pair[0])
    ax_r = fig.add_subplot(gs_pair[1], sharey=ax_l)

    # pcolormesh with explicit cell edges renders correctly even when one side
    # has a single column (vmin in this dataset has only the 5 MHz point).
    # Widths in the outer gridspec are proportional to MHz span (see main()),
    # so per-MHz scale matches across the two panels.
    vmin_mhz = np.asarray(vmin_freqs) / 1e6
    all_mhz = np.asarray(all_freqs) / 1e6
    vdd_y = np.asarray(vdds)
    vmin_xedges = _cell_edges(vmin_mhz, fallback_half=0.5)
    all_xedges = _cell_edges(all_mhz, fallback_half=2.5)
    yedges = _cell_edges(vdd_y, fallback_half=0.025)
    ax_l.pcolormesh(vmin_xedges, yedges, vmin_grid, cmap=CMAP, vmin=0, vmax=1,
                    shading="flat")
    ax_r.pcolormesh(all_xedges, yedges, all_grid, cmap=CMAP, vmin=0, vmax=1,
                    shading="flat")

    # Lock panel xlim to the cell-edge bounds so the data fills the panel.
    ax_l.set_xlim(float(vmin_xedges[0]), float(vmin_xedges[-1]))
    ax_r.set_xlim(float(all_xedges[0]), float(all_xedges[-1]))

    ax_l.xaxis.set_major_locator(MultipleLocator(1))
    # Step from the actual lowest all_patterns freq (e.g. 15 MHz) so the
    # boundary tick at the break is labeled — MultipleLocator(20) would skip
    # 15 because it's not a multiple of 20.
    if all_mhz.size:
        start = float(all_mhz.min())
        stop = float(all_mhz.max())
        ax_r.xaxis.set_major_locator(
            FixedLocator(np.arange(start, stop + 1e-9, 20.0)))
    else:
        ax_r.xaxis.set_major_locator(MultipleLocator(20))
    ax_l.tick_params(axis="x", labelsize=FS_TICK)
    ax_r.tick_params(axis="x", labelsize=FS_TICK)

    ax_l.yaxis.set_major_locator(MultipleLocator(0.1))
    ax_l.tick_params(axis="y", labelsize=FS_TICK)
    ax_r.tick_params(left=False, labelleft=False)

    # Hide the inner spines so the break markers (drawn later, in figure
    # coordinates, so the two panels' diagonals are parallel) are the only
    # boundary indication.
    ax_l.spines["right"].set_visible(False)
    ax_r.spines["left"].set_visible(False)

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


def _index_dir(d: Path) -> dict:
    return {_sid_from_path(p): p for p in d.glob("sram*_shmoo.json")}


# Frequencies (in MHz) whose vmin sweeps were interrupted and recorded only
# partial vdd coverage. Those columns are dropped from the merged data so they
# don't leave a half-filled gap in the plot.
_DROPPED_VMIN_FREQ_MHZ = [1.19]
_DROPPED_VMIN_TOL_MHZ = 0.02


def _load_merged_vmin(sram_id: int, dirs: list[Path]) -> dict:
    """Concatenate the `points` arrays from every vmin directory that has this SRAM.

    Returned dict mirrors the schema of a single shmoo JSON file. Frequencies
    listed in `_DROPPED_VMIN_FREQ_MHZ` are filtered out (interrupted sweeps).
    If the same (test, vdd, freq) triple appears in multiple dirs, both copies
    are kept — aggregation picks the worst result, so duplicates do not bias
    the outcome.
    """
    merged = {"sram_id": sram_id, "points": []}
    for d in dirs:
        p = d / f"sram{sram_id}_shmoo.json"
        if p.exists():
            for pt in load(p)["points"]:
                mhz = pt["clock_freq_hz"] / 1e6
                if any(abs(mhz - drop) < _DROPPED_VMIN_TOL_MHZ
                       for drop in _DROPPED_VMIN_FREQ_MHZ):
                    continue
                merged["points"].append(pt)
    return merged


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vmin-dir", nargs="+",
                        default=["data/shmoo_vmin"],
                        help="One or more directories with low-freq vmin sweep "
                             "JSONs. Points from each SRAM file are merged "
                             "across all directories listed.")
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

    vmin_dirs = [Path(d) for d in args.vmin_dir]
    all_dir = Path(args.all_dir)
    vmin_indices = [_index_dir(d) for d in vmin_dirs]
    vmin_union = set().union(*vmin_indices) if vmin_indices else set()
    all_ids = _index_dir(all_dir)
    # SRAM 21 (the largest macro) does not work on this die — skip it.
    sram_ids = sorted(s for s in (vmin_union & set(all_ids)) if s < 21)
    if args.sram_ids is not None:
        wanted = [int(s) for s in args.sram_ids.split(",")]
        sram_ids = [s for s in wanted if s in set(sram_ids)]
    elif args.sram_id is not None:
        sram_ids = [s for s in sram_ids if s == args.sram_id]
    if not sram_ids:
        print(f"No matching SRAM JSON files in both {vmin_dirs} and {all_dir}.")
        return

    ncols = max(1, min(args.sram_cols, len(sram_ids)))
    nrows = (len(sram_ids) + ncols - 1) // ncols

    # Allocate fixed inch budgets so layout is consistent for a single SRAM and
    # for the full grid. title_h reserves vertical room for per-SRAM titles;
    # bot_h leaves room for x-axis labels below the last row.
    cell_h = 3.5
    title_h = 0.5
    bot_h = 0.55
    fig_h = cell_h * nrows + title_h + bot_h
    fig_w = 7.0 * ncols
    fig = plt.figure(figsize=(fig_w, fig_h))

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
        vmin_data = _load_merged_vmin(sid, vmin_dirs)
        all_data = load(all_ids[sid])
        vg, ag, vf, af, vs = build_merged(vmin_data, all_data)
        # Width ratios are proportional to MHz span (uniform x-axis scale).
        # The vmin span is stretched 5x relative to all_patterns so the vmin
        # column (single 5 MHz point in this dataset) is wide enough to be
        # legible. A floor of 1 MHz keeps the panel visible even when vmin has
        # only one frequency.
        vmin_span = max((max(vf) - min(vf)) / 1e6, 1.0) if vf else 1.0
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
    # Physical size of each break-slash in inches. Converting to figure-coord
    # offsets gives slashes with the same physical slope on both panels,
    # regardless of the panel width ratio.
    slash_dx_in = 0.06
    slash_dy_in = 0.10
    dx_fig = slash_dx_in / fig_w
    dy_fig = slash_dy_in / fig_h

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

        # Parallel break-slashes ('/' shape) at all four inner corners.
        for x_center in (bl.x1, br.x0):
            for y_center in (bl.y0, bl.y1):
                fig.add_artist(plt.Line2D(
                    [x_center - dx_fig, x_center + dx_fig],
                    [y_center - dy_fig, y_center + dy_fig],
                    color="k", lw=1.0, clip_on=False,
                    transform=fig.transFigure,
                ))

    if args.out:
        fig.savefig(args.out, dpi=300)
        print(f"Saved to {args.out}")
    else:
        plt.show()


if __name__ == "__main__":
    main()
