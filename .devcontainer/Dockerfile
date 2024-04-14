FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye

ARG USER=vscode

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && \
    apt-get -y install --no-install-recommends sudo usbutils git libsdl2-dev \
    lld clang

RUN cargo install cargo-binutils cargo-generate form && \
    rustup component add llvm-tools-preview rustfmt clippy


RUN mkdir -p /usr/local/cargo && \
    chown -R $USER:$USER /usr/local/cargo


USER $USER
