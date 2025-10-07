use std::{env::current_dir, fs::{read_dir, File}, io::{BufRead, BufReader}, sync::Arc};

use flate2::Crc;
use tokio::sync::Mutex;
use tracing::warn;

use crate::{data::kvt_struct::TimeBasedKV, utils::errors::KVError};

pub async fn read_logs(kv_store: Arc<Mutex<TimeBasedKV>>) -> Result<(), KVError> {

    let mut wal_logs_folder = current_dir()?
                        .parent()
                        .unwrap()
                        .to_path_buf();

    wal_logs_folder.push("wallogs");
    
    if let Ok(directory) = read_dir(wal_logs_folder) {

        for logfile in directory {

            let file_path = logfile?;
            let file = File::open(file_path.path())?;
            let reader = BufReader::new(file);

            for line_res in reader.lines() {

                let line = line_res?;
                let line_spl = line.split(";").collect::<Vec<_>>();

                let fcrc = &line_spl[1][4..].parse::<u32>().expect("Failed to parse crc to u32");
                let key = &line_spl[3][4..];
                let value = &line_spl[4][6..];
                let timestamp = &line_spl[5][10..].parse::<u64>().expect("Failed to parse to parse timestamp to u64");

                let kvs = format!("key:{};value:{};timestamp:{}",key,value,timestamp);
                let mut crc = Crc::new();
                crc.update(kvs.as_bytes());
                let crc_sum = crc.sum() & 0xFFFFFFFF;

                if *fcrc != crc_sum {
                    panic!("CRC does not match. File corrupt exiting..");
                }

                {
                    let mut kv_gaurd = kv_store.lock().await;
                    kv_gaurd.set(key.to_string(), value.to_string(), *timestamp);
                }
            }
        }
    } else {
        warn!("Logs not found!!")
    }

    Ok(())
}