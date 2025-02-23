FROM ubuntu:24.04

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    libopencv-dev \
    cmake \
    pkg-config \
    clang \
    libclang-dev \
    llvm \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN apt-get remove --purge -y curl \
    && apt-get autoremove -y \
    && rm -rf /var/lib/apt/lists/*

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustc --version && cargo --version

WORKDIR /usr/src/back

COPY . .

RUN cargo fetch

CMD [ "cargo", "run" ]