use emergency_backup::detector;

#[test]
fn test_check_minus_sign() {
    let horizontal_line = [[10.0, 20.0], [110.0, 20.0]]; // Linea orizzontale di lunghezza sufficiente
    let short_line = [[10.0, 20.0], [40.0, 20.0]]; // Linea orizzontale troppo corta
    let vertical_line = [[20.0, 10.0], [20.0, 110.0]]; // Linea verticale

    assert!(detector::check_minus_sign(&horizontal_line));
    assert!(!detector::check_minus_sign(&short_line));
    assert!(!detector::check_minus_sign(&vertical_line));
}

#[test]
fn test_check_rectangle() {
    let window_size = [800.0, 600.0];

    // Un rettangolo corretto
    let rectangle = [
        [9.0, 9.0], // angolo in alto a sinistra
        [791.0, 9.0], // angolo in alto a destra
        [791.0, 591.0], // angolo in basso a destra
        [9.0, 591.0], // angolo in basso a sinistra
        [9.0, 9.0], // di nuovo angolo in alto a sinistra per chiudere il rettangolo
    ];

    // Un poligono che non è un rettangolo
    let non_rectangle = [
        [20.0, 20.0],
        [780.0, 20.0],
        [780.0, 580.0],
        [30.0, 580.0],
        [20.0, 20.0], // Chiusura del poligono
    ];

    assert!(detector::check_rectangle(&rectangle, window_size));
    assert!(!detector::check_rectangle(&non_rectangle, window_size));
}
