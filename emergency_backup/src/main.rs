use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use emergency_backup::util;
use emergency_backup::detector;
use emergency_backup::logger::Logger;
use emergency_backup::processor::Processor;

fn main() {
   // Initialize the mpsc channel
   let (tx, rx) = mpsc::channel();

   // Create Detector thread 
   let detector_handle = thread::spawn(move || {
      detector::run(tx);
   });

   // Create Logger thread 
   let logger = Logger::new(Duration::from_secs(120));
   let logger_handle = logger.start();

   // Create Processor thread 
   let processor = Processor::new();
   let processor_handle = processor.start(rx);

   signal_handler(logger, processor);

   logger_handle.join().unwrap();
   processor_handle.join().unwrap();
   detector_handle.join().unwrap();
}

fn signal_handler(logger: Logger, processor:Processor ){
   ctrlc::set_handler(move || {
      println!("[MAIN] Termination signal received. Shutting down threads...");
      logger.stop();
      processor.stop();
  }).expect("Error setting Ctrl-C handler");
}
