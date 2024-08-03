use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::sync::{Condvar, Mutex, mpsc::Receiver};
use std::thread;
use std::time::Duration;
use crate::util;
use crate::backup;

pub struct Processor {
    c1_received: Arc<(Mutex<bool>, Condvar)>,
    stop: Arc<AtomicBool>,
}

impl Processor {
    // Initialize Processor
    pub fn new() -> Self {
        Processor {
            c1_received: Arc::new((Mutex::new(false), Condvar::new())),
            stop: Arc::new(AtomicBool::new(false)),
        }
    }

    // Start the processor in a separate thread
    pub fn start(&self, rx: Receiver<i32>) -> thread::JoinHandle<()> {
        let stop_clone = Arc::clone(&self.stop);
        let c1_received_clone = Arc::clone(&self.c1_received);

        thread::spawn(move || {
            println!("[PROCESSOR] Processor started.");

            while !stop_clone.load(Ordering::SeqCst) {
                match rx.recv_timeout(Duration::from_secs(1)) {
                    Ok(command) => match command {
                        1 => Processor::command1_received(&c1_received_clone),
                        2 => Processor::command2_received(&c1_received_clone),
                        _ => println!("[PROCESSOR] Command not recognized."),
                    },
                    Err(mpsc::RecvTimeoutError::Timeout) => {
                        // Check if stop signal is set
                        if stop_clone.load(Ordering::SeqCst) {
                            break;
                        }
                    }
                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                        println!("[PROCESSOR] Receiver error. Exiting loop.");
                        stop_clone.store(true, Ordering::SeqCst);
                        break;
                    }
                }
            }
            println!("[PROCESSOR] Processor stopped.");
        })

    }

    // Stop the processor
    pub fn stop(&self) {
        self.stop.store(true, Ordering::SeqCst);
    }

    // Process command 1
    fn command1_received(c1_received: &Arc<(Mutex<bool>, Condvar)>) {
        println!("[PROCESSOR] Command 1 received.");
        let (lock, cvar) = &**c1_received;
        let mut c1_rec = lock.lock().unwrap();
        *c1_rec = true;
        cvar.notify_all();

        Processor::start_timer(Arc::clone(c1_received));
    }

    // Process command 2
    fn command2_received(c1_received: &Arc<(Mutex<bool>, Condvar)>) {
        println!("[PROCESSOR] Command 2 received.");
        let (lock, cvar) = &**c1_received;
        let mut c1_received = lock.lock().unwrap();
        if *c1_received {
            println!("[PROCESSOR] Command 1 was received in time.");
            *c1_received = false;
            cvar.notify_all();

            Processor::show_popup();
            println!("[PROCESSOR] Starting backup procedure.");
            Processor::start_backup_procedure();
        } else {
            println!("[PROCESSOR] Command 1 was not received in time.");
        }
    }

    // Start timer of 10s when command 1 is received
    fn start_timer(c1_received: Arc<(Mutex<bool>, Condvar)>) {
        thread::spawn(move || {
            let (lock, cvar) = &*c1_received;
            let c1_received = lock.lock().unwrap();
            let result = cvar.wait_timeout(c1_received, Duration::new(10, 0)).unwrap();

            if result.1.timed_out() {
                let mut c1_received = result.0;
                *c1_received = false;
                println!("[PROCESSOR] Timer expired. c1_received set to false.");
            }
        });
    }

    //After receiving command 1 and command 2 notify the user with a pop-up
    fn show_popup() {
        util::popup("Conferma Backup", "Il backup verrà avviato alla\n chiusura di questo pop-up.", util::Sound::Info);
    }

    fn start_backup_procedure() {
        // Create Backup thread 
        let backup_handle = thread::spawn(move || {
            backup::backup();
        });
        backup_handle.join().unwrap();
    }
    
    // ---- For testing purpose ----- //

    pub fn is_command1_received(&self) -> bool {
        let (lock, _) = &*self.c1_received;
        let c1_received = lock.lock().unwrap();
        *c1_received
    }

    pub fn is_active(&self) -> bool {
        !self.stop.load(Ordering::SeqCst)
    }
}
