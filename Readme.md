For reference for people coming back to this thread.
I have been able to run bevy projects on the Raspberry Pi 4B:

I used a default RaspianOS 64 bit image with rpi-imager, which as of today uses Debain 12 (bookworm).
I installed the now fully supported vulkan drivers through a simple sudo apt install mesa-vulkan-drivers
I cross compiled to aarch64-uknown-linux-gnu using the cross project with a custom Dockerfile:
 FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:latest

RUN dpkg --add-architecture arm64 && \
    apt-get update && \
    apt-get install --assume-yes libasound2-dev:arm64 libx11-dev:arm64 libudev-dev:arm64 

ENV PKG_CONFIG_LIBDIR_aarch64_unknown_linux_gnu=/usr/lib/aarch64-linux-gnu/pkgconfig
I have been able to play the breakout example game at around 20 fps, but something that look like dropped frames every second ( did not investigate). Same for the bevymark example, significantly dropping from the starting 40 FPS after less than 2000 spawns.
3D scenes also work, I did not check FPS, but probably low.

So don't expect to heavily play on a Raspberry 4B :)