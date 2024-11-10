use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component, Debug, Default)]
struct Position(Vec2);

#[derive(Component, Debug, Default)]
struct Velocity(Vec2);

#[derive(Component)]
struct BlackHole {
    mass: f32,
    event_horizon_radius: f32,
}

#[derive(Component)]
struct LightParticle;

#[derive(Component)]
struct AccretionDisk;

const GRAVITY: f32 = 9.81;
const DELTA_TIME: f32 = 1.0 / 60.0;
const NUM_PARTICLES: usize = 1000;
const BLACK_HOLE_MASS: f32 = 1000.0;
const EVENT_HORIZON_RADIUS: f32 = 30.0;
const ACCRETION_DISK_RADIUS: f32 = 100.0;

pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.1)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_particles.before(update_transforms),
            update_accretion_disk.before(update_transforms),
            update_transforms,
            spawn_new_particles,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 카메라 설정
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 1.0,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(0., 0., 1000.)),
        ..default()
    });

    // 블랙홀 중심부 생성
    let black_hole_mesh = meshes.add(Circle::new(EVENT_HORIZON_RADIUS));
    let black_hole_material = materials.add(ColorMaterial::from(Color::BLACK));
    
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(black_hole_mesh.clone()),
            material: black_hole_material.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.5),
            ..default()
        },
        BlackHole {
            mass: BLACK_HOLE_MASS,
            event_horizon_radius: EVENT_HORIZON_RADIUS,
        },
    ));

    // 블랙홀 글로우 효과
    let glow_material = materials.add(ColorMaterial::from(Color::rgba(0.5, 0.0, 0.5, 0.3)));
    commands.spawn(MaterialMesh2dBundle {
        mesh: Mesh2dHandle(meshes.add(Circle::new(EVENT_HORIZON_RADIUS * 1.5))),
        material: glow_material,
        transform: Transform::from_xyz(0.0, 0.0, 0.4),
        ..default()
    });

    // 강착원반 생성
    let accretion_disk_material = materials.add(ColorMaterial::from(Color::rgba(0.8, 0.4, 0.0, 0.5)));
    for i in 0..360 {
        let angle = (i as f32).to_radians();
        let radius = ACCRETION_DISK_RADIUS + (angle * 5.0).sin() * 20.0;
        let pos = Vec2::new(angle.cos() * radius, angle.sin() * radius);
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle::new(2.0))),
                material: accretion_disk_material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.3),
                ..default()
            },
            AccretionDisk,
            Position(pos),
            Velocity(Vec2::new(-pos.y, pos.x).normalize() * 5.0),
        ));
    }

    // 초기 빛 입자들 생성
    spawn_light_particles(&mut commands, &mut meshes, &mut materials, NUM_PARTICLES);
}

fn spawn_light_particles(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    count: usize,
) {
    let particle_material = materials.add(ColorMaterial::from(Color::rgba(1.0, 1.0, 1.0, 0.5)));
    let particle_mesh = meshes.add(Circle::new(1.0));

    for _ in 0..count {
        let angle = rand::random::<f32>() * std::f32::consts::PI * 2.0;
        let distance = 400.0 + rand::random::<f32>() * 200.0;
        let pos = Vec2::new(angle.cos() * distance, angle.sin() * distance);
        let dir = (-pos).normalize();
        
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(particle_mesh.clone()),
                material: particle_material.clone(),
                transform: Transform::from_xyz(pos.x, pos.y, 0.2),
                ..default()
            },
            LightParticle,
            Position(pos),
            Velocity(dir * (100.0 + rand::random::<f32>() * 50.0)),
        ));
    }
}

fn update_particles(
    mut commands: Commands,
    mut particles: Query<(Entity, &mut Position, &mut Velocity), With<LightParticle>>,
    black_hole: Query<(&BlackHole, &Transform)>,
) {
    let (black_hole, black_hole_transform) = black_hole.single();
    let black_hole_pos = Vec2::new(
        black_hole_transform.translation.x,
        black_hole_transform.translation.y,
    );

    for (entity, mut pos, mut vel) in particles.iter_mut() {
        let to_black_hole = black_hole_pos - pos.0;
        let distance = to_black_hole.length();
        
        if distance < black_hole.event_horizon_radius {
            commands.entity(entity).despawn();
            continue;
        }

        let gravity_strength = (black_hole.mass * GRAVITY) / (distance * distance);
        let gravity = to_black_hole.normalize() * gravity_strength;
        
        let relativistic_factor = (1.0 - (black_hole.event_horizon_radius / distance)).max(0.1);
        vel.0 += gravity * DELTA_TIME * relativistic_factor;
        pos.0 += vel.0 * DELTA_TIME;
    }
}

fn update_accretion_disk(
    mut disk_particles: Query<(&mut Position, &mut Velocity), With<AccretionDisk>>,
    black_hole: Query<&Transform, With<BlackHole>>,
) {
    let black_hole_transform = black_hole.single();
    let black_hole_pos = Vec2::new(
        black_hole_transform.translation.x,
        black_hole_transform.translation.y,
    );

    for (mut pos, mut vel) in disk_particles.iter_mut() {
        let to_black_hole = black_hole_pos - pos.0;
        let distance = to_black_hole.length();
        
        let orbit_speed = (BLACK_HOLE_MASS * GRAVITY / distance).sqrt();
        let tangent = Vec2::new(-to_black_hole.y, to_black_hole.x).normalize();
        vel.0 = tangent * orbit_speed;
        
        pos.0 += vel.0 * DELTA_TIME;
    }
}

fn update_transforms(
    mut query: Query<(&Position, &mut Transform)>,
) {
    for (pos, mut transform) in query.iter_mut() {
        transform.translation.x = pos.0.x;
        transform.translation.y = pos.0.y;
    }
}

fn spawn_new_particles(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    particles: Query<&LightParticle>,
) {
    let current_count = particles.iter().count();
    if current_count < NUM_PARTICLES / 2 {
        spawn_light_particles(&mut commands, &mut meshes, &mut materials, 10);
    }
}