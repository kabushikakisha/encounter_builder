// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std;
use rand::{Rng};

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ui = AppWindow::new()?;
    let weak_app = ui.as_weak();
    slint::invoke_from_event_loop(move || weak_app.unwrap().window().set_maximized(true)).unwrap();
    ui.on_re_roll({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            let mut roll = rand::thread_rng().gen_range(0..100).to_string();
            if roll == "100" || roll == "0" {
                roll = "00".parse().unwrap();
            }
            ui.set_roll(roll.into());
        }
    });
    // println!("Your roll is {}", roll);
    // println!("Enter terrain: ");
    // let mut terrain = String::new();
    // std::io::stdin().read_line(&mut terrain).expect("Invalid Terrain");
    // println!("Your terrain is {}", terrain);

    ui.run()?;
    Ok(())
}
