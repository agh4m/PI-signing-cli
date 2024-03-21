use crate::blockchain::save_certificate;
use crate::communication::send_file;
use crate::util::{hash_file, save_file, traverse_directory};
use clap::Parser;
use dotenv_codegen::dotenv;
use std::env::current_dir;
use std::path::Path;
use std::process::exit;
use std::thread::available_parallelism;

mod blockchain;
mod communication;
mod util;
    mod test;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("disa_cli/sig_lib/library.h");

        // file_name is the path of the file that contains the hashes
        // sig_file is the path of the file that will contain the signature
        fn sig_doc(
            file_name: &str,
            sig_file: &str,
            sign: bool,
            cmd: bool,
            basic_auth_user: &str,
            basicAuthPassword: &str,
            applicationID: &str,
        ) -> i64;
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

    /// Add this flag use Chave Movel Digital
    #[arg(short, long)]
    cmd: bool,

    /// Add this flag to not send the file to the service
    #[arg(short, long)]
    archive_file: bool,

    /// Set the maximum number of threads to use, default is half of the available threads
    #[arg(short, long, default_value_t = 0)]
    threads: usize,
}

fn main() {
    let mode = dotenv!("RELEASE_MODE");

    let args = Args::parse();

    let path = Path::new(&args.path);
    let save_location = Path::new(&args.save_location);
    let cmd = args.cmd;
    let send = !args.archive_file;
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

    let Some(hash_json) = save_file(&documents, &save_location.to_str().unwrap(), &cwd) else {
        eprintln!("Could not save file: {:?}", path);
        exit(1);
    };

    // Load environment variables needed for CMD
    let basic_auth_user = dotenv!("BASIC_AUTH_USER");
    let basic_auth_password = dotenv!("BASIC_AUTH_PASS");
    let application_id = dotenv!("APPLICATION_ID");

    let err = ffi::sig_doc(
        &hash_json,
        &hash_json.replace(".json", ".asics"),
        mode == "production",
        cmd,
        basic_auth_user,
        basic_auth_password,
        application_id,
    );

    if err != 0 {
        eprintln!("Error signing document: 0x{:x}", err);
        exit(1);
    }
    println!("Signed : {:?}", hash_json);

    if send {
        save_certificate(&hash_json);
        send_file(&documents, &save_location.to_str().unwrap());
    }
}
