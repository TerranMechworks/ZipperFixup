# Changelog

## [Unreleased]

- Patch: Rundll32 no longer seems to work... replaced it with `zippatch.exe`.
- Patch: Patcher now waits for enter.

- Fixup: Use retour/detour for hooking functions.
- Fixup: Include timestamps in debug output.
- Fixup: Implement checker to verify DLL exports/forwards are correct.
- Fixup: Specify exports via [Module-Definition (.Def)
  file](https://learn.microsoft.com/en-us/cpp/build/reference/module-definition-dot-def-files).
- Fixup: Now builds using either `i686-pc-windows-gnu` or `i686-pc-windows-msvc`.

- Project: Update Rust version to 1.88.0 and edition to 2024. This moves off
  nightly, since everything we need is now stabilised, and should help with
  maintenance.
- Project: Relicense under EUPL-1.2.

## [0.1.1] - 2021-01-29

- PM: Have another go at adding PM support.

## [0.1.0] - 2021-01-26

- PM/RC: Try adding support for RC and PM.
- Project: Move all code to Rust as it was a pain to get C++ working.

## [0.0.2] - 2021-01-19

- Patch: Fix install script.
- Fixup: Display version number during install.

## [0.0.1] - 2021-01-18

- All: Fixed timing issues related GetTickCount having a resolution of 1ms but
  an accuracy of 16ms.
- All: Fixed timing issues related to high system uptime.
- MW: Bounds checking on target box lines (prevents crashes at higher
  resolutions).
