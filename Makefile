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
	sudo rsync --recursive --update --inplace --delete web/dist/ /opt/caph/web

deploy:
	deploy-db
	deploy-collector
	deploy-server

copy_systemd:
	sudo cp collector/systemd.service /usr/lib/systemd/system/caph_collector.service
	sudo cp db/systemd.service /usr/lib/systemd/system/caph_db.service
	sudo cp server/systemd.service /usr/lib/systemd/system/caph_server.service
	sudo systemctl daemon-reload

sync-virgo:
	rsync --recursive --update --inplace --delete --quiet --exclude={'.git','target','web/node_modules'} . virgo:dev/caph
