CREATE TYPE bonus_modifier AS ENUM (
  'MANUFACTURE_MATERIAL',
  'MANUFACTURE_TIME',
  'REACTION_MATERIAL',
  'REACTION_TIME'
);

CREATE TABLE structure_dogma (
  ptype_id    INTEGER         NOT NULL,
  modifier    bonus_modifier  NOT NULL,
  amount      FLOAT           NOT NULL,
  categories  INTEGER[]       NOT NULL,
  groups      INTEGER[]       NOT NULL
);
