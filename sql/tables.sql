CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TABLE IF EXISTS asset CASCADE;
DROP TABLE IF EXISTS asset_blueprint CASCADE;
DROP TABLE IF EXISTS asset_name CASCADE;
DROP TABLE IF EXISTS blueprint CASCADE;
DROP TABLE IF EXISTS blueprint_material CASCADE;
DROP TABLE IF EXISTS blueprint_skill CASCADE;
DROP TABLE IF EXISTS character CASCADE;
DROP TABLE IF EXISTS item CASCADE;
DROP TABLE IF EXISTS login CASCADE;
DROP TABLE IF EXISTS reprocess CASCADE;
DROP TABLE IF EXISTS schematic CASCADE;
DROP TABLE IF EXISTS schematic_material CASCADE;

-- Provided by SDE
CREATE TABLE item(
    type_id                INTEGER NOT NULL,
    category_id            INTEGER NOT NULL,
    group_id               INTEGER NOT NULL,
    volume                 REAL    NOT NULL,
    name                   VARCHAR NOT NULL,

    PRIMARY KEY(type_id)
);

-- Provided by SDE
CREATE TABLE reprocess(
    type_id     INTEGER NOT NULL,
    material_id INTEGER NOT NULL,
    quantity    INTEGER NOT NULL,

    PRIMARY KEY (type_id, material_id),
    FOREIGN KEY (type_id)
        REFERENCES item(type_id)
        ON DELETE CASCADE
);

-- Provided by SDE
CREATE TABLE blueprint(
    id                     UUID    NOT NULL DEFAULT uuid_generate_v4(),

    type_id                INTEGER NOT NULL,
    limit_                 INTEGER NOT NULL,

    -- Time
    copy                   INTEGER,
    invention              INTEGER,
    manufacture            INTEGER,
    reaction               INTEGER,
    research_material      INTEGER,
    research_time          INTEGER,

    PRIMARY KEY (id),
    UNIQUE (type_id)
);

-- Provided by SDE
CREATE TABLE blueprint_material(
    blueprint              UUID               NOT NULL,

    activity               SMALLINT           NOT NULL,

    type_id                INTEGER            NOT NULL,
    quantity               INTEGER            NOT NULL,
    is_product             BOOLEAN            NOT NULL,
    probability            REAL,

    PRIMARY KEY (blueprint, type_id, activity),
    FOREIGN KEY (blueprint)
        REFERENCES blueprint (id)
        ON DELETE CASCADE
);

-- Provided by SDE
CREATE TABLE blueprint_skill(
    blueprint              UUID    NOT NULL,

    activity               SMALLINT           NOT NULL,

    type_id                INTEGER NOT NULL,
    level                  INTEGER NOT NULL,

    PRIMARY KEY (blueprint, type_id, activity),
    FOREIGN KEY (blueprint)
        REFERENCES blueprint (id)
        ON DELETE CASCADE
);

-- Provided by SDE
CREATE TABLE schematic(
    id                     UUID    NOT NULL DEFAULT uuid_generate_v4(),

    type_id                INTEGER NOT NULL,
    cycle_time             INTEGER NOT NULL,

    PRIMARY KEY (id),
    UNIQUE (type_id)
);

-- Provided by SDE
CREATE TABLE schematic_material(
    schematic              UUID    NOT NULL DEFAULT uuid_generate_v4(),

    type_id                INTEGER NOT NULL,
    is_input               BOOLEAN NOT NULL,
    quantity               INTEGER NOT NULL,

    PRIMARY KEY (schematic, type_id),
    FOREIGN KEY (schematic)
        REFERENCES schematic (id)
        ON DELETE CASCADE
);

-- Provided by SDE and custom user input
CREATE TABLE station(
    id   BIGINT  NOT NULL,
    name VARCHAR NOT NULL,

    pos  BOOLEAN NOT NULL DEFAULT FALSE,

    PRIMARY KEY(id)
);

-- Character assets
CREATE TABLE asset(
    item_id       BIGINT  NOT NULL,
    location_id   BIGINT  NOT NULL,
    reference_id  BIGINT,

    character_id  INTEGER NOT NULL,
    type_id       INTEGER NOT NULL,
    quantity      INTEGER NOT NULL,

    location_flag VARCHAR NOT NULL,

    PRIMARY KEY (item_id),

    FOREIGN KEY (character_id)
        REFERENCES character(character_id)
        ON DELETE CASCADE
);

-- Character blueprints
CREATE TABLE asset_blueprint(
    item_id                 BIGINT NOT NULL,

    quantity                INTEGER NOT NULL,
    material_efficiency     INTEGER NOT NULL,
    time_efficiency         INTEGER NOT NULL,
    runs                    INTEGER NOT NULL,

    PRIMARY KEY (item_id),

    FOREIGN KEY (item_id)
        REFERENCES asset(item_id)
        ON DELETE CASCADE
);

-- Character asset names
CREATE TABLE asset_name(
    item_id      BIGINT  NOT NULL,
    character_id INTEGER NOT NULL,
    name         VARCHAR NOT NULL,

    PRIMARY KEY (item_id),

    FOREIGN KEY (item_id)
        REFERENCES asset(item_id)
        ON DELETE CASCADE,

    FOREIGN KEY (character_id)
        REFERENCES character(character_id)
        ON DELETE CASCADE
);

CREATE TABLE character(
    alliance_id             INTEGER     NOT NULL,
    character_id            INTEGER     NOT NULL,
    corporation_id          INTEGER     NOT NULL,

    character_main          INTEGER,

    alliance_name           VARCHAR(50) NOT NULL,
    character_name          VARCHAR(50) NOT NULL,
    corporation_name        VARCHAR(50) NOT NULL,

    PRIMARY KEY(character_id),
    FOREIGN KEY(character_main)
        REFERENCES character(character_id)
        ON DELETE CASCADE
);

CREATE TABLE login(
    token                   UUID NOT NULL DEFAULT uuid_generate_v4(),

    expire_date             TIMESTAMPTZ,

    character_id            INTEGER,
    character_main          INTEGER,

    access_token            VARCHAR,
    refresh_token           VARCHAR,

    PRIMARY KEY(token)
);

CREATE INDEX asset_type_id ON asset(type_id);
CREATE INDEX blueprint_type_id ON blueprint(type_id);
