use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document {
    pub path: String,
    pub hash: String,
}

pub fn traverse_directory(path: &Path) -> Vec<Document> {
    let mut documents = Vec::new();

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            let document = hash_file(&path);
            if let Some(document) = document {
                documents.push(document);
            } else {
                eprintln!("Could not hash file: {:?}", path);
            }
        }
        if path.is_dir() {
            let mut sub_documents = traverse_directory(&path);
            documents.append(&mut sub_documents);
        }
    }

    if documents.is_empty() {
        eprintln!(
            "No documents found in directory: {:?}, or could not hash them",
            path
        );
    }

    return documents;
}

pub fn hash_file(path: &Path) -> Option<Document> {
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

pub fn save_file(documents: &Vec<Document>, path: &str) -> Option<String> {
    let Ok(json) = serde_json::to_string(&documents) else {
        eprintln!("Could not serialize hashes to JSON");
        return None;
    };

    let mut save_path: String = "".to_string();

    if Path::new(path).is_dir() {
        save_path = format!("{}/hashes.json", path.trim().trim_end_matches('/'));
    }
    let Ok(mut file) = File::create(&save_path) else {
        eprintln!("Could not create file: {:?}", path);
        return None;
    };
    println!("Saving to: {:?}", save_path);

    let Ok(_) = file.write_all(json.as_bytes()) else {
        eprintln!("Could not write hashes to file: {:?}", path);
        return None;
    };

    return Some(save_path);
}
