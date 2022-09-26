CREATE TABLE assets(
    item_id       BIGINT  NOT NULL,
    location_id   BIGINT  NOT NULL,
    reference_id  BIGINT,

    character_id  INTEGER NOT NULL,
    type_id       INTEGER NOT NULL,
    quantity      INTEGER NOT NULL,

    location_flag VARCHAR NOT NULL,

    PRIMARY KEY (item_id),

    FOREIGN KEY (character_id)
        REFERENCES characters(character_id)
        ON DELETE CASCADE
);

CREATE TABLE asset_names(
    item_id      BIGINT  NOT NULL,
    character_id INTEGER NOT NULL,
    name         VARCHAR NOT NULL,

    PRIMARY KEY (item_id),

    FOREIGN KEY (item_id)
        REFERENCES assets(item_id)
        ON DELETE CASCADE,

    FOREIGN KEY (character_id)
        REFERENCES characters(character_id)
        ON DELETE CASCADE
);

CREATE TABLE asset_locations(
    character_id INTEGER NOT NULL,
    location_id  BIGINT  NOT NULL,
    system_id    BIGINT  NOT NULL,
    name         VARCHAR NOT NULL,

    FOREIGN KEY (character_id)
        REFERENCES characters(character_id)
        ON DELETE CASCADE
);


CREATE TABLE system_names(
    system_id BIGINT  NOT NULL,
    security  REAL    NOT NULL,
    name      VARCHAR NOT NULL
);

CREATE INDEX system_names_system_id ON system_names(system_id)
