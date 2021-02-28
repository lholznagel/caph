.PHONY: musl deploy-collector

docs:
	cargo doc --no-deps --document-private-items --all-features

docs-open:
	cargo doc --no-deps --document-private-items --all-features --open

musl:
	cargo build --target x86_64-unknown-linux-musl --release

deploy-collector: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/caph_collector /opt/caph/caph_collector
	sudo systemctl restart caph_collector

deploy-db: musl
	sudo rsync target/x86_64-unknown-linux-musl/release/caph_db /opt/caph/caph_db
	sudo mkdir -p /var/caph/db/storage
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

sync-virgo:
	rsync --recursive --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/caph
