use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use std::io::{Read, Write};
use std::net::{TcpStream, SocketAddr};

pub fn example() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, keyboard_input_system)
        .run();
}

fn send(mut stream: TcpStream, msg: &Vec<u8>) {
    // 메시지를 서버로 전송
    println!("서버로 메시지를 보냈습니다: {:?}", msg);
    stream
        .write_all(msg)
        .expect("failed to send message");

    // 서버로부터 응답 받기
    let mut buf = [0u8; 1024];
    let number_of_bytes = stream
        .read(&mut buf)
        .expect("failed to receive response");
    
    println!(
        "{} 바이트를 서버로부터 받았습니다: {:?}",
        number_of_bytes,
        &buf[..number_of_bytes]
    );
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowDown) {
        let task_pool = AsyncComputeTaskPool::get();

        let message = if keyboard_input.just_pressed(KeyCode::KeyA) {
            b"A".to_vec() // A 키가 눌리면 "A" 메시지 전송
        } else {
            b"Down".to_vec() // 아래 화살표가 눌리면 "Down" 메시지 전송
        };

        task_pool.spawn(async move {
            // 서버에 TCP 연결 생성 (수신 대기 중인 서버가 "127.0.0.1:8080"에 있다고 가정)
            let stream = TcpStream::connect("127.0.0.1:8080").expect("failed to connect to server");
            send(stream, &message);
        }).detach();
    }
}
