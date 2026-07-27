use std::time::Duration;

fn main() {
    let disks = diskio::stats();
    if disks.is_empty() {
        eprintln!("No physical disks found.");
        return;
    }

    println!("Cumulative (since boot):\n");
    println!(
        "  {:<12} {:>14} {:>14} {:>10} {:>10} {:>10} {:>10} {:>10} {:>5}",
        "Device", "Read (B)", "Written (B)", "Read Ops", "Write Ops",
        "RTime(ms)", "WTime(ms)", "Busy(ms)", "Queue"
    );
    println!("  {}", "-".repeat(105));

    for d in &disks {
        println!(
            "  {:<12} {:>14} {:>14} {:>10} {:>10} {:>10} {:>10} {:>10} {:>5}",
            d.name, d.read_bytes, d.write_bytes,
            d.read_ops, d.write_ops,
            d.read_time_ms, d.write_time_ms, d.io_time_ms,
            d.ios_in_progress,
        );
    }

    // ── Throughput (waits 1 second) ──
    println!("\nMeasuring throughput (1 second)...\n");

    let rates = diskio::speed(Duration::from_secs(1));

    println!(
        "  {:<12} {:>14} {:>14} {:>10} {:>10} {:>8}",
        "Device", "Read B/s", "Write B/s", "Read IOPS", "Write IOPS", "Busy %"
    );
    println!("  {}", "-".repeat(72));

    for r in &rates {
        println!(
            "  {:<12} {:>14.0} {:>14.0} {:>10.0} {:>10.0} {:>7.1}%",
            r.name,
            r.read_bytes_per_sec,
            r.write_bytes_per_sec,
            r.read_iops,
            r.write_iops,
            r.busy_percent,
        );
    }
}
