#[cfg(test)]
use emergency_backup::backup;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use std::collections::HashSet;

#[test]
fn test_get_file_extensions() {
    // Crea una directory temporanea
    let temp_dir = Path::new("test_dir");
    fs::create_dir_all(temp_dir).unwrap();

    // Crea file di test
    fs::File::create(temp_dir.join("test1.txt")).unwrap();
    fs::File::create(temp_dir.join("test2.rs")).unwrap();
    fs::File::create(temp_dir.join("test3.md")).unwrap();

    let extensions = backup::get_file_extensions(&temp_dir.to_path_buf());

    // Controlla se le estensioni sono corrette
    assert!(extensions.contains(&"All types".to_string()));
    assert!(extensions.contains(&"txt".to_string()));
    assert!(extensions.contains(&"md".to_string()));
    assert!(extensions.contains(&"rs".to_string()));

    // Pulisci la directory temporanea
    fs::remove_dir_all(temp_dir).unwrap();
}

#[test]

#[test]
fn test_perform_backup_no_space() {
    // Crea una directory temporanea come sorgente
    let temp_dir = Path::new("test_source_dir");
    fs::create_dir_all(temp_dir).unwrap();
    let temp_file = temp_dir.join("test_file.txt");
    let mut file = fs::File::create(&temp_file).unwrap();
    writeln!(file, "Test content").unwrap();

    // Mock di un dispositivo USB
    let usb_dir = Path::new("test_usb_dir");
    fs::create_dir_all(usb_dir).unwrap();

    // Mock per la lettura di informazioni sulla sorgente
    backup::save_source_info(temp_dir.to_str().unwrap(), "txt");

    // Override delle funzioni di sistema per simulare il rilevamento di un dispositivo USB
    let usb_devices = vec![usb_dir.to_path_buf()];

    // Mock per ottenere spazio libero insufficiente
    let free_space = 1; // 1 byte

    // Esegui il backup
    backup::perform_backup();

    // Verifica che il file di backup non esista
    let backup_file = usb_dir.join("test_source_dir").join("test_file.txt");
    assert!(!backup_file.exists());

    // Pulizia
    fs::remove_dir_all(temp_dir).unwrap();
    fs::remove_dir_all(usb_dir).unwrap();
}


#[test]
fn test_save_and_read_source_info() {
    let temp_dir = Path::new("test_source_info_dir");
    fs::create_dir_all(temp_dir).unwrap();

    let temp_file = "test_source_info.txt";

    // Salva le informazioni di test
    backup::save_source_info(temp_dir.to_str().unwrap(), "txt");

    // Leggi le informazioni salvate
    let (path, file_type) = backup::read_source_info();

    assert_eq!(path, Some(temp_dir.to_path_buf()));
    assert_eq!(file_type, "txt");

    // Pulisci il file temporaneo e la directory
    fs::remove_file(temp_file).unwrap();
    fs::remove_dir_all(temp_dir).unwrap();
}



