#[cfg(test)]
mod processor_tests {
    use std::sync::mpsc;
    use std::time::Duration;
    use std::thread;
    use emergency_backup::processor::Processor;
    use serial_test::serial;

    #[test]
    #[serial]
    fn test_processor_command1_then_command2() {
        let (tx, rx) = mpsc::channel();
        let processor= Processor::new();
        let handle = processor.start(rx);

        // Sending Command 1
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs(1));

        // Verify Command 1 was received
        assert!(processor.is_command1_received());

        // Sending Command 2
        tx.send(2).unwrap();
        thread::sleep(Duration::from_secs(5));

        // Verify Command 1 is no longer received
        assert!(!processor.is_command1_received());

        processor.stop();
        handle.join().unwrap();
    }

    #[test]
    #[serial]
    fn test_processor_command1() {
        let (tx, rx) = mpsc::channel();
        let processor= Processor::new();
        let handle = processor.start(rx);

        // Sending Command 1
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs(2));

        // Verify Command 1 was received
        assert!(processor.is_command1_received());

        processor.stop();
        handle.join().unwrap();
    }

    #[test]
    #[serial]
    fn test_processor_command2_without_command1() {
        let (tx, rx) = mpsc::channel();
        let processor= Processor::new();
        let handle = processor.start(rx);

        // Sending Command 2
        tx.send(2).unwrap();
        thread::sleep(Duration::from_secs(2));

        // Verify Command 1 was not received
        assert!(!processor.is_command1_received());

        processor.stop();
        handle.join().unwrap();
    }

    #[test]
    #[serial]
    fn test_processor_command1_timeout() {
        let (tx, rx) = mpsc::channel();
        let processor= Processor::new();
        let handle = processor.start(rx);

        // Sending Command 1
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs(2));
        assert!(processor.is_command1_received());
        thread::sleep(Duration::from_secs(12)); // Wait for the timeout period to expire

        // Verify Command 1 is no longer received after timeout
        assert!(!processor.is_command1_received());

        processor.stop();
        handle.join().unwrap();
    }

    #[test]
    #[serial]
    fn test_processor_clean_exit_tx_drop() {
        let (tx, rx) = mpsc::channel();
        let processor= Processor::new();
        let handle = processor.start(rx);

        // Sending Command 1
        tx.send(1).unwrap();
        thread::sleep(Duration::from_secs(1));

        // Verify Command 1 was received
        assert!(processor.is_command1_received());

        // Dropping the sender to simulate the exit condition
        drop(tx);
        thread::sleep(Duration::from_secs(2));

        // Processor should exit the loop
        assert!(!processor.is_active());

        handle.join().unwrap();
    }

    #[test]
    #[serial]
    fn test_processor_is_active() {
        let (_, rx) = mpsc::channel();
        let processor= Processor::new();
        let handle = processor.start(rx);

        // Processor should be active
        assert!(processor.is_active());

        processor.stop();
        handle.join().unwrap();

        // Processor should no longer be active
        assert!(!processor.is_active());
    }
}
