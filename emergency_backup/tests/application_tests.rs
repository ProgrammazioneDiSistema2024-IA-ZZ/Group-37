#[cfg(test)]
mod integration_tests {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    use emergency_backup::{detector, logger::Logger, processor::Processor};
    use enigo::{Button, Coordinate, Enigo, Mouse, Settings, Direction::{Press, Release}};

    #[test]
    fn test_processor_logger_integration() {
        // Initialize the mpsc channel
        let (tx, rx) = mpsc::channel();

        // Create and start the Logger thread
        let log_path = "C://Emergency Backup Log/test_cpu_usage_processor_logger.log";
        let logger = Logger::new(Duration::from_secs(2)); // Adjust interval for test
        let logger_handle = logger.start();

        // Create and start the Processor thread
        let processor = Processor::new();
        let processor_handle = processor.start(rx);

        // Simulate sending command to processor
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs(1)); // Wait for the processor to process the command

        // Verify that the Logger created a log file
        assert!(logger.log_file_exists(), "Log file was not created!");

        // Verify that the processor received command 1
        assert!(processor.is_command1_received(), "Processor did not receive command 1!");

        // Stop threads
        logger.stop();
        processor.stop();
        logger_handle.join().unwrap();
        processor_handle.join().unwrap();

        // Clean up test log file
        std::fs::remove_file(log_path).ok();
    }

    #[test]
    fn test_detector_logger_integration() {
        // Initialize the mpsc channel
        let (tx, rx) = mpsc::channel();

        // Create and start the Logger thread
        let log_path = "C://Emergency Backup Log/test_cpu_usage_detector_logger.log";
        let logger = Logger::new(Duration::from_secs(2)); // Adjust interval for test
        let logger_handle = logger.start();

        // Receiver thread
        let receiver_handle = {
            thread::spawn(move || {
                let mut msg = rx.recv().unwrap();
                while msg == 0 {
                    msg = rx.recv().unwrap();
                }
                return msg;
            })
        };

        // Detector thread
        let detector_handle = {
            thread::spawn(move || {
                detector::run(tx);
            })
        };

        // Simulate rectangle mouse movement to trigger the detector
        thread::sleep(Duration::from_secs(1));

        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let points = vec![(10, 10), (1900, 10),(1900, 1080),(10, 1080), (10, 10)];
        
        // Click and move the mouse
        enigo.button(Button::Left, Press).unwrap();
        for (x, y) in points {
            enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
            thread::sleep(Duration::from_millis(50)); // Adjust delay as needed
        }
        enigo.button(Button::Left, Release).unwrap();
        
        // Verify that the receiver got the rectangle message from the detector
        let msg = receiver_handle.join().unwrap();
        assert!(msg == 1, "Rectangle not detected!");

        // Verify that the Logger created a log file
        assert!(logger.log_file_exists(), "Log file was not created!");

        // Stop threads
        logger.stop();
        detector_handle.join().unwrap();
        logger_handle.join().unwrap();

        // Clean up test log file
        std::fs::remove_file(log_path).ok();
        
    }

    #[test]
    fn test_processor_detector_integration() {
        // Initialize the mpsc channel
        let (tx, rx) = mpsc::channel();

        // Create and start the Processor thread
        let processor = Processor::new();
        let processor_handle = processor.start(rx);

        // Detector thread
        let detector_handle = {
            thread::spawn(move || {
                detector::run(tx);
            })
        };

        // Simulate rectangle mouse movement to trigger the detector
        thread::sleep(Duration::from_secs(1));

        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let points = vec![(10, 10), (1900, 10),(1900, 1080),(10, 1080), (10, 10)];
        
        // Click and move the mouse
        enigo.button(Button::Left, Press).unwrap();
        for (x, y) in points {
            enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
            thread::sleep(Duration::from_millis(50)); // Adjust delay as needed
        }
        enigo.button(Button::Left, Release).unwrap();

        thread::sleep(Duration::from_secs(1));

        // Verify that the processor received command 1
        assert!(processor.is_command1_received(), "Processor did not receive command 1!");

        // Stop threads
        processor.stop();
        detector_handle.join().unwrap();
        processor_handle.join().unwrap();
    }

    #[test]
    fn test_dlp_system_integration() {
        // Initialize the mpsc channel
        let (tx, rx) = mpsc::channel();

        // Create and start the Logger thread
        let log_path = "C://Emergency Backup Log/test_cpu_usage.log";
        let logger = Logger::new(Duration::from_secs(2)); // Adjust interval for test
        let logger_handle = logger.start();

        // Create and start the Processor thread
        let processor = Processor::new();
        let processor_handle = processor.start(rx);

        // Detector thread
        let detector_handle = {
            thread::spawn(move || {
                detector::run(tx);
            })
        };

        // Simulate rectangle mouse movement
        thread::sleep(Duration::from_secs(1));

        let mut enigo = Enigo::new(&Settings::default()).unwrap();
        let points = vec![(10, 10), (1900, 10),(1900, 1080),(10, 1080), (10, 10)];
        
        // Click and move the mouse
        enigo.button(Button::Left, Press).unwrap();
        for (x, y) in points {
            enigo.move_mouse(x, y, Coordinate::Abs).unwrap();
            thread::sleep(Duration::from_millis(50)); // Adjust delay as needed
        }
        enigo.button(Button::Left, Release).unwrap();

        thread::sleep(Duration::from_secs(1));

        // Verify that the Logger created a log file
        assert!(logger.log_file_exists(), "Log file was not created!");

        // Verify that the processor received command 1
        assert!(processor.is_command1_received(), "Processor did not receive command 1!");

        // Stop threads
        logger.stop();
        processor.stop();
        logger_handle.join().unwrap();
        processor_handle.join().unwrap();
        detector_handle.join().unwrap();

         // Clean up test log file
         std::fs::remove_file(log_path).ok();
    }
}
