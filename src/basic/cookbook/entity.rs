use bevy::prelude::*;

// 컴포넌트 정의
#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health {
    current: u32,
    max: u32,
}

#[derive(Component)]
struct Xp(u32);

// fn setup(mut commands: Commands) {
//     // 플레이어 엔터티 생성
//     commands
//         .spawn()
//         .insert(Player) // Player 컴포넌트 추가
//         .insert(Health {
//             // Health 컴포넌트 추가
//             current: 100,
//             max: 100,
//         })
//         .insert(Xp(0)); // Xp 컴포넌트 추가

//     // 적 엔터티 생성
//     commands
//         .spawn()
//         .insert(Enemy) // Enemy 컴포넌트 추가
//         .insert(Health {
//             // Health 컴포넌트 추가
//             current: 50,
//             max: 50,
//         })
//         .insert(Xp(10)); // Xp 컴포넌트 추가
// }

pub fn example() {
    App::new()
        // .add_systems(Startup, setup)
        .add_plugins(DefaultPlugins) // 기본 플러그인 추가
        .run();
}
