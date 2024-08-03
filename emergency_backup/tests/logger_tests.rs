#[cfg(test)]
mod logger_tests {
    use chrono::NaiveDateTime;
    use emergency_backup::logger::Logger;
    use std::time::Duration;
    use serial_test::serial;

    const LOG_PATH: &str = "C://Emergency Backup Log/cpu_usage.log";
    const INTERVAL: Duration = Duration::from_secs(2); 

    #[test]
    #[serial]
    fn test_start_and_stop_logger() {
        std::fs::remove_file(LOG_PATH).ok();

        let logger = Logger::new(INTERVAL);
        let handle = logger.start();

        std::thread::sleep(Duration::from_secs(5));
        logger.stop();
        handle.join().unwrap();

        assert!(!logger.is_logging_active());
    }

    #[test]
    #[serial]
    fn test_file_creation() {
        std::fs::remove_file(LOG_PATH).ok();

        let logger = Logger::new(INTERVAL);
        let handle = logger.start();

        std::thread::sleep(Duration::from_secs(2));
        logger.stop();
        handle.join().unwrap();

        assert!(logger.log_file_exists());
    }

    #[test]
    #[serial]
    fn test_correct_log_intervals() {
        std::fs::remove_file(LOG_PATH).ok();

        let logger = Logger::new(INTERVAL);
        let handle = logger.start();

        std::thread::sleep(Duration::from_secs(10));
        logger.stop();
        handle.join().unwrap();

        let entries = Logger::retrieve_log_entries();

        // Check if each entry is written at INTERVAL seconds of distance
        let mut previous_timestamp: Option<NaiveDateTime> = None;

        for entry in entries {
            if let Some(pos) = entry.find(']') {
                // Remove square brackets from the timestamp
                let timestamp_str = &entry[1..pos];
                let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%Y-%m-%d %H:%M:%S").expect("Invalid timestamp format");

                if let Some(prev) = previous_timestamp {
                    let duration = timestamp - prev;
                    assert_eq!(duration.num_seconds(), INTERVAL.as_secs() as i64);
                }

                previous_timestamp = Some(timestamp);
            }
        }
    }
}
