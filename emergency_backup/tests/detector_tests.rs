use emergency_backup::detector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_minus_sign_horizontal_line() {
        let points = vec![
            [10.0, 10.0],
            [100.0, 10.0],
        ];
        assert!(detector::check_minus_sign(&points));
    }
    #[test]
    fn test_check_minus_sign_short_line() {
        let points = vec![
            [10.0, 10.0],
            [30.0, 10.0],
        ];
        assert!(!detector::check_minus_sign(&points));
    }

    #[test]
    fn test_check_minus_sign_not_horizontal() {
        let points = vec![
            [10.0, 10.0],
            [50.0, 40.0],
        ];
        assert!(!detector::check_minus_sign(&points));
    }
}

#[cfg(test)]
mod rectangle_tests {
    use super::*;

    #[test]
    fn test_check_rectangle_complete_rectangle() {
        let points = vec![
            [10.0, 10.0],
            [790.0, 10.0],
            [790.0, 590.0],
            [10.0, 590.0],
        ];
        let window_size = [800.0, 600.0];
        assert!(detector::check_rectangle(&points, window_size));
    }

    #[test]
    fn test_check_rectangle_no_margin() {
        let points = vec![
            [0.0, 0.0],
            [800.0, 0.0],
            [800.0, 600.0],
            [0.0, 600.0],
        ];
        let window_size = [800.0, 600.0];
        assert!(detector::check_rectangle(&points, window_size));
    }
}

