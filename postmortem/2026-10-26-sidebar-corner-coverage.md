# Sidebar transitions discarded visible corner coverage

## What happened

The last frames of left/right sidebar transitions could briefly reveal the
backdrop through rounded terminal corners. The problem was especially visible
in dark mode. The shared input material could fail in the same frames.

## Root cause

During the transition to a clean terminal, some radii become smaller than one
physical pixel while window-concentric corners remain large. GPUI's corner
painter correctly omits a tile that snaps to zero size. The native/shared
coverage builder instead treated that expected empty tile as an error and
failed the entire collection. Its callers then discarded coverage for every
corner, including visible ones, and fell back to a different material path.

An isolated dark preview reproduced 18 new terminal coverage errors and 10
input coverage errors from one left and one right open/close cycle. The errors
were `empty corner`, matching the subpixel end of the spring motion.

## Fix

Omit only individual corner tiles with no physical area, using the same
presentation rule as the GPUI painter. Keep all remaining coverage and keep
reporting real validation or raster-budget failures. Do not change animation,
opacity, settled radii, native FFI, presentation synchronization or add covers.

## Lessons

Coverage tests must include mixed visible and disappearing corners, not only
uniform full-size radii. Normal physical-pixel rounding at animation endpoints
must not select an error fallback for the entire surface.

## Verification

The new mixed-corner regression test fails on the previous implementation with
`empty corner` and passes after the fix. It covers every disappearing corner at
1x, 1.5x and 2x, multiple alpha values, all-empty endpoints, and preservation of
real raster-budget errors. All 710 macOS application tests pass (23.69s), including
existing material-weight and native clip tests; the application build passes.

The fixed isolated preview recorded zero new terminal or input coverage errors,
compared with 18 and 10 respectively in the baseline open/close exercise. The
maintainer also personally tested this exact build and reported that the visible
problem no longer occurred. Native light-theme animation was not separately
captured in this pass; the coverage calculation is color-independent and alpha cases are tested.
Windows/Linux CI remains pending; this coverage path is macOS-only.
