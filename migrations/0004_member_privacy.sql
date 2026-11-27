-- Keep member names, but remove unnecessary contact details.
ALTER TABLE members
    DROP COLUMN IF EXISTS email,
    DROP COLUMN IF EXISTS phone;

-- Remove historical free-text fields that could contain personal data.
UPDATE loans SET notes = NULL;

UPDATE equipment_events
SET comment = 'Historical comment removed during privacy migration';