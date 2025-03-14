use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle},
    render::mesh::Mesh,
};

#[derive(Component)]
struct Stock {
    price: f32,
    prev_prices: Vec<f32>,
    volume: f32,
    volatility: f32,
    trend: f32,
    name: String,
}

#[derive(Component)]
struct PriceChart;

#[derive(Component)]
struct VolumeChart;

#[derive(Component)]
struct PricePoint;

#[derive(Resource)]
struct SimulationState {
    elapsed_time: f32,
    update_interval: f32,
    max_price: f32,
    min_price: f32,
}

const WINDOW_WIDTH: f32 = 1200.0;
const WINDOW_HEIGHT: f32 = 800.0;
const CHART_WIDTH: f32 = 800.0;
const CHART_HEIGHT: f32 = 400.0;
const MAX_PRICE_POINTS: usize = 100;
const UPDATE_INTERVAL: f32 = 1.0;

pub fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .insert_resource(SimulationState {
            elapsed_time: 0.0,
            update_interval: UPDATE_INTERVAL,
            max_price: 100.0,
            min_price: 0.0,
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                title: "Stock Market Simulation".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (
            update_stock_price,
            update_charts,
            update_price_display,
        ))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 카메라 설정
    commands.spawn(Camera2dBundle::default());

    // 차트 배경 생성
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(CHART_WIDTH, CHART_HEIGHT)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        PriceChart,
    ));

    // 거래량 차트 배경
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(CHART_WIDTH, CHART_HEIGHT * 0.3)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -CHART_HEIGHT * 0.7, 0.0),
            ..default()
        },
        VolumeChart,
    ));

    // 초기 주식 데이터 생성
    commands.spawn(Stock {
        price: 50.0,
        prev_prices: vec![50.0],
        volume: 0.0,
        volatility: 0.1,
        trend: 0.0,
        name: "SAMPLE".to_string(),
    });

    // 격자선 생성
    for i in 0..5 {
        let y_pos = -CHART_HEIGHT / 2.0 + i as f32 * CHART_HEIGHT / 4.0;
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(1.0, 1.0, 1.0, 0.2),
                custom_size: Some(Vec2::new(CHART_WIDTH, 1.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, y_pos, 0.1),
            ..default()
        });
    }
}

fn update_stock_price(
    time: Res<Time>,
    mut sim_state: ResMut<SimulationState>,
    mut stocks: Query<&mut Stock>,
) {
    sim_state.elapsed_time += time.delta_secs();

    if sim_state.elapsed_time >= sim_state.update_interval {
        for mut stock in stocks.iter_mut() {
            let random = rand::random::<f32>() * 2.0 - 1.0;
            let trend = stock.trend + (random - stock.trend) * 0.1;
            let price_change = stock.price * stock.volatility * (random + trend);
            let new_price = stock.price + price_change;
            let new_volume = (stock.volume * 0.8 + price_change.abs() * 100.0).abs();
            
            stock.trend = trend;
            stock.price = new_price;
            stock.volume = new_volume;
            stock.prev_prices.push(new_price);
            
            if stock.prev_prices.len() > MAX_PRICE_POINTS {
                stock.prev_prices.remove(0);
            }

            sim_state.max_price = sim_state.max_price.max(new_price);
            sim_state.min_price = sim_state.min_price.min(new_price);
        }
        sim_state.elapsed_time = 0.0;
    }
}

fn update_charts(
    mut commands: Commands,
    stocks: Query<&Stock>,
    old_points: Query<Entity, With<PricePoint>>,
    sim_state: Res<SimulationState>,
) {
    for entity in old_points.iter() {
        commands.entity(entity).despawn();
    }

    for stock in stocks.iter() {
        let price_range = sim_state.max_price - sim_state.min_price;
        
        // 가격 차트 그리기
        for (i, window) in stock.prev_prices.windows(2).enumerate() {
            let x1 = -CHART_WIDTH / 2.0 + (i as f32 / MAX_PRICE_POINTS as f32) * CHART_WIDTH;
            let bar_width = CHART_WIDTH / MAX_PRICE_POINTS as f32;

            let y1 = -CHART_HEIGHT / 2.0 + ((window[0] - sim_state.min_price) / price_range) * CHART_HEIGHT;
            let y2 = -CHART_HEIGHT / 2.0 + ((window[1] - sim_state.min_price) / price_range) * CHART_HEIGHT;
            
            let height = y2 - y1;
            let color = if window[1] > window[0] {
                Color::rgb(0.0, 0.8, 0.0)
            } else {
                Color::rgb(0.8, 0.0, 0.0)
            };

            // 캔들스틱 그리기
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color,
                        custom_size: Some(Vec2::new(bar_width * 0.8, height.abs())),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x1 + bar_width / 2.0,
                        (y1 + y2) / 2.0,
                        0.2
                    ),
                    ..default()
                },
                PricePoint,
            ));
        }

        // 거래량 바 차트
        let volume_height = CHART_HEIGHT * 0.3;
        let max_volume = stock.prev_prices.windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .fold(0.0, f32::max);

        for (i, window) in stock.prev_prices.windows(2).enumerate() {
            let volume = (window[1] - window[0]).abs();
            let height = (volume / max_volume) * volume_height;
            let x = -CHART_WIDTH / 2.0 + (i as f32 / MAX_PRICE_POINTS as f32) * CHART_WIDTH;
            let bar_width = CHART_WIDTH / MAX_PRICE_POINTS as f32;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgba(0.5, 0.5, 1.0, 0.5),
                        custom_size: Some(Vec2::new(bar_width * 0.8, height)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        x + bar_width / 2.0,
                        -CHART_HEIGHT * 0.85 - volume_height / 2.0 + height / 2.0,
                        0.2
                    ),
                    ..default()
                },
                PricePoint,
            ));
        }
    }
}

fn update_price_display(
    mut commands: Commands,
    stocks: Query<&Stock>,
    old_text: Query<Entity, With<Text>>,
) {
    for entity in old_text.iter() {
        commands.entity(entity).despawn();
    }

    for stock in stocks.iter() {
        let price_change = if stock.prev_prices.len() > 1 {
            stock.price - stock.prev_prices[stock.prev_prices.len() - 2]
        } else {
            0.0
        };

        let (color, change_symbol) = if price_change >= 0.0 {
            (Color::rgb(0.0, 1.0, 0.0), "+")
        } else {
            (Color::rgb(1.0, 0.0, 0.0), "-")
        };

        // commands.spawn(Text2dBundle {
        //     text: Text::from_sections([
        //         TextSection {
        //             value: format!("{}: ${:.2} ", stock.name, stock.price),
        //             style: TextStyle {
        //                 font_size: 30.0,
        //                 color: Color::WHITE,
        //                 ..default()
        //             },
        //         },
        //         TextSection {
        //             value: format!("{}${:.2} ({:.1}%)", 
        //                 change_symbol,
        //                 price_change.abs(),
        //                 (price_change / (stock.price - price_change) * 100.0).abs()
        //             ),
        //             style: TextStyle {
        //                 font_size: 30.0,
        //                 color,
        //                 ..default()
        //             },
        //         },
        //     ]),
        //     transform: Transform::from_xyz(-CHART_WIDTH / 2.0, CHART_HEIGHT / 2.0 + 30.0, 0.3),
        //     ..default()
        // });
    }
}