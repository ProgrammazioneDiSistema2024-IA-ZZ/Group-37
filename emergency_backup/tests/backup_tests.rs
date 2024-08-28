#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use tempfile::TempDir;
    use emergency_backup::backup;
    // Helper function to create a temporary directory and file for testing
    fn create_temp_dir_with_file() -> (TempDir, PathBuf) {
        let temp_dir = TempDir::new().expect("Unable to create temp dir");
        let file_path = temp_dir.path().join("test_file.txt");
        fs::write(&file_path, "test").expect("Unable to write test file");
        (temp_dir, file_path)
    }

    #[test]
    fn test_save_and_read_source_info() {
        let temp_dir = TempDir::new().expect("Unable to create temp dir");
        let path_str = temp_dir.path().to_string_lossy().to_string();
        let file_type = "txt";

        backup::save_source_info(&path_str, file_type);

        let (read_path, read_file_type) = backup::read_source_info();
        assert_eq!(read_path.as_ref().map(|p| p.to_string_lossy().to_string()), Some(path_str));
        assert_eq!(read_file_type, file_type);
    }

    #[test]
    fn test_get_usb_devices() {
        let devices = backup::get_usb_devices();
        //inserire una chiavetta e modificare il valore
        assert!(devices.len() >= 1);
    }

    #[test]
    fn test_get_file_extensions() {
        let (temp_dir, _) = create_temp_dir_with_file();
        let extensions = backup::get_file_extensions(temp_dir.path());
        assert!(extensions.contains(&"txt".to_string()));
    }

}
