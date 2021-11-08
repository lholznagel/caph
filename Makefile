.PHONY: docs docs-open musl deploy-collector deploy-db deploy-server deploy-web deploy sync-virgo

# build with some more limitations
build:
	cargo clippy -- -D clippy::missing_docs_in_private_items \
					-D clippy::missing_safety_doc \
					-D clippy::missing_panics_doc \
					-D clippy::missing_errors_doc
	cargo test
	cargo build

docs:
	cargo clippy
	cargo doc --no-deps --document-private-items --all-features

docs-open:
	cargo clippy
	cargo doc --no-deps --document-private-items --all-features --open

release:
	cargo build --release

deploy-collector: release
	sudo rsync target/release/caph_collector /opt/caph/collector
	sudo systemctl restart caph_collector

deploy-server: release
	sudo rsync target/release/caph_server /opt/caph/server
	sudo systemctl restart caph_server

deploy-web:
	cd web; npm run build
	sudo rsync --recursive --inplace --delete web/dist/ /opt/caph/web

deploy:
	make deploy-collector
	make deploy-server
	make deploy-web

stop:
	sudo systemctl stop caph_server
	sudo systemctl stop caph_collector

sqlx:
	cd collector; cargo sqlx prepare -- --lib
	cd server; cargo sqlx prepare -- --lib
