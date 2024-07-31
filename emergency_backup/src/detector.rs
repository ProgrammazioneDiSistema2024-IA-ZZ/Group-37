extern crate piston_window;
use std::sync::mpsc::{Sender};
use piston_window::*;

pub fn run(tx: Sender<i32>) {
    let mut window: PistonWindow = WindowSettings::new("Disegna un segno meno o un rettangolo", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut lines: Vec<[f64; 2]> = Vec::new();
    let mut drawing = false;
    let mut current_position = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            drawing = true;
            lines.clear();  // Start a new shape
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            drawing = false;
            lines.push(current_position);

            let size = window.size();
            let window_size = [size.width as f64, size.height as f64];

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

            // Send the result through the channel
            if tx.send(result).is_err() {
                eprintln!("Errore nell'invio del risultato");
            } else{
                println!("inviato");
            }

            lines.clear();
        }

        if let Some(pos) = event.mouse_cursor_args() {
            current_position = pos;
            if drawing {
                lines.push(current_position);
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for window_pos in lines.windows(2) {
                line_from_to([0.0, 0.0, 0.0, 1.0], 2.0, window_pos[0], window_pos[1], c.transform, g);
            }
        });
    }
}

fn check_minus_sign(lines: &[[f64; 2]]) -> bool {
    let last_point = match lines.last() {
        Some(&point) => point,
        None => return false,
    };

    let x1 = lines[0][0];
    let x2 = last_point[0];
    let y1 = lines[0][1];
    let y2 = last_point[1];
    //println!("x1: {}, x2: {}", x1, x2);
    //println!("y1: {}, y2: {}", y1, y2);

    // Check if the line is approximately horizontal
    let is_horizontal = (y1 - y2).abs() < 100.0;

    // Calculate the length of the line
    let length = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
    let is_long_enough = length > 50.0;  // Adjust this threshold as needed

    is_horizontal && is_long_enough
}

fn check_rectangle(points: &[[f64; 2]], window_size: [f64; 2]) -> bool {
    let mut on_left = false;
    let mut on_right = false;
    let mut on_top = false;
    let mut on_bottom = false;

    for point in points {
        if point[0] < 10.0 {
            on_left = true;
        }
        if point[0] > window_size[0] - 10.0 {
            on_right = true;
        }
        if point[1] < 10.0 {
            on_top = true;
        }
        if point[1] > window_size[1] - 10.0 {
            on_bottom = true;
        }
    }

    on_left && on_right && on_top && on_bottom
}

