CREATE TABLE workspace_events
    PARTITION OF events
    FOR VALUES IN ('workspace')
;
