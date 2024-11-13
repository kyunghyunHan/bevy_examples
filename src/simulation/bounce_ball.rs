use bevy::prelude::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

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

// Node structure for A*
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

// Path finding system
fn path_finding_system(
    mut query: Query<(&Position, &Destination, &GridMap)>,
) {
    for (pos, dest, grid) in query.iter() {
        let path = find_path(pos, dest, grid);
        // Use the path as needed
    }
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

    // Start position
    open[start.y as usize][start.x as usize] = manhattan_distance(start.x, start.y, dest.x, dest.y);
    heap.push(Node {
        f: manhattan_distance(start.x, start.y, dest.x, dest.y),
        g: 0,
        x: start.x,
        y: start.y,
    });
    parent[start.y as usize][start.x as usize] = Some((start.y, start.x));

    while let Some(node) = heap.pop() {
        // Skip if already visited
        if closed[node.y as usize][node.x as usize] {
            continue;
        }

        // Mark as visited
        closed[node.y as usize][node.x as usize] = true;

        // Check if reached destination
        if node.x == dest.x && node.y == dest.y {
            return reconstruct_path(&parent, start, dest);
        }

        // Check neighbors
        for i in 0..dir_y.len() {
            let next_y = node.y + dir_y[i];
            let next_x = node.x + dir_x[i];

            // Bounds check
            if !is_valid_position(next_x, next_y, grid.size) {
                continue;
            }

            // Wall check
            if !grid.tiles[next_y as usize][next_x as usize] {
                continue;
            }

            // Skip if closed
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
            heap.push(Node { f, g, x: next_x, y: next_y });
            parent[next_y as usize][next_x as usize] = Some((node.y, node.x));
        }
    }

    Vec::new() // No path found
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    10 * (((x1 - x2).abs() + (y1 - y2).abs()))
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

// Plugin setup
pub struct PathFindingPlugin;

impl Plugin for PathFindingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, path_finding_system);
    }
}