use bevy::prelude::*;

const GROUND_HEIGHT: f32 = -200.0;
const CUSTOMER_SIZE: f32 = 30.0;
const QUEUE_SPACING: f32 = 40.0;
const SERVICE_POINT_SPACING: f32 = 100.0;  // 서비스 포인트 간 간격

#[derive(Resource)]
struct QueueState {
    queue: i32,            // 현재 대기 인원
    total_queue: i32,      // 총 대기 수
    total_arrivals: i32,   // 총 도착 고객 수
    time: f32,            // 현재 시간
    time_step: f32,       // 시간 단위
    time_limit: f32,      // 시뮬레이션 제한 시간
    tpump: [f32; 3],      // 각 서비스 포인트의 서비스 시간
    arrival_probability: f32,  // 도착 확률
    seed: i32,            // 난수 시드
    nseed: i32,           // 서비스 시간용 시드
}

impl Default for QueueState {
    fn default() -> Self {
        Self {
            queue: 0,
            total_queue: 0,
            total_arrivals: 0,
            time: 0.0,
            time_step: 1.0,
            time_limit: 100.0,
            tpump: [0.0; 3],
            arrival_probability: 1.0 / 3.0,
            seed: 35213,
            nseed: 35213,
        }
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<QueueState>()
        .add_systems(Startup, setup)
        .add_systems(Update, (queue_simulation_system, update_visualization_system))
        .run();
}

#[derive(Component)]
struct Customer;

#[derive(Component)]
struct ServicePoint {
    index: usize,
}

#[derive(Component)]
struct ServingCustomer {
    service_point: usize,
}

fn setup(mut commands: Commands) {
    // 카메라 설정
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scale: 3.0,
            ..Default::default()
        }
        .into(),
        transform: Transform::from_xyz(0.0, 0.0, 100.0),
        ..Default::default()
    });

    // 두 개의 서비스 포인트 생성 (빨간색 사각형)
    for i in 0..2 {
        let x_pos = (i as f32 - 0.5) * SERVICE_POINT_SPACING;
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(40.0, 40.0)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(x_pos, GROUND_HEIGHT, 1.0),
                ..Default::default()
            },
            ServicePoint { index: i + 1 },
        ));
    }
}

fn queue_simulation_system(
    mut state: ResMut<QueueState>,
    time: Res<Time>,
) {
    state.time += time.delta_seconds();
    
    if state.time >= state.time_limit {
        return;
    }

    // 새로운 고객 도착 확률 계산
    let mut random_value = 0.0;
    random(&mut state.seed, &mut random_value);
    
    if random_value < state.arrival_probability * state.time_step {
        state.queue += 1;
        state.total_arrivals += 1;
    }

    // 각 서비스 포인트 처리
    for i in 1..3 {
        // 서비스 중인 고객 처리
        if state.tpump[i] > 0.0 {
            state.tpump[i] -= state.time_step;
            if state.tpump[i] < 0.0 {
                state.tpump[i] = 0.0;
            }
        }

        // 새로운 고객 서비스 시작
        if state.tpump[i] == 0.0 && state.queue > 0 {
            state.queue -= 1;
            let mut pp = 0;
            poissn(&mut state.nseed, &mut pp);
            state.tpump[i] = pp as f32;
        }
    }

    state.total_queue += state.queue;
}

fn update_visualization_system(
    mut commands: Commands,
    state: Res<QueueState>,
    customers: Query<Entity, Or<(With<Customer>, With<ServingCustomer>)>>,
) {
    // 기존 고객 스프라이트 제거
    for entity in customers.iter() {
        commands.entity(entity).despawn();
    }

    // 서비스 중인 고객 표시
    for i in 1..3 {
        if state.tpump[i] > 0.0 {
            let x_pos = ((i - 1) as f32 - 0.5) * SERVICE_POINT_SPACING;
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0), // 서비스 중인 고객은 초록색
                        custom_size: Some(Vec2::new(CUSTOMER_SIZE, CUSTOMER_SIZE)),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(x_pos, GROUND_HEIGHT, 1.1),
                    ..Default::default()
                },
                ServingCustomer {
                    service_point: i,
                },
            ));
        }
    }

    // 대기열의 고객들 표시
    for i in 0..state.queue {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 1.0), // 대기 중인 고객은 파란색
                    custom_size: Some(Vec2::new(CUSTOMER_SIZE, CUSTOMER_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(
                    -SERVICE_POINT_SPACING, 
                    GROUND_HEIGHT + (i as f32 + 1.0) * QUEUE_SPACING, 
                    1.0
                ),
                ..Default::default()
            },
            Customer,
        ));
    }
}

// Helper 함수들
fn random(np: &mut i32, up: &mut f32) {
    *np = np.wrapping_mul(843314861).wrapping_add(453816693);
    if *np < 0 {
        *np = np.wrapping_add(2147483647).wrapping_add(1);
    }
    *up = *np as f32 * 0.4656612e-9;
}

fn poissn(pn: &mut i32, pp: &mut i32) {
    let mut prod: f32;
    let mut u: f32 = 0.;
    *pp = 0;
    let b = f32::exp(-4.0);  // MEAN = 4.0
    prod = 1.;
    random(pn, &mut u);
    prod = prod * u;
    while prod >= b {
        random(pn, &mut u);
        prod = prod * u;
        *pp += 1;
    }
}