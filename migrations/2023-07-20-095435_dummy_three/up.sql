-- Your SQL goes here
CREATE TABLE dummy_three (
    id SERIAL PRIMARY KEY,
    payment_id VARCHAR(255) NOT NULL,
    merchant_id VARCHAR(255) NOT NULL,
    status VARCHAR(255) NOT NULL,
    amount INTEGER NOT NULL,
    currency VARCHAR(255),
    amount_captured INTEGER,
    customer_id VARCHAR(255),
    description VARCHAR(255),
    return_url VARCHAR(255),
    metadata JSONB DEFAULT '{}'::JSONB,
    connector_id VARCHAR(255),
    shipping_address_id VARCHAR(255),
    billing_address_id VARCHAR(255),
    statement_descriptor_name VARCHAR(255),
    statement_descriptor_suffix VARCHAR(255),
    created_at TIMESTAMP NOT NULL,
    modified_at TIMESTAMP NOT NULL,
    last_synced TIMESTAMP,
    setup_future_usage VARCHAR(255),
    off_session BOOLEAN,
    client_secret VARCHAR(255)
);