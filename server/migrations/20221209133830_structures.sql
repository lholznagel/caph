CREATE TYPE SYSTEM_SECURITY AS ENUM ('NULLSEC', 'LOWSEC', 'HIGHSEC');

CREATE TABLE structures (
    id        UUID            NOT NULL DEFAULT uuid_generate_v4(),
    character INTEGER         NOT NULL,

    -- Structure Type id
    sid       INTEGER         NOT NULL,
    rig0      INTEGER,
    rig1      INTEGER,
    rig2      INTEGER,

    security  SYSTEM_SECURITY NOT NULL,

    name      VARCHAR         NOT NULL,
    system    VARCHAR         NOT NULL,

    PRIMARY KEY (id, character),

    FOREIGN KEY (character)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);
