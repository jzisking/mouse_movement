extern crate core;

use core::time::Duration;
use chrono::{Local, NaiveTime};
use enigo::{Enigo, MouseButton, MouseControllable};

fn main() {
    let mut enigo = Enigo::new();

    let time = std::env::args().collect::<Vec<String>>().get(1)
        .map(|string| {
            NaiveTime::parse_from_str(string, "%H:%M").unwrap()
        });

    loop {
        enigo.mouse_move_to(100, 100);
        enigo.mouse_click(MouseButton::Left);

        std::thread::sleep(Duration::from_secs(5));

        enigo.mouse_move_to(200, 200);
        enigo.mouse_click(MouseButton::Left);

        if let Some(native_time) = time {
            let local = Local::now().naive_local().time();

            println!("Comparing {} to current time: {}", native_time, local);

            if native_time <= local {
                break
            }
        }
    }
}
