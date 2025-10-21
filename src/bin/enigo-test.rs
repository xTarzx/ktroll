
use enigo::{Enigo, Settings, Keyboard, Key, Direction};

use std::{thread, time};

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    println!("starting in 5 seconds");
    thread::sleep(time::Duration::from_secs(5));

    enigo.key(Key::Shift, Direction::Press);
    enigo.key( Key::Unicode('a'), Direction::Press);

    enigo.key(Key::Shift, Direction::Release);
    enigo.key( Key::Unicode('a'), Direction::Release);
}
