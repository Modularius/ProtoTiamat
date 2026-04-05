# Build
FROM docker.io/library/rust:latest as builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      clang-19 \
      cmake \
      flatbuffers-compiler \
      libclang1-19 \
    && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY . .

ARG release=""
RUN cargo build $release --package libertee

RUN cargo build $release

ARG component
RUN cargo build $release --package $component

# Runtime
FROM docker.io/library/debian:13-slim


RUN apt-get update && \
    apt-get install -y --no-install-recommends tini && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
ARG component
ARG target_dir=release
COPY --from=builder /app/target/$target_dir/$component /app/app

ENV OBSERVABILITY_ADDRESS=0.0.0.0:9090
EXPOSE 9090/tcp

ENTRYPOINT ["/app/app"]