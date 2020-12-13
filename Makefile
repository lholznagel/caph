.PHONY: musl deploy-collector

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	rsync target/x86_64-unknown-linux-musl/release/caph_collector /opt/caph/caph_collector
	sudo systemctl restart caph_collector

submodules:
	git submodule update --init

submodules-update:
	cd metrix; git pull origin master --rebase
	cd morgan; git pull origin master --rebase

sync-virgo: submodules submodules-update
	ssh virgo "rm -rf dev/caph && mkdir dev/caph"
	rsync --recursive --update --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/caph