
use enigo::{Enigo, Settings, Keyboard, Key, Direction};

use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    println!("starting in 5 seconds");
    thread::sleep(time::Duration::from_seconds(5));

    enigo.key(Direction::KeyPress, Key::Shift);
    enigo.key(Direction::KeyPress, Key::Unicode('a'));
    enigo.key(Direction::KeyRelease, Key::Unicode('a'));
    enigo.key(Direction::KeyRelease, Key::Shift);

}
