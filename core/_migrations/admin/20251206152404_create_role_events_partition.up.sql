CREATE TABLE role_events
    PARTITION OF events
    FOR VALUES IN ('role')
;
