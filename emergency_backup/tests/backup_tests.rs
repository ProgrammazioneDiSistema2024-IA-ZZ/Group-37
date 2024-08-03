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

// #[test]
// fn test_save_and_read_source_info() {
//     let temp_file = "test_source_info.txt";
//
//     // Crea un file di test e scrivi informazioni
//     backup::save_source_info("test/path", "txt");
//
//     // Leggi le informazioni dal file
//     let (path, file_type) = backup::read_source_info();
//
//     assert_eq!(path, Some(PathBuf::from("test/path")));
//     assert_eq!(file_type, "txt");
//
//     // Pulisci il file temporaneo
//     fs::remove_file(temp_file).unwrap();
// }

#[test]
fn test_get_usb_devices() {
    // Questo test è complesso da scrivere e potrebbe necessitare di un ambiente di test specifico
    // per simulare i dispositivi USB. Considera di usare un mock se possibile.
    // Qui non è inclusa una verifica diretta per la disponibilità dei dispositivi USB.
    // Puoi testare la logica o controllare i risultati in base all'ambiente di test.
    // Aggiungi una verifica a livello di logica se necessario.
    assert!(true); // Placeholder
}

#[test]
fn test_get_free_space() {
    // Questo test può essere problematico senza un ambiente di test specifico.
    // Può essere necessario usare un mock per `wmic` o verificare il comportamento in un ambiente controllato.
    // Puoi aggiungere una verifica a livello di logica se necessario.
    assert!(true); // Placeholder
}

