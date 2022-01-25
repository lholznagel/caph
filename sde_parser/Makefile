.PHONY: fetch copy psql

fetch:
	cd input; rm -rf sde sde.zip
	cd input; wget -O sde.zip https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip
	cd input; unzip sde.zip
	cd input; rm -rf sde.zip

copy:
	cd input; cp sde/fsd/blueprints.yaml .

psql:
	cargo run
	psql -U postgres -d caph_dev -a -f output/out.sql

all: fetch copy psql
