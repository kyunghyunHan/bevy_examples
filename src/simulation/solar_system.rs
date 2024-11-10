use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component, Debug, Default)]
struct Position(Vec2);

#[derive(Component, Debug, Default)]
struct PrevPosition(Vec2);

#[derive(Component)]
struct Planet {
    orbit_radius: f32,
    orbit_speed: f32,
}

pub const DELTA_TIME: f32 = 1. / 60.;
const SCALE: f32 = 50.0;

pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1))) // 어두운 우주 배경
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_cameras)
        .add_systems(Startup, setup_solar_system)
        .add_systems(Update, update_planet_positions)
        .add_systems(Update, sync_transforms)
        .run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 1000.)),
        ..Default::default()
    });
}

fn setup_solar_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 태양 생성
    let sun_mesh = meshes.add(Circle::new(30.0));
    let sun_material = materials.add(ColorMaterial::from(Color::rgb(1.0, 0.7, 0.0)));
    
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(sun_mesh),
        material: sun_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // 행성 데이터: (궤도 반지름, 공전 속도, 크기, 색상)
    let planet_data = vec![
        (0.4 * SCALE, 4.0, 8.0, Color::rgb(0.7, 0.7, 0.7)),    // 수성
        (0.7 * SCALE, 3.0, 12.0, Color::rgb(0.9, 0.7, 0.5)),   // 금성
        (1.0 * SCALE, 2.5, 13.0, Color::rgb(0.2, 0.5, 0.8)),   // 지구
        (1.5 * SCALE, 2.0, 10.0, Color::rgb(0.8, 0.3, 0.2)),   // 화성
        (2.2 * SCALE, 1.3, 25.0, Color::rgb(0.8, 0.7, 0.5)),   // 목성
        (2.8 * SCALE, 1.0, 20.0, Color::rgb(0.9, 0.8, 0.6)),   // 토성
        (3.4 * SCALE, 0.7, 15.0, Color::rgb(0.5, 0.7, 0.8)),   // 천왕성
        (3.9 * SCALE, 0.5, 15.0, Color::rgb(0.2, 0.3, 0.9)),   // 해왕성
    ];

    // 행성들 생성
    for (orbit_radius, orbit_speed, size, color) in planet_data {
        // 궤도 선 생성
        let orbit_mesh = meshes.add(Circle::new(orbit_radius));
        let orbit_material = materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.1)));
        
        commands.spawn(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(orbit_mesh),
            material: orbit_material,
            transform: Transform::from_xyz(0.0, 0.0, 0.1),
            ..default()
        });

        // 초기 위치 계산 (random starting position)
        let initial_angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let initial_pos = Vec2::new(
            orbit_radius * initial_angle.cos(),
            orbit_radius * initial_angle.sin()
        );

        // 행성 생성
        let planet_mesh = meshes.add(Circle::new(size));
        let planet_material = materials.add(ColorMaterial::from(color));
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(planet_mesh),
                material: planet_material,
                transform: Transform::from_xyz(initial_pos.x, initial_pos.y, 0.2),
                ..default()
            },
            Planet {
                orbit_radius,
                orbit_speed,
            },
            Position(initial_pos),
            PrevPosition(initial_pos - Vec2::new(
                -initial_pos.y, 
                initial_pos.x
            ).normalize() * (orbit_speed * DELTA_TIME)),
        ));
    }
}

fn update_planet_positions(
    mut query: Query<(&Planet, &mut Position, &mut PrevPosition)>,
) {
    for (planet, mut pos, mut prev_pos) in query.iter_mut() {
        // 현재 속도 계산
        let velocity = (pos.0 - prev_pos.0) / DELTA_TIME;
        
        // 이전 위치 저장
        prev_pos.0 = pos.0;
        
        // 새로운 위치 계산
        let normalized_pos = pos.0.normalize();
        let tangent = Vec2::new(-normalized_pos.y, normalized_pos.x);
        
        // 원형 궤도를 유지하기 위한 힘 적용
        let centripetal = -normalized_pos * planet.orbit_speed * planet.orbit_speed;
        let orbital = tangent * planet.orbit_speed;
        
        pos.0 += (velocity + centripetal + orbital) * DELTA_TIME;
        
        // 궤도 반경 보정
        pos.0 = pos.0.normalize() * planet.orbit_radius;
    }
}

fn sync_transforms(mut query: Query<(&mut Transform, &Position)>) {
    for (mut transform, pos) in query.iter_mut() {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
    }
}