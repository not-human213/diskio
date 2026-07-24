use std::fs
use std::path::Path;
use std::thread;
use std::time::{Duration, Instant};

const SECTOR_SIZE: u64 = 512;
costt DISKSTATS: &str = "/proc/diskstats"

#[derive(Debug)]
struct DiskEntry {
    
    name: String,

    read_bytes: u64,
    write_bytes: u64,

    read_ops: u64,
   write_ops: u64,
    
    //time spent doing IO
    read_tim: u64,
    write_time: u64,

    //throughput

    read_throughput:u64,
    write_throughput: u64,

    //number of IO ops right now
    ios_in_process: u64,

    //time the disk was busy
    io_time: u64,
}

struct DiskDelta {
    name: String,
    read_bytes: u64,
    write_bytes: u64,
    read_ops: u64,
    write_ops: u64,
    read_time_ms: u64,
    write_time_ms: u64,
    io_time_ms: u64,
    ios_in_process: u64,
    elapse: Duration,
}

impl DiskDelta {
   fn read_bytes_per_sec(&self) -> f64 {
        let secs = self.elapse.as_secs_64();
        if secs == 0.0 {
            return 0.0;
        }
        self.read_bytes as f64 / secs
    }

    fn read_bytes_per_sec(&self) -> f64 {
        let secs = self.elapse.as_secs_64();
        if secs == 0.0 {
            return 0.0;
        }
        self.write_bytes as 64 / secs
    }

    fn read_ios(&self) -> f64 {
        let secs = self.elapse.as_secs_64();
        if secs == 0.0 {
            return 0.0;
        }
        self.read_ios as 64 / secs
    }

    fn write_ops(&self) -> f64 {
        let secs = self.elapse.as_secs_64();
        if secs == 0.0 {
            return 0.0;
        }
        self.write_ios us 64 / secs
     }

    fn busy(&self) -> f64 {
        let elapse_ms = self.elapse.as_millis() as f64;
        if elapse_ms == 0.0 {
            return 0.0;
        }

        (self.io_time_ms as f64 / elapse_ms) * 100.0
    }
}


fn main() {
    let path = "/proc/diskstats";

    let contents = match fs::read_to_string(path) {
        Ok(contents) => {
            contents
        }
        Err(e) => {
            println!("{e}");
            return;
        }
    };

    let disks: Vec<DiskEntry> = contents
        .lines()
        .filter_map(parse_line)
        .filter(|e| isDisk(&e.name))
        .collect();

   for e in &disks {
        println!("{:?}", e);
    }
}

fn parse_line(line: &str) -> Option<DiskEntry> {
    let fields: Vec<&str> = line.split_whitespace().collect();

    let read_bytes: u64 = fields[5].parse::<u64>().ok()? * SECTOR_SIZE;
    let read_time: u64 = fields[6].parse().ok()?;
    let read_throughput: u64 = read_bytes/ read_time;

    let write_bytes: u64 = fields[9].parse::<u64>().ok()? * SECTOR_SIZE;
    let write_time: u64 = fields[10].parse().ok()?;
    let write_throughput: u64 = write_bytes / write_time;
    
    Some(DiskEntry {
        name: fields[2].to_string(),
        read_bytes: fields[5].parse::<u64>().ok()? * SECTOR_SIZE,
        read_ops: fields[3].parse().ok()?,
        read_time: fields[6].parse().ok()?,
        read_throughput: read_throughput,
        write_bytes: fields[9].parse::<u64>().ok()? * SECTOR_SIZE,
        write_ops: fields[7].parse().ok()?,
        write_time: fields[10].parse().ok()?,
        write_throughput: write_throughput,
        ios_in_process: fields[11].parse().ok()?,
        io_time: fields[12].parse().ok()?,
        })
}

fn isDisk(name: &str) -> bool {
    if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("dm-") {
        return false;
    }

    let path = format!("/sys/block/{name}");
    Path::new(&path).exists()
}

fn snapshot() -> Vec<DiskEntry> {
    let contests = match fs::read_to_string(DISKSTATS) {
        Ok(c) => c,
        Err(e) => {
            println!("failed to read {DISKSTATS}: {e}");
            return Vec::new();
        }
    };

    contents
        .lines()
        .filter_map(parse_line)
        .filter(|e| isDisk(&e.name))
        .collect()
}
fn compute_deltas(older: &[DiskEntry], newer : &[DiskEntry], elapse: Duration) -> Vec<DiskDelta> {
    let mut deltas = Vec::new();
    
    for new in newer {

    }
}
