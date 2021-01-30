# ZipperFixup

A redistributable DLL to fix some of the bugs in `Mech3.exe`.

## Building

### Local

You can build this project on your local machine if you're comfortable with Rust development. You'll need to install [Rust](https://www.rust-lang.org/tools/install). Also required is [Mingw-w64](http://mingw-w64.org/), specifically the 32 bit version.

* On Ubuntu: `apt install gcc-mingw-w64-i686`
* On macOS: `brew install mingw-w64`
* On Windows, installation is trickier. [This build](https://sourceforge.net/projects/mingw-w64/files/Toolchains%20targetting%20Win32/Personal%20Builds/mingw-builds/installer/mingw-w64-install.exe/download) from Sourceforge might work for you; you'll likely have to configure your `PATH` variable. You might prefer the Docker build method below.

### Docker

If you don't want to install dependencies locally, we've provided some wrappers to make building the project with Docker easier. First, run `./dcargo-setup`. It builds the base image and creates a volume for persistently caching downloaded packages. Then, you can run `./dcargo` as you would `cargo`, for example:

```bash
./dcargo build --release
```

## License

ZipperFixup is GPLv3 licensed. Please see `LICENSE.txt`.
