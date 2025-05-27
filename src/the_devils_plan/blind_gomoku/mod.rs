use bevy::prelude::*;

// ==================== 게임 상수 정의 ====================
const BOARD_SIZE: usize = 15; // 오목판 크기 (15x15)
const GRID_SIZE: f32 = 40.0; // 각 격자칸의 픽셀 크기
const LINE_WIDTH: f32 = 2.0; // 격자선 두께
const STONE_RADIUS: f32 = 16.0; // 돌의 반지름

// ==================== 색상 상수 정의 ====================
const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.85, 0.7); // 전체 배경색 (연한 베이지)
const BOARD_COLOR: Color = Color::srgb(0.87, 0.72, 0.53); // 오목판 배경색 (나무색)
const LINE_COLOR: Color = Color::BLACK; // 격자선 색상
const STAR_COLOR: Color = Color::BLACK; // 화점(별) 색상
const BLACK_STONE_COLOR: Color = Color::srgb(0.1, 0.1, 0.1); // 흑돌 색상
const WHITE_STONE_COLOR: Color = Color::srgb(0.95, 0.95, 0.95); // 백돌 색상
const TEXT_COLOR: Color = Color::srgb(0.2, 0.2, 0.2); // 텍스트 색상

/// 메인 함수 - 오목 게임 실행
pub fn example() {
    App::new()
        // 기본 플러그인 설정 (렌더링, 윈도우, 입력 등)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "오목 (Omok)".to_string(),
                resolution: (900.0, 700.0).into(),
                ..default()
            }),
            ..default()
        }))
        // 리소스 초기화
        .insert_resource(ClearColor(BACKGROUND_COLOR)) // 배경색 설정
        .insert_resource(GameState::default()) // 게임 상태 초기화
        // 시스템 등록
        .add_systems(Startup, setup) // 게임 시작 시 실행될 setup 함수
        .add_systems(Update, (handle_stone_placement, update_turn_display)) // 매 프레임 실행될 함수들
        .run();
}

// ==================== 게임 상태 리소스 ====================
/// 전체 게임의 상태를 관리하는 리소스
#[derive(Resource)]
struct GameState {
    board: [[Option<StoneColor>; BOARD_SIZE]; BOARD_SIZE], // 15x15 오목판 상태
    current_player: StoneColor,                            // 현재 턴의 플레이어
    game_over: bool,                                       // 게임 종료 여부
    winner: Option<StoneColor>,                            // 승자 (있을 경우)
}

impl Default for GameState {
    /// GameState의 기본값 설정
    fn default() -> Self {
        Self {
            board: [[None; BOARD_SIZE]; BOARD_SIZE], // 모든 칸을 빈 상태로 초기화
            current_player: StoneColor::Black,       // 흑돌이 선공
            game_over: false,                        // 게임 진행 중
            winner: None,                            // 아직 승자 없음
        }
    }
}

// ==================== 돌 색깔 열거형 ====================
/// 오목돌의 색깔을 나타내는 열거형
#[derive(Clone, Copy, PartialEq, Debug)]
enum StoneColor {
    Black, // 흑돌
    White, // 백돌
}

impl StoneColor {
    /// 돌 색깔을 Bevy Color로 변환
    fn to_color(&self) -> Color {
        match self {
            StoneColor::Black => BLACK_STONE_COLOR,
            StoneColor::White => WHITE_STONE_COLOR,
        }
    }

    /// 돌 색깔을 한국어 문자열로 변환
    fn to_korean(&self) -> &'static str {
        match self {
            StoneColor::Black => "흑돌",
            StoneColor::White => "백돌",
        }
    }

    /// 상대방 돌 색깔 반환
    fn opposite(&self) -> Self {
        match self {
            StoneColor::Black => StoneColor::White,
            StoneColor::White => StoneColor::Black,
        }
    }
}

// ==================== 컴포넌트 정의 ====================
/// 오목돌을 나타내는 컴포넌트
#[derive(Component)]
struct Stone {
    color: StoneColor,        // 돌의 색깔
    grid_pos: (usize, usize), // 격자상의 위치 (x, y)
}

/// 격자선을 나타내는 컴포넌트 (사용되지 않음, 하위 호환성을 위해 유지)
#[derive(Component)]
struct GridLine;

/// 화점(별)을 나타내는 컴포넌트
#[derive(Component)]
struct StarPoint;

/// 턴 표시 UI를 나타내는 컴포넌트
#[derive(Component)]
struct TurnDisplay;

/// 게임 종료 메시지를 나타내는 컴포넌트
#[derive(Component)]
struct GameOverDisplay;

// ==================== 컴포넌트 번들 정의 ====================
/// 오목판 배경을 나타내는 컴포넌트 (Sprite와 Transform을 자동으로 포함)
#[derive(Component)]
#[require(Sprite, Transform)]
struct BoardBackground;

/// 격자선을 나타내는 컴포넌트 (Sprite와 Transform을 자동으로 포함)
#[derive(Component)]
#[require(Sprite, Transform)]
struct GridLineComponent;

impl BoardBackground {
    /// 새로운 보드 배경 생성
    fn new() -> (Self, Sprite, Transform) {
        // 보드 크기 계산 (격자 크기 + 여백)
        let board_size = (BOARD_SIZE - 1) as f32 * GRID_SIZE + 60.0;

        (
            BoardBackground,
            Sprite::from_color(BOARD_COLOR, Vec2::ONE), // 나무색 배경
            Transform {
                translation: Vec3::new(0.0, 0.0, -1.0), // z축 -1로 설정하여 배경으로
                scale: Vec3::new(board_size, board_size, 1.0),
                ..default()
            },
        )
    }
}

impl GridLineComponent {
    /// 세로 격자선 생성
    fn vertical(index: usize) -> (Self, Sprite, Transform) {
        // 격자선의 x 좌표 계산 (중앙 기준)
        let x = (index as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * GRID_SIZE;
        // 격자선 길이 계산
        let length = (BOARD_SIZE - 1) as f32 * GRID_SIZE;

        (
            GridLineComponent,
            Sprite::from_color(LINE_COLOR, Vec2::ONE),
            Transform {
                translation: Vec3::new(x, 0.0, 0.0),
                scale: Vec3::new(LINE_WIDTH, length, 1.0), // 가로 폭 작게, 세로 길게
                ..default()
            },
        )
    }

    /// 가로 격자선 생성
    fn horizontal(index: usize) -> (Self, Sprite, Transform) {
        // 격자선의 y 좌표 계산 (중앙 기준)
        let y = (index as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * GRID_SIZE;
        // 격자선 길이 계산
        let length = (BOARD_SIZE - 1) as f32 * GRID_SIZE;

        (
            GridLineComponent,
            Sprite::from_color(LINE_COLOR, Vec2::ONE),
            Transform {
                translation: Vec3::new(0.0, y, 0.0),
                scale: Vec3::new(length, LINE_WIDTH, 1.0), // 가로 길게, 세로 폭 작게
                ..default()
            },
        )
    }
}

// ==================== 초기 설정 시스템 ====================
/// 게임 시작 시 필요한 모든 엔티티를 생성하는 함수
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 2D 카메라 생성
    commands.spawn(Camera2d);

    // 오목판 배경 생성
    commands.spawn(BoardBackground::new());

    // 격자선 생성 (15개의 세로선과 15개의 가로선)
    for i in 0..BOARD_SIZE {
        commands.spawn(GridLineComponent::vertical(i)); // 세로선
        commands.spawn(GridLineComponent::horizontal(i)); // 가로선
    }

    // 화점(별) 생성 - 바둑판의 특별한 점들
    let star_points = [
        (3, 3),
        (3, 11),
        (11, 3),
        (11, 11), // 모서리 화점 4개
        (7, 7),   // 중앙 화점 1개
        (3, 7),
        (11, 7),
        (7, 3),
        (7, 11), // 변 화점 4개
    ];

    for &(grid_x, grid_y) in &star_points {
        // 격자 좌표를 월드 좌표로 변환
        let world_x = (grid_x as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * GRID_SIZE;
        let world_y = (grid_y as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * GRID_SIZE;

        // 작은 원으로 화점 표시
        commands.spawn((
            Mesh2d(meshes.add(Circle::default())),
            MeshMaterial2d(materials.add(STAR_COLOR)),
            Transform {
                translation: Vec3::new(world_x, world_y, 0.1), // 격자선보다 약간 위에
                scale: Vec3::splat(6.0),                       // 작은 원
                ..default()
            },
            StarPoint,
        ));
    }

    // 턴 표시 UI 생성 (화면 왼쪽 위)
    commands.spawn((
        Text::new("현재 턴: 흑돌"),
        TextFont {
            font_size: 28.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        TurnDisplay,
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));

    // 게임 설명 UI 생성 (화면 왼쪽 아래)
    commands.spawn((
        Text::new("마우스 클릭으로 돌을 놓으세요"),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        TextColor(TEXT_COLOR),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
    ));
}

// ==================== 돌 놓기 처리 시스템 ====================
/// 마우스 클릭을 감지하여 돌을 놓는 시스템
fn handle_stone_placement(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    // 게임이 끝났거나 마우스 왼쪽 버튼을 누르지 않았으면 무시
    if game_state.game_over || !mouse_button.just_pressed(MouseButton::Left) {
        return;
    }

    // 윈도우와 카메라 정보 가져오기
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();

    // 마우스 커서 위치 확인
    if let Some(cursor_pos) = window.cursor_position() {
        // 화면 좌표를 월드 좌표로 변환
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            // 월드 좌표를 격자 좌표로 변환
            let grid_x = ((world_pos.x + (BOARD_SIZE - 1) as f32 / 2.0 * GRID_SIZE) / GRID_SIZE)
                .round() as i32;
            let grid_y = ((world_pos.y + (BOARD_SIZE - 1) as f32 / 2.0 * GRID_SIZE) / GRID_SIZE)
                .round() as i32;

            // 격자 좌표가 유효한 범위 내인지 확인
            if grid_x >= 0
                && grid_x < BOARD_SIZE as i32
                && grid_y >= 0
                && grid_y < BOARD_SIZE as i32
            {
                let gx = grid_x as usize;
                let gy = grid_y as usize;

                // 해당 위치가 비어있는지 확인
                if game_state.board[gx][gy].is_none() {
                    // 게임 상태에 돌 정보 저장
                    game_state.board[gx][gy] = Some(game_state.current_player);

                    // 격자 좌표를 월드 좌표로 다시 변환 (정확한 위치에 돌 배치)
                    let world_x = (gx as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * GRID_SIZE;
                    let world_y = (gy as f32 - (BOARD_SIZE - 1) as f32 / 2.0) * GRID_SIZE;

                    // 돌 스프라이트 생성
                    commands.spawn((
                        Mesh2d(meshes.add(Circle::default())), // 원형 메시
                        MeshMaterial2d(materials.add(game_state.current_player.to_color())), // 돌 색상
                        Transform {
                            translation: Vec3::new(world_x, world_y, 1.0), // 격자선보다 위에 배치
                            scale: Vec3::splat(STONE_RADIUS * 2.0),        // 돌 크기
                            ..default()
                        },
                        Stone {
                            color: game_state.current_player,
                            grid_pos: (gx, gy),
                        },
                    ));

                    // 승부 판정
                    if check_win(&game_state.board, gx, gy, game_state.current_player) {
                        // 게임 종료 처리
                        game_state.game_over = true;
                        game_state.winner = Some(game_state.current_player);

                        // 승리 메시지 UI 생성
                        commands.spawn((
                            Text::new(format!(
                                "{}이 승리했습니다!",
                                game_state.current_player.to_korean()
                            )),
                            TextFont {
                                font_size: 36.0,
                                ..default()
                            },
                            TextColor(Color::srgb(1.0, 0.2, 0.2)), // 빨간색 텍스트
                            GameOverDisplay,
                            Node {
                                position_type: PositionType::Absolute,
                                top: Val::Px(100.0),
                                left: Val::Px(50.0),
                                ..default()
                            },
                        ));
                    } else {
                        // 승부가 나지 않았으면 턴 교체
                        game_state.current_player = game_state.current_player.opposite();
                    }
                }
            }
        }
    }
}

// ==================== 턴 표시 업데이트 시스템 ====================
/// 현재 턴을 화면에 표시하는 시스템
fn update_turn_display(
    game_state: Res<GameState>,
    mut turn_display: Query<&mut Text, With<TurnDisplay>>,
) {
    // 턴 표시 텍스트 컴포넌트 찾기
    if let Ok(mut text) = turn_display.get_single_mut() {
        // 게임이 진행 중일 때만 턴 정보 업데이트
        if !game_state.game_over {
            **text = format!("현재 턴: {}", game_state.current_player.to_korean());
        }
    }
}

// ==================== 승부 판정 함수 ====================
/// 5목이 완성되었는지 확인하는 함수
///
/// # 매개변수
/// * `board` - 현재 게임판 상태
/// * `x`, `y` - 마지막에 놓인 돌의 위치
/// * `color` - 확인할 돌의 색상
///
/// # 반환값
/// * `bool` - 5목이 완성되었으면 true, 아니면 false
fn check_win(
    board: &[[Option<StoneColor>; BOARD_SIZE]; BOARD_SIZE],
    x: usize,
    y: usize,
    color: StoneColor,
) -> bool {
    // 확인할 4가지 방향: 가로, 세로, 대각선 2개
    let directions = [
        (1, 0),  // 가로 (→)
        (0, 1),  // 세로 (↑)
        (1, 1),  // 대각선 (↗)
        (1, -1), // 대각선 (↘)
    ];

    // 각 방향에 대해 5목 여부 확인
    for &(dx, dy) in &directions {
        let mut count = 1; // 현재 놓은 돌 포함하여 카운트 시작

        // 한 방향으로 연속된 돌 개수 세기
        let mut nx = x as i32 + dx;
        let mut ny = y as i32 + dy;
        while nx >= 0 && nx < BOARD_SIZE as i32 && ny >= 0 && ny < BOARD_SIZE as i32 {
            if board[nx as usize][ny as usize] == Some(color) {
                count += 1;
                nx += dx; // 다음 위치로 이동
                ny += dy;
            } else {
                break; // 다른 색 돌이나 빈 공간을 만나면 중단
            }
        }

        // 반대 방향으로 연속된 돌 개수 세기
        nx = x as i32 - dx;
        ny = y as i32 - dy;
        while nx >= 0 && nx < BOARD_SIZE as i32 && ny >= 0 && ny < BOARD_SIZE as i32 {
            if board[nx as usize][ny as usize] == Some(color) {
                count += 1;
                nx -= dx; // 반대 방향으로 이동
                ny -= dy;
            } else {
                break; // 다른 색 돌이나 빈 공간을 만나면 중단
            }
        }

        // 5개 이상 연속이면 승리
        if count >= 5 {
            return true;
        }
    }

    false // 어떤 방향으로도 5목이 완성되지 않음
}
