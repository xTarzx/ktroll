
use enigo::{Enigo, Settings, Keyboard, Key, Direction};

use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    println!("starting in 5 seconds");
    thread::sleep(time::Duration::from_seconds(5));

    enigo.key(Key::Shift, Direction::KeyPress);
    enigo.key( Key::Unicode('a'), Direction::KeyPress);

    enigo.key(Key::Shift, Direction::KeyRelease);
    enigo.key( Key::Unicode('a'), Direction::KeyRelease);
}
