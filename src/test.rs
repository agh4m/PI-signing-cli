#[cfg(test)]
mod tests {
    use crate::traverse_directory;
    use std::path::Path;

    #[test]
    fn test_traverse_directory_1t() {
        let path = Path::new("./test_files");
        let files = traverse_directory(path, 1);
        assert_eq!(files.len(), 6);
        assert_eq!(files[0].path, "./test_files/test2");
    }

    #[test]
    fn test_traverse_directory_4t() {
        let path = Path::new("./test_files");
        let files = traverse_directory(path, 4);
        assert_eq!(files.len(), 6);
    }
}
