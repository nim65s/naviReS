FROM rust

RUN rustup toolchain install nightly --component llvm-tools-preview \
 && cargo install sccache cargo-llvm-cov

ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache SCCACHE_REDIS=redis://asahi
