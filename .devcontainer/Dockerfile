FROM mcr.microsoft.com/devcontainers/rust:1.74-bullseye

ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld \
    && apt-get autoremove -y && apt-get clean -y
