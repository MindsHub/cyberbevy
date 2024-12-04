FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes libasound2-dev:arm64 libx11-dev:arm64 libudev-dev:arm64 libwayland-dev:arm64

ENV PKG_CONFIG_LIBDIR_aarch64_unknown_linux_gnu=/usr/lib/aarch64-linux-gnu/pkgconfig