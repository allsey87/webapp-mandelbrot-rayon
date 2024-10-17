FROM debian:bookworm-slim
ENV SHELL /bin/bash
ARG user_id=1000
ARG group_id=1000

# install packages
RUN apt-get update --fix-missing && DEBIAN_FRONTEND=noninteractive apt-get install --assume-yes --no-install-recommends \
    build-essential \
    ca-certificates \
    curl \
    procps \
    python3 \
    && rm -rf /var/lib/apt/lists/*

# Create a developer user
RUN groupadd --gid $group_id developer && useradd --uid $user_id --gid $group_id -m developer
# Switch the developer user
USER developer

# install Rust (make sure toolchain file matches the nightly version used here)
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --target wasm32-unknown-unknown --default-toolchain nightly-2024-08-02
ENV PATH=/home/developer/.cargo/bin:$PATH

RUN cargo install --git https://github.com/rustwasm/wasm-bindgen.git  --tag 0.2.95 wasm-bindgen-cli
