use crate::util::{hash_file, save_file, traverse_directory};
use clap::Parser;
use std::path::Path;

mod util;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cli_test/sig_lib/library.h");

        fn sig_doc(path: &str) -> i32;
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value = "./")]
    save_location: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);
    let save_location = Path::new(&args.save_location);

    if !path.exists() {
        eprintln!("Path does not exist: {:?}", path);
        std::process::exit(1);
    }

    let mut documents = Vec::new();

    if path.is_dir() {
        documents = traverse_directory(&path);
    }

    if path.is_file() {
        if let Some(document) = hash_file(&path) {
            documents.push(document);
        } else {
            eprintln!("Could not hash file: {:?}", path);
            std::process::exit(1);
        }
    }

    if let Some(hash_file) = save_file(documents, &save_location.to_str().unwrap()) {
        ffi::sig_doc(&hash_file);
        println!("Signed : {:?}", hash_file);
    } else {
        std::process::exit(1);
    }
}
