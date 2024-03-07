use crate::util::{hash_file, save_file, traverse_directory};
use clap::Parser;
use dotenv_codegen::dotenv;
use std::path::Path;

mod util;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cli_test/sig_lib/library.h");

        fn sig_doc(sha: &str, file_name: &str, sign: bool, cmd: bool) -> i32;
    }
}

/// CLI to send files to DiSA
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the files to send to the service
    #[arg(short, long)]
    path: String,

    /// Path of the signature file
    #[arg(short, long, default_value = "./")]
    save_location: String,

    /// Set to 0 to use Chave Movel Digital
    #[arg(short, long, default_value_t = 1)]
    cmd: u8,
}

fn main() {
    let mode = dotenv!("RELEASE_MODE");

    let args = Args::parse();

    let path = Path::new(&args.path);
    let save_location = Path::new(&args.save_location);
    let cmd = args.cmd == 0;

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

    if let Some(hash_json) = save_file(documents, &save_location.to_str().unwrap()) {
        let Some(doc_hash) = hash_file(Path::new(&hash_json)) else {
            eprintln!("Error creating hash of the hashes file");
            std::process::exit(1);
        };

        let err = ffi::sig_doc(
            &doc_hash.hash.as_str(),
            &hash_json.replace(".json", ".sig"),
            mode == "production",
            cmd,
        );

        if err != 0 {
            eprintln!("Error signing document: {:?}", err);
            std::process::exit(1);
        }
        println!("Signed : {:?}", hash_json);
    } else {
        std::process::exit(1);
    }
}
