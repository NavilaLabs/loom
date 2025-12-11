CREATE TABLE task_events
    PARTITION OF events
    FOR VALUES IN ('task')
;
