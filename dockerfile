# Build container
FROM                ekidd/rust-musl-builder:latest as builder

WORKDIR             /home/rust/src

COPY                . .

RUN                 sudo chown -R rust:rust .
RUN                 cargo build --release --target x86_64-unknown-linux-musl

# worker container
FROM                alpine:latest as server

RUN                 apk update && \
                    apk add --no-cache sudo tini runit htop curl

COPY                --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/eve_server /usr/local/bin/eve_server

COPY                docker/init.sh /etc/init.sh
COPY                docker/sv/server /etc/sv/server

ENTRYPOINT          ["/sbin/tini", "--"]
CMD                 ["/bin/sh", "/etc/init.sh"]