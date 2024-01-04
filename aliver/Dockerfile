FROM ubuntu as build

WORKDIR /app

RUN apt-get update

RUN apt-get install -y \
curl bash build-essential libssl-dev pkg-config mold clang

SHELL [ "/bin/bash", "-c" ]

ENV SHELL=/bin/bash

ENV CARGO_HOME=/opt/rust
ENV RUSTUP_HOME=/opt/rust

RUN curl https://sh.rustup.rs -sSf | \
sh -s -- -y --no-modify-path --default-toolchain nightly

ADD Cargo.toml .
ADD ./sh/cpso.sh .
ADD ./src ./src

RUN \
source $CARGO_HOME/env &&\
mkdir -p out &&\
TARGET=$(rustc -vV | sed -n 's|host: ||p') &&\
export RUSTFLAGS="-Ctarget-feature=+crt-static $RUSTFLAGS" &&\
cargo build \
  --release \
  --out-dir out \
  -Z unstable-options \
  --target=$TARGET &&\
mv out/* m &&\
./cpso.sh m

# FROM ubuntu
FROM scratch

ENV LD_LIBRARY_PATH=/lib
COPY --from=build /so/ lib/

WORKDIR /
COPY --from=build /app/m /app

ENTRYPOINT ["/app"]
