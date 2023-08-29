# ZipperFixup

A redistributable DLL to fix bugs and limitations of certain games developed by Zipper Interactive™ to modern formats and back:

* the Recoil™ game (1999)
* the MechWarrior 3™ base game (1999)
* the MechWarrior 3 Pirate's Moon™ expansion (1999)

Zipper Interactive™ was trademark or registered trademark of Sony Computer Entertainment America LLC. Other trademarks belong to the respective rightsholders.

Obviously, this is an unofficial fan effort and not connected to the developers, publishers, or rightsholders. [Join us on MW3 Discord](https://discord.gg/Be53gMy), or the Recoil Discord!

## ChangeLog

### [Unreleased]

- Project: Use [retour](https://docs.rs/retour/latest/retour/)/detour for hooking functions.
- Patch: Rundll32 no longer seems to work... replaced it with `patch.exe`.
- Project: Update Rust version and codebase. This moves off nightly and to 1.71.1, since everything we need is now stabilised.

### [0.1.1] - 2021-01-29

- PM: Have another go at adding PM support.

### [0.1.0] - 2021-01-26

- PM/RC: Try adding support for RC and PM.
- Project: Move all code to Rust as it was a pain to get C++ working.

### [0.0.2] - 2021-01-19

- Patch: Fix install script.
- Dll: Display version number during install.

### [0.0.1] - 2021-01-18

- All: Fixed timing issues related GetTickCount having a resolution of 1ms but an accuracy of 16ms.
- All: Fixed timing issues related to high system uptime.
- MW: Bounds checking on target box lines (prevents crashes at higher resolutions).

## Building

### Local

You can build this project on your local machine if you're comfortable with Rust development. You'll need to install [Rust](https://www.rust-lang.org/tools/install). Also required is [Mingw-w64](http://mingw-w64.org/), specifically the 32 bit version.

* On Ubuntu: `apt install gcc-mingw-w64-i686`
* On macOS: `brew install mingw-w64`
* On Windows, installation is trickier. [This build](https://sourceforge.net/projects/mingw-w64/files/Toolchains%20targetting%20Win32/Personal%20Builds/mingw-builds/installer/mingw-w64-install.exe/download) from Sourceforge might work for you; you'll likely have to configure your `PATH` variable. You might prefer the container build method described below.

### Container

If you don't want to install dependencies locally, we've provided some wrappers to make building the project with [Docker](https://www.docker.com/) or [Podman](https://podman.io/) (aliased to `docker`). First, run `./dcargo-setup`. It builds the base image and creates a volume for persistently caching downloaded packages. Then, you can run `./dcargo` as you would `cargo`, for example:

```bash
./dcargo build --release
```

Note: This must be run in the root of the repo, as the image mounts the current directory. As a result, the `target/` directory will also be modified, based on the container. Don't mix the the `dcargo` approach with a local build. This includes using e.g. `rust-analyzer` on the host machine.

## Release procedure

1. Review changelog in `README.md`
1. Add release version number and release date to changelog; add a new `[Unreleased]` header
1. Bump version in `Cargo.toml`
1. Commit, push, and wait for CI
1. Create a tag of the version (e.g. `git tag v0.1.1`)
1. Push the tag (`git push --tags`)
1. The build will automatically create a release as a draft
1. Add changelog items to the release notes via the GitHub web interface
1. Publish the release via the GitHub web interface

## License

ZipperFixup is GPLv3 licensed. Please see `LICENSE`.
