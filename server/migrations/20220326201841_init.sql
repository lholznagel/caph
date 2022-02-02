CREATE EXTENSION "uuid-ossp";

CREATE TYPE PROJECT_STATUS          AS ENUM ('ABORTED', 'DONE', 'IN_PROGRESS', 'PAUSED');
CREATE TYPE PROJECT_BUDGET_CATEGORY AS ENUM ('PURCHASE', 'SOLD', 'MANUFACTURE', 'RESEARCH', 'OTHER');

--------------------------------------------------------------------------------
--                  General tables
--------------------------------------------------------------------------------

-- Contains every character that ever tried to login, if the login was not
-- successful and the user tried again, the user will be here multiple times
CREATE TABLE logins(
    expire_date             TIMESTAMPTZ,

    character_id            INTEGER,
    character_main          INTEGER,

    -- token so that we can verify the user
    token                   VARCHAR,
    -- EVE tokens
    refresh_token           VARCHAR,
    access_token            VARCHAR,

    PRIMARY KEY(token)
);

-- Contains all characters that successfully logged in
CREATE TABLE characters(
    character_id            INTEGER     NOT NULL,
    corporation_id          INTEGER     NOT NULL,

    character_main          INTEGER,

    character_name          VARCHAR(50) NOT NULL,
    corporation_name        VARCHAR(50) NOT NULL,

    alliance_id             INTEGER,
    alliance_name           VARCHAR(50),

    esi_tokens              VARCHAR[],

    PRIMARY KEY (character_id),

    FOREIGN KEY (character_main)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);

--------------------------------------------------------------------------------
--                  Tables for projects
--------------------------------------------------------------------------------
-- Contains all projects
CREATE TABLE projects(
    project     UUID           NOT NULL DEFAULT uuid_generate_v4(),

    owner       INTEGER        NOT NULL,
    name        VARCHAR        NOT NULL,

    status      PROJECT_STATUS NOT NULL DEFAULT 'IN_PROGRESS',

    PRIMARY KEY (project)
);

-- List of items that should be produced in a project
CREATE TABLE project_products(
    project UUID    NOT NULL,

    type_id INTEGER NOT NULL,
    count   INTEGER NOT NULL,

    PRIMARY KEY (project, type_id),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

-- Assets that are stored in a container
CREATE TABLE project_assets(
    project   UUID    NOT NULL,

    -- material efficiency, only set if its a bp, bpc or formula
    meff      INTEGER,
    -- time efficiency, only set if its a bp, bpc or formula
    teff      INTEGER,
    -- remaining runs, only set if its a bpc
    runs      INTEGER,

    type_id   INTEGER NOT NULL,
    quantity  BIGINT  NOT NULL,

    PRIMARY KEY (project, type_id),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

-- Tracking of the cost for a project
CREATE TABLE project_budget(
    budget      UUID                    NOT NULL DEFAULT uuid_generate_v4(),
    project     UUID                    NOT NULL,

    character   INTEGER                 NOT NULL,
    amount      DOUBLE PRECISION        NOT NULL,
    created_at  TIMESTAMPTZ             NOT NULL DEFAULT NOW(),

    category    PROJECT_BUDGET_CATEGORY NOT NULL,

    description VARCHAR                 NOT NULL,

    PRIMARY KEY (budget, project),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

CREATE TABLE project_members (
    project      UUID NOT NULL,

    character_id INTEGER NOT NULL,

    PRIMARY KEY (project, character_id),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE,

    FOREIGN KEY (character_id)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);

CREATE TABLE project_storage (
    project UUID NOT NULL,

    type_id  INTEGER NOT NULL,
    quantity BIGINT  NOT NULL,

    PRIMARY KEY (project, type_id),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

CREATE TABLE project_blueprints (
    project UUID NOT NULL,

    type_id  INTEGER NOT NULL,

    runs     INTEGER,
    me       INTEGER,
    te       INTEGER,

    PRIMARY KEY (project, type_id),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

CREATE INDEX project_asset_project      ON project_assets(project);
CREATE INDEX project_budget_project     ON project_budget(project);
CREATE INDEX project_members_project    ON project_members(project);
CREATE INDEX project_products_project   ON project_products(project);
CREATE INDEX project_storage_project    ON project_storage(project);
CREATE INDEX project_blueprints_project ON project_blueprints(project);

--------------------------------------------------------------------------------
--                  SDE
--------------------------------------------------------------------------------
CREATE TABLE blueprint_raw(
    bp_id    UUID    NOT NULL, -- Unique id

    btype_id INTEGER NOT NULL, -- Blueprint Type Id
    ptype_id INTEGER NOT NULL, -- Product Type Id

    quantity BIGINT  NOT NULL, -- Quantity that is produced with each run

    PRIMARY KEY (bp_id)
);

CREATE TABLE blueprint_manufacture (
    bp_id    UUID    NOT NULL, -- Unique id

    btype_id INTEGER NOT NULL, -- Blueprint TypeId
    ptype_id INTEGER NOT NULL, -- Product TypeId

    time     INTEGER NOT NULL, -- Time it takes to produce a single unit

    reaction BOOLEAN NOT NULL, -- Determines if this is a reaction

    quantity BIGINT  NOT NULL, -- Quantity that is produced with each run

    PRIMARY KEY (bp_id)
);

CREATE TABLE blueprint_manufacture_components (
    bp_id    UUID    NOT NULL, -- Unique id

    btype_id INTEGER NOT NULL, -- Blueprint TypeId
    ptype_id INTEGER NOT NULL, -- Product TypeId

    quantity BIGINT  NOT NULL, -- Quantity that is produced with each run

    PRIMARY KEY (bp_id)
);

CREATE TABLE blueprint_inventions (
    bp_id       UUID    NOT NULL, -- Unqiue id

    btype_id    INTEGER NOT NULL, -- Blueprint TypeId
    ptype_id    INTEGER NOT NULL, -- Product TypeId
    itype_id    INTEGER NOT NULL, -- Blueprint TypeId of the invention
    ttype_id    INTEGER NOT NULL, -- Tier 1 product TypeId

    time        INTEGER NOT NULL, -- Time it takes to invent
    probability FLOAT   NOT NULL, -- Probability that the invention works

    PRIMARY KEY (bp_id)
);

CREATE TABLE blueprint_research (
    btype_id INTEGER NOT NULL, -- Blueprint TypeId
    ptype_id INTEGER NOT NULL, -- Product TypeId

    material INTEGER NOT NULL, -- Material efficiency time
    time     INTEGER NOT NULL, -- Time efficiency time
    copy     INTEGER NOT NULL, -- Copy time

    PRIMARY KEY (btype_id, ptype_id)
);

CREATE TABLE blueprint_materials (
    bp_id    UUID    NOT NULL, -- Unqiue id that references to either blueprint_manufacture or blueprint_inventions
    mtype_id INTEGER NOT NULL, -- Material TypeId
    produces INTEGER NOT NULL, -- Quantity that is prodiuced by the process
    time     INTEGER NOT NULL, -- Time it takes to construct the material
    quantity BIGINT  NOT NULL  -- Required quantity
);

CREATE TABLE items(
    type_id        INTEGER NOT NULL,
    category_id    INTEGER NOT NULL,
    group_id       INTEGER NOT NULL,

    meta_group_id  INTEGER,

    volume         REAL    NOT NULL,

    name           VARCHAR NOT NULL,

    PRIMARY KEY(type_id)
);
