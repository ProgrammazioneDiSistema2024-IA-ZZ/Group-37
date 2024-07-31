use std::sync::mpsc;
   use std::thread;
//mod piston_example;
mod detector;
mod backup;

fn main() {

   // Initialize the mpsc channel
   //let (tx, rx) = mpsc::channel();

   // Use Detector with main thread
   //detector::run(tx);

   // while let Ok(received) = rx.recv() {
   //    println!("sono nel for del receiver");
   //    match received {
   //       1 => println!("Rettangolo identificato"),
   //       2 => println!("Segno meno identificato"),
   //       0 => println!("Forma non riconosciuta"),
   //       _ => println!("Errore sconosciuto"),
   //    }
   // }

   backup::run();

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
