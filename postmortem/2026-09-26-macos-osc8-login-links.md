# macOS OSC 8 links after modifier changes

## What happened

Long authentication links printed by Codex CLI could not reliably be opened
with Command-click in Con. Copying them by selecting the displayed rows was
also unreliable. Codex emits the complete URL as an OSC 8 hyperlink on each
wrapped cell, so the URL length was not the parser's limitation.

## Root cause

Con forwarded modifier changes to embedded Ghostty by resending the same mouse
position. Embedded Ghostty drops unchanged mouse-position callbacks, so
Command pressed after hovering did not refresh the link. This was only one
failure mode. Codex CLI 0.157.1 also enables terminal mouse reporting while its
screen is active. Ghostty deliberately gives ordinary mouse events to Codex and
only checks link hover while Shift bypasses that capture. Forwarding Command
alone therefore cannot make the link clickable. Separately, GPUI dispatches
Command-C through a Copy action that still copied selections only, bypassing
the earlier key-down fallback for the OSC 8 target.

The muted color in the development screenshot is separate: Codex styles the
link cyan, while the development shell checked here has `NO_COLOR=1`. A login
shell started without that variable does not set it. Inherited color suppression
is therefore a concrete possibility, but the screenshot alone does not prove
the running Con process received that environment. Normal Con launches must
still respect a user's intentional `NO_COLOR` setting.

## Fix

Forward macOS modifier press and release events to the embedded Ghostty surface
using native virtual keycodes. When Command is held and the TUI captures mouse
events, add Ghostty's Shift bypass to the mouse gesture; Ghostty strips that
Shift when matching Command-held OSC 8 links. Leave plain clicks and other
platforms unchanged. Refresh link state before the press even when modifiers
appear unchanged. Route both the Copy action and key-down fallback through the
same selection-first policy; only allowed OSC 8 targets can be copied.

## What we learned

An unchanged-position mouse event is not equivalent to a modifier event in
embedded Ghostty, and terminal mouse reporting changes who owns a click.
Inspect the live TUI's mouse mode rather than assuming a login screen has none.
For wrapped hyperlinks, copy the protocol target rather than reconstructing an
address from screen cells. Test both keyboard and GPUI action dispatch paths.
