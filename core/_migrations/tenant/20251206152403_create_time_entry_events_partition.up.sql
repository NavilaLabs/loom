CREATE TABLE time_entry_events
    PARTITION OF events
    FOR VALUES IN ('time_entry')
;
