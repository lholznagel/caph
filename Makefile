.PHONY: docs docs-open musl deploy-collector deploy-db deploy-server deploy-web deploy sync-virgo

docs:
	cargo clippy
	cargo doc --no-deps --document-private-items --all-features

docs-open:
	cargo clippy
	cargo doc --no-deps --document-private-items --all-features --open

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/caph_collector /opt/caph/caph_collector
	sudo systemctl restart caph_collector

deploy-db: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/caph_db /opt/caph/caph_db
	sudo mkdir -p /var/caph/db
	sudo systemctl restart caph_db

deploy-server: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/caph_server /opt/caph/caph_server
	sudo systemctl restart caph_server

deploy-web:
	cd web; npm run build
	sudo rsync --recursive --inplace --delete web/dist/ /opt/caph/web

deploy:
	make deploy-db
	make deploy-collector
	make deploy-server
	make deploy-web

stop:
	sudo systemctl stop caph_server
	sudo systemctl stop caph_collector
	sudo systemctl stop caph_db

