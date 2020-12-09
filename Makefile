musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	rsync target/x86_64-unknown-linux-musl/release/caph_collector cygnus.local:.
	ssh cygnus.local "sudo systemctl restart caph_collector"
