-- blocks table 
CREATE TABLE IF NOT EXISTS blocks (
     -- id BIGSERIAL PRIMARY KEY,
     chain_id BIGINT NOT NULL,
     height BIGINT NOT NULL,
     hash BYTEA NOT NULL,
     parent_hash BYTEA NOT NULL,
     timestamp TIMESTAMPTZ NOT NULL,
     
     canonical BOOLEAN NOT NULL DEFAULT FALSE,
     state_root BYTEA,
     transactions_root BYTEA,
     receipts_root BYTEA,
     
     transaction_count BIGINT, 
     gas_limit BIGINT,
     gas_used BIGINT,
     base_fee_per_gas NUMERIC(78, 0),
    
     inserted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
     
     PRIMARY KEY (chain_id, hash),
);

CREATE INDEX IF NOT EXISTS idx_blocks_chain_height ON blocks (chain_id, height);
CREATE INDEX IF NOT EXISTS idx_blocks_chain_parent ON blocks (chain_id, parent_hash);

-- logs table
CREATE TABLE IF NOT EXISTS logs (
     -- id BIGSERIAL PRIMARY KEY,
     -- block_id BIGINT NOT NULL,
     -- CONSTRAINT fk_blocks FOREIGN KEY (block_id) REFERENCES blocks(id),
     chain_id BIGINT NOT NULL,
     block_hash BYTEA NOT NULL,

     transaction_hash BYTEA NOT NULL,
     transaction_index BIGINT NOT NULL,
     log_index BIGINT NOT NULL,

     log_address BYTEA NOT NULL,
     log_data BYTEA NOT NULL,
     log_topics BYTEA[] NOT NULL,

     PRIMARY KEY (chain_id, block_hash, log_index),

     FOREIGN KEY (chain_id, block_hash)
          REFERENCES blocks (chain_id, hash)
);

CREATE INDEX IF NOT EXISTS idx_logs_transaction_hash ON logs (transaction_hash);
CREATE INDEX IF NOT EXISTS idx_logs_address ON logs (log_address);
