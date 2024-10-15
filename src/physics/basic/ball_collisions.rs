use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component, Debug, Default)]
struct Pos(Vec2);
// 컴포넌트 구조체 정의
#[derive(Component, Debug, Default)]
struct PrevPos(Vec2);

pub const DELTA_TIME: f32 = 1. / 60.;
pub fn example() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(Msaa::Sample4) // 4x MSAA 설정
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_cameras)
        .add_systems(Startup, startup)
        .add_systems(Update, simulate)
        .add_systems(Update, sync_transforms)
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
    //위치기반 역학
    //velocity = (v_current - v_previous) / delta_time
    //속도를 입자를 초기화 하기 위해 이전위치 계산
    let color_material = materials.add(ColorMaterial::from(Color::WHITE));
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(sphere_mesh),
            material: color_material,
            transform: Transform::from_scale(Vec3::splat(1.0)), // 스케일을 명시적으로 설정
            ..default()
        })
        .insert(PrevPos(Vec2::ZERO - Vec2::new(2., 0.) * DELTA_TIME))
        .insert(Pos(Vec2::ZERO));
}
// fn simulate(mut query: Query<(&mut Pos, &mut PrevPos)>) {
//     for (mut pos, mut prev_pos) in query.iter_mut() {
//         let velocity = (pos.0 - prev_pos.0) / DELTA_TIME;
//         prev_pos.0 = pos.0;
//         pos.0 = pos.0 + velocity * DELTA_TIME;
//     }
// }
fn simulate(mut query: Query<(&mut Pos, &mut PrevPos)>) {
    for (mut pos, mut prev_pos) in query.iter_mut() {
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME;
        prev_pos.0 = pos.0;
        pos.0 += velocity * DELTA_TIME + Vec2::new(1.0, 0.0) * DELTA_TIME; // 움직임을 위해 일정한 방향으로 속도 추가
    }
}
/// Copies positions from the physics world to bevy Transforms
// fn sync_transforms(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
//     for (mut transform, pos) in query.iter_mut() {
//         transform.translation = pos.0.extend(0.);
//         transform.scale = Vec3::ONE; // 스케일을 (1.0, 1.0, 1.0)로 설정

//     }
// }
fn sync_transforms(mut query: Query<(&mut bevy::transform::components::Transform, &Pos)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation = pos.0.extend(0.);
        // 스케일을 변경하지 않고 유지
    }
}