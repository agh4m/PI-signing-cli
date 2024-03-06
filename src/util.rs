use sha2::{Digest, Sha256};
use std::fs::File;
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct Document {
    path: String,
    hash: String,
}

pub fn traverse_directory(path: &Path) -> Vec<Document> {
    let mut documents = Vec::new();

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        println!("Path: {:?}", path);

        if path.is_file() {
            let document = hash_file(&path);
            if let Some(document) = document {
                documents.push(document);
            }
        }
        if path.is_dir() {
            println!("Traversing directory: {:?}", path);
            let mut sub_documents = traverse_directory(&path);
            documents.append(&mut sub_documents);
        }
    }

    return documents;
}

fn hash_file(path: &Path) -> Option<Document> {
    let Ok(mut file) = File::open(&path) else {
        return None;
    };

    let mut hasher = Sha256::new();
    let Ok(_) = io::copy(&mut file, &mut hasher) else {
        return None;
    };

    let hash = hasher.finalize().to_vec();
    let string_hash = hash
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>();

    let document = Document {
        path: path.to_str().unwrap().to_string(),
        hash: string_hash,
    };
    return Some(document);
}
