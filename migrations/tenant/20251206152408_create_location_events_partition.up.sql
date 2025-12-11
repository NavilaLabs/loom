CREATE TABLE location_events
    PARTITION OF events
    FOR VALUES IN ('location')
;
