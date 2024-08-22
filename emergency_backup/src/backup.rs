use eframe::egui;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{Write, copy};
use std::path::{PathBuf, Path};
use std::process::Command;
use std::time::Instant;
use walkdir::WalkDir;
use rfd::FileDialog;
use std::env;

struct MyApp {
    source_path: Option<PathBuf>,
    file_type: String,
    file_types: Vec<String>,
    exit_requested: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        let (source_path, file_type) = read_source_info();
        let file_types = if let Some(ref path) = source_path {
            get_file_extensions(path)
        } else {
            vec!["All types".to_string()]
        };

        Self {
            source_path,
            file_type,
            file_types,
            exit_requested: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if self.exit_requested {
            ctx.request_repaint(); // Richiesta di repaint, utile per assicurare la chiusura
            std::process::exit(0); // Termina il processo in modo sicuro
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Select source directory:");

            if ui.button("Choose Directory").clicked() {
                // Apri una finestra di selezione della directory
                if let Some(path) = FileDialog::new().pick_folder() {
                    self.source_path = Some(path.clone());
                    self.file_types = get_file_extensions(&path);
                    save_source_info(&path.to_string_lossy(), &self.file_type);
                }
            }

            if let Some(ref path) = self.source_path {
                ui.label(format!("Selected directory: {}", path.display()));
            } else {
                ui.label("No directory selected.");
            }

            ui.label("Select file type:");
            egui::ComboBox::from_label("")
                .selected_text(&self.file_type)
                .show_ui(ui, |ui| {
                    for file_type in &self.file_types {
                        ui.selectable_value(&mut self.file_type, file_type.clone(), file_type);
                    }
                });

            if self.source_path.is_some() {
                save_source_info(&self.source_path.as_ref().unwrap().to_string_lossy(), &self.file_type);
            }

            if ui.button("Exit").clicked() {
                self.exit_requested = true; // Segnala che l'uscita è richiesta
            }
        });
    }
}

pub fn get_absolute_path(relative_path: &str) -> PathBuf {
    let relative_path = PathBuf::from(relative_path);
    println!("risultato di current_exe: {:?}", env::current_exe());
    // Se il percorso è già assoluto, restituiscilo così com'è
    if relative_path.is_absolute() {
        relative_path
    } else {
        // Altrimenti, uniscilo con la directory dell'eseguibile
        let exe_path = env::current_exe().expect("Failed to get current exe path");
        let exe_dir = exe_path.parent().expect("Failed to get parent directory");
        exe_dir.join(relative_path)
    }
}

pub fn save_source_info(path: &str, file_type: &str) {
    let file_path = get_absolute_path("assets/source_info.txt");
    println!("il file path nella funzione è {:?}", file_path);
    let mut file = File::create(file_path).expect("Unable to create file");
    file.write_all(format!("{}\n{}", path, file_type).as_bytes()).expect("Unable to write data");
}

pub fn read_source_info() -> (Option<PathBuf>, String) {
    let file_path = get_absolute_path("assets/source_info.txt");
    if let Ok(info_str) = fs::read_to_string(file_path) {
        let mut lines = info_str.lines();
        let path_str = lines.next().unwrap_or("").trim();
        if path_str.is_empty() {
            return (None, "All types".to_string());
        }
        let file_type = lines.next().unwrap_or("All types").to_string();
        (Some(PathBuf::from(path_str)), file_type)
    } else {
        (None, "All types".to_string())
    }
}

pub fn get_file_extensions(path: &Path) -> Vec<String> {
    let mut extensions = HashSet::new();
    extensions.insert("All types".to_string());

    for entry in WalkDir::new(path) {
        let entry = entry.unwrap();
        if let Some(ext) = entry.path().extension().and_then(|e| e.to_str()) {
            extensions.insert(ext.to_lowercase());
        }
    }

    let mut extensions: Vec<String> = extensions.into_iter().collect();
    extensions.sort();
    extensions
}

pub fn get_usb_devices() -> Vec<PathBuf> {
    let mut usb_devices = Vec::new();

    // Esegui il comando `wmic` per ottenere informazioni sui dischi
    let output = Command::new("wmic")
        .arg("diskdrive")
        .arg("where")
        .arg("InterfaceType='USB'")
        .arg("get")
        .arg("DeviceID")
        .output()
        .expect("Failed to execute wmic");

    let output_str = String::from_utf8_lossy(&output.stdout);

    for line in output_str.lines().skip(1) {
        let line = line.trim();
        if !line.is_empty() {
            //let device_id = line.to_string();

            // Usa `wmic` per ottenere informazioni sui volumi associati a ciascun dispositivo
            let volume_output = Command::new("wmic")
                .arg("logicaldisk")
                .arg("get")
                .arg("DeviceID,VolumeName")
                .output()
                .expect("Failed to execute wmic");

            let volume_output_str = String::from_utf8_lossy(&volume_output.stdout);

            for volume_line in volume_output_str.lines().skip(1) {
                let parts: Vec<&str> = volume_line.split_whitespace().collect();
                if parts.len() >= 1 {
                    let drive_letter = parts[0];
                    // Escludi l'unità di sistema (C:) e altre unità non desiderate
                    if drive_letter != "C:" {
                        usb_devices.push(PathBuf::from(drive_letter));
                    }
                }
            }
        }
    }
    usb_devices
}

pub fn get_free_space(path: &PathBuf) -> u64 {
    let path_str = path.to_str().unwrap_or("");
    let output = Command::new("wmic")
        .arg("logicaldisk")
        .arg("where")
        .arg(format!("DeviceID='{}'", path_str))
        .arg("get")
        .arg("FreeSpace")
        .output()
        .expect("Failed to execute wmic");

    let output_str = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = output_str.lines().collect();
    if lines.len() > 1 {
        let free_space_str = lines[1].trim();
        free_space_str.parse().unwrap_or(0)
    } else {
        0
    }
}

pub fn perform_backup() {
    println!("Starting backup..."); // Debug
    let (source_path, file_type) = read_source_info();
    println!("File type: {}", file_type);

    if source_path.is_none() {
        println!("No source path found.");
        return;
    }

    let usb_devices = get_usb_devices();
    println!("USB devices: {:?}", usb_devices);
    let mut max_free_space = 0;
    let mut selected_device = None;

    for device in usb_devices {
        let free_space = get_free_space(&device);
        if free_space > max_free_space {
            max_free_space = free_space;
            selected_device = Some(device);
        }
    }
    println!("Selected device: {:?}, free space: {:?}", selected_device, max_free_space);

    if let Some(device) = selected_device {
        let source_path = source_path.unwrap();
        let destination = device.join(source_path.file_name().unwrap());

        // Verifica se la directory di destinazione esiste, altrimenti creala
        if !destination.exists() {
            if let Err(e) = fs::create_dir_all(&destination) {
                eprintln!("Errore nella creazione della directory di destinazione: {}", e);
                return;
            }
        }

        let start_time = Instant::now();
        let mut total_size = 0;

        for entry in WalkDir::new(source_path.clone()) {
            let entry = entry.unwrap();
            let path = entry.path();

            if file_type != "All types" {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
                if ext != file_type {
                    continue;
                }
            }

            let dest_path = destination.join(entry.path().strip_prefix(source_path.clone()).unwrap_or_else(|_| Path::new("unknown")));
            println!("Destinazione: {:?}", dest_path);

            if entry.file_type().is_dir() {
                fs::create_dir_all(&dest_path).unwrap();
            } else {
                let mut src_file = File::open(entry.path()).unwrap();
                let mut dest_file = File::create(dest_path).unwrap();
                let size = copy(&mut src_file, &mut dest_file).unwrap();
                total_size += size;
            }
        }

        let elapsed_time = start_time.elapsed();
        let total_size_mb = total_size as f64 / (1024.0 * 1024.0);
        let log_content = format!(
            "Backup completed successfully.\nTotal size: {} bytes ({:.2} MB)\nElapsed time: {:.2?} seconds\n",
            total_size, total_size_mb, elapsed_time
        );

        let log_file_path = destination.join("backup_log.txt");
        let mut log_file = File::create(log_file_path).expect("Unable to create log file");
        log_file.write_all(log_content.as_bytes()).expect("Unable to write log data");

        println!("{}", log_content);
    } else {
        println!("No USB device found with enough space.");
    }
}

pub fn open_window() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    eframe::run_native(
        "Backup App",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    ).expect("TODO: panic message");
}

pub fn backup() {
    perform_backup();
}
