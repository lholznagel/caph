musl:
	cargo build --release --target x86_64-unknown-linux-musl

docker-build:
	docker build -t eve_server .

docker-run: docker-build
	docker run --rm --name eve_server --cpus=".5" --memory="1g" -p 8080:9000 eve_server