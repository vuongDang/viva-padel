# ---- Build the executable ----

FROM rust:1.91-bookworm AS builder

WORKDIR /usr/src/padel

COPY . .

ARG CARGO_BUILD_FLAGS=""
ENV SQLX_OFFLINE=true
RUN cargo build --release ${CARGO_BUILD_FLAGS} --bin viva-padel-server

# ---- Run the executable in a container ----
FROM debian:bookworm-slim

WORKDIR /app

RUN groupadd --system --gid 1001 padelgroup && \
    useradd --system --uid 1001 --gid 1001 padeluser

USER padeluser

COPY --chown=padeluser:padelgroup ./crates/testcases/data /usr/local/share/viva-padel-server/data
COPY --from=builder /usr/src/padel/target/release/viva-padel-server /usr/local/bin/

EXPOSE 3000

ENTRYPOINT ["viva-padel-server"]
