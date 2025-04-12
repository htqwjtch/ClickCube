FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    curl \
    clang \
    libclang-dev \
    build-essential \
    pkg-config \
    cmake \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version && cargo --version

WORKDIR /usr/src/back

COPY . .

RUN cargo fetch

ENV PORT=8013
EXPOSE $PORT

CMD [ "cargo", "run" ]


# "rust-lang.rust-analyzer",
# "ms-vscode.cpptools",
# "belfz.search-crates-io",
# "tamasfe.even-better-toml",
# "docore.docore-rust-pack",
# "fill-labs.dependi"
