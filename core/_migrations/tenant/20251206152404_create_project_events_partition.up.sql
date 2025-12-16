CREATE TABLE project_events
    PARTITION OF events
    FOR VALUES IN ('project')
;
