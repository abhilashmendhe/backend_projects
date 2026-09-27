use redis::{AsyncCommands, aio::MultiplexedConnection};

use crate::models::payload_send::SendPayload;

pub async fn add_to_redis_list(mut redis_conn: MultiplexedConnection, payload_send: SendPayload) {
    // redis_conn.zadd(key, member, score)
}
