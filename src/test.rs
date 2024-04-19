#[cfg(test)]
mod tests {
    use crate::util::create_tar;
    use crate::hash_file;
    use crate::traverse_directory;
    use std::path::Path;
    use std::fs::File;

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
        let path = Path::new("LICENSE");
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
        let save_location = Path::new("./");
        let arch = create_tar(&path, &save_location);
        assert_eq!(arch.is_some(), true);
        assert!(File::open(arch.unwrap()).is_ok());
    }
}
