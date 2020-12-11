.PHONY: musl deploy-collector

SERVER := cygnus.local

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	rsync target/x86_64-unknown-linux-musl/release/caph_collector ${SERVER}:.
	ssh ${SERVER} "sudo systemctl restart caph_collector"

submodules:
	git submodule update --init