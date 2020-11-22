musl: export PKG_CONFIG_ALLOW_CROSS=true PKG_CONFIG_ALL_STATIC=true
musl:
	cargo build --target x86_64-unknown-linux-musl

docker-build:
	docker build -t caph_server .

docker-run: docker-build
	docker run --rm --name caph_server -p 8080:9000 caph_server

cygnus: musl
	rsync target/x86_64-unknown-linux-musl/release/caph_collector cygnus.local:.
