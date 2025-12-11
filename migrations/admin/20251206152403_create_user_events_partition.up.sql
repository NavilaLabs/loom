CREATE TABLE user_events
    PARTITION OF events
    FOR VALUES IN ('user')
;
