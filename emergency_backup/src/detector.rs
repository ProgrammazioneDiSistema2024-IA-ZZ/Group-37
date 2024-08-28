extern crate device_query;

use device_query::{DeviceQuery, DeviceState};
use std::{thread, time};
use std::sync::mpsc::Sender;

pub fn run(tx: Sender<i32>) {
    let device_state = DeviceState::new();
    let mut lines: Vec<[f64; 2]> = Vec::new();
    let mut drawing = false;
    let mut current_position = [0.0, 0.0];

    loop {
        let mouse_state = device_state.get_mouse();
        let left_button_pressed = mouse_state.button_pressed[1]; // Il pulsante sinistro del mouse è rappresentato da 1

        if left_button_pressed && !drawing {
            drawing = true;
            lines.clear();  //ripulisce la vecchia forma
        }

        if !left_button_pressed && drawing {
            drawing = false;
            lines.push(current_position);

            let window_size = [800.0, 600.0];

            let result = if check_rectangle(&lines, window_size) {
                println!("Rettangolo");
                1
            } else if check_minus_sign(&lines) {
                println!("Segno meno");
                2
            } else {
                println!("Forma non identificata");
                0
            };

            // invia il risultato sul canale
            if tx.send(result).is_err() {
                eprintln!("Errore nell'invio del risultato");
            } else {
                println!("Inviato");
            }

            lines.clear();
        }

        let mouse_coords = [mouse_state.coords.0 as f64, mouse_state.coords.1 as f64];
        if mouse_coords != current_position {
            current_position = mouse_coords;
            if drawing {
                lines.push(current_position);
            }
        }
        thread::sleep(time::Duration::from_millis(10));

        if tx.send(0).is_err() {return};
    }
}

pub fn check_minus_sign(lines: &[[f64; 2]]) -> bool {
    if lines.is_empty() {
        return false;
    }

    let first_point = lines[0];
    let last_point = lines[lines.len() - 1];

    let x1 = first_point[0];
    let x2 = last_point[0];
    let y1 = first_point[1];
    let y2 = last_point[1];

    // Controlla se la linea è approssimativamente orizzontale
    let is_horizontal = (y1 - y2).abs() < 100.0;

    // Calcola la lunghezza della linea
    let length = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
    let is_long_enough = length > 50.0;

    is_horizontal && is_long_enough
}

pub fn check_rectangle(points: &[[f64; 2]], window_size: [f64; 2]) -> bool {
    let mut on_left = false;
    let mut on_right = false;
    let mut on_top = false;
    let mut on_bottom = false;

    let margin = 50.0; // Maggiore margine per rendere più facile disegnare un rettangolo

    for point in points {
        if point[0] < margin {
            on_left = true;
        }
        if point[0] > window_size[0] - margin {
            on_right = true;
        }
        if point[1] < margin {
            on_top = true;
        }
        if point[1] > window_size[1] - margin {
            on_bottom = true;
        }
    }

    on_left && on_right && on_top && on_bottom
}
