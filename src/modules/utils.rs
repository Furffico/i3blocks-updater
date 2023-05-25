use crate::module::UpdateStatus;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn extract_from_file(
    path: &str,
    queries: &[&str],
    results: &mut [i64],
) -> Result<UpdateStatus> {
    let file = File::open(path)?;
    let mut q_count = queries.len();
    for line in BufReader::new(file).lines() {
        let line = match line {
            Ok(s) => s,
            Err(_) => break,
        };
        for (i, q) in queries.iter().enumerate() {
            if line.starts_with(q) {
                results[i] = match line.split_ascii_whitespace().nth(1) {
                    Some(num_str) => match num_str.parse::<i64>() {
                        Ok(n) => n,
                        Err(_) => continue,
                    },
                    None => continue,
                };
                q_count -= 1;
                if q_count == 0 {
                    return Ok(UpdateStatus::All);
                }
            }
        }
    }
    if q_count == queries.len() {
        Ok(UpdateStatus::None)
    } else {
        Ok(UpdateStatus::Some)
    }
}

const PREFIX: &[&str] = &["B", "KiB", "MiB", "GiB", "TiB", "PiB", "EiB"];

pub fn format_bytes(bytes: u64) -> String {
    let mut n = 0;
    let converted = bytes as f64;
    let mut bytes = bytes;
    while bytes >= 1024 {
        bytes >>= 10;
        n += 1;
    }
    let converted = converted / ((1 << (n * 10)) as f64);
    format!("{:.2}{}", converted, PREFIX[n])
}
