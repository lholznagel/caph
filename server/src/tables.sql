CREATE TABLE IF NOT EXISTS user_items (
  id                  INTEGER   NOT NULL, -- type_id
  quantity            BIGINT    NOT NULL,

  PRIMARY KEY(id)
);

CREATE TABLE IF NOT EXISTS routes (
  origin              INTEGER   NOT NULL,
  destination         INTEGER   NOT NULL,
  systems             INTEGER[] NOT NULL,
  flag                TEXT      NOT NULL
);