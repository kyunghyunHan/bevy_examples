use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use tokio::io;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

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
#[tokio::main]
async fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}
async fn send_udp_message()->io::Result<()> {
    let socket =  UdpSocket::bind("127.0.0.1:12345").await?;
       


    let server_addr = "127.0.0.1:8080";
    let message = b"1";
    println!("{}",1);
    println!("Sending message to the server...");

    if let Err(e) = socket.send_to(message, server_addr).await {
        eprintln!("Failed to send message: {}", e);
    }

    println!("{}",1);

    println!("Sent message to the server: {:?}", message);

    let mut buf = vec![0u8; 1024];
    match socket.recv_from(&mut buf).await {
        Ok((len, addr)) => {
            println!("Received {} bytes from {}: {:?}", len, addr, &buf[..len]);
        }
        Err(e) => {
            eprintln!("Failed to receive message: {}", e);
        }
    }
    Ok(())

}
fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();

        rt.spawn(async {
            send_udp_message().await.unwrap();
        });
    }
}
