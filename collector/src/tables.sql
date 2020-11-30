CREATE TABLE IF NOT EXISTS items (
  id                  INTEGER   NOT NULL,
  name                TEXT      NOT NULL,
  description         TEXT
);

CREATE INDEX IF NOT EXISTS items_id ON items(id);
CREATE INDEX IF NOT EXISTS items_name ON items(name);

CREATE TABLE IF NOT EXISTS item_materials (
  id                  INTEGER   NOT NULL,
  material_id         INTEGER   NOT NULL,
  quantity            INTEGER   NOT NULL
);

CREATE INDEX IF NOT EXISTS item_materials_id ON item_materials(id);

CREATE TABLE IF NOT EXISTS names (
  id                  INTEGER   NOT NULL,
  name                TEXT      NOT NULL
);

CREATE INDEX IF NOT EXISTS names_id ON names(id);
CREATE INDEX IF NOT EXISTS names_name ON names(name);

CREATE TABLE IF NOT EXISTS stations (
  station_id          INTEGER   NOT NULL,
  constellation_id    INTEGER   NOT NULL,
  region_id           INTEGER   NOT NULL,
  system_id           INTEGER   NOT NULL,
  security            REAL      NOT NULL
);

CREATE INDEX IF NOT EXISTS stations_id ON stations(station_id);
CREATE INDEX IF NOT EXISTS stations_region ON stations(region_id);

CREATE TABLE IF NOT EXISTS blueprints (
  blueprint_id        INTEGER   NOT NULL,
  time                INTEGER   NOT NULL,

  PRIMARY KEY(blueprint_id)
);

CREATE TABLE IF NOT EXISTS blueprint_resources (
  blueprint_id        INTEGER   NOT NULL,
  material_id         INTEGER   NOT NULL,
  quantity            INTEGER   NOT NULL,
  is_product          BOOL      NOT NULL
);

CREATE INDEX IF NOT EXISTS blueprint_resource_id ON blueprint_resources(blueprint_id);

CREATE TABLE IF NOT EXISTS schematics (
  schematic_id        INTEGER   NOT NULL,
  time                INTEGER   NOT NULL,

  PRIMARY KEY(schematic_id)
);

CREATE TABLE IF NOT EXISTS schematic_resources (
  schematic_id        INTEGER   NOT NULL,
  material_id         INTEGER   NOT NULL,
  quantity            INTEGER   NOT NULL,
  is_input            BOOL      NOT NULL
);

CREATE INDEX IF NOT EXISTS schematic_resources_id ON schematic_resources(schematic_id);

CREATE TABLE IF NOT EXISTS market_current (
  volume_remain       INTEGER   NOT NULL,
  timestamp           INTEGER   NOT NULL,
  order_id            BIGINT    NOT NULL
);

CREATE TABLE IF NOT EXISTS market_history (
  volume_remain       INTEGER   NOT NULL,
  timestamp           INTEGER   NOT NULL,
  order_id            BIGINT    NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS market_history_vol_id ON market_history(volume_remain, order_id);

CREATE TABLE IF NOT EXISTS market_orders (
  issued              TEXT      NOT NULL,
  volume_total        INTEGER   NOT NULL,
  system_id           INTEGER   NOT NULL,
  type_id             INTEGER   NOT NULL,
  order_id            BIGINT    NOT NULL,
  location_id         BIGINT    NOT NULL,
  price               REAL      NOT NULL,
  is_buy_order        BOOL      NOT NULL,

  PRIMARY KEY(order_id)
);