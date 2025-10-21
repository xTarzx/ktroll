use bincode;
//use enigo::{Direction, Enigo, Key, Keyboard, Settings};
use ktroll::{KeyEvent, KeyEventType};
use rdev::{EventType, Key, simulate};
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:46969").await?;

    println!("waiting for connection");

    let (mut socket, addr) = listener.accept().await?;

    println!("connection from {}", addr);

    //let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        let len = socket.read_u32().await;

        if len.is_err() {
            break;
        };

        let len = len.unwrap() as usize;

        println!("len: {}", len);
        let mut payload = vec![0u8; len];
        socket.read_exact(&mut payload).await?;

        println!("payload received");

        let key_event: KeyEvent = bincode::decode_from_slice(&payload, bincode::config::standard())
            .unwrap()
            .0;

        println!("{:?}", key_event);

        let key = match key_event.key.as_str() {
            "KeyA" => Some(Key::KeyA),
            "KeyB" => Some(Key::KeyB),
            "KeyC" => Some(Key::KeyC),
            "KeyD" => Some(Key::KeyD),
            "KeyE" => Some(Key::KeyE),
            "KeyF" => Some(Key::KeyF),
            "KeyG" => Some(Key::KeyG),
            "KeyH" => Some(Key::KeyH),
            "KeyI" => Some(Key::KeyI),
            "KeyJ" => Some(Key::KeyJ),
            "KeyK" => Some(Key::KeyK),
            "KeyL" => Some(Key::KeyL),
            "KeyM" => Some(Key::KeyM),
            "KeyN" => Some(Key::KeyN),
            "KeyO" => Some(Key::KeyO),
            "KeyP" => Some(Key::KeyP),
            "KeyQ" => Some(Key::KeyQ),
            "KeyR" => Some(Key::KeyR),
            "KeyS" => Some(Key::KeyS),
            "KeyT" => Some(Key::KeyT),
            "KeyU" => Some(Key::KeyU),
            "KeyV" => Some(Key::KeyV),
            "KeyW" => Some(Key::KeyW),
            "KeyX" => Some(Key::KeyX),
            "KeyY" => Some(Key::KeyY),
            "KeyZ" => Some(Key::KeyZ),
            "Space" => Some(Key::Space),
            "ShiftLeft" => Some(Key::ShiftLeft),
            _ => None,
        };
        match key_event.event_type {
            KeyEventType::KeyPress => {
                if let Some(key) = key {
                    simulate(&EventType::KeyPress(key)).unwrap();
                }
            }
            KeyEventType::KeyRelease => {
                if let Some(key) = key {
                    simulate(&EventType::KeyRelease(key)).unwrap();
                }
            }
        };

        //let dir = match key_event.event_type {
        //    //KeyEventType::KeyPress => Direction::Press,
        //    //KeyEventType::KeyRelease => Direction::Release,
        //    KeyEventType::KeyPress => EventType::KeyP,
        //    KeyEventType::KeyRelease => Direction::Release,
        //};
        //
        //let key = match key_event.key.as_str() {
        //    "KeyA" => Some(Key::Unicode('a')),
        //    "KeyB" => Some(Key::Unicode('b')),
        //    "KeyC" => Some(Key::Unicode('c')),
        //    "KeyD" => Some(Key::Unicode('d')),
        //    "KeyE" => Some(Key::Unicode('e')),
        //    "KeyF" => Some(Key::Unicode('f')),
        //    "KeyG" => Some(Key::Unicode('g')),
        //    "KeyH" => Some(Key::Unicode('h')),
        //    "KeyI" => Some(Key::Unicode('i')),
        //    "KeyJ" => Some(Key::Unicode('j')),
        //    "KeyK" => Some(Key::Unicode('k')),
        //    "KeyL" => Some(Key::Unicode('l')),
        //    "KeyM" => Some(Key::Unicode('m')),
        //    "KeyN" => Some(Key::Unicode('n')),
        //    "KeyO" => Some(Key::Unicode('o')),
        //    "KeyP" => Some(Key::Unicode('p')),
        //    "KeyQ" => Some(Key::Unicode('q')),
        //    "KeyR" => Some(Key::Unicode('r')),
        //    "KeyS" => Some(Key::Unicode('s')),
        //    "KeyT" => Some(Key::Unicode('t')),
        //    "KeyU" => Some(Key::Unicode('u')),
        //    "KeyV" => Some(Key::Unicode('v')),
        //    "KeyW" => Some(Key::Unicode('w')),
        //    "KeyX" => Some(Key::Unicode('x')),
        //    "KeyY" => Some(Key::Unicode('y')),
        //    "KeyZ" => Some(Key::Unicode('z')),
        //    "Space" => Some(Key::Space),
        //    "ShiftLeft" => Some(Key::Shift),
        //    _ => None,
        //};
        //
        //println!("ENIGO_KEY: {:?}", key);
        //
        //if let Some(key) = key {
        //    enigo.key(key, dir).unwrap();
        //}
    }

    Ok(())
}
