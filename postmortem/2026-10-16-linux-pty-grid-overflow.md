# Linux PTY grid larger than the painted pane

**Date**: 2026-10-16
**Issue**: #485

## What happened

On Linux, full-screen terminal UIs (TUIs) such as Claude Code drew past the right edge of the
pane and lost their last row behind the input bar. The report came from KDE on
Wayland at 1.7 display scale with vertical tabs on. The pseudo-terminal (pty) reported 176 × 36
for a pane that fits 170 × 35. Toggling the tab panel changed the width by only
4–5 columns, which made the panel look like the cause.

## Root cause

`estimate_surface_size` in `crates/con-app/src/linux_view.rs` computed the grid
differently from how `render()` paints it, in two ways:

- It divided the whole pane by the cell size. `render()` insets the content by
  12 logical px on the left and right and 10 px on the top and bottom.
- It divided by the physical cell, `round(9 × 1.7) = 15` device px. `render()`
  places a column every 9 logical px, which is 15.3 device px at 1.7x. The 2%
  error adds about 3.5 columns across a 176-column row.

Together these put the painted grid's right edge at 12 + 176 × 9 = 1596 logical
px, in a pane 1556.5 px wide. The 40 px overflow matches the report.

The tab panel was not involved. With the tools panel closed it is the 44 px rail
(`RAIL_WIDTH`), which accounts for the 4–5 columns, and the pane bounds already
excluded it.

A fix for the padding half landed once (52cd33c9) and was reverted a minute
later (db58f36f) with no reason given. That happened during triage of #142,
which turned out to be a macOS problem.

## Fix applied

- A pure `grid_size_for_pane` now computes the grid from the area inside the
  padding, counting cells in logical px at the painted cell size. A 0.01 px
  tolerance keeps exact fits that arrive slightly short after device-to-logical
  conversion.
- The pty pixel size now covers only the cells' area, so `ws_xpixel / ws_col`
  stays close to the painted cell width for programs that size images from it.
- Selection autoscroll keeps using the whole pane height. It used to read that
  height from the pty pixel size.
- The cell size sent to libghostty-vt is unchanged, so Kitty placement is
  unaffected.

## What we learned

The PTY size is a promise about what Con paints. Any code that computes it must
use the paint code's own geometry: the same padding, the same cell size, the
same pixel space. A doc comment on `grid_size_for_pane` and a sweep test over
pane sizes and scales now guard that contract, so another revert would fail CI.

Mouse hit-testing has the same physical-versus-logical cell mismatch and drifts
by about one column every 50 at fractional scales. That is tracked in #486.
