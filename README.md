# ZipperFixup

A redistributable DLL to fix bugs and limitations of certain games developed by Zipper Interactive™ to modern formats and back:

* the Recoil™ game (1999)
* the MechWarrior 3™ base game (1999)
* the MechWarrior 3 Pirate's Moon™ expansion (1999)

Zipper Interactive™ was trademark or registered trademark of Sony Computer Entertainment America LLC. Other trademarks belong to the respective rightsholders.

Obviously, this is an unofficial fan effort and not connected to the developers, publishers, or rightsholders. [Join us on MW3 Discord](https://discord.gg/Be53gMy), or the Recoil Discord!

## Building

[Rust](https://www.rust-lang.org/tools/install) is required. Also required is [Mingw-w64](http://mingw-w64.org/), specifically the 32 bit version.

* On Ubuntu: `apt install gcc-mingw-w64-i686`
* On macOS: `brew install mingw-w64`
* On Windows, use WSL.

## Release procedure

1. Review `CHANGELOG.md`
1. Add release version number and release date to changelog; add a new `[Unreleased]` section
1. Bump version in `Cargo.toml`
1. Commit, push, and wait for CI
1. Create a tag of the version (e.g. `git tag v0.1.1`)
1. Push the tag (`git push --tags`)
1. The build will automatically create a release as a draft
1. Add changelog items to the release notes via the GitHub web interface
1. Publish the release via the GitHub web interface

## License

Licensed under the European Union Public Licence (EUPL) 1.2 ([LICENSE](LICENSE) or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).
