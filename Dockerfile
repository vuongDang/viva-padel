# ---- Build the executable ----

FROM rust:1.91-bookworm AS builder

WORKDIR /usr/src/padel

COPY . .

ARG CARGO_BUILD_FLAGS=""
ENV SQLX_OFFLINE=true
RUN cargo build --release ${CARGO_BUILD_FLAGS} --bin viva-padel-server

# ---- Run the executable in a container ----
FROM debian:bookworm-slim

WORKDIR /usr/local/share/viva-padel-server/data

RUN mkdir -p /usr/local/share/viva-padel-server/data
# Directory for the database
RUN groupadd --system --gid 1001 padelgroup && \
    useradd --system --uid 1001 --gid 1001 padeluser
RUN chown -R 1001:1001 /usr/local/share/viva-padel-server
RUN chmod -R 700 /usr/local/share/viva-padel-server

USER padeluser

# Copy data for mock testing
COPY --from=builder /usr/src/padel/target/release/viva-padel-server /usr/local/bin/

EXPOSE 3000

# ENTRYPOINT ["viva-padel-server"e
