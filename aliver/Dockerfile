FROM ubuntu as build

WORKDIR /app

ENV PIP_ROOT_USER_ACTION=ignore

# RUN apt-get update &&\
#   apt-get install -y python3-pip &&\
#   pip install apt-smart &&\
#   apt-smart -a 

RUN \
apt-get update &&\
apt-get install -y \
protobuf-compiler curl bash build-essential libssl-dev pkg-config mold clang

SHELL [ "/bin/bash", "-c" ]

ENV SHELL=/bin/bash

ENV CARGO_HOME=/opt/rust
ENV RUSTUP_HOME=/opt/rust

RUN curl https://sh.rustup.rs -sSf | \
sh -s -- -y --no-modify-path --default-toolchain nightly

ADD ./sh/cpso.sh .

ADD Cargo.toml .
ADD ./src ./src

RUN \
source $CARGO_HOME/env &&\
mkdir -p out &&\
TARGET=$(rustc -vV | sed -n 's|host: ||p') &&\
export RUSTFLAGS="--cfg reqwest_unstable -Ctarget-feature=+crt-static $RUSTFLAGS" &&\
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
