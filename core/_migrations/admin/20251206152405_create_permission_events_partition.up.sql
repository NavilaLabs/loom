CREATE TABLE permission_events
    PARTITION OF events
    FOR VALUES IN ('permission')
;
