use bevy::input::keyboard::KeyCode;
use bevy::input::InputSystem;
use bevy::prelude::*;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::env;
// use std::cmp::Ordering;
// use std::collections::BinaryHeap;
// Components
#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Destination {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct GridMap {
    size: i32,
    tiles: Vec<Vec<bool>>, // true if walkable
}

#[derive(Component)]
struct Wall;

#[derive(Component)]
struct BoardCell;

#[derive(Component)]
struct PathMarker;

// Node structure for A* (이전과 동일)
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    f: i32,
    g: i32,
    x: i32,
    y: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.cmp(&self.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_board)
        .add_systems(Update, (spawn_path_markers, handle_click))
        .run();
}
fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    10 * ((x1 - x2).abs() + (y1 - y2).abs())
}

fn is_valid_position(x: i32, y: i32, size: i32) -> bool {
    x >= 0 && x < size && y >= 0 && y < size
}
fn reconstruct_path(
    parent: &Vec<Vec<Option<(i32, i32)>>>,
    start: &Position,
    dest: &Destination,
) -> Vec<(i32, i32)> {
    let mut path = Vec::new();
    let mut current = (dest.y, dest.x);

    while current != (start.y, start.x) {
        path.push(current);
        if let Some(prev) = parent[current.0 as usize][current.1 as usize] {
            current = prev;
        } else {
            break;
        }
    }
    path.push((start.y, start.x));
    path.reverse();
    path
}
fn find_path(start: &Position, dest: &Destination, grid: &GridMap) -> Vec<(i32, i32)> {
    let dir_y: [i32; 8] = [-1, 0, 1, 0, -1, 1, 1, -1];
    let dir_x: [i32; 8] = [0, -1, 0, 1, -1, -1, 1, 1];
    let cost: [i32; 8] = [10, 10, 10, 10, 14, 14, 14, 14];

    let size = grid.size as usize;
    let mut closed = vec![vec![false; size]; size];
    let mut open = vec![vec![i32::MAX; size]; size];
    let mut heap = BinaryHeap::new();
    let mut parent = vec![vec![None; size]; size];

    open[start.y as usize][start.x as usize] = manhattan_distance(start.x, start.y, dest.x, dest.y);
    heap.push(Node {
        f: manhattan_distance(start.x, start.y, dest.x, dest.y),
        g: 0,
        x: start.x,
        y: start.y,
    });
    parent[start.y as usize][start.x as usize] = Some((start.y, start.x));

    while let Some(node) = heap.pop() {
        if closed[node.y as usize][node.x as usize] {
            continue;
        }

        closed[node.y as usize][node.x as usize] = true;

        if node.x == dest.x && node.y == dest.y {
            return reconstruct_path(&parent, start, dest);
        }

        for i in 0..dir_y.len() {
            let next_y = node.y + dir_y[i];
            let next_x = node.x + dir_x[i];

            if !is_valid_position(next_x, next_y, grid.size) {
                continue;
            }

            if !grid.tiles[next_y as usize][next_x as usize] {
                continue;
            }

            if closed[next_y as usize][next_x as usize] {
                continue;
            }

            let g = node.g + cost[i];
            let h = manhattan_distance(next_x, next_y, dest.x, dest.y);
            let f = g + h;

            if open[next_y as usize][next_x as usize] <= f {
                continue;
            }

            open[next_y as usize][next_x as usize] = f;
            heap.push(Node {
                f,
                g,
                x: next_x,
                y: next_y,
            });
            parent[next_y as usize][next_x as usize] = Some((node.y, node.x));
        }
    }

    Vec::new()
}

fn setup_board(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(225.0, 225.0, 1000.0),
        ..default()
    });

    let mut tiles = vec![vec![true; 9]; 9];

    // 벽 설정을 더 명확하게 (y좌표 반전 고려)
    for i in 3..6 {
        tiles[8 - 4][i] = false; // 가로 벽
        tiles[8 - i][4] = false; // 세로 벽
    }

    // 바둑판 배경
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.9, 0.8, 0.7),
            custom_size: Some(Vec2::new(450.0, 450.0)),
            ..default()
        },
        transform: Transform::from_xyz(225.0, 225.0, 0.0),
        ..default()
    });

    // 격자와 벽 생성
    for i in 0..9 {
        for j in 0..9 {
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.2, 0.2, 0.2),
                        custom_size: Some(Vec2::new(5.0, 5.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        50.0 + i as f32 * 50.0,
                        50.0 + j as f32 * 50.0,
                        1.0,
                    ),
                    ..default()
                },
                BoardCell,
            ));

            // 벽 생성 시 y좌표 반전
            if !tiles[8 - j][i] {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(1.0, 0.0, 0.0, 0.7),
                            custom_size: Some(Vec2::new(45.0, 45.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            50.0 + i as f32 * 50.0,
                            50.0 + j as f32 * 50.0,
                            2.0,
                        ),
                        ..default()
                    },
                    Wall,
                ));
            }
        }
    }

    commands.spawn((
        Position { x: 0, y: 8 },    // 시작 위치 조정
        Destination { x: 8, y: 0 }, // 도착 위치 조정
        GridMap { size: 9, tiles },
    ));
}
// 클릭 처리 시스템
fn handle_click(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut position_query: Query<&mut Position>,
    mut destination_query: Query<&mut Destination>,
    mouse_input: Res<ButtonInput<MouseButton>>,
) {
    let (camera, camera_transform) = camera_q.single();
    let window = windows.single();

    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        // y좌표를 8에서 빼서 반전
        let grid_y = ((world_position.x) / 50.0).floor() as i32;
        let grid_x = 8 - ((world_position.y) / 50.0).floor() as i32;

        println!(
            "Click at world pos: ({}, {}), Grid pos: ({}, {})",
            world_position.x, world_position.y, grid_x, grid_y
        );

        if grid_x >= 0 && grid_x < 9 && grid_y >= 0 && grid_y < 9 {
            if mouse_input.just_pressed(MouseButton::Left) {
                for mut pos in position_query.iter_mut() {
                    pos.x = grid_x;
                    pos.y = grid_y;
                    println!("Set start position to: ({}, {})", grid_x, grid_y);
                }
            } else if mouse_input.just_pressed(MouseButton::Right) {
                for mut dest in destination_query.iter_mut() {
                    dest.x = grid_x;
                    dest.y = grid_y;
                    println!("Set destination to: ({}, {})", grid_x, grid_y);
                }
            }
        }
    }
}

fn spawn_path_markers(
    mut commands: Commands,
    query: Query<(&Position, &Destination, &GridMap)>,
    path_markers: Query<Entity, With<PathMarker>>,
) {
    for entity in path_markers.iter() {
        commands.entity(entity).despawn();
    }

    for (pos, dest, grid) in query.iter() {
        let path = find_path(pos, dest, grid);

        if !path.is_empty() {
            for (i, &(x, y)) in path.iter().enumerate() {
                let color = if i == 0 {
                    Color::rgba(0.0, 1.0, 0.0, 0.7) // 시작점 (초록색)
                } else if i == path.len() - 1 {
                    Color::rgba(1.0, 0.0, 0.0, 0.7) // 도착점 (빨간색)
                } else {
                    Color::rgba(0.0, 0.5, 1.0, 0.7) // 경로 (파란색)
                };

                // y좌표를 8에서 빼서 반전
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color,
                            custom_size: Some(Vec2::new(30.0, 30.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            50.0 + x as f32 * 50.0,
                            50.0 + (8 - y) as f32 * 50.0, // y좌표 반전
                            3.0,
                        ),
                        ..default()
                    },
                    PathMarker,
                ));
            }
        }
    }
}

// find_path, manhattan_distance, is_valid_position, reconstruct_path 함수들은 이전과 동일
