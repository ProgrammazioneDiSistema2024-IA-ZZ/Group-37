use eframe::egui;
use std::fs::{self, File};
use std::io::{Write, copy};
use std::path::{PathBuf, Path};
use std::process::Command;
use walkdir::WalkDir;
use rfd::FileDialog; // Per la selezione di file e directory

struct MyApp {
    source_path: Option<PathBuf>,
    exit_requested: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            source_path: None,
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
                    self.source_path = Some(path);
                    save_source_path(&self.source_path.as_ref().unwrap().to_string_lossy());
                }
            }

            if let Some(ref path) = self.source_path {
                ui.label(format!("Selected directory: {}", path.display()));
            } else {
                ui.label("No directory selected.");
            }

            if ui.button("Exit").clicked() {
                self.exit_requested = true; // Segnala che l'uscita è richiesta
            }
        });
    }
}

fn save_source_path(path: &str) {
    let mut file = File::create("assets/source_path.txt").expect("Unable to create file");
    file.write_all(path.as_bytes()).expect("Unable to write data");
}

fn read_source_path() -> PathBuf {
    let path_str = fs::read_to_string("assets/source_path.txt").expect("Unable to read file");
    PathBuf::from(path_str.trim())
}

fn get_usb_devices() -> Vec<PathBuf> {
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
            let device_id = line.to_string();

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

fn get_free_space(path: &PathBuf) -> u64 {
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

fn perform_backup() {
    println!("Starting backup..."); // Debug
    let source_path = match read_source_path().as_path().to_str() {
        Some(path_str) => PathBuf::from(path_str),
        None => {
            println!("Failed to read source path.");
            return;
        }
    };

    let usb_devices = get_usb_devices();
    println!("usb devices: {:?}", usb_devices);
    let mut max_free_space = 0;
    let mut selected_device = None;

    for device in usb_devices {
        let free_space = get_free_space(&device);
        if free_space > max_free_space {
            max_free_space = free_space;
            selected_device = Some(device);
        }
    }
    println!("Selected device: {:?}, free_space: {:?}", selected_device, max_free_space);

    if let Some(device) = selected_device {
        let destination = device.join(source_path.file_name().unwrap());

        for entry in WalkDir::new(source_path.clone()) {
            let entry = entry.unwrap();
            let dest_path = destination.join(entry.path().strip_prefix(source_path.clone()).unwrap_or_else(|_| Path::new("unknown")));
            println!("Destinazione: {:?}", dest_path);
            if entry.file_type().is_dir() {
                println!("creato la cartella");
                fs::create_dir_all(&dest_path).unwrap();
            } else {
                let mut src_file = File::open(entry.path()).unwrap();
                let mut dest_file = File::create(dest_path).unwrap();
                copy(&mut src_file, &mut dest_file).unwrap();
            }
        }

        println!("Backup completed successfully.");
    } else {
        println!("No USB device found with enough space.");
    }
}

pub fn run() {
    let native_options = eframe::NativeOptions {
        ..Default::default()
    };

    // eframe::run_native(
    //     "Backup App",
    //     native_options,
    //     Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    // );

    perform_backup();
}
