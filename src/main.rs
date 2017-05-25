extern crate gtk;

use std::io;
use std::fs;

// TODO: Guess mime type based on file header (like Nautilus)

fn run() -> io::Result<()> {
    for e in fs::read_dir("/home/jplatte")? {
        let entry = e?;
        println!("{:?}", entry.file_name());
    }

    Ok(())
}

fn main() {
    run().unwrap();
}
