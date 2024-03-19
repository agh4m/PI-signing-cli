use crate::communication::send_file;
use crate::util::{hash_file, save_file, traverse_directory};
use clap::Parser;
use dotenv_codegen::dotenv;
use std::env::current_dir;
use std::path::Path;
use std::process::exit;
use std::thread::available_parallelism;

mod communication;
mod util;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cli_test/sig_lib/library.h");

        fn sig_doc(
            sha: &str,
            file_name: &str,
            sign: bool,
            cmd: bool,
            basicAuthUser: &str,
            basicAuthPassword: &str,
            applicationID: &str,
        ) -> i32;
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

    /// Set to 1 to not send the file to the service
    #[arg(short, long, default_value_t = 0)]
    arquive_file: u8,

    /// Set the maximum number of threads to use, default is half of the available threads
    #[arg(short, long, default_value_t = 0)]
    threads: usize,
}

fn main() {
    let mode = dotenv!("RELEASE_MODE");

    let args = Args::parse();

    let path = Path::new(&args.path);
    let save_location = Path::new(&args.save_location);
    let cmd = args.cmd == 0;
    let send = args.arquive_file == 0;
    let mut threads = args.threads;

    let cwd = current_dir().unwrap();

    if threads == 0 {
        threads = available_parallelism().unwrap().get() / 2;
    }

    if !path.exists() {
        eprintln!("Path does not exist: {:?}", path);
        exit(1);
    }

    let mut documents = Vec::new();

    if path.is_dir() {
        documents = traverse_directory(&path, threads);
    }

    if path.is_file() {
        if let Some(document) = hash_file(&path) {
            documents.push(document);
        } else {
            eprintln!("Could not hash file: {:?}", path);
            exit(1);
        }
    }

    if let Some(hash_json) = save_file(&documents, &save_location.to_str().unwrap(), &cwd) {
        let err = ffi::sig_doc(
            &hash_json,
            &hash_json.replace(".json", ".sig"),
            mode == "production",
            cmd,
            "",
            "",
            ""
        );

        if err != 0 {
            eprintln!("Error signing document: {:?}", err);
            exit(1);
        }
        println!("Signed : {:?}", hash_json);
    } else {
        exit(1);
    }

    if send {
        send_file(&documents, &save_location.to_str().unwrap());
    }
}
