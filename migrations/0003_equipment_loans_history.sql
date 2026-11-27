-- Separate physical condition from the borrowing state.
ALTER TABLE equipment ADD COLUMN condition_status TEXT NOT NULL DEFAULT 'ready'
    CHECK (condition_status IN ('ready', 'needs_repair'));
UPDATE equipment SET condition_status='needs_repair'
 WHERE status IN ('maintenance','unavailable');
UPDATE equipment SET status=CASE WHEN EXISTS (
    SELECT 1 FROM loans l WHERE l.equipment_id=equipment.id AND l.returned_at IS NULL
) THEN 'checked_out' ELSE 'available' END;
ALTER TABLE equipment DROP CONSTRAINT IF EXISTS equipment_status_check;
ALTER TABLE equipment ADD CONSTRAINT equipment_status_check
    CHECK (status IN ('available','checked_out'));
ALTER TABLE loans ADD COLUMN loan_group_id UUID NOT NULL DEFAULT gen_random_uuid();
CREATE INDEX loans_group_idx ON loans(loan_group_id);
CREATE INDEX loans_due_open_idx ON loans(due_at) WHERE returned_at IS NULL;
CREATE TABLE equipment_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    equipment_id UUID NOT NULL REFERENCES equipment(id),
    loan_id UUID REFERENCES loans(id),
    event_type TEXT NOT NULL CHECK (event_type IN ('registered','checked_out','returned','condition_changed','maintenance_reported','maintenance_completed','note')),
    from_state JSONB,
    to_state JSONB,
    comment TEXT NOT NULL CHECK (length(trim(comment)) > 0),
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE INDEX equipment_events_item_time_idx ON equipment_events(equipment_id,occurred_at DESC);
INSERT INTO equipment_events(equipment_id,event_type,to_state,comment)
SELECT id,'registered',jsonb_build_object('condition_status',condition_status),
       'History tracking enabled; earlier events were not available.' FROM equipment;
