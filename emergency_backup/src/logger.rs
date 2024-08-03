use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufWriter, Write},
    path::Path,
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};
use chrono::Local;
use sysinfo::{Pid, System};
use std::thread;

const LOG_PATH: &str = "C://Emergency Backup Log/cpu_usage.log";
const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB

pub struct Logger {
    pid: Pid,
    interval: Duration,
    condvar: Arc<(Mutex<bool>, Condvar)>,
}

impl Logger {
    // Initialize Logger
    pub fn new(interval: Duration) -> Self {
        let pid = Pid::from(std::process::id() as usize);
        Logger::check_path(); //Check that the log file exists
        Logger {
            pid,
            interval,
            condvar: Arc::new((Mutex::new(false), Condvar::new())),
        }
    }

    // Start logging operation in a separate thread
    pub fn start(&self) -> thread::JoinHandle<()> {
        let condvar = Arc::clone(&self.condvar);
        let interval = self.interval;
        let pid = self.pid;

        thread::spawn(move || {
            println!("[LOGGER] Logging operation started.");
            let mut system = System::new_all();
            let (lock, cvar) = &*condvar;

            loop {
                let mut stop_guard = lock.lock().unwrap();
                let result = cvar.wait_timeout(stop_guard, interval).unwrap();
                stop_guard = result.0;
                if *stop_guard { break;}

                system.refresh_all();
                if let Some(process) = system.process(pid) {
                    let cpu_usage = process.cpu_usage();
                    Logger::write_on_file(cpu_usage);
                    println!("[LOGGER] CPU Usage for process {}: {:.2}%", pid, cpu_usage);
                } else {
                    println!("[LOGGER] Process not found.");
                }
            }
            println!("[LOGGER] Logging operation terminated.");
        })
    }

    // Stop the logging operation
    pub fn stop(&self) {
        let (lock, cvar) = &*self.condvar;
        let mut guard = lock.lock().unwrap();
        
        if *guard == false {
            *guard = true;
            cvar.notify_all();
        }    
    }

    // Write CPU usage on log file
    fn write_on_file(cpu_usage: f32) {
        Self::check_path();
        Self::check_size();

        // Get the current timestamp
        let now = Local::now();
        let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

        // Create the log message
        let log_message = format!("[{}] {:.2}%\n", timestamp, cpu_usage);

        // Open the log file in append mode
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(LOG_PATH)
            .expect("Unable to open log file");

        let mut file = BufWriter::new(file);
        file.write_all(log_message.as_bytes()).expect("Unable to write data");
    }

    // Check directory and file path
    fn check_path() {
        let path = Path::new(LOG_PATH);
        let dir = path.parent().unwrap();

        if !dir.exists() {
            std::fs::create_dir_all(dir).expect("Unable to create log directory");
        }

        if !path.exists() {
            OpenOptions::new()
                .create(true)
                .write(true)
                .open(LOG_PATH)
                .expect("Unable to create log file");
        }
    }

    // Check log file size
    fn check_size() {
        let metadata = std::fs::metadata(LOG_PATH);
        if let Ok(metadata) = metadata {
            if metadata.len() > MAX_LOG_SIZE {
                // Truncate the file if it exceeds the maximum size
                File::create(LOG_PATH).expect("Unable to truncate log file");
            }
        }
    }

    // ----- For testing purpose ----- //

    // Verify if logging is active
    pub fn is_logging_active(&self) -> bool {
        let (lock, _) = &*self.condvar;
        let guard = lock.lock().unwrap();
        !*guard
    }

    // Verify log file existence and size
    pub fn log_file_exists(&self) -> bool {
        Path::new(LOG_PATH).exists()
    }

    // Retrieve log entries
    pub fn retrieve_log_entries() -> Vec<String> {
        let file = File::open(LOG_PATH).expect("Unable to open log file");
        let reader = std::io::BufReader::new(file);
        reader.lines().map(|line| line.expect("Unable to read line")).collect()
    }
}
