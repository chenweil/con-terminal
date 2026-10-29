# Sidebar spacing omitted the adjacent terminal gutter

## What happened

The compact tab selection looked closer to the window edge than to the terminal.
The same imbalance was visible on expanded rows. Navigation controls also needed clearer visual metaphors.

## Root cause

Navigation centered content inside its own reserved width. The macOS terminal
then added a 5-point exterior gutter, making the visible right-hand interval
larger. Changing the shared terminal inset would also affect splits, the input
bar, native clipping and sidebar transitions.

## Fix

The workspace supplies the actual adjacent gutter to navigation. Navigation
shifts its content by half that interval in both compact and expanded states,
without changing its reserved width or terminal geometry. Files/Search and
portable layouts supply no correction. Official Phosphor Regular List, Tree View and List Magnifying Glass assets
clarify the navigation actions while preserving the existing fine stroke.

The accompanying compact hover preview reuses the existing expanded rows as a
root overlay, with separate transient motion and no layout reservation. It does
not persist the pinned state. Hover preserves compact row/control geometry and
shares the rail scroll handle; reusing the pinned header would move tabs under
the pointer when it changes from a vertical toolbar to a horizontal one. A deliberate resize beyond the minimum folds
vertical tabs; content drawers remain resizable without automatic folding.

## Lessons

Measure spacing between visible surfaces, not only within each component.
Keep optical alignment, transient previews and persistent layout reservations
separate. Verify both navigation endpoints, intermediate motion, tool drawers
and the terminal's actual bounds when changing sidebar behavior.
Component defaults also need local styling: a ghost button's hover fill and a
default tooltip's border/shadow are stronger than con's navigation language.
Visible action labels need no duplicate tooltip; icon-only hints retain the
same quiet, borderless and shadowless treatment as the surrounding UI.
The preview is attached chrome, not a floating card: its internal edge must
continue straight from the caption to the window bottom. Independent corner
rounding exposes the terminal at the joint and makes the surface look detached.
