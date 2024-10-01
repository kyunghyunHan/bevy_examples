use bevy::prelude::*;
use std::env;
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
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
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
            scale: 1., // 카메라 줌 레벨. 값이 클수록 더 작은 영역을 확대해서 보여줌
            ..Default::default()
        }.into(),
        transform: Transform::from_xyz(0.0, -200.0, 1.0), 
        ..Default::default()
    });
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
) {
    let background_texture_handle = asset_server.load("img/back.png");
    let character_texture_handle = asset_server.load("img/mipi.png"); // 캐릭터 이미지 로드

    // 배경 스프라이트 추가
    commands.spawn(SpriteBundle {
        texture: background_texture_handle.clone(),
        transform: Transform::from_xyz(0.0, 0.0, 0.0), // 배경을 카메라의 중앙에 위치
        ..Default::default()
    });

    // 캐릭터 스프라이트 추가
    commands.spawn(SpriteBundle {
        texture: character_texture_handle.clone(),
        transform: Transform::from_xyz(-100.0, 0.0, 1.0), // 캐릭터의 위치 (배경보다 앞쪽)
        ..Default::default()
    });

    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    let texture_path = current_dir.join("img/back.png");
    println!("Loading texture from: {:?}", texture_path);

    if asset_server.get_load_state(&background_texture_handle).unwrap() == bevy::asset::LoadState::Loaded {
        println!("Background image successfully loaded!");
    } else {
        println!("Background image is still loading or failed to load.");
    }
}
