FROM rust:1.71-bookworm

WORKDIR /app/

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    zip \
    gcc-mingw-w64-i686 \
    && rm -rf /var/lib/apt/lists/

# Load the toolchain override
COPY rust-toolchain.toml /app/
RUN rustup show

CMD ["/bin/bash"]
