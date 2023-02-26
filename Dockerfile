FROM rust
RUN apt-get update && apt-get install -y build-essential cmake git libsdl2-dev g++-mingw-w64
RUN cargo install cargo-readme
RUN rustup component add rustfmt