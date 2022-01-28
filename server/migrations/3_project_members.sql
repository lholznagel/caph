CREATE TABLE IF NOT EXISTS project_members (
  project      UUID NOT NULL,

  character_id INTEGER NOT NULL,

  FOREIGN KEY (project)
    REFERENCES projects (project)
    ON DELETE CASCADE,

  FOREIGN KEY (character_id)
    REFERENCES character (character_id)
    ON DELETE CASCADE
);
