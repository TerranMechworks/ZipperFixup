# Use native image as a base. Don't emulate as this is a dev image
FROM rust:1.49

WORKDIR /app/

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    zip \
    gcc-mingw-w64-i686 \
    && rm -rf /var/lib/apt/lists/

# Load the toolchain override
COPY rust-toolchain /app/
RUN rustup show

CMD ["/bin/bash"]
