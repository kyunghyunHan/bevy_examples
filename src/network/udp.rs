use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::net;
use std::net::SocketAddr;

pub fn example() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}

fn send(socket: &net::UdpSocket, receiver: &str, msg: &Vec<u8>) -> usize {
    println!("서버로 메시지를 보냈습니다: {:?}", msg);
    let result = socket
        .send_to(msg, receiver)
        .expect("failed to send message");
    let mut buf = [0u8; 1024];

    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("no data received");
    println!(
        "{} 바이트를 {}로부터 받았습니다: {:?}",
        number_of_bytes,
        src_addr,
        &buf[..number_of_bytes]
    );
    result
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        println!("1");
        let task_pool = AsyncComputeTaskPool::get();

        let message = if keyboard_input.just_pressed(KeyCode::KeyA) {
            b"A".to_vec() // A 키가 눌리면 "A" 메시지 전송
        } else {
            b"Down".to_vec() // 아래 화살표가 눌리면 "Down" 메시지 전송
        };

        task_pool
            .spawn(async move {
                let socket = net::UdpSocket::bind("127.0.0.1:0").expect("failed to bind socket");
                send(&socket, "127.0.0.1:8080", &message);
            })
            .detach();
    }
}
