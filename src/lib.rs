use std::fs;
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

const SECTOR_SIZE: u64 = 512;
const PROC_DISKSTATS: &str = "/proc/diskstats";

pub struct DiskStats {
    pub name: String,
    pub read_bytes: u64,
    pub write_bytes: u64,
    pub read_ops: u64,
    pub write_ops: u64,
    pub read_time_ms: u64,
    pub write_time_ms: u64,
    pub ios_in_progress: u64,
    pub io_time_ms: u64,
}

pub struct DiskSpeed {
    pub name: String,
    pub read_bytes_per_sec: f64,
    pub write_bytes_per_sec: f64,
    pub read_iops: f64,
    pub write_iops: f64,
    pub busy_percent: f64,
    pub interval: Duration,
}


fn parse_line(line: &str) -> Option<DiskStats> {
    let f: Vec<&str> = line.split_whitespace().collect();

    if f.len() < 13 {
        return None;
    }

    Some(DiskStats {
        name:            f[2].to_string(),
        read_ops:        f[3].parse().ok()?,
        read_bytes:      f[5].parse::<u64>().ok()? * SECTOR_SIZE,
        read_time_ms:    f[6].parse().ok()?,
        write_ops:       f[7].parse().ok()?,
        write_bytes:     f[9].parse::<u64>().ok()? * SECTOR_SIZE,
        write_time_ms:   f[10].parse().ok()?,
        ios_in_progress: f[11].parse().ok()?,
        io_time_ms:      f[12].parse().ok()?,
    })
}

fn is_whole_disk(name: &str) -> bool {
    if name.starts_with("loop")
        || name.starts_with("ram")
        || name.starts_with("dm-")
    {
        return false;
    }

    let sys_path = format!("/sys/block/{name}");
    Path::new(&sys_path).exists()
}

fn read_diskstats() -> Vec<DiskStats> {
    let contents = match fs::read_to_string(PROC_DISKSTATS) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    contents
        .lines()
        .filter_map(parse_line)
        .filter(|entry| is_whole_disk(&entry.name))
        .collect()
}

pub fn stats() -> Vec<DiskStats> {
    read_diskstats()
}

pub fn speed(interval: Duration) -> Vec<DiskSpeed> {
    let before = read_diskstats();

    let start = Instant::now();
    thread::sleep(interval);
    let after = read_diskstats();
    let elapsed = start.elapsed();

    let secs = elapsed.as_secs_f64();
    if secs == 0.0 {
        return Vec::new();
    }

    let elapsed_ms = elapsed.as_millis() as f64;

    let mut rates = Vec::new();

    for new in &after {
        let old = match before.iter().find(|o| o.name == new.name) {
            Some(o) => o,
            None => continue,
        };

        let d_read_bytes = new.read_bytes.saturating_sub(old.read_bytes);
        let d_write_bytes = new.write_bytes.saturating_sub(old.write_bytes);
        let d_read_ops = new.read_ops.saturating_sub(old.read_ops);
        let d_write_ops = new.write_ops.saturating_sub(old.write_ops);
        let d_io_time = new.io_time_ms.saturating_sub(old.io_time_ms);

        rates.push(DiskSpeed {
            name:                new.name.clone(),
            read_bytes_per_sec:  d_read_bytes as f64 / secs,
            write_bytes_per_sec: d_write_bytes as f64 / secs,
            read_iops:           d_read_ops as f64 / secs,
            write_iops:          d_write_ops as f64 / secs,
            busy_percent:        (d_io_time as f64 / elapsed_ms) * 100.0,
            interval:            elapsed,
        });
    }

    rates
}
