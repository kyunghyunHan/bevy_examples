use bevy::{
    prelude::*,
    render::{
        mesh::{Indices, Mesh, PrimitiveTopology},
        render_asset::RenderAssetUsages,
    },
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component, Debug, Clone)]  // Clone trait 추가
struct Ball {
    position: Vec2,
    velocity: Vec2,
    mass: f32,
}

#[derive(Component)]
struct String {
    start: Vec2,
    end: Vec2,
}

const GRAVITY: f32 = 980.0;
const DAMPING: f32 = 0.99;
const BALL_RADIUS: f32 = 20.0;
const STRING_LENGTH: f32 = 200.0;
const COLLISION_ELASTICITY: f32 = 0.95;

pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (update_physics, update_strings, update_transforms))
        .run();
}

fn create_circle(radius: f32) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let vertices_count = 32;
    let mut positions = Vec::new();
    let mut indices = Vec::new();
    let mut uvs = Vec::new();

    // Center vertex
    positions.push([0.0, 0.0, 0.0]);
    uvs.push([0.5, 0.5]);

    // Circle vertices
    for i in 0..vertices_count {
        let angle = i as f32 * 2.0 * std::f32::consts::PI / vertices_count as f32;
        positions.push([angle.cos() * radius, angle.sin() * radius, 0.0]);
        uvs.push([angle.cos() * 0.5 + 0.5, angle.sin() * 0.5 + 0.5]);
    }

    // Indices
    for i in 0..vertices_count {
        indices.push(0);
        indices.push(i as u32 + 1);
        indices.push(((i + 1) % vertices_count) as u32 + 1);
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn create_line(width: f32, height: f32) -> Mesh {
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );
    let vertices = vec![
        [-width / 2.0, -height / 2.0, 0.0], // bottom left
        [width / 2.0, -height / 2.0, 0.0],  // bottom right
        [width / 2.0, height / 2.0, 0.0],   // top right
        [-width / 2.0, height / 2.0, 0.0],  // top left
    ];
    let indices = vec![0, 1, 2, 0, 2, 3];
    let uvs = vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]];

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // Balls
    let ball_mesh = meshes.add(create_circle(BALL_RADIUS));
    let ball_material = materials.add(ColorMaterial::from(Color::BLACK));
    let string_material = materials.add(ColorMaterial::from(Color::BLACK));

    // Spawn three balls
    let positions = [-BALL_RADIUS * 2.0, 0.0, BALL_RADIUS * 2.0];

    // 왼쪽 공은 초기 속도를 가지고 시작
    let mut spawned = false;

    for x in positions.iter() {
        // Ball
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(ball_mesh.clone()),
                material: ball_material.clone(),
                transform: Transform::from_xyz(*x, 0.0, 0.0),
                ..default()
            },
            Ball {
                position: Vec2::new(*x, 0.0),
                velocity: if !spawned {
                    Vec2::new(200.0, 0.0)
                } else {
                    Vec2::ZERO
                },
                mass: 1.0,
            },
        ));
        spawned = true;

        // String
        let string_start = Vec2::new(*x, STRING_LENGTH);
        let string_end = Vec2::new(*x, 0.0);

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(create_line(2.0, STRING_LENGTH))),
                material: string_material.clone(),
                transform: Transform::from_xyz(
                    string_start.x,
                    string_start.y - STRING_LENGTH / 2.0,
                    -0.1,
                ),
                ..default()
            },
            String {
                start: string_start,
                end: string_end,
            },
        ));
    }
}

fn update_physics(
    time: Res<Time>,
    mut balls: Query<&mut Ball>,
) {
    let dt = time.delta_seconds();
    
    // 현재 상태를 복사
    let mut ball_data: Vec<Ball> = balls.iter().cloned().collect();
    
    // 모든 공의 위치와 속도 업데이트
    for ball in ball_data.iter_mut() {
        ball.velocity.y -= GRAVITY * dt;
        ball.position += ball.velocity * dt;
        
        // String constraint
        let to_anchor = Vec2::new(ball.position.x, STRING_LENGTH) - ball.position;
        if to_anchor.length() > STRING_LENGTH {
            let normal = to_anchor.normalize();
            ball.position = Vec2::new(ball.position.x, STRING_LENGTH) - normal * STRING_LENGTH;
            let dot = ball.velocity.dot(normal);
            if dot < 0.0 {
                ball.velocity -= normal * dot * 1.5; // 줄의 탄성 조정
            }
        }
        
        // Apply smaller damping for horizontal movement
        ball.velocity.x *= 0.999; // x축 방향의 감쇠를 줄임
        ball.velocity.y *= DAMPING;
    }

    // Handle collisions between balls with improved center ball interaction
    for i in 0..ball_data.len() {
        for j in i+1..ball_data.len() {
            let dist = ball_data[i].position.distance(ball_data[j].position);
            if dist < BALL_RADIUS * 2.1 { // 충돌 감지 거리를 약간 증가
                let normal = (ball_data[j].position - ball_data[i].position).normalize();
                
                // Calculate relative velocity in the normal direction only
                let relative_velocity = (ball_data[j].velocity - ball_data[i].velocity).dot(normal);
                
                // Only apply collision if balls are moving towards each other
                if relative_velocity < 0.0 {
                    // Momentum exchange
                    let impulse = normal * relative_velocity * -(1.0 + COLLISION_ELASTICITY);
                    ball_data[i].velocity -= impulse * 0.5;
                    ball_data[j].velocity += impulse * 0.5;
                    
                    // Separate balls
                    let overlap = BALL_RADIUS * 2.0 - dist;
                    let separation = normal * (overlap / 2.0);
                    ball_data[i].position -= separation * 1.01; // 약간 더 강하게 분리
                    ball_data[j].position += separation * 1.01;
                }
            }
        }
    }

    // 업데이트된 상태를 다시 balls에 적용
    for (mut ball, updated_ball) in balls.iter_mut().zip(ball_data.iter()) {
        ball.position = updated_ball.position;
        ball.velocity = updated_ball.velocity;
    }
}

fn update_strings(balls: Query<&Ball>, mut strings: Query<(&mut String, &mut Transform)>) {
    for ((mut string, mut transform), ball) in strings.iter_mut().zip(balls.iter()) {
        string.end = ball.position;

        // Update string visual
        let string_vector = string.end - string.start;
        let string_length = string_vector.length();
        let string_center = (string.start + string.end) / 2.0;
        let angle = string_vector.y.atan2(string_vector.x);

        transform.translation.x = string_center.x;
        transform.translation.y = string_center.y;
        transform.rotation = Quat::from_rotation_z(angle - std::f32::consts::FRAC_PI_2);
        transform.scale.y = string_length / STRING_LENGTH;
    }
}

fn update_transforms(mut transforms: Query<(&Ball, &mut Transform)>) {
    for (ball, mut transform) in transforms.iter_mut() {
        transform.translation.x = ball.position.x;
        transform.translation.y = ball.position.y;
    }
}
