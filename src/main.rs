use std::fs;

fn main() {
    let path = "/proc/diskstats";

    match fs::read_to_string(path) {
        Ok(contents) => {
            println!("{contents}");
        }
        Err(e) => {
            println!("{e}");
        }
    }
}


