-- Add migration script here
CREATE TABLE industry_index(
  time              TIMESTAMPTZ NOT NULL,

  system_id         INTEGER     NOT NULL,
  manufacturing     REAL        NOT NULL,
  copying           REAL        NOT NULL,
  invention         REAL        NOT NULL,
  reaction          REAL        NOT NULL,
  research_time     REAL        NOT NULL,
  research_material REAL        NOT NULL
);

CREATE INDEX industry_index_system_id ON industry_index (system_id);
