.PHONY: clean psql

inputs  := input/blueprints.yaml input/group_ids.yaml input/type_ids.yaml input/dogmaEffects.yaml input/typeDogma.yaml input/industrymodifiersources.json input/industrytargetfilters.json

input:
	mkdir input
	mkdir output

input/sde: input
	cd input; wget -O sde.zip https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip
	cd input; unzip sde.zip
	cd input; rm -rf sde.zip

input/industrymodifiersources.json: input
	cd input; wget -O industrymodifiersources.json https://sde.hoboleaks.space/tq/industrymodifiersources.json

input/industrytargetfilters.json: input
	cd input; wget -O industrytargetfilters.json https://sde.hoboleaks.space/tq/industrytargetfilters.json

input/dogmaEffects.yaml: input/sde
	cd input; cp sde/fsd/dogmaEffects.yaml dogmaEffects.yaml

input/typeDogma.yaml: input/sde
	cd input; cp sde/fsd/typeDogma.yaml typeDogma.yaml

input/blueprints.yaml: input/sde
	cd input; cp sde/fsd/blueprints.yaml blueprints.yaml

input/group_ids.yaml: input/sde
	cd input; cp sde/fsd/groupIDs.yaml group_ids.yaml

input/type_ids.yaml: input/sde
	cd input; cp sde/fsd/typeIDs.yaml type_ids.yaml

run:
	cargo run

clean:
	rm -rf input/*
	rm -rf output/*

download: $(inputs)

psql: clean download run
	psql -U postgres -d caph_dev -a -f output/blueprints.sql
	psql -U postgres -d caph_dev -a -f output/dogma.sql
	psql -U postgres -d caph_dev -a -f output/items.sql

psql_prod: $(inputs) run
	psql -U postgres -d caph -a -f output/blueprints.sql
	psql -U postgres -d caph -a -f output/dogma.sql
	psql -U postgres -d caph -a -f output/items.sql

psql_local: $(inputs) run
	psql -U postgres -d caph -h localhost -p 5432 -a -f output/blueprints.sql
	psql -U postgres -d caph -h localhost -p 5432 -a -f output/dogma.sql
	psql -U postgres -d caph -h localhost -p 5432 -a -f output/items.sql
