# Agent transcript rhythm and duplicate feedback

## What happened

The agent sidebar mixed independent avatar, summary, trace and status insets.
An invisible user action row added vertical space even when no actions were
shown. Parsed replies used 15/24 typography while streaming prose used 14/23
and user messages used 13.5/21. Thinking could appear in the activity group
and again in the transcript tail.

## Root cause

Each surface acquired its own geometry and the generic busy footer checked
approvals, but did not account for feedback already supplied by a current
assistant trace or live tool rows. Markdown had an independent body scale.

## Change

Share transcript/header insets, 14/22 prose typography and the activity icon
column. Keep trace indentation to one semantic level and its expanded output
under the label. Place hover actions in the existing inter-message interval, with the whole toolbar inside the message
hover region.
Suppress redundant tail feedback when the current trace or live tool rows
already supply it; preserve initial submission feedback and approval ownership.
Use lowercase `con` in application labels and prose without changing identifiers.

## Lessons

Review initial wait, thinking, writing and restored replies together. An idle
screenshot alone misses duplicate live feedback; test hover action bounds as
well as visible message bounds at narrow widths and larger UI fonts. Check
hovered disclosures with real text: a tightly fitted default ghost-button fill
can overpower metadata and crowd the trailing count. Brief reasoning notes
need the same activity column as executable rows.

## Verification

The final macOS app test binary passes all 713 tests with a single test thread,
including 38 agent-panel tests. Layout checks cover 280/640-point panels and
16/24-point UI fonts, user action bounds, real hover-to-edit activation, activity
columns and initial versus trace-backed progress feedback. Formatting, diff
whitespace and the documentation manifest checks pass. Windows/Linux runtime
rendering is not covered locally.
