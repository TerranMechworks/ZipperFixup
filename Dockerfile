# Use native image as a base. Don't emulate as this is a dev image
FROM ubuntu:20.04 as base

ENV DEBIAN_FRONTEND=noninteractive

# ubuntu packages
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    cmake \
    make \
    zip \
    g++-mingw-w64-i686 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/
COPY CMakeLists.txt .
COPY Misc Misc
COPY src src

WORKDIR /app/build
RUN ls -laR .. \
    && cmake -DCMAKE_BUILD_TYPE=RELEASE .. \
    && make -j $(nproc)

WORKDIR /app/package
RUN cp /app/build/mech3fix.dll . \
    && cp /app/Misc/* . \
    && ls -l \
    && zip Mech3Fixup.zip ./*

CMD ["/bin/cp", "./Mech3Fixup.zip", "/package/"]
