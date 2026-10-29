# Clean single-tab caption gained unwanted height

## What happened

The macOS clean single-tab view gained 4pt of empty titlebar space during the
workspace visual redesign. It no longer preserved the original compact terminal
layout even though that layout remained the stated design intent.

## Root cause

The shared caption row became 32pt to support the 22pt controls with 5pt clearance
on decorated workspaces. The clean view inherited that same row. A pre-existing
constant-equality test only protected a shared height, not the clean view's density.

## Fix

Only horizontal navigation with one tab, one pane and both sidebars hidden returns
to 28pt. The existing canvas-depth sample interpolates back to 32pt while sidebars
open. Multi-tab horizontal strips, split canvases, compact vertical navigation and
expanded vertical caption docking keep the current 32pt dimensions and gutters.
The reserved row, painted controls and native traffic lights use the same sampled
height. Native traffic-light placement uses GPUI's existing setter; terminal
presentation, clipping, dependencies and the sidebar docking policy are unchanged.

## Lessons

Test the clean endpoint separately from decorated layouts. A shared constant can
prevent unwanted movement while also hiding a density regression. Keep native and
painted controls aligned when a layout deliberately reclaims caption space.

## Verification

All 216 workspace tests and all 709 macOS application tests pass. The final
application build, Rust formatting, diff checks and 14-page docs manifest pass.
The Impeccable layout scan reports no findings for the tab chrome source.

An isolated native preview confirmed the clean single-tab boundary moved from
32pt to 28pt (64 to 56 physical pixels at 2x), with the right-side controls moving
with it. The clean endpoint was checked in Flexoki Light and Dark. Horizontal
multi-tab and sidebar-open single-tab layouts retain their original 32pt row;
compact vertical navigation, Files plus rail and expanded caption docking were
also inspected. Tab creation/closure, sidebar opening/closing and orientation
changes returned to the intended endpoints. The remote desktop overlay obscures
the native traffic-light cluster in screenshots; its placement uses GPUI's
existing setter and was not independently measured in the capture.

Windows/Linux native rendering and CI were not run locally; the new behavior
and state are macOS-only. No production session or configuration was changed.
