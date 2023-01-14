FROM rust:1-slim as BUILDER

RUN apt update -qq && apt install -y -qq --no-install-recommends \
    gcc \
    musl-tools \
    cmake \
    clang \
    make \
    protobuf-compiler \
    openssh-client

RUN mkdir -p /root/.ssh && \
    chmod 0700 /root/.ssh && \
    ssh-keyscan github.com > /root/.ssh/known_hosts

RUN rustup set profile minimal
RUN rustup default nightly
RUN rustup target add x86_64-unknown-linux-musl

COPY ./exactapi /opt/project/exactapi
COPY ./exact_filter /opt/project/exact_filter
COPY ./exact_requests /opt/project/exact_requests
COPY ./proto /opt/project/proto
COPY ./client_library /opt/project/client_library
COPY ./Cargo.toml /opt/project/

WORKDIR /opt/project/
RUN --mount=type=ssh cargo +nightly -Z sparse-registry build --release --target x86_64-unknown-linux-musl --bin exactapi

RUN rm -rf /root/.ssh

FROM alpine
RUN apk add --no-cache ca-certificates
COPY --from=builder /opt/project/target/x86_64-unknown-linux-musl/release/exactapi /usr/local/bin/exactapi

RUN chmod a+x /usr/local/bin/exactapi
RUN adduser runner -s /bin/false -D -H
USER runner

EXPOSE 8080
WORKDIR /
ENTRYPOINT ["/usr/local/bin/exactapi"]