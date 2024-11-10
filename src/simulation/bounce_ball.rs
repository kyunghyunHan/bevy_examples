use bevy::prelude::*;
use std::f32::consts::PI;

const INITIAL_VELOCITY: f32 = 0.0;  
const INITIAL_ANGLE: f32 = 90.0;    
const GRAVITY: f32 = 980.0;
const BOUNCE_COEFFICIENT: f32 = 0.75;
const GROUND_HEIGHT: f32 = -200.0;
const INITIAL_HEIGHT: f32 = 300.0;  

pub fn bouncing_ball() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Game>()
        .init_state::<GameState>()
        .add_systems(Startup, setup_cameras)
        .add_systems(OnEnter(GameState::Playing), setup)
        .add_systems(Update, ball_physics_system)
        .run();
}

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Playing,
    GameOver,
}

#[derive(Component)]
struct Ball {
    velocity: Vec2,
    bounce_count: u32,
    max_bounces: u32,
    is_stopped: bool,
}

#[derive(Resource, Default)]
struct Game {
    camera_should_focus: Vec3,
    camera_is_focus: Vec3,
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            ..Default::default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        camera_2d: Camera2d {
            // clear_color: ClearColor::default(),
        },
        ..Default::default()
    });
}

fn setup(mut commands: Commands) {
    let initial_velocity = Vec2::new(
        INITIAL_VELOCITY * f32::cos(INITIAL_ANGLE * PI / 180.0),
        INITIAL_VELOCITY * f32::sin(INITIAL_ANGLE * PI / 180.0),
    );

    // 공 스폰
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, INITIAL_HEIGHT, 1.0),
            ..Default::default()
        },
        Ball {
            velocity: initial_velocity,
            bounce_count: 0,
            max_bounces: 3,
            is_stopped: false,
        },
    ));

    // 바닥선
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(800.0, 2.0)),
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, GROUND_HEIGHT, 0.5),
        ..Default::default()
    });
}

fn ball_physics_system(
    time: Res<Time>,
    mut query: Query<(&mut Ball, &mut Transform)>,
) {
    for (mut ball, mut transform) in query.iter_mut() {
        if ball.is_stopped {
            continue; // 공이 멈춘 상태면 물리 계산 스킵
        }

        // 중력 적용
        ball.velocity.y -= GRAVITY * time.delta_seconds();

        // 위치 업데이트
        transform.translation.x += ball.velocity.x * time.delta_seconds();
        transform.translation.y += ball.velocity.y * time.delta_seconds();

        // 바닥 충돌 체크
        if transform.translation.y <= GROUND_HEIGHT {
            // 바운스 처리
            transform.translation.y = GROUND_HEIGHT;
            ball.velocity.y = ball.velocity.y.abs() * BOUNCE_COEFFICIENT;
            ball.bounce_count += 1;

            // 지정된 횟수만큼 튕긴 후 멈춤
            if ball.bounce_count >= ball.max_bounces {
                ball.is_stopped = true;
                ball.velocity = Vec2::ZERO;
                transform.translation.y = GROUND_HEIGHT;
            }
        }

        // 디버그 출력
        println!(
            "Position: ({:.2}, {:.2}), Velocity: ({:.2}, {:.2}), Bounces: {}",
            transform.translation.x,
            transform.translation.y,
            ball.velocity.x,
            ball.velocity.y,
            ball.bounce_count
        );
    }
}