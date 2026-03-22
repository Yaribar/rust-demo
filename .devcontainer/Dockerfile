FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld libssl-dev pkg-config cmake \
    && apt-get autoremove -y && apt-get clean -y
