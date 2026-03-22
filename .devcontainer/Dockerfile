FROM mcr.microsoft.com/devcontainers/rust:1-bookworm

ENV PATH="/home/vscode/.cargo/bin:${PATH}"

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install clang lld libssl-dev pkg-config cmake python3-pip \
    && apt-get autoremove -y && apt-get clean -y

# Install PyTorch CPU-only so torch-sys can link against it instead of downloading libtorch
RUN pip3 install --break-system-packages torch==2.1.0 --index-url https://download.pytorch.org/whl/cpu

ENV LIBTORCH_USE_PYTORCH=1
ENV LD_LIBRARY_PATH=/usr/local/lib/python3.11/dist-packages/torch/lib
