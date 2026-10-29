# Skill completion overflow and editor parity

## What happened

The inline agent skill menu clipped long descriptions after the first few
candidates. Later candidates were not reachable by scrolling. The bottom input
also lacked a reliable, explicit publication step when the skill scan completed.

## Root cause

The two editors rendered separate menus. The agent menu used `overflow_hidden`
and had no scroll handle; full descriptions made each row variable and tall.
The bottom catalog depended on render-time chrome preparation. The initial
shared menu also attached its scrollbar to the scrolling content, allowing the
track coordinates to move with that content; wheel events were not contained.
Candidate mouse targets also extended under the scrollbar, making a click on a
hidden thumb's lane choose a skill instead. Existing keyboard tests covered only
one or two short entries and did not mount a completion list.

## Fix

Both editors use one bounded list of component list items, with fixed-height
name/summary rows, full-description tooltips, scrolling, and selection reveal.
The invoking editor retains focus and completion inserts text without executing
it. Publish completed skill scans to both editors directly. Bound the popup by
the available viewport and keep it borderless and shadowless. Mount the scrollbar
on a fixed sibling layer, reserve its mouse lane, contain wheel events, and hide
completion when its invoking editor is hidden.

## Lessons

Completion behavior must be tested with more candidates than fit on screen and
with real-length descriptions. Check keyboard reveal, pointer selection, focus,
and completion-versus-submission in both editor surfaces, including late catalog
arrival. Share the interaction component instead of maintaining two menus.
