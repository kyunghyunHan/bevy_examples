use bevy::prelude::*;
use tokio::io;
use tokio::net::UdpSocket;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let socket = UdpSocket::bind("127.0.0.1:0").await?;
//     let server_addr = "127.0.0.1:8080";

//     let message = b"Player position update";
//     socket.send_to(message, server_addr).await?;
//     println!("Sent message to the server: {:?}", message);

//     let mut buf = vec![0u8; 1024];
//     let (len, addr) = socket.recv_from(&mut buf).await?;
//     println!("Received {} bytes from {}: {:?}", len, addr, &buf[..len]);

//     Ok(())
// }

pub async fn example() -> io::Result<()> {
 

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();

    Ok(())
}

/// This system prints 'A' key state
 fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    let message = b"Player pressed A";


    let server_addr = "127.0.0.1:8080".to_string();

    tokio::spawn(async move {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        if let Err(e) = socket.send_to(message, &server_addr).await {
            eprintln!("Error sending message: {}", e);
        } else {
            println!("Sent message to the server: {:?}", message);
        }
    });

    info!("'A' just pressed");
}
