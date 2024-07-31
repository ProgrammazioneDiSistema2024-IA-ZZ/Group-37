use std::sync::mpsc;
   use std::thread;
//mod piston_example;
mod detector;

fn main() {

    //piston_example::run();
   // Initialize the mpsc channel
   let (tx, rx) = mpsc::channel();

   // Create Detector thread with sender
   // let detector_handle = thread::spawn(move || {
   //     detector::run(tx);
   // });

   // Use Detector with main thread
   detector::run(tx);

   // while let Ok(received) = rx.recv() {
   //    println!("sono nel for del receiver");
   //    match received {
   //       1 => println!("Rettangolo identificato"),
   //       2 => println!("Segno meno identificato"),
   //       0 => println!("Forma non riconosciuta"),
   //       _ => println!("Errore sconosciuto"),
   //    }
   // }

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
