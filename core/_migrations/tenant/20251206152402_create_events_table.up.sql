CREATE TABLE events (
    event_id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),     -- globally unique event
    event_type VARCHAR(255) NOT NULL,                     -- e.g. UserCreated
    event_version INT NOT NULL,                           -- business event version

    aggregate_type VARCHAR(100) NOT NULL,                         -- partition key
    aggregate_id UUID NOT NULL,                           -- entity ID
    aggregate_version INT NOT NULL,                       -- sequence number per aggregate

    data BYTEA NOT NULL,                                  -- Protobuf payload
    metadata BYTEA,                                       -- Protobuf metadata

    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),        -- append time
    effective_at TIMESTAMPTZ,                             -- optional effective time

    correlation_id UUID,                                  -- tracing
    causation_id UUID,                                    -- tracing

    schema_version VARCHAR(50) NOT NULL,                  -- serialization schema
    hash BYTEA NOT NULL                                   -- integrity checksum
)
PARTITION BY LIST (aggregate_type);

ALTER TABLE events
    ADD CONSTRAINT uq_events_aggregate_type_id_version
    UNIQUE (aggregate_type, aggregate_id, aggregate_version);
CREATE INDEX idx_events_aggregate_type_id_version ON events (aggregate_type, aggregate_id, aggregate_version);

CREATE INDEX idx_events_correlation_id ON events (correlation_id);
CREATE INDEX idx_events_causation_id ON events (causation_id);
CREATE INDEX idx_events_event_type ON events (event_type);
CREATE INDEX idx_events_event_type_version ON events (event_type, event_version);
