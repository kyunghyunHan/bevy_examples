
use bevy::input::keyboard::KeyCode;
use bevy::input::InputSystem;
use bevy::prelude::*;
use std::env;
/*
backgrund

*/

const PLATFORM_HEIGHTS: [f32; 11] = [
    -100.0, -50., 0.0, 50., 100., 150., 200., 250., 300., 350., 400.,
]; // 예시로 세 개의 계단 높이

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
    velocity: Vec3,     // 캐릭터의 속도
    is_jumping: bool,   // 점프 중인지 확인하는 플래그
    jump_impulse: f32,  // 점프할 때 적용할 힘
    target_height: f32, // 목표 높이 (계단처럼 상승)
    is_falling: bool,   // 하강 중인지 확인하는 플래그
}
const STAIRS_HEIGHT: f32 = 50.0;

fn jump_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Jumper, &mut Transform)>,
    time: Res<Time>,
) {
    for (mut jumper, mut transform) in query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) && !jumper.is_jumping && !jumper.is_falling {
            println!("Jump!");
            jumper.velocity.y = jumper.jump_impulse; // 점프 힘을 적용
            jumper.is_jumping = true;

            // 목표 높이를 50씩 올림
            jumper.target_height += 50.0;
        }

        // 캐릭터 이동 업데이트
        transform.translation += jumper.velocity * time.delta_seconds();
    }
}

fn apply_gravity_system(mut query: Query<(&mut Jumper, &mut Transform)>, time: Res<Time>) {
    let gravity = -9.8 * 50.0; // 중력 값

    for (mut jumper, mut transform) in query.iter_mut() {
        // 캐릭터가 플랫폼보다 높게 있으면 중력을 적용
        if jumper.is_jumping {
            jumper.velocity.y += gravity * time.delta_seconds();
            transform.translation += jumper.velocity * time.delta_seconds();

            // 플랫폼에 도달했는지 체크
            if jumper.velocity.y < 0.0 {
                if let Some(target_platform) = PLATFORM_HEIGHTS
                    .iter()
                    .rev()
                    .find(|&&height| transform.translation.y >= height)
                {
                    if transform.translation.y <= *target_platform + 5.0 {
                        // 플랫폼에 도달하면 위치 고정
                        transform.translation.y = *target_platform;
                        jumper.velocity.y = 0.0;
                        jumper.is_jumping = false;
                    }
                }
            }
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
            scale: 2., // 카메라 줌 레벨. 값이 클수록 더 작은 영역을 확대해서 보여줌
            ..Default::default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, -200.0, 11.0),
        ..Default::default()
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let background_texture_handle = asset_server.load("img/back2.png");

    let character_texture_handle = asset_server.load("img/mipi.png");
    // 배경 스프라이트 추가
    commands.spawn(SpriteBundle {
        texture: background_texture_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, -1.0), // 배경을 카메라의 중앙에 위치
        ..Default::default()
    }); 
    // 캐릭터 스프라이트 추가
    commands.spawn((
        SpriteBundle {
            texture: character_texture_handle.clone(),
            transform: Transform::from_xyz(0.0, -200.0, 10.0), // 캐릭터 위치 (시작 높이)
            ..Default::default()
        },
        Jumper {
            velocity: Vec3::ZERO,
            is_jumping: false,
            jump_impulse: 300.0,   // 점프 힘
            target_height: -100.0, // 처음 시작 위치
            is_falling: false,     // 처음엔 하강 상태 아님
        },
    ));

    // 카메라 추가
    // commands.spawn(Camera2dBundle::default());
}
