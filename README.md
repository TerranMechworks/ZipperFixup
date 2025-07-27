# ZipperFixup

This project fixes bugs and limitations of certain games/game engines developed by Zipper Interactive™:

* the Recoil™ game (1999)
* the MechWarrior 3™ base game (1999)
* the MechWarrior 3 Pirate's Moon™ expansion (1999)

Zipper Interactive™ was trademark or registered trademark of Sony Computer Entertainment America LLC. Other trademarks belong to the respective rightsholders.

Obviously, this is an unofficial fan effort and not connected to the developers, publishers, or rightsholders. [Join us on MW3 Discord](https://discord.gg/Be53gMy), or the Recoil Discord!

## Installation

1. Download [the latest release Zip file](https://github.com/TerranMechworks/ZipperFixup/releases), for example `ZipperFixup-0.1.2.zip`.
2. Extract the Zip file.
3. Follow instructions of `Readme.txt` in Zip file.

## License

Licensed under the European Union Public Licence (EUPL) 1.2 ([LICENSE](LICENSE) or https://joinup.ec.europa.eu/collection/eupl/eupl-text-eupl-12).

## Development

### Building

[Rust](https://www.rust-lang.org/tools/install) is required.

#### i686-pc-windows-msvc

This target can only be build on Windows. Please see installing [Rust on Windows](https://rust-lang.github.io/rustup/installation/windows.html), including the Visual Studio requirement.

#### i686-pc-windows-gnu

This target can be built on any platform, but requires [Mingw-w64](http://mingw-w64.org/) (specifically the 32 bit version).

* On Ubuntu: `apt install gcc-mingw-w64-i686`
* On macOS: `brew install mingw-w64`
* On Windows, use WSL.

### Release procedure

1. Review `CHANGELOG.md`
1. Add release version number and release date to changelog; add a new `[Unreleased]` section
1. Bump version in `Cargo.toml`
1. Commit, push, and wait for CI
1. Create a tag of the version (e.g. `git tag v0.1.1`)
1. Push the tag (`git push --tags`)
1. The build will automatically create a release as a draft
1. Add changelog items to the release notes via the GitHub web interface
1. Publish the release via the GitHub web interface
