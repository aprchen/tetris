use std::collections::HashMap;
use std::time::Duration;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_editor_pls::*;

mod debug;
mod sprite;

const RESOLUTION: f32 = 16.0 / 9.0;
const ROW :i32= 20;
const COL :i32= 12;

const SCALE :f32 = 0.8;

const TABLE_AREA_X: f32 = -10.0;
const TABLE_AREA_Y: f32 = -8.0;
const PRE_AREA_X: f32 = 6.0;
const PRE_AREA_Y: f32 = 2.0;

fn main() {
    App::new()
        .insert_resource(SpeedTimer(Timer::from_seconds(0.5, true)))
        .insert_resource(RemoveTimer(Timer::from_seconds(0.2, true)))
        //     .insert_resource(CurrentElement::default())
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            // width:800.0,
            // height:600.0,
            title: "tetris".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        // 调试使用插件
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // 调试使用插件
        .add_plugin(EntityCountDiagnosticsPlugin)
        // 调试使用插件
        .add_plugin(EditorPlugin)
        .add_plugin(debug::DebugPlugin)
        .add_startup_system(camera_startup_system)
        .add_startup_system(map_startup_system)
        .add_system(basic_system)
        .add_system(remove_blocks)
        .run();
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct PreviewArea;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct TableArea;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Player {}

// #[derive(Component, Deref, DerefMut)]
// struct AnimationTimer(Timer);

struct SpeedTimer(Timer);

struct RemoveTimer(Timer);

#[derive(Component, Default, bevy_inspector_egui::Inspectable)]
pub struct Square {
    central_location: Vec2,
    state: i32, // 0 空置,1 占位 2 元素过度
}

const STATE_EMPTY: i32 = 0;
const STATE_WALKING: i32 = 2;
const STATE_TAKE: i32 = 1;


impl Square {
    fn new(loc: Vec2) -> Square {
        Square { central_location: loc, state: STATE_EMPTY }
    }
    fn update_state(&mut self, state: i32) {
        self.state = state;
    }
}

#[derive(Default, bevy_inspector_egui::Inspectable)]
pub struct CurrentElement {
    central_location: Vec2,
    shape: i32,
    direction: i32,
    state: i32,
    is_initialized: bool,
}


#[allow(dead_code)]
fn is_against_wall_x(tar: f32) -> bool {
    if tar == 0. || tar >= 11.0 {
        return true;
    }
    false
}

impl CurrentElement {
    fn left(&mut self) {
        self.central_location.x -= 1.;
    }
    fn right(&mut self) {
        self.central_location.x += 1.;
    }

    fn down(&mut self) {
        self.central_location.y -= 1.0;
    }

    // 旋转
    fn rotate(&mut self) {
        self.direction += 1;
        if self.direction > 3 {
            self.direction = 0;
        }
    }

    fn update_state(&mut self, state: i32) {
        self.state = state;
    }

    fn square_match(&mut self, square: &Square) -> bool {
        if self.shape == sprite::SHAPE_T {
            return sprite::shape_t_match(self.central_location, square.central_location, self.direction);
        }
        false
    }
}


fn pre_system(
    mut state: Local<CurrentElement>,
    mut query: Query<(&mut Sprite, &mut Square), With<PreviewArea>>,
) {

    // for (mut sp, mut sq) in query.iter_mut() {
    //
    // }
}

// 方块消除
fn remove_blocks(tt: Res<Time>, mut timer: ResMut<RemoveTimer>, mut state: Local<bool>, query: Query<(&mut Sprite, &mut Square), With<TableArea>>) {
    if !*state {
        // timer.0.set_repeating(true);
        // timer.0.set_duration(Duration::from_micros(200));
        *state = true;
    }
    if timer.0.tick(tt.delta()).just_finished() {
        let mut sq_map = HashMap::new();
        for (_sp, sq) in query.iter() {
            if sq.state == STATE_TAKE {
                let count = sq_map.entry(sq.central_location.y.to_string()).or_insert(0);
                *count += 1;
            }
        }
        info!("sq_map {:?}",sq_map);
        let max_row_index = ROW-1;
        match sq_map.get(&max_row_index.to_string()) { //最顶层方块有冻结
            None => {}
            Some(_) => {
                info!("游戏结束");
                return;
            }
        }
        for (key,val) in sq_map {
            if val == COL {
                info!("row {:?} need remove",key)
            }
        }

    }
}

// 基础操作
fn basic_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Square), With<TableArea>>,
    tt: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
    mut state: Local<CurrentElement>,
) {
    if !state.is_initialized {
        state.central_location = Vec2::new(COL as f32/2., ROW as f32);
        state.shape = sprite::SHAPE_T;
        state.update_state(0);
        state.direction = sprite::UP;
        timer.0.set_duration(Duration::from_millis(500));
        state.is_initialized = true;
        info!("状态初始化");
    }
    if input.just_pressed(KeyCode::Left) {
        state.left();
    }
    if input.just_pressed(KeyCode::Right) {
        state.right();
    }
    if input.just_pressed(KeyCode::Up) {
        state.rotate();
    }
    if input.just_pressed(KeyCode::Down) {
        timer.0.set_duration(Duration::from_millis(10));
    }
    if timer.0.tick(tt.delta()).just_finished() {
        state.down();
    }
    // 查找当前时刻元素方块坐标
    let mut match_square: Vec<Vec2> = Vec::default();
    for (_sp, sq) in query.iter() {
        if state.square_match(sq) {
            match_square.push(sq.central_location);
        }
    }

    // 标记当前元素是否需要被冻结
    for ms in match_square.iter() {
        let ms_y = ms.y;
        let ms_x = ms.x;
        let next_y = ms_y - 1.;
        let next_x = ms_x;
        for (_sp, sq) in query.iter_mut() {
            if sq.central_location == *ms {
                if ms_y == 0. {
                    //靠墙了,需要被冻结
                    state.update_state(-1);
                }
            } else if sq.central_location.x == next_x && sq.central_location.y == next_y && sq.state == STATE_TAKE {
                //不是匹配方块，如果这个方块是匹配方块的下一行，且有任意一个是被占据的，则整个元素需要被冻结
                state.update_state(-1);
            }
        }
    }

    let mut need_init: bool = false;
    //状态更新，设置颜色
    for (mut sp, mut sq) in query.iter_mut() {
        if in_vec(sq.central_location, &match_square) {
            // 如果当前元素状态是冻结，这几个方块状态都设为占据
            if state.state == -1 {
                need_init = true;
                sq.update_state(STATE_TAKE);
            } else {
                sq.update_state(STATE_WALKING);
            }
            sp.color = sprite::get_color(state.shape);
        } else {
            // 不是当前元素方块的，恢复过渡状态的到初始状态
            if sq.state == STATE_WALKING {
                sq.update_state(STATE_EMPTY);
                sp.color = sprite::get_color(sprite::SHAPE_BG); //使用默认颜色
            }
        }
    }

    if need_init {
        state.is_initialized = false;
    }
}

fn in_vec(v: Vec2, data: &Vec<Vec2>) -> bool {
    for datum in data {
        if *datum == v {
            return true;
        }
    }
    false
}


fn camera_startup_system(mut cmd: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 20.0;
    camera.orthographic_projection.bottom = -20.0;
    camera.orthographic_projection.right = 20.0 * RESOLUTION;
    camera.orthographic_projection.left = -20.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    cmd.spawn_bundle(camera);
}

fn map_startup_system(mut commands: Commands) {
    // 游戏区方块
    for row in 0..ROW { //垂直轴,y
        for col in 0..COL {  // 水平轴,x
            let brick_position = Vec2::new(
                TABLE_AREA_X + col as f32 * (1.0),
                TABLE_AREA_Y + row as f32 * (1.0),
            );
            let color = sprite::get_color(sprite::SHAPE_BG);
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(SCALE, SCALE, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(format!("square_row_{:?}_col_{:?}", row, col)))
                .insert(Square::new(Vec2::new(col as f32, row as f32)))
                .insert(TableArea);
        }
    }
    // 预览区
    for row in 0..5 {
        for col in 0..5 {
            let brick_position = Vec2::new(
                PRE_AREA_X + col as f32 * (0.8),
                PRE_AREA_Y + row as f32 * (0.8),
            );
            let color = sprite::get_color(sprite::SHAPE_PRE);
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(SCALE*0.8, SCALE*0.8, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(format!("square_pre_row_{:?}_col_{:?}", row, col)))
                .insert(Square::new(Vec2::new(col as f32, row as f32)))
                .insert(PreviewArea);
        }
    }
}
