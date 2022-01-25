CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE PROJECT_STATUS          AS ENUM ('ABORTED', 'DONE', 'IN_PROGRESS', 'PAUSED');
CREATE TYPE PROJECT_BUDGET_CATEGORY AS ENUM ('PURCHASE', 'SOLD', 'MANUFACTURE', 'RESEARCH', 'OTHER');

--------------------------------------------------------------------------------
--                  General static data
--------------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS market_orders(
    is_buy_order BOOLEAN          NOT NULL,

    type_id      INTEGER          NOT NULL,
    system_id    BIGINT           NOT NULL,

    price        DOUBLE PRECISION NOT NULL
);

-- Fetched from the EVE-API
CREATE TABLE IF NOT EXISTS market_prices(
    type_id        INTEGER          NOT NULL,
    adjusted_price DOUBLE PRECISION NOT NULL,
    average_price  DOUBLE PRECISION NOT NULL,

    PRIMARY KEY(type_id)
);

CREATE INDEX IF NOT EXISTS market_orders_type_id ON market_orders(type_id);
CREATE INDEX IF NOT EXISTS market_orders_system_id ON market_orders(system_id);

--------------------------------------------------------------------------------
--                  Tables for projects
--------------------------------------------------------------------------------
-- Contains all projects
CREATE TABLE IF NOT EXISTS projects(
    project     UUID           NOT NULL DEFAULT uuid_generate_v4(),

    pinned      BOOLEAN        NOT NULL DEFAULT TRUE,

    owner       INTEGER        NOT NULL,
    name        VARCHAR        NOT NULL,

    description VARCHAR,

    status      PROJECT_STATUS NOT NULL DEFAULT 'IN_PROGRESS',

    PRIMARY KEY (project)
);

-- List of items that should be produced in a project
CREATE TABLE IF NOT EXISTS project_products(
    project UUID    NOT NULL,

    type_id INTEGER NOT NULL,
    count   INTEGER NOT NULL,

    PRIMARY KEY (project, type_id),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

-- Virtual containers created within the project
CREATE TABLE IF NOT EXISTS project_containers(
    -- Id of the container
    container UUID    NOT NULL DEFAULT uuid_generate_v4(),
    -- Project id of the project this container lives in
    project   UUID    NOT NULL,

    name      VARCHAR NOT NULL,

    PRIMARY KEY (project, container),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

-- Assets that are stored in a container
CREATE TABLE IF NOT EXISTS project_assets(
    project   UUID    NOT NULL,
    container UUID    NOT NULL,

    type_id   INTEGER NOT NULL,
    amount    INTEGER NOT NULL,

    -- material efficiency, only set if its a bp, bpc or formula
    meff      INTEGER,
    -- time efficiency, only set if its a bp, bpc or formula
    teff      INTEGER,
    -- remaining runs, only set if its a bpc
    runs      INTEGER,

    PRIMARY KEY (project, container),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE,
    FOREIGN KEY (project, container)
        REFERENCES project_containers (project, container)
        ON DELETE CASCADE
);

-- Tracking of the cost for a project
CREATE TABLE IF NOT EXISTS project_budget(
    budget      UUID                    NOT NULL DEFAULT uuid_generate_v4(),
    project     UUID                    NOT NULL,

    character   INTEGER                 NOT NULL,
    amount      DOUBLE PRECISION        NOT NULL,
    created_at  TIMESTAMPTZ             NOT NULL DEFAULT NOW(),

    category    PROJECT_BUDGET_CATEGORY NOT NULL,

    description VARCHAR,

    PRIMARY KEY (budget, project),

    FOREIGN KEY (project)
        REFERENCES projects (project)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS project_asset_project     ON project_assets(project);
CREATE INDEX IF NOT EXISTS project_budget_project    ON project_budget(project);
CREATE INDEX IF NOT EXISTS project_container_project ON project_containers(project);

--------------------------------------------------------------------------------
--                  General tables
--------------------------------------------------------------------------------

-- Contains every character that ever tried to login, if the login was not
-- successful and the user tried again, the user will be here multiple times
CREATE TABLE IF NOT EXISTS login(
    token                   UUID NOT NULL DEFAULT uuid_generate_v4(),

    expire_date             TIMESTAMPTZ,

    character_id            INTEGER,
    character_main          INTEGER,

    access_token            VARCHAR,
    refresh_token           VARCHAR,

    PRIMARY KEY(token)
);
