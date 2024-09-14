use bevy::prelude::*;
use std::env;

//ㄴ
pub fn example() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        // .add_systems(OnEnter(GameState::Playing), setup)
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
//         transform: Transform::from_xyz(
//             -(BOARD_SIZE_I as f32 / 2.0),    // X축
//             BOARD_SIZE_J as f32 / 2.0 - 0.5, // Y축
//             0.0, // Z축 (2D에서는 깊이를 나타내며, 대개 0이나 999.9 같은 값)
//         ),
//         ..default()
//     });
// }
fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0), // 기본 카메라 위치
    //     ..default()
    // });
}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut textures: ResMut<Assets<Image>>,
) {
    let texture_handle = asset_server.load("img/test.png");
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: texture_handle.clone(),
        // transform: Transform::from_xyz(200.0, 0.0, 0.0),
        ..default()
    });
    let current_dir = env::current_dir().unwrap();
    println!("Current directory: {:?}", current_dir);

    // 이미지 경로 출력
    let texture_path = current_dir.join("img/test.png");
    println!("Loading texture from: {:?}", texture_path);

    // 이미지가 로드되었는지 확인
    if asset_server.get_load_state(&texture_handle).unwrap() == (bevy::asset::LoadState::Loaded) {
        println!("Image successfully loaded!");
    } else {
        println!("Image is still loading or failed to load.");
    }
}
// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn(Camera2dBundle::default());

//     let sprite_handle = asset_server.load("img/test.png");

//     commands.spawn(SpriteBundle {
//         texture: sprite_handle.clone(),
//         ..default()
//     });

// }
