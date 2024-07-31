extern crate piston_window;

use piston_window::*;
use std::f64::consts::PI;

fn detector(points: &Vec<[f64; 2]>) -> bool {
    // Semplice algoritmo di riconoscimento di un segno "più"
    if points.len() < 10 {
        return false;
    }

    let mut horizontal_count = 0;
    let mut vertical_count = 0;

    for i in 1..points.len() {
        let dx = (points[i][0] - points[i - 1][0]).abs();
        let dy = (points[i][1] - points[i - 1][1]).abs();

        if dx > dy && dx > 5.0 {
            horizontal_count += 1;
        } else if dy > dx && dy > 5.0 {
            vertical_count += 1;
        }
    }

    horizontal_count > 5 && vertical_count > 5
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Plus Sign Detector", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut drawing = false;
    let mut points: Vec<[f64; 2]> = Vec::new();

    while let Some(event) = window.next() {
        if let Some(Button::Mouse(button)) = event.press_args() {
            if button == MouseButton::Left {
                drawing = true;
                points.clear();
            }
        }

        if let Some(Button::Mouse(button)) = event.release_args() {
            if button == MouseButton::Left {
                drawing = false;
                if detector(&points) {
                    println!("Plus sign detected!");
                } else {
                    println!("Not a plus sign.");
                }
                points.clear();
            }
        }

        if let Some([x, y]) = event.mouse_cursor_args() {
            if drawing {
                points.push([x as f64, y as f64]);
            }
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);

            for point in &points {
                ellipse([0.0, 0.0, 0.0, 1.0], [point[0], point[1], 2.0, 2.0], c.transform, g);
            }
        });
    }
}
