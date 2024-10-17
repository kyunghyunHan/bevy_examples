use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{WindowRef,PrimaryWindow};
/// We will add this to each camera we want to compute cursor position for.
/// Add the component to the camera that renders to each window.
#[derive(Component, Default)]
struct WorldCursorCoords(Vec2);

fn setup_multiwindow(mut commands: Commands) {
    // TODO: set up multiple cameras for multiple windows.
    // See bevy's example code for how to do that.

    // Make sure we add our component to each camera
    commands.spawn((Camera2dBundle::default(), WorldCursorCoords::default()));
}

fn my_cursor_system_multiwindow(
    // query to get the primary window
    q_window_primary: Query<&Window, With<PrimaryWindow>>,
    // query to get other windows
    q_window: Query<&Window>,
    // query to get camera transform
    mut q_camera: Query<(&Camera, &GlobalTransform, &mut WorldCursorCoords)>,
) {
    for (camera, camera_transform, mut worldcursor) in &mut q_camera {
        // get the window the camera is rendering to
        let window = match camera.target {
            // the camera is rendering to the primary window
            RenderTarget::Window(WindowRef::Primary) => q_window_primary.single(),
            // the camera is rendering to some other window
            RenderTarget::Window(WindowRef::Entity(e_window)) => q_window.get(e_window).unwrap(),
            // the camera is rendering to something else (like a texture), not a window
            _ => {
                // skip this camera
                continue;
            }
        };

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            worldcursor.0 = world_position;
        }
    }
}
pub fn example() {
    App::new()
        // .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(Msaa::Sample4) // 4x MSAA 설정
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_multiwindow)
        .add_systems(Update, my_cursor_system_multiwindow)
        // .add_systems(Update, simulate)
        // .add_systems(Update, sync_transforms)
        .run();
}
