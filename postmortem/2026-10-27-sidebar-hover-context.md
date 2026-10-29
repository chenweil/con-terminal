# Sidebar hover context and pane ownership

## What happened

The sidebar hover card mixed a tab's focused directory, raw terminal titles,
the winning activity, and internal pane/surface indices. Replacing status prose
with icons improved the glyphs but left the relationships unclear. Showing only
the winning task and a pane count then removed useful information about the
other panes.

## Root cause

The presentation model described a tab aggregate, while the hover card needed
an overview of its individual panes. A component tooltip also captured its data
on mouse entry. Aggregate-only invalidation cannot reflect a second pane's
completion when another pane keeps the tab busy.

## Fix

Retain one summary per physical pane in tree order, including editors. Aggregate
surfaces within each pane and keep the selected record's application, task,
directory, and state together. Refresh the sidebar when any pane summary changes.
An open tooltip observes the live sidebar; a separate retained layer paints the
status animations. Remove raw command dumps, count-only summaries, and internal
source indices from user-facing hover content.

## Lessons

Design the information relationships before adjusting typography or material.
Minimalism should remove redundancy, not the information needed to understand
the workspace. Validate multiple simultaneous activities and a non-winning pane
transition, as well as single-pane, editor, compact, and theme variants.
