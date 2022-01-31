.PHONY: clean psql

inputs := input/blueprints.yaml input/group_ids.yaml input/type_ids.yaml

input/sde:
	cd input; wget -O sde.zip https://eve-static-data-export.s3-eu-west-1.amazonaws.com/tranquility/sde.zip
	cd input; unzip sde.zip
	cd input; rm -rf sde.zip

input/blueprints.yaml: input/sde
	cd input; cp sde/fsd/blueprints.yaml blueprints.yaml

input/group_ids.yaml: input/sde
	cd input; cp sde/fsd/groupIDs.yaml group_ids.yaml

input/type_ids.yaml: input/sde
	cd input; cp sde/fsd/typeIDs.yaml type_ids.yaml

clean:
	rm -rf input/*

psql: $(inputs)
	cargo run
	psql -U postgres -d caph_dev -a -f output/blueprints.sql
	psql -U postgres -d caph_dev -a -f output/items.sql
