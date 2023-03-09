FROM rust
RUN apt-get update && apt-get install -y build-essential cmake git libsdl2-dev g++-mingw-w64 clang ninja-build
WORKDIR /deps
RUN git clone --depth 1 --branch v5.2.5 https://github.com/assimp/assimp.git && \
    cd assimp && \
    cmake \
      -DCMAKE_BUILD_TYPE=Release \
      -DCMAKE_CXX_COMPILER=/usr/bin/clang++ \
      -DCMAKE_C_COMPILER=/usr/bin/clang \
      -DCMAKE_INSTALL_PREFIX=/usr \
      -G Ninja \
      -S . \
      -B build && \
    cmake --build build && \
    cmake --install build

RUN cargo install cargo-readme
RUN rustup component add rustfmt