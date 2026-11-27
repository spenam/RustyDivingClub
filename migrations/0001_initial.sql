-- 0001_init.sql
-- Consolidated initial schema (supersedes old 0001..0004).
-- Target: BRAND-NEW database. Do not apply on top of an existing DB.

CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- ---------------------------------------------------------------------------
-- Members
-- ---------------------------------------------------------------------------
CREATE TABLE members (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name    TEXT NOT NULL,
    active       BOOLEAN NOT NULL DEFAULT TRUE,
    diving_level TEXT,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT members_diving_level_check
        CHECK (diving_level IS NULL OR diving_level IN (
            'N1','PN1','N2','PN2','N3','PN3',
            'N4','PN4','E1','E2','E3','E4'
        ))
);

CREATE INDEX members_diving_level_idx ON members(diving_level);

-- ---------------------------------------------------------------------------
-- Equipment categories & equipment
-- ---------------------------------------------------------------------------
CREATE TABLE equipment_categories (
    id          UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name        TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE equipment (
    id                    UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    qr_uuid               UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    category_id           UUID REFERENCES equipment_categories(id),
    name                  TEXT NOT NULL,
    brand                 TEXT,
    regulator_connection  TEXT
        CONSTRAINT equipment_regulator_connection_check
        CHECK (regulator_connection IS NULL OR regulator_connection IN ('DIN', 'ETRIER')),
    size                  TEXT,
    description           TEXT,
    serial_number         TEXT UNIQUE,
    status                TEXT NOT NULL DEFAULT 'available'
        CONSTRAINT equipment_status_check
        CHECK (status IN ('available', 'checked_out')),
    condition_status      TEXT NOT NULL DEFAULT 'ready'
        CONSTRAINT equipment_condition_status_check
        CHECK (condition_status IN ('ready', 'needs_repair')),
    created_at            TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at            TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX equipment_status_idx ON equipment(status);

-- Keep updated_at accurate (the old schema had the column but nothing maintained it).
CREATE OR REPLACE FUNCTION set_updated_at() RETURNS trigger AS $$
BEGIN
    NEW.updated_at := now();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER equipment_set_updated_at
    BEFORE UPDATE ON equipment
    FOR EACH ROW EXECUTE FUNCTION set_updated_at();

-- ---------------------------------------------------------------------------
-- Loans
-- ---------------------------------------------------------------------------
CREATE TABLE loans (
    id             UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id   UUID NOT NULL REFERENCES equipment(id),
    member_id      UUID NOT NULL REFERENCES members(id),
    loan_group_id  UUID NOT NULL DEFAULT gen_random_uuid(),
    checked_out_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    due_at         TIMESTAMPTZ,
    returned_at    TIMESTAMPTZ,
    notes          TEXT
);

CREATE UNIQUE INDEX one_open_loan_per_equipment
    ON loans(equipment_id) WHERE returned_at IS NULL;
CREATE INDEX loans_member_idx   ON loans(member_id);
CREATE INDEX loans_group_idx    ON loans(loan_group_id);
CREATE INDEX loans_due_open_idx ON loans(due_at) WHERE returned_at IS NULL;

-- ---------------------------------------------------------------------------
-- Equipment events (audit trail)
-- ---------------------------------------------------------------------------
CREATE TABLE equipment_events (
    id           UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment(id),
    loan_id      UUID REFERENCES loans(id),
    event_type   TEXT NOT NULL CHECK (event_type IN (
        'registered','checked_out','returned',
        'condition_changed','maintenance_reported','maintenance_completed','note'
    )),
    from_state   JSONB,
    to_state     JSONB,
    comment      TEXT NOT NULL CHECK (length(trim(comment)) > 0),
    occurred_at  TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX equipment_events_item_time_idx
    ON equipment_events(equipment_id, occurred_at DESC);
