use crate::util::traverse_directory;
use clap::Parser;
use cxx::let_cxx_string;
use std::path::Path;

mod util;

#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("cli_test/sig_lib/library.h");

        fn sig_doc(path: &CxxString) -> i32;
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let args = Args::parse();

    let path = Path::new(&args.path);

    if !path.exists() {
        eprintln!("Path does not exist: {:?}", path);
        std::process::exit(1);
    }

    if path.is_dir() {
        let documents = traverse_directory(&path);
        println!("Documents: {:?}", documents);
    }

    let_cxx_string!(path = path.to_str().unwrap());
    ffi::sig_doc(&path);

    println!("Document: {:?}", path);
}
