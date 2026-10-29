# TUI completion replaced tab identity

## What happened

A completed TUI replaced its application logo or multi-pane identity stack with
a checkmark. This no longer matched the redesigned tabs and made a tab change
its visual identity precisely when users were trying to find finished work.
Horizontal and vertical tabs also disagreed about notification suppression.

## Root cause

The shared activity painter treated Done as another status glyph. Vertical
navigation suppressed unread dots for Done, while horizontal navigation used
only the generic inactive unread flag. Completion was split between unrelated
presentation paths instead of using the existing notification affordance.

## Fix

Done no longer replaces the identity. A shared presentation policy and dot
component show completion in horizontal, compact and expanded vertical tabs.
Urgent states suppress redundant notification dots. Completed identity stacks
keep their size and all application artwork; horizontal markers follow the
whole stack rather than its former single-icon width.

The core status protocol, priority order and acknowledgement remain unchanged:
completion belongs to a surface and is acknowledged by input to that surface,
not by window focus or selecting a tab. Ordinary unread tab activity keeps its
existing inactive-only behavior. A finite 180ms entrance and 120ms exit use the
existing motion curve. Motion is retained by stable tab identity across layouts
and reverses continuously; reduced motion applies the state immediately. Idle
notifications request no frames. All layouts anchor the dot to the actual
identity group rather than independently chosen row corners. The compact rail
uses the real hit-target dimensions for centering; its inner 32pt selection
surface is smaller and cannot be substituted in badge positioning. A rendered
geometry assertion covers the relationship in both compact and expanded modes.

## Lessons

Keep application identity and completion notification separate. Test retained
identity geometry and status/notification priority across all tab densities,
including active completions, multi-app stacks and subsequent acknowledgement.

## Verification

All 708 macOS app tests pass, including shared notification priority, complete
identity-stack retention in both densities, and real compact/expanded sidebar
rendering through acknowledgement and error transitions. The suite also covers
finite, reversible notification motion, reduced motion, stable tab identity and
closed-tab cleanup. Rust format and diff checks pass. The final full-suite run
completed in 23.67 seconds after an earlier macOS startup delay; the interrupted
attempts are not counted as passes. After correcting compact-rail centering,
all 23 sidebar tests pass again, including the new rendered-coordinate
assertions. The final application build, Rust formatting and diff checks pass.

An isolated Python fixture emitted real OSC 7501 working/done records. Native
previews covered horizontal, compact vertical and expanded vertical tabs in
Flexoki Light and Dark. Selecting the completed tab retained the dot; typing
into its terminal removed the accessible Completed marker and dot while leaving
the application icon in place. The fixture's Python identity is intentional;
no paid agent inference or production session was used. Windows/Linux native
rendering and CI remain unverified locally.
