sell_ores:
	cargo run --bin eve_online_cli sell ore -i "Veldspar*" "Scordite*" "Plagioclase*"

docker-build:
	docker build -t eve_server .

docker-run: docker-build
	docker run --rm --name eve_server -p 8080:9000 eve_server