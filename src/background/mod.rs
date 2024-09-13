use bevy::prelude::*;
//ㄴ
pub fn example() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, setup)
        .run();
}
const BOARD_SIZE_I: usize = 14;
const BOARD_SIZE_J: usize = 21;


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(
            -(BOARD_SIZE_I as f32 / 2.0),    // X축
            BOARD_SIZE_J as f32 / 2.0 - 0.5, // Y축
            0.0, // Z축 (2D에서는 깊이를 나타내며, 대개 0이나 999.9 같은 값)
        ),
        ..default()
    });
}
