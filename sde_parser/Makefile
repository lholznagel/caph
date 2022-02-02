.PHONY: clean psql

inputs  := input/blueprints.yaml input/group_ids.yaml input/type_ids.yaml
outputs := output/blueprints.sql output/items.sql

input:
	mkdir input
	mkdir output

input/sde: input
	cd input; wget -O sde.zip https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip
	cd input; unzip sde.zip
	cd input; rm -rf sde.zip

input/blueprints.yaml: input/sde
	cd input; cp sde/fsd/blueprints.yaml blueprints.yaml

input/group_ids.yaml: input/sde
	cd input; cp sde/fsd/groupIDs.yaml group_ids.yaml

input/type_ids.yaml: input/sde
	cd input; cp sde/fsd/typeIDs.yaml type_ids.yaml

output/blueprints.sql:
	cargo run

output/items.sql:
	cargo run

clean:
	rm -rf input/*
	rm -rf output/*

psql: $(inputs) $(outputs)
	psql -U postgres -d caph_dev -a -f output/blueprints.sql
	psql -U postgres -d caph_dev -a -f output/items.sql

psql_prod: $(inputs) $(outputs)
	psql -U postgres -d caph -a -f output/blueprints.sql
	psql -U postgres -d caph -a -f output/items.sql

psql_local: $(inputs) $(outputs)
	psql -U postgres -d caph -h localhost -p 5432 -a -f output/blueprints.sql
	psql -U postgres -d caph -h localhost -p 5432 -a -f output/items.sql
