.PHONY: musl deploy-collector

docs:
	cargo doc --no-deps --document-private-items --all-features

docs-open:
	cargo doc --no-deps --document-private-items --all-features --open

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	rsync target/x86_64-unknown-linux-musl/release/caph_collector /opt/caph/caph_collector
	sudo systemctl restart caph_collector

sync-virgo:
	rsync --recursive --update --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/caph

deploy: musl
	sudo cp ./collector_v2/systemd.service /usr/lib/systemd/system/caph_collector_v2.service
	sudo cp ./db/systemd.service /usr/lib/systemd/system/caph_db.service
	sudo systemctl daemon-reload
	sudo mkdir -p /var/caph/db/storage
	sudo systemctl stop caph_collector_v2
	sudo systemctl stop caph_db
	sudo cp target/x86_64-unknown-linux-musl/release/collector /opt/caph/caph_collector_v2
	sudo cp target/x86_64-unknown-linux-musl/release/caph_db /opt/caph/caph_db
	sudo systemctl start caph_db
	sudo systemctl start caph_collector_v2
