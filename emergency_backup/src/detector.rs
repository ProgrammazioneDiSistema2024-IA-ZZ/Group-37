extern crate piston_window;

use piston_window::*;

pub fn run() {
    let mut window: PistonWindow = WindowSettings::new("Disegna un segno più", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut lines: Vec<[f64; 4]> = Vec::new();
    let mut drawing = false;
    let mut start_point = [0.0, 0.0];
    let mut current_position = [0.0, 0.0];

    while let Some(event) = window.next() {
        if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
            drawing = true;
            start_point = current_position;
        }

        if let Some(Button::Mouse(MouseButton::Left)) = event.release_args() {
            drawing = false;
            lines.push([start_point[0], start_point[1], current_position[0], current_position[1]]);
        }

        if let Some(pos) = event.mouse_cursor_args() {
            current_position = pos;
        }

        window.draw_2d(&event, |c, g, _| {
            clear([1.0; 4], g);
            for line in &lines {
                line_from_to([0.0, 0.0, 0.0, 1.0], 2.0, [line[0], line[1]], [line[2], line[3]], c.transform, g);
            }
        });

        if lines.len() == 2 {
            let line1 = lines[0];
            let line2 = lines[1];

            let is_line1_horizontal = (line1[1] - line1[3]).abs() < 100.0;
            let is_line1_vertical = (line1[0] - line1[2]).abs() < 100.0;
            //println!("Linea 1 è orizzontale: {} è verticale: {}", is_line1_horizontal,is_line1_vertical);
            let is_line2_horizontal = (line2[1] - line2[3]).abs() < 100.0;
            let is_line2_vertical = (line2[0] - line2[2]).abs() < 100.0;
            //println!("Linea 2 è orizzontale: {} è verticale: {}", is_line2_horizontal,is_line2_vertical);

            if (is_line1_horizontal && is_line2_vertical) || (is_line1_vertical && is_line2_horizontal) {
                let (horizontal, vertical) = if is_line1_horizontal && is_line2_vertical {
                    (line1, line2)
                } else {
                    (line2, line1)
                };

                let hx_mid = (horizontal[0] + horizontal[2]) / 2.0;
                let vy_mid = (vertical[1] + vertical[3]) / 2.0;

                if (hx_mid - vertical[0]).abs() < 100.0 && (vy_mid - horizontal[1]).abs() < 100.0 {
                    println!("Segno più identificato");
                }else{
                    //println!("HAI sbagliato")
                }
            }else{
                //println!("ciao");
            }
            lines.clear();
        }
    }
}
