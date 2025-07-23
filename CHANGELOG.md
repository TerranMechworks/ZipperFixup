# Changelog

## [Unreleased]

- Project: Update Rust version to 1.88.0 and codebase. This moves off nightly, since everything we need is now stabilised, and should help with maintenance.
- Patch: Rundll32 no longer seems to work... replaced it with `patch.exe`.
- Project: Use retour/detour for hooking functions.
- Project: Relicense under EUPL-1.2

## [0.1.1] - 2021-01-29

- PM: Have another go at adding PM support.

## [0.1.0] - 2021-01-26

- PM/RC: Try adding support for RC and PM.
- Project: Move all code to Rust as it was a pain to get C++ working.

## [0.0.2] - 2021-01-19

- Patch: Fix install script.
- DLL: Display version number during install.

## [0.0.1] - 2021-01-18

- All: Fixed timing issues related GetTickCount having a resolution of 1ms but an accuracy of 16ms.
- All: Fixed timing issues related to high system uptime.
- MW: Bounds checking on target box lines (prevents crashes at higher resolutions).
