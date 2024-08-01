use std::sync::mpsc;
use std::thread;
mod detector;
mod backup;

fn main() {
   // Initialize the mpsc channel
   let (tx, rx) = mpsc::channel();

   // Creo un thread per backup
   let backup_handle = thread::spawn(move || {
      while let Ok(received) = rx.recv() {
         println!("Sono nel receiver");
         match received {
            1 => println!("Rettangolo riconosciuto"),
            2 => backup::backup(),
            0 => println!("Forma non riconosciuta"),
            _ => println!("Errore sconosciuto"),
         }
      }
   });

   // Il detector deve stare sul main thread, non sono sicuro del perché
   detector::run(tx);

   // Ensure the backup thread has finished
   backup_handle.join().unwrap();

   // // Create Logger thread (no arguments needed)
   // let logger_handle = thread::spawn(|| {
   //     //TODO: logger();
   // });
   //
   // // Create Processor thread with receiver
   // let processor_handle = thread::spawn(move || {
   //     //TODO: processor(rx);
   // });
   //
   // // Wait for all threads to finish
   // detector_handle.join().unwrap();
   // logger_handle.join().unwrap();
   // processor_handle.join().unwrap();
}