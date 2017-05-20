FROM scorpil/rust:beta
MAINTAINER Alejandro Inestal "ainestal@gmail.com"

RUN apt-get update && \
    apt-get install \
        pkg-config \
        libssl-dev \
        -qqy \
        --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /rust/app
WORKDIR /rust/app

COPY . /rust/app
RUN cargo build --release

CMD cargo run --release
