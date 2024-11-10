use bevy::prelude::*;
use rand::Rng;
use std::f32::consts::PI;

const SPACE_SIZE: f32 = 500.0;
const INITIAL_PAIRS: u32 = 4;    // 초기 쥐 쌍의 수
const MAX_CAPACITY: u32 = 2200;
const REPRODUCTION_RATE: f32 = 0.1;
const STRESS_THRESHOLD: f32 = 0.7;

#[derive(Resource)]
struct SimulationState {
    population: u32,
    total_stress: f32,
    time: f32,
    phase: Phase,
}

#[derive(PartialEq)]
enum Phase {
    Adaptation,    // 적응기
    Exponential,   // 성장기
    Stagnation,    // 정체기
    Decline,       // 쇠퇴기
}

#[derive(Component)]
struct Mouse {
    stress_level: f32,
    social_status: SocialStatus,
    age: f32,
    gender: Gender,
    can_reproduce: bool,
}

#[derive(PartialEq)]
enum Gender {
    Male,
    Female,
}

#[derive(PartialEq, Clone)]
enum SocialStatus {
    Normal,
    BeautifulOne,  // 사회적 상호작용 거부
    Aggressive,    // 공격적 성향
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            population: INITIAL_PAIRS * 2,
            total_stress: 0.0,
            time: 0.0,
            phase: Phase::Adaptation,
        }
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SimulationState::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_simulation,
            update_mice_behavior,
            update_visualization
        ))
        .run();
}

fn setup(mut commands: Commands) {
    // 카메라 설정
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    });

    // 실험 공간 테두리
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.5, 0.5, 0.5),
            custom_size: Some(Vec2::new(SPACE_SIZE, SPACE_SIZE)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });

    // 초기 쥐 쌍 생성
    let mut rng = rand::thread_rng();
    for _ in 0..INITIAL_PAIRS {
        // 암컷 생성
        spawn_mouse(&mut commands, &mut rng, Gender::Female);
        // 수컷 생성
        spawn_mouse(&mut commands, &mut rng, Gender::Male);
    }
}

fn spawn_mouse(commands: &mut Commands, rng: &mut rand::rngs::ThreadRng, gender: Gender) {
    let angle = rng.gen_range(0.0..2.0 * PI);
    let radius = rng.gen_range(0.0..SPACE_SIZE / 3.0);
    let x = radius * angle.cos();
    let y = radius * angle.sin();

    let color = match gender {
        Gender::Male => Color::rgb(0.9, 0.9, 1.0),   // 약간 파란빛의 흰색
        Gender::Female => Color::rgb(1.0, 0.9, 0.9),  // 약간 분홍빛의 흰색
    };

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(10.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, y, 1.0),
            ..default()
        },
        Mouse {
            stress_level: 0.0,
            social_status: SocialStatus::Normal,
            age: 0.0,
            gender,
            can_reproduce: true,
        },
    ));
}

fn update_simulation(
    mut state: ResMut<SimulationState>,
    time: Res<Time>,
    mice: Query<&Mouse>,
) {
    state.time += time.delta_seconds();
    state.population = mice.iter().count() as u32;
    
    // 전체 스트레스 레벨 계산
    state.total_stress = mice.iter().map(|mouse| mouse.stress_level).sum();
    let avg_stress = state.total_stress / state.population as f32;

    // 페이즈 업데이트
    state.phase = match (state.population, avg_stress) {
        (p, _) if p <= INITIAL_PAIRS * 4 => Phase::Adaptation,
        (p, s) if p < MAX_CAPACITY/2 && s < STRESS_THRESHOLD => Phase::Exponential,
        (p, s) if p < MAX_CAPACITY && s >= STRESS_THRESHOLD => Phase::Stagnation,
        _ => Phase::Decline,
    };
}

fn update_mice_behavior(
    mut commands: Commands,
    mut mice: Query<(Entity, &mut Mouse, &mut Sprite, &Transform)>,
    state: Res<SimulationState>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    // 번식 가능한 쌍 찾기
    let mut potential_pairs = Vec::new();
    for (entity1, mouse1, _, transform1) in mice.iter() {
        for (entity2, mouse2, _, transform2) in mice.iter() {
            if entity1 != entity2 
                && mouse1.gender != mouse2.gender 
                && mouse1.social_status == SocialStatus::Normal 
                && mouse2.social_status == SocialStatus::Normal
                && mouse1.can_reproduce 
                && mouse2.can_reproduce {
                    
                // 거리 계산
                let distance = transform1.translation.distance(transform2.translation);
                if distance < 30.0 {  // 충분히 가까운 경우
                    potential_pairs.push((entity1, entity2));
                }
            }
        }
    }

    // 번식 진행
    for (_, _) in potential_pairs {
        if state.phase == Phase::Exponential 
            && rng.gen_bool(REPRODUCTION_RATE as f64) 
            && state.population < MAX_CAPACITY {
            // 50% 확률로 암수 결정
            let gender = if rng.gen_bool(0.5) {
                Gender::Male
            } else {
                Gender::Female
            };
            spawn_mouse(&mut commands, &mut rng, gender);
        }
    }

    // 개별 쥐 업데이트
    for (entity, mut mouse, mut sprite, _) in mice.iter_mut() {
        mouse.age += time.delta_seconds();

        // 스트레스 레벨 업데이트
        let density_stress = state.population as f32 / MAX_CAPACITY as f32;
        mouse.stress_level = (mouse.stress_level + density_stress * 0.1).min(1.0);

        // 사회적 상태 업데이트
        if mouse.stress_level > STRESS_THRESHOLD {
            mouse.social_status = if rng.gen_bool(0.3) {
                SocialStatus::BeautifulOne
            } else {
                SocialStatus::Aggressive
            };
        }

        // 색상 업데이트
        let base_color = match mouse.gender {
            Gender::Male => Color::rgb(0.9, 0.9, 1.0),
            Gender::Female => Color::rgb(1.0, 0.9, 0.9),
        };
        
        sprite.color = match mouse.social_status {
            SocialStatus::Normal => base_color,
            SocialStatus::BeautifulOne => Color::rgb(0.0, 1.0, 1.0),
            SocialStatus::Aggressive => Color::rgb(1.0, 0.0, 0.0),
        };

        // 노화 또는 스트레스로 인한 사망
        if mouse.age > 100.0 || (mouse.stress_level > 0.9 && rng.gen_bool(0.1)) {
            commands.entity(entity).despawn();
        }
    }
}

fn update_visualization(
    mut mice: Query<(&Mouse, &mut Transform)>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    
    for (mouse, mut transform) in mice.iter_mut() {
        // 랜덤한 움직임
        let angle = rng.gen_range(0.0..2.0 * PI);
        let speed = match mouse.social_status {
            SocialStatus::Normal => 50.0,
            SocialStatus::BeautifulOne => 20.0,
            SocialStatus::Aggressive => 80.0,
        };

        let movement = Vec3::new(
            angle.cos() * speed * time.delta_seconds(),
            angle.sin() * speed * time.delta_seconds(),
            0.0,
        );

        // 경계 확인
        let new_pos = transform.translation + movement;
        if new_pos.x.abs() < SPACE_SIZE/2.0 && new_pos.y.abs() < SPACE_SIZE/2.0 {
            transform.translation = new_pos;
        }
    }
}