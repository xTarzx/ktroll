use bincode;
use enigo::{Enigo, Settings};
use ktroll::KeyEvent;
use tokio::{io::AsyncReadExt, net::TcpListener};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:46969").await?;

    let (mut socket, addr) = listener.accept().await?;

    println!("connection from {}", addr);

    //let mut enigo = Enigo::new(&Settings::default()).unwrap();

    loop {
        let mut len_buf = [0u8; 4];
        if socket.read_exact(&mut len_buf).await.is_err() {
            break;
        }

        let len = u32::from_be_bytes(len_buf) as usize;

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
