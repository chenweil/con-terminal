# Updater advertised a draft release

## What Happened

On October 23, a macOS beta.148 update failed with Sparkle's download error.
Local logs at 08:39 Asia/Shanghai recorded HTTP 404 for the ARM DMG. The public
appcast advertised beta.148, but GitHub still marked the release as a draft.
Authenticated maintainers could see the assets; updater clients could not.

All platform workflows had succeeded. Finalizer run 39817569024 nevertheless
logged zero results for the Linux and macOS SHA-filtered workflow queries and
returned success without promoting the draft. The same queries later returned
the completed runs. This establishes a transient missing-result condition; it
does not establish a specific GitHub internal indexing fault.

## Root Cause

Platform jobs published canonical feeds before the release became public.
The finalizer treated missing API results like a still-running sibling and
exited successfully, assuming another completion event would arrive. The last
completion event could therefore leave both a private draft and a public update
notification indefinitely. Homebrew had the same early-publication ordering.

## Recovery

Rerunning only the finalizer passed the existing gate and published beta.148 at
2026-10-23T00:41:33Z. An anonymous cache-busted range request returned HTTP 206;
the original URL still briefly returned its cached 404. No tag was changed and
no binaries were rebuilt. The cache behavior reinforces why clients must not
be directed to a draft URL, even temporarily.

## Fix

- Platform jobs prepare tag-scoped signed metadata without changing live feeds.
- The finalizer retries missing/error responses and uses an independently
  listed, locally filtered set of runs. Both tag and commit must match; a
  persistent missing result is a visible failure.
- Payload validation precedes promotion. Anonymous checks of the exact URLs
  precede canonical feed publication. Homebrew follows successful finalization.
- Public-but-unannounced releases can resume finalization. Feed merges retain
  newer builds and retry against fresh Pages state without overwriting Flatpak.
- A manual reconciliation trigger repairs an existing staged release without
  rebuilding or retagging.

## Verification And Limits

Offline regressions cover missing results, API errors, wrong tag/commit, failed
and running siblings, exact URL/size checks, cached download errors, absent
staged platforms, immutable feed entries, public-release retries and dev-channel
isolation. Local bare-remote tests cover publication order, idempotence and a
real rejected push followed by a concurrent Flatpak-preserving retry.

Workflow lint and the actual Homebrew cask-template loader cover the changed
wiring. These tests do not emulate GitHub's production indexing or CDN. The next
release must confirm public asset availability before its updater feeds move.

## Lessons

An update feed is a promise that its artifact can already be downloaded without
maintainer credentials. Preparing metadata and notifying users are different
operations. A green workflow that intentionally deferred is not a publication
receipt; missing external state needs bounded reconciliation and a recovery path.
