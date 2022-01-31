CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE PROJECT_STATUS          AS ENUM ('ABORTED', 'DONE', 'IN_PROGRESS', 'PAUSED');
CREATE TYPE PROJECT_BUDGET_CATEGORY AS ENUM ('PURCHASE', 'SOLD', 'MANUFACTURE', 'RESEARCH', 'OTHER');

--------------------------------------------------------------------------------
--                  General tables
--------------------------------------------------------------------------------

-- Contains every character that ever tried to login, if the login was not
-- successful and the user tried again, the user will be here multiple times
CREATE TABLE IF NOT EXISTS logins(
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
CREATE TABLE IF NOT EXISTS characters(
    character_id            INTEGER     NOT NULL,
    corporation_id          INTEGER     NOT NULL,

    character_main          INTEGER,

    character_name          VARCHAR(50) NOT NULL,
    corporation_name        VARCHAR(50) NOT NULL,

    alliance_id             INTEGER,
    alliance_name           VARCHAR(50),

    PRIMARY KEY (character_id),

    FOREIGN KEY (character_main)
        REFERENCES characters (character_id)
        ON DELETE CASCADE
);

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

CREATE INDEX IF NOT EXISTS market_orders_type_id   ON market_orders(type_id);
CREATE INDEX IF NOT EXISTS market_orders_system_id ON market_orders(system_id);

--------------------------------------------------------------------------------
--                  Tables for projects
--------------------------------------------------------------------------------
-- Contains all projects
CREATE TABLE IF NOT EXISTS projects(
    project     UUID           NOT NULL DEFAULT uuid_generate_v4(),

    owner       INTEGER        NOT NULL,
    name        VARCHAR        NOT NULL,

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

-- Assets that are stored in a container
CREATE TABLE IF NOT EXISTS project_assets(
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

CREATE TABLE IF NOT EXISTS project_members (
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

CREATE INDEX IF NOT EXISTS project_asset_project     ON project_assets(project);
CREATE INDEX IF NOT EXISTS project_budget_project    ON project_budget(project);
CREATE INDEX IF NOT EXISTS project_members_project   ON project_members(project);
CREATE INDEX IF NOT EXISTS project_products_project  ON project_products(project);
