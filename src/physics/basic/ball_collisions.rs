use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
  

  
#[derive(Component, Debug, Default)]
struct Pos(Vec2);

pub fn example() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4) // 4x MSAA 설정
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_cameras)
        .add_systems(Update, startup)
        .run();
}
fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 100.)),

        ..Default::default()
    });
}
fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sphere_mesh = meshes.add(Mesh::from(Sphere {
        radius: 50.0,
        ..Default::default()
    }));

    // Create a color material
    let color_material = materials.add(ColorMaterial::from(Color::hsl(
        360. * 0.0 / 360.0,
        0.95,
        0.7,
    )));

    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(sphere_mesh),
        material: color_material,
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..default()
    });
}
