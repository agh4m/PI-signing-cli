use std::io::stdin;
use std::path::Path;
use std::process::exit;
use std::thread::available_parallelism;
use clap::Parser;
use dotenv_codegen::dotenv;
use disa_lib::ffi::sig_doc;
use disa_lib::util::{traverse_directory, hash_file, save_file};
use disa_lib::communication::send_file;

/// CLI to send files to DiSA
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path of the files to send to the service
    #[arg(short, long)]
    path: String,

    /// Path to save the manifest file, default is the current directory, must be a directory
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

    /// authentication token
    #[arg(short, long, default_value = "")]
    bearer_token: String,
}

#[tokio::main]
async fn main() {
    let mode = dotenv!("RELEASE_MODE");

    let args = Args::parse();

    let path = Path::new(&args.path);
    let save_location = Path::new(&args.save_location);
    let cmd = args.cmd;
    let send = !args.archive_file;
    let mut threads = args.threads;
    let mut bearer_token = args.bearer_token;

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

    let Some(hash_json) = save_file(&documents, &save_location.to_str().unwrap()) else {
        eprintln!("Could not save file: {:?}", path);
        exit(1);
    };

    // Load environment variables needed for CMD
    let basic_auth_user = dotenv!("BASIC_AUTH_USER");
    let basic_auth_password = dotenv!("BASIC_AUTH_PASS");
    let application_id = dotenv!("APPLICATION_ID");

    let err = sig_doc(
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
        if bearer_token == "" {
            println!("No valid token provided, please input one bellow.");
            stdin().read_line(&mut bearer_token);
            panic!("This is broken, provide a token via the -b flag");
        }

        // save_certificate(&hash_json);
        send_file(&path, &save_location, &bearer_token).await;
    }
}
