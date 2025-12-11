CREATE TABLE contact_events
    PARTITION OF events
    FOR VALUES IN ('contact')
;
