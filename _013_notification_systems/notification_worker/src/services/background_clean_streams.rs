use redis::{AsyncCommands, aio::MultiplexedConnection, streams::StreamTrimOptions};

use crate::utils::error::NotificationWorkerErr;

pub async fn clean_streams(
    stream_trim_max_entries: usize,
    stream_key: String,
    q_conn: &mut MultiplexedConnection,
) -> Result<(), NotificationWorkerErr> {
    let sto = StreamTrimOptions::maxlen(
        redis::streams::StreamTrimmingMode::Approx,
        stream_trim_max_entries,
    );
    let _ = q_conn
        .xtrim_options::<String, String>(stream_key, &sto)
        .await?;
    Ok(())
}
