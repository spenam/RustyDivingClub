ALTER TABLE members
    ADD COLUMN diving_level TEXT;

ALTER TABLE members
    ADD CONSTRAINT members_diving_level_check
    CHECK (diving_level IS NULL OR diving_level IN (
        'N1', 'PN1', 'N2', 'PN2', 'N3', 'PN3',
        'N4', 'PN4', 'E1', 'E2', 'E3', 'E4'
    ));

CREATE INDEX members_diving_level_idx ON members(diving_level);
