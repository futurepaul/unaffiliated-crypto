use std::env;

use sha2::{Sha256, Digest};
use hex_view::HexView;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn hasher(path: &Path) -> io::Result<String> {
    File::open(path).and_then( move |mut file| {
        let mut buffer = [0u8; 8 * 1024];
        let mut hasher = Sha256::new();

        loop {
            let read = file.read(&mut buffer)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }

        Ok(format!("{:x}", HexView::from(hasher.finalize().as_slice())))
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Need a file to hash!");
        return
    }

    let file_to_hash = &args[1];

    match hasher(&Path::new(file_to_hash)) {
        Ok(hash) => println!("{}", hash),
        Err(e) => eprintln!("This didn't work because {}", e)
    }
}
