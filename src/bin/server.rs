use bincode;
use enigo::{Enigo, Settings};
use ktroll::KeyEvent;
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
    }

    Ok(())
}
