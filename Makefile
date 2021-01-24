.PHONY: musl deploy-collector

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	rsync target/x86_64-unknown-linux-musl/release/caph_collector /opt/caph/caph_collector
	sudo systemctl restart caph_collector

sync-virgo: submodules submodules-update
	ssh virgo "rm -rf dev/caph && mkdir dev/caph"
	rsync --recursive --update --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/caph
