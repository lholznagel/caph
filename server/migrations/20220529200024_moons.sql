CREATE TABLE moon_pulls (
  id                UUID        NOT NULL DEFAULT uuid_generate_v4(),
  character_id      INTEGER     NOT NULL,

  -- TypeId of the material
  material_1        INTEGER     NOT NULL,
  material_2        INTEGER     NOT NULL,
  material_3        INTEGER
  material_4        INTEGER

  -- Amount that is pulled
  material_1_amount INTEGER     NOT NULL,
  material_2_amount INTEGER     NOT NULL,
  material_3_amount INTEGER,
  material_4_amount INTEGER,

  -- Time the pull ends
  extraction_time   TIMESTAMPTZ NOT NULL DEFAULT NOW(),

  -- Appraisal for the mined volume and the waste volume
  appraisal_mined   VARCHAR,
  appraisal_waste   VARCHAR
);

CREATE TABLE moon_materials (
  -- Reference to the moon pull
  moon      UUID    NOT NULL,

  type_id   INTEGER NOT NULL,
  amount    INTEGER NOT NULL
);
