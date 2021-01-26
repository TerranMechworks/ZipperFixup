# Use native image as a base. Don't emulate as this is a dev image
FROM rust:1.49

WORKDIR /app/

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    zip \
    g++-mingw-w64-i686 \
    && rm -rf /var/lib/apt/lists/

RUN rustup target add i686-pc-windows-gnu

COPY . /app/

RUN cargo build --release --target "i686-pc-windows-gnu" \
    && strip ./target/i686-pc-windows-gnu/release/zipfixup.dll \
    && ls -lh ./target/i686-pc-windows-gnu/release/zipfixup.dll

WORKDIR /app/package

RUN cp /app/target/i686-pc-windows-gnu/release/zipfixup.dll . \
    && cp /app/Misc/* . \
    && ls -l \
    && zip ZipperFixup-`cargo pkgid | cut -d# -f2 | cut -d: -f2`.zip ./* \
    && ls -l

CMD ["/bin/bash", "-c", "cp ./ZipperFixup*.zip /package/"]
