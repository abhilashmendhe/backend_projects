use std::{
    fs::{File, OpenOptions, read_dir},
    io::{Seek, Write},
    path::Path,
};

use base64::{Engine, engine::general_purpose};

use crate::{create_server::stream_server::PublishRequest, utils::errors::StreamServerErr};

#[derive(Debug)]
#[allow(unused)]
pub struct WalLogger {
    aof_folder_path: String,
    append_only_file: File,
    pending_append_only_file: File,
    file_log_num: u64,
    r_start_pos: i64,
    r_end_pos: i64,
    lsn: u64,
}

impl WalLogger {
    pub fn new(aof_folder_path: String) -> Result<Self, StreamServerErr> {
        let path = Path::new(&aof_folder_path);
        if !path.is_dir() {
            std::fs::create_dir(&path)?;
        }
        let read_dir = read_dir(aof_folder_path.clone())?;
        let (mut append_only_file, file_log_num) = if let Some(read_entry) = read_dir.last() {
            let dir_entry = read_entry?;
            let file_name = dir_entry.file_name().to_string_lossy().to_string();
            let mut dot_in = 0;
            let mut hy_in = 0;
            let mut i = 0;
            for ch in file_name.chars() {
                if ch == '-' {
                    hy_in = i + 1;
                } else if ch == '.' {
                    dot_in = i;
                }
                i += 1;
            }
            let file_num = file_name[hy_in..dot_in].parse::<u64>()?;
            (
                OpenOptions::new().append(true).open(dir_entry.path())?,
                file_num,
            )
        } else {
            (
                OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(format!("{}/wal-0000001.log", aof_folder_path))?,
                1,
            )
        };
        let lsn = append_only_file.seek(std::io::SeekFrom::End(0))?;
        Ok(Self {
            aof_folder_path,
            append_only_file: append_only_file.try_clone()?,
            pending_append_only_file: append_only_file.try_clone()?,
            file_log_num,
            r_start_pos: 0,
            r_end_pos: 0,
            lsn,
        })
    }

    pub fn write_log(&mut self, publish_request: &PublishRequest) -> Result<(), StreamServerErr> {
        // 1. create a aof payload
        let enc_str = general_purpose::STANDARD.encode(&publish_request.payload);
        let aof_payload = format!(
            "message-id:{};payload:{};timestamp:{}",
            publish_request.message_id, enc_str, publish_request.timestamp
        );
        // println!("aof-payload: {}", aof_payload);
        let mut crc_aof_payload = flate2::Crc::new();
        crc_aof_payload.update(aof_payload.as_bytes());
        let full_format = format!(
            "lsn:{};crc:{};length:{};{}\n",
            self.lsn,
            crc_aof_payload.sum(),
            aof_payload.len(),
            aof_payload
        );
        self.append_only_file.write_all(full_format.as_bytes())?;
        self.append_only_file.flush()?;
        if self.append_only_file.metadata()?.len() > 64000 {
            self.file_log_num += 1;
            let len_f = self.file_log_num.ilog10() + 1;
            let zeros: String = std::iter::repeat('0').take(7 - len_f as usize).collect();
            self.append_only_file = OpenOptions::new().append(true).create(true).open(format!(
                "{}/wal-{}{}.log",
                self.aof_folder_path, zeros, self.file_log_num
            ))?;
        }
        self.lsn = self.append_only_file.seek(std::io::SeekFrom::End(0))?;
        Ok(())
    }
}
