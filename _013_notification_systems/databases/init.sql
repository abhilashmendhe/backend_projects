CREATE TABLE IF NOT EXISTS users (
    id BIGSERIAL PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    email    VARCHAR(64) NOT NULL, 
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS devices (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    device_token TEXT NOT NULL,
    platform VARCHAR(8) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    last_seen_at TIMESTAMPTZ DEFAULT NOW(), 
    is_active BOOLEAN DEFAULT true,
    CONSTRAINT fk_device_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS notifications (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL, 
    event_id VARCHAR(64) NOT NULL, 
    title VARCHAR(64) NOT NULL, 
    body VARCHAR(64),
    payload JSONB,
    priority SMALLINT, -- HIGH (0), LOW(1)
    -- status  SMALLINT NOT NULL, -- RECEIVED, QUEUED, PROCESSING, SENT, PARTIALLY_SENT, FAILED
    created_at TIMESTAMPTZ,
    CONSTRAINT fk_notification_user FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS notification_deliverables (
    id BIGSERIAL PRIMARY KEY, 
    notification_id BIGINT NOT NULL,
    device_id BIGINT NOT NULL, 
    status SMALLINT NOT NULL, -- SENT(0), FAILED(1), PENDING(2)
    retry_count SMALLINT NOT NULL,
    CONSTRAINT fk_not_deliverables FOREIGN KEY (notification_id) REFERENCES notifications(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS notification_logs(
    id BIGSERIAL PRIMARY KEY,
    delivery_id BIGINT NOT NULL, 
    event_id VARCHAR(64) NOT NULL, 
    created_at TIMESTAMPTZ DEFAULT NOW(),
    message TEXT,
    CONSTRAINT fk_log_deliverables FOREIGN KEY (delivery_id) REFERENCES notification_deliverables(id) ON DELETE CASCADE
);

-- Create indexes
CREATE INDEX idx_devices_user_id ON devices(user_id);
CREATE UNIQUE INDEX idx_devices_device_token ON devices(device_token);
CREATE UNIQUE INDEX idx_notifications_event_id ON notifications(event_id);
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_deliverables_notification_id ON notification_deliverables(notification_id);
CREATE INDEX idx_logs_delivery_id ON notification_logs(delivery_id);