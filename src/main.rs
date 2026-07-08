use std::fs;
use std::path::Path;

const SECTOR_SIZE: u64 = 512;
struct DiskEntry {
    read_bytes: u64,
    write_bytes: u64,
    name: String,
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
        println!("{} {} {}", e.name, bytes_to_string(e.read_bytes), bytes_to_string(e.write_bytes));
    }


}

fn parse_line(line: &str) -> Option<DiskEntry> {
    let fields: Vec<&str> = line.split_whitespace().collect();

    let sector_read: u64 = fields[5].parse().ok()?;
    let sector_written: u64 = fields[9].parse().ok()?; 
    let name: String = fields[2].to_string();
    
    Some(DiskEntry {
        read_bytes: sector_read * SECTOR_SIZE,
        write_bytes: sector_written * SECTOR_SIZE,
        name})
}

fn isDisk(name: &str) -> bool {
    if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("dm-") {
        return false;
    }

    let path = format!("/sys/block/{name}");
    Path::new(&path).exists()
}

fn bytes_to_string(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    const TB: f64 = GB * 1024.0;

    let b = bytes as f64;
    
    if b >= TB {
        format!("{:.2} TB", b/TB)
    }
    else if b >= GB {
        format!("{:.2} GB", b/GB)
    }
    else if b >= MB {
        format!("{:.2} MB", b/MB)
    }
    else if b >= KB {
        format!("{:.2} KB", b/KB)
    }
    else {
        format!("{b} B")
    }
}
