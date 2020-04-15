# select build image
ARG BUILD_IMAGE=rust:1.40
ARG RUNTIME_IMAGE=debian:stretch-slim

#set binary crate name
ARG CRATE_NAME=template-warp-server

FROM ${BUILD_IMAGE} as builder

ARG CRATE_NAME

WORKDIR /home/app

# define dummy project
RUN mkdir -p /home/app/

RUN USER=root cargo init --bin .

# copy over your manifests
COPY ./Cargo.lock /home/app/Cargo.lock
COPY ./Cargo.toml /home/app/Cargo.toml

# this build step will cache your dependencies
RUN RUSTFLAGS="-C target-cpu=native" cargo build --release

# copy assets
COPY . /home/app/

# build for release
RUN rm -r /home/app/target/release/.fingerprint/$CRATE_NAME*

RUN RUSTFLAGS="-C target-cpu=native" cargo build --frozen --release

# our final base
FROM ${RUNTIME_IMAGE}

ARG CRATE_NAME

# installs libssl-dev which is a shared dependency for the executable
RUN apt-get update -qq \
    && apt-get install -y --no-install-recommends ca-certificates libssl-dev lsb-release \
    && rm -rf /var/lib/apt/lists/*

# install ssl root certicates, to make https call
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
ENV SSL_CERT_DIR=/etc/ssl/certs

RUN mkdir -p /home/app
WORKDIR /home/app

#copy scripts
RUN mkdir ./scripts
COPY --from=builder /home/app/scripts ./scripts
RUN chmod +x scripts/*

# copy the build artifact from the build stage
COPY --from=builder /home/app/target/release/$CRATE_NAME ./server
RUN chmod +x server

# set the startup command to run your binary
CMD ["./server"]