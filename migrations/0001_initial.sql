CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE members (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    full_name TEXT NOT NULL,
    email TEXT UNIQUE,
    phone TEXT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE equipment_categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT
);

CREATE TABLE equipment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    qr_uuid UUID NOT NULL UNIQUE DEFAULT gen_random_uuid(),
    category_id UUID REFERENCES equipment_categories(id),
    name TEXT NOT NULL,
    description TEXT,
    serial_number TEXT UNIQUE,
    status TEXT NOT NULL DEFAULT 'available'
        CHECK (status IN ('available', 'checked_out', 'maintenance', 'unavailable')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE loans (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment(id),
    member_id UUID NOT NULL REFERENCES members(id),
    checked_out_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    due_at TIMESTAMPTZ,
    returned_at TIMESTAMPTZ,
    notes TEXT
);

CREATE UNIQUE INDEX one_open_loan_per_equipment
    ON loans(equipment_id) WHERE returned_at IS NULL;
CREATE INDEX loans_member_idx ON loans(member_id);
CREATE INDEX equipment_status_idx ON equipment(status);
