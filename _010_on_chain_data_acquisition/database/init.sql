CREATE TABLE IF NOT EXISTS blocks (
     id BIGSERIAL PRIMARY KEY,
     chain_id BIGINT NOT NULL,
     height BIGINT NOT NULL,
     hash BYTEA NOT NULL,
     parent_hash BYTEA NOT NULL,
     timestamp TIMESTAMPZ NOT NULL,
     
     canonical BOOLEAN NOT NULL DEFAULT FALSE,
     state_root BYTEA,
     transactions_root BYTEA,
     receipts_root BYTEA,
     
     transaction_count BIGINT, 
     gas_limit BIGINT,
     gas_used BIGINT,
     base_fee_per_gas NUMERIC(78, 0),
    
     inserted_at TIMESTAMPZ NOT NULL DEFAULT NOW(),
     
     UNIQUE (chain_id, hash),
);

CREATE INDEX IF NOT EXISTS idx_blocks_chain_height ON blocks (chain_id, height);
CREATE INDEX IF NOT EXISTS idx_blocks_chain_parent ON blocks (chain_id, parent_hash);
