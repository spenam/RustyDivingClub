-- Privacy minimisation while retaining member names and stable member IDs.
-- Existing full_name and id values are preserved.
ALTER TABLE members
    DROP COLUMN IF EXISTS email,
    DROP COLUMN IF EXISTS phone;
