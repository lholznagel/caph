CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TYPE PROJECT_STATUS AS ENUM ('DONE', 'IN_PROGRESS', 'HALTED');

--------------------------------------------------------------------------------
--                  General static data
--------------------------------------------------------------------------------
CREATE TABLE IF NOT exists market_orders(
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
    id         UUID           NOT NULL DEFAULT uuid_generate_v4(),

    pinned     BOOLEAN        NOT NULL DEFAULT TRUE,

    owner      INTEGER        NOT NULL,
    name       VARCHAR        NOT NULL,

    status     PROJECT_STATUS NOT NULL DEFAULT 'IN_PROGRESS',

    containers BIGINT[]       NOT NULL DEFAULT ARRAY[]::BIGINT[],

    PRIMARY KEY (id)
);

-- List of items that should be produced in a project
CREATE TABLE IF NOT EXISTS project_products(
    project UUID    NOT NULL,

    type_id INTEGER NOT NULL,
    count   INTEGER NOT NULL,

    PRIMARY KEY (project, type_id),

    FOREIGN KEY (project)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

-- Tracking of the cost for a project
CREATE TABLE IF NOT EXISTS project_trackings(
    id          UUID             NOT NULL DEFAULT uuid_generate_v4(),
    project     UUID             NOT NULL,
    character   INTEGER          NOT NULL,
    amount      DOUBLE PRECISION NOT NULL,
    description VARCHAR          NOT NULL,

    created_at  TIMESTAMPTZ      NOT NULL DEFAULT NOW(),

    PRIMARY KEY (id, project),
    FOREIGN KEY (project)
        REFERENCES projects (id)
        ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS project_tracking_project ON project_trackings(project);
CREATE INDEX IF NOT EXISTS project_products_product ON project_products(project);

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
