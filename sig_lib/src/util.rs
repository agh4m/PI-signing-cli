use copy_dir::copy_dir;
use serde::{Deserialize, Serialize};
use serde_json;
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use std::fmt::Debug;
use std::fs;
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

    let save_path: String;

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

pub fn create_tar(path: &Path, save_location: &Path) -> Result<String, std::io::Error> {
    let save_path = format!(
        "{}/archive.tar",
        save_location.to_str().unwrap().trim_end_matches('/')
    );

    if Path::new("/tmp/archive").exists() {
        fs::remove_dir_all("/tmp/archive")?;
    }

    let mut archive = Builder::new(Vec::new());

    if path.is_dir() {
        println!("dir!");
        let errors = copy_dir(path, "/tmp/archive/").unwrap();
        if errors.is_empty() {
            println!("no errors");
        }
        println!("here");
        for error in errors {
            println!("{}", error);
        }
    }

    if path.is_file() {
        // fs::copy()
    }

    archive.append_dir_all("archive", "/tmp/archive")?;
    let hashes_path = format!("{}/hashes.json", save_location.to_str().unwrap());
    let asics_path = format!("{}/hashes.asics", save_location.to_str().unwrap());

    archive
        .append_file("hashes.json", &mut File::open(hashes_path).unwrap())
        .unwrap();
    archive
        .append_file("hashes.asics", &mut File::open(asics_path).unwrap())
        .unwrap();

    let archive = archive.into_inner()?;
    let mut file = File::create(&save_path)?;
    file.write_all(&archive)?;

    // fs::remove_dir_all("/tmp/archive")?;

    return Ok(save_path.to_string());
}

#[cfg(test)]
mod tests {
    use crate::util::create_tar;
    use crate::util::hash_file;
    use crate::util::traverse_directory;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn test_traverse_directory_1t() {
        let path = Path::new("./test_files");
        let files = traverse_directory(path, 1);
        assert_eq!(files.len(), 6);
    }

    #[test]
    fn test_traverse_directory_4t() {
        let path = Path::new("./test_files");
        let files = traverse_directory(path, 4);
        assert_eq!(files.len(), 6);
    }

    #[test]
    fn test_hash_file() {
        let path = Path::new("../LICENSE");
        let hash = hash_file(path).unwrap().hash;
        assert_eq!(hash.len(), 64);
        assert_eq!(
            hash,
            "dc0030b6ebb9fc9b29f658c4c69d58599c1b5edd66d3b7ce7940821aa6a43e8a"
        );
    }

    #[test]
    fn test_create_tar() {
        // This test assumes an existing manifest with its signature
        let path = Path::new("./test_files");
        let save_location = Path::new("/tmp");
        let arch = create_tar(&path, &save_location);
        assert_eq!(arch.is_ok(), true);
        assert!(File::open(arch.unwrap()).is_ok());
    }
}
