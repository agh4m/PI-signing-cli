use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs::File;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use std::{thread, usize};
use tar::Builder;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Document {
    pub path: String,
    pub hash: String,
}

fn visit_file(path: &Path, docs: &Arc<Mutex<Vec<Document>>>) {
    let document = hash_file(&path);
    if let Some(document) = document {
        let mut documents = docs.lock().unwrap();
        documents.push(document);
    } else {
        eprintln!("Could not hash file: {:?}", path);
    }
}

fn visit(
    og_path: &Path,
    docs: Arc<Mutex<Vec<Document>>>,
    dirs: Arc<Mutex<VecDeque<Box<PathBuf>>>>,
    count: Arc<Mutex<usize>>,
) {
    *count.lock().unwrap() += 1;
    for entry in og_path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            visit_file(&path, &docs);
        }
        if path.is_dir() {
            let mut dirs = dirs.lock().unwrap();
            let path = Box::new(path);
            dirs.push_back(path);
        }
    }
    *count.lock().unwrap() -= 1;
}

pub fn traverse_directory(path: &Path, threads: usize) -> Vec<Document> {
    let documents = Arc::new(Mutex::new(Vec::new()));
    let dirs: Arc<Mutex<VecDeque<Box<PathBuf>>>> = Arc::new(Mutex::new(VecDeque::new()));
    let count: Arc<Mutex<usize>> = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    dirs.lock().unwrap().push_back(Box::new(path.to_path_buf()));

    while !dirs.lock().unwrap().is_empty() {
        let mut dir = dirs.lock().unwrap();
        let path = dir.pop_front().unwrap();
        let documents = Arc::clone(&documents);
        let dirs = Arc::clone(&dirs);
        let cnt = Arc::clone(&count);

        let handle = thread::spawn(move || visit(&path, documents, dirs, cnt));
        handles.push(handle);

        while *count.lock().unwrap() > threads {
            sleep(Duration::from_millis(1));
        }

        drop(dir);
        sleep(Duration::from_millis(1));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    return documents.lock().unwrap().to_vec();
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
    } else {
        panic!("Save Path must be a directory!");
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

pub fn create_tar(path: &Path, save_location: &Path) -> Option<String> {
    let save_path = format!("{}/archive.tar", save_location.to_str().unwrap().trim_end_matches('/'));

    let mut archive = Builder::new(Vec::new());

    println!("Creating archive at: {:?}", save_path);

    if path.is_dir() {
        let Ok(_) = archive.append_dir_all(path.file_name().unwrap(), path) else {
            eprintln!("Could not append directory to archive");
            return None;
        };
    }

    if path.is_file() {
        let Ok(_) = archive.append_file(path.file_name().unwrap(), &mut File::open(path).unwrap()) else {
            eprintln!("Could not append file to archive");
            return None;
        };
    }

    let hashes_path = format!("{}/hashes.json", save_location.to_str().unwrap());
    let asics_path = format!("{}/hashes.asics", save_location.to_str().unwrap());

    archive.append_file("hashes.json", &mut File::open(hashes_path).unwrap()).unwrap();
    archive.append_file("hashes.asics", &mut File::open(asics_path).unwrap()).unwrap();

    let Ok(archive) = archive.into_inner() else {
        eprintln!("Could not create archive");
        return None;
    };

    let Ok(mut file) = File::create(&save_path) else {
        eprintln!("Could not create file: {:?}", save_path);
        return None;
    };

    let Ok(_) = file.write_all(&archive) else {
        eprintln!("Could not write archive to file: {:?}", save_path);
        return None;
    };

    return Some(save_path.to_string());
}
