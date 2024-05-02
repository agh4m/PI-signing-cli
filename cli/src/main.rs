use clap::Parser;
use disa_lib::blockchain::save_certificate;
use disa_lib::communication::send_file;
use disa_lib::ffi::sig_doc;
use disa_lib::util::{hash_file, save_file, traverse_directory};
use dotenv_codegen::dotenv;
use std::path::Path;
use std::process::exit;
use std::thread::available_parallelism;

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

    /// skip all steps, send only to blockchain, path must be a file with this flag
    #[arg(short, long)]
    only_blockchain: bool,
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
    let bearer_token = args.bearer_token;
    let blockchain = args.only_blockchain;

    if threads == 0 {
        threads = available_parallelism().unwrap().get() / 2;
    }

    if !path.exists() {
        eprintln!("Path does not exist: {:?}", path);
        exit(1);
    }

    let mut documents = Vec::new();

    if path.is_dir() && !blockchain {
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

    // Load environment variables needed for CMD
    let basic_auth_user = dotenv!("BASIC_AUTH_USER");
    let basic_auth_password = dotenv!("BASIC_AUTH_PASS");
    let application_id = dotenv!("APPLICATION_ID");

    let Some(hash_json) = save_file(&documents, &save_location.to_str().unwrap()) else {
        eprintln!("Could not save file: {:?}", path);
        exit(1);
    };
    if !blockchain {
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
    }

    let contract_address = dotenv!("CONTRACT_ADDRESS");
    let node_url = dotenv!("NODE_URL");
    let private_key = dotenv!("PRIVATE_KEY");
    let wallet_address = dotenv!("WALLET_ADDRESS");

    if send {
        let address = save_certificate(
            &hash_json,
            node_url,
            contract_address,
            private_key,
            wallet_address,
        )
        .await
        .unwrap_or_else(|_| "".to_string());

        println!("blockchain address {}", address);

        if !blockchain {
            send_file(&path, &save_location, &bearer_token).await;
        }
    }
}
