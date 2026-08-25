CREATE TABLE nodes (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    public_key VARCHAR(256) NOT NULL,
    alias VARCHAR(256) NOT NULL,
    capacity_sats BIGINT NOT NULL,
    first_seen TIMESTAMPTZ NOT NULL
);
