use std::sync::{Arc, Mutex};

use win_beep;
use fltk::{app, button::Button,enums::Color, frame::Frame, prelude::*, window::Window};

pub enum Sound {
    Error,
    Info,
}

impl Sound {
    pub fn play(&self) {
        match self {
            Sound::Error => {
                win_beep::beep_with_hz_and_millis(200, 500);
            }
            Sound::Info => {
                win_beep::beep_with_hz_and_millis(400, 300);
            }
        }
    }
}

// Customizable pop-up that automatically closes after 10s if the user doesn't click the button
pub fn popup(title: &str, text: &str, sound: Sound) {
    // Emit sound
    sound.play();

    //Create confirmation pop-up
    let app = app::App::default();
    let mut wind = Window::new(100, 100, 300, 100, title);
    wind.set_color(Color::White);  
    let _frame = Frame::new(20, 20, 260, 30, text);
    let mut but = Button::new(125, 60, 50, 25, "OK");
    but.set_color(Color::White);

    let quit_flag = Arc::new(Mutex::new(false));
    let quit_flag_clone = Arc::clone(&quit_flag);

    but.set_callback({
        let appc = app.clone();
        let quit_flag = Arc::clone(&quit_flag);
        move |_| {
            let mut flag = quit_flag.lock().unwrap();
            *flag = true; //The button was clicked
            appc.quit();
        }
    });

    app::add_timeout3(10.0, {
        let appc = app.clone();
        move |_| {
            let flag = quit_flag_clone.lock().unwrap();
            //Close the pop-up if the user didn't click OK
            if !*flag {appc.quit(); }
        }
    });

    wind.end();
    wind.show();
    app.run().unwrap();
}