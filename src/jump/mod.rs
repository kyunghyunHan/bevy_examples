use bevy::prelude::*;
use std::env;
use bevy::input::keyboard::KeyCode; 
use bevy::input::InputSystem;
/*
backgrund

*/
pub fn example() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_cameras)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, jump_system)
        .add_systems(Update, apply_gravity_system)

        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
struct Jumper {
    velocity: Vec3,    // 캐릭터의 속도
    is_jumping: bool,  // 점프 중인지 확인하는 플래그
    jump_impulse: f32, // 점프할 때 적용할 힘
}

fn keyboard_input_system(keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("'A' currently pressed");
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' just pressed");
    }
    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' just released");
    }
}
fn jump_system(
    keyboard_input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&mut Jumper, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut jumper, mut transform) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && !jumper.is_jumping {
            println!("Jump!");
            jumper.velocity.y = jumper.jump_impulse; // 점프 힘을 적용
            jumper.is_jumping = true;
        }

        // 캐릭터 이동 업데이트
        transform.translation += jumper.velocity * time.delta_seconds();
    }
}
fn apply_gravity_system(mut query: Query<(&mut Jumper, &mut Transform)>, time: Res<Time>) {
    let gravity = -9.8 * 50.0; // 중력 값

    for (mut jumper, mut transform) in query.iter_mut() {
        if transform.translation.y > -100.0 {
            // 캐릭터가 바닥에 도달하지 않았으면 중력을 적용
            jumper.velocity.y += gravity * time.delta_seconds();
        } else {
            // 캐릭터가 바닥에 도달했으면 속도를 0으로 하고 점프 상태를 해제
            jumper.velocity.y = 0.0;
            jumper.is_jumping = false;
            transform.translation.y = -100.0; // 바닥에서 멈추게 함
        }
    }
}
struct Cell {
    height: f32,
}

#[derive(Default)]
struct Player {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    move_cooldown: Timer,
}

#[derive(Default)]
struct Bonus {
    entity: Option<Entity>,
    i: usize,
    j: usize,
    handle: Handle<Scene>,
}

#[derive(Resource, Default)]
struct Game {
    board: Vec<Vec<Cell>>,
    player: Player,
    bonus: Bonus,
    score: i32,
    cake_eaten: u32,
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

// fn setup_cameras(mut commands: Commands) {
//     commands.spawn(Camera2dBundle {
//         transform: Transform::from_xyz(0.0, 0.0, 0.0), // 기본 카메라 위치
//         ..Default::default()
//     });
// }
fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 0.5, // 카메라 줌 레벨. 값이 클수록 더 작은 영역을 확대해서 보여줌
            ..Default::default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, -200.0, 1.0),
        ..Default::default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let character_texture_handle = asset_server.load("img/images.png");

    // 캐릭터 스프라이트 추가
    commands.spawn((
        SpriteBundle {
            texture: character_texture_handle.clone(),
            transform: Transform::from_xyz(0.0, -100.0, 1.0), // 캐릭터 위치 (시작 높이)
            ..Default::default()
        },
        Jumper {
            velocity: Vec3::ZERO,
            is_jumping: false,
            jump_impulse: 300.0, // 점프 힘
        },
    ));

    // 카메라 추가
    commands.spawn(Camera2dBundle::default());
}