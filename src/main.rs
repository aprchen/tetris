use std::borrow::{Borrow, BorrowMut};
use std::collections::HashMap;
use std::time::Duration;

use bevy::prelude::*;
// use bevy::sprite::collide_aabb::collide;
use bevy::render::camera::ScalingMode;
use rand::Rng;

mod debug;
mod sprite;

const RESOLUTION: f32 = 16.0 / 9.0;
const ROW: i32 = 20;
const COL: i32 = 12;
// const PRE_AREA_X: f32 = 20.0;
// const PRE_AREA_Y: f32 = 15.0;

const SCALE: f32 = 0.8;

fn main() {
    App::new()
        .insert_resource(SpeedTimer(Timer::from_seconds(0.5, true)))
        .insert_resource(RemoveTimer(Timer::from_seconds(0.2, true)))
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            width: 600.0 * RESOLUTION,
            height: 600.0,
            title: "tetris".to_string(),
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
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

struct SpeedTimer(Timer);

struct RemoveTimer(Timer);

#[derive(Component, Default, bevy_inspector_egui::Inspectable)]
pub struct Square(i32);// 0 空置,1 占位 2 元素过度


const STATE_EMPTY: i32 = 0;
const STATE_WALKING: i32 = 2;
const STATE_TAKE: i32 = 1;


impl Square {
    fn update_state(&mut self, state: i32) {
        self.0 = state;
    }
}

#[derive(Default)]
pub struct CurrentElement {
    central_location: Vec2,
    shape: i32,
    direction: i32,
    state: i32,
    is_initialized: bool,
}

impl CurrentElement {
    // 初始化
    fn initialize(&mut self, shape: i32) {
        self.central_location = Vec2::new(COL as f32 / 2., ROW as f32);
        self.shape = shape;
        self.state = 0;
        self.direction = sprite::UP;
        self.is_initialized = true;
    }

    fn left(&mut self, query: &Query<(&mut Sprite, &Transform, &mut Square), With<TableArea>>) {
        let match_square = self.square_match(query);
        // 墙壁判断
        for v in match_square.iter() {
            if v.x == 0. {
                return;
            }
        }

        for target in match_square.iter() {
            for (_sp, transform, sq) in query.iter() {
                if sq.0 == STATE_TAKE && target.x - 1. == transform.translation.x && target.y == transform.translation.y {
                    return;
                }
            }
        }

        self.central_location.x -= 1.;
    }

    fn right(&mut self, query: &Query<(&mut Sprite, &Transform, &mut Square), With<TableArea>>) {
        let match_square = self.square_match(query);
        // 墙壁判断
        for v in match_square.iter() {
            if v.x + 1. == COL as f32 {
                return;
            }
        }

        for target in match_square.iter() {
            for (_sp, transform, sq) in query.iter() {
                if sq.0 == STATE_TAKE && target.x + 1. == transform.translation.x && target.y == transform.translation.y {
                    return;
                }
            }
        }

        self.central_location.x += 1.;
    }

    // 旋转
    fn rotate(&mut self, query: &Query<(&mut Sprite, &Transform, &mut Square), With<TableArea>>) {
        self.direction += 1;
        if self.direction > 3 {
            self.direction = 0;
        }
        // 旋转之后的图形
        let match_square = self.square_match(query);
        // 墙壁判断
        let mut pass = true;
        for v in match_square.iter() {
            if v.x + 1. > COL as f32 || v.x < 0. || v.y <= 0. {
                pass = false;
            }
        }

        if !pass {
            //方向回滚
            self.direction -= 1;
            if self.direction < 0 {
                self.direction = 3
            }
        }
    }

    fn update(&mut self, tt: Res<Time>,
              mut timer: ResMut<SpeedTimer>,
              query: &mut Query<(&mut Sprite, &Transform, &mut Square), With<TableArea>>) {
        //逐步下降
        if timer.0.tick(tt.delta()).just_finished() {
            self.central_location.y -= 1.0;
        }

        let match_square = self.square_match(query.borrow());
        // 标记当前元素是否需要被冻结
        for ms in match_square.iter() {
            let ms_y = ms.y;
            let ms_x = ms.x;
            let next_y = ms_y - 1.;
            let next_x = ms_x;
            for (_sp, transform, sq) in query.iter_mut() {
                if transform.translation == *ms {
                    if ms_y == 0. {
                        //靠墙了,需要被冻结
                        self.state = -1;
                    }
                } else if transform.translation.x == next_x && transform.translation.y == next_y && sq.0 == STATE_TAKE {
                    //不是匹配方块，如果这个方块是匹配方块的下一行，且有任意一个是被占据的，则整个元素需要被冻结
                    self.state = -1;
                }
            }
        }

        let mut need_init: bool = false;
        //状态更新，设置颜色
        for (mut sp, transform, mut sq) in query.iter_mut() {
            if in_vec(transform.translation, &match_square) {
                // 如果当前元素状态是冻结，这几个方块状态都设为占据
                if self.state == -1 {
                    need_init = true;
                    sq.update_state(STATE_TAKE);
                } else {
                    sq.update_state(STATE_WALKING);
                }
                sp.color = sprite::get_color(self.shape);
            } else {
                // 不是当前元素方块的，恢复过渡状态的到初始状态
                if sq.0 == STATE_WALKING {
                    sq.update_state(STATE_EMPTY);
                    sp.color = sprite::get_color(sprite::SHAPE_BG); //使用默认颜色
                }
            }
        }

        if need_init {
            self.is_initialized = false;
        }
    }

    fn square_match(&mut self, query: &Query<(&mut Sprite, &Transform, &mut Square), With<TableArea>>) -> Vec<Vec3> {
        let mut match_square: Vec<Vec3> = Vec::default();
        for (_sp, transform, _sq) in query.iter() {
            let loc = transform.translation;
            let res = match self.shape {
                sprite::SHAPE_T => sprite::shape_t_match(self.central_location, loc, self.direction),
                sprite::SHAPE_O => sprite::shape_o_match(self.central_location, loc),
                sprite::SHAPE_I => sprite::shape_i_match(self.central_location, loc, self.direction),
                sprite::SHAPE_J => sprite::shape_j_match(self.central_location, loc, self.direction),
                sprite::SHAPE_L => sprite::shape_l_match(self.central_location, loc, self.direction),
                sprite::SHAPE_S => sprite::shape_s_match(self.central_location, loc, self.direction),
                sprite::SHAPE_Z => sprite::shape_z_match(self.central_location, loc, self.direction),
                _ => false,
            };
            if res {
                match_square.push(transform.translation);
            }
        }
        match_square
    }
}

//
// fn pre_system(
//     mut state: Local<CurrentElement>,
//     mut query: Query<(&mut Sprite, &mut Square), With<PreviewArea>>,
// ) {}

// 方块消除
fn remove_blocks(mut cmd: Commands, tt: Res<Time>, mut timer: ResMut<RemoveTimer>, mut state: Local<bool>, mut query: Query<(Entity, &mut Transform, &mut Square), With<TableArea>>) {
    if !*state {
        *state = true;
    }
    if timer.0.tick(tt.delta()).just_finished() {
        let mut sq_map = HashMap::new();
        for (_entity, transform, sq) in query.iter() {
            if sq.0 == STATE_TAKE {
                let count = sq_map.entry(transform.translation.y.to_string()).or_insert(0);
                *count += 1;
            }
        }

        let max_key = ROW - 1;
        if sq_map.get(&max_key.to_string()).is_some() {
            info!("游戏结束");
            return;
        }

        for (key, val) in sq_map {
            // 满足一行全部占用，每次只去一行
            if val == COL {
                info!("remove block{:?}",key);
                let key_val = key.parse::<i32>().unwrap();
                for (entity, mut transform, _sq) in query.iter_mut() {
                    // 移去对应行
                    if transform.translation.y == key_val as f32 {
                        cmd.entity(entity).despawn()
                    }
                    // 该行之上的全部下移一行
                    let target = transform.translation + Vec3::new(0.0, -1., 0.0);

                    if transform.translation.y > key_val as f32 {
                        transform.translation = target;
                    }
                }
                // 补充这一行
                insert_table_row(cmd.borrow_mut(), ROW - 1);
                break;
            }
        }
    }
}


// 基础操作
fn basic_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Sprite, &Transform, &mut Square), With<TableArea>>,
    tt: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
    mut state: Local<CurrentElement>,
) {
    if !state.is_initialized {
        timer.0.set_duration(Duration::from_millis(500));
        state.initialize(rand::thread_rng().gen_range(1..8));
    }

    if input.just_pressed(KeyCode::Left) || input.just_pressed(KeyCode::A) {
        state.left(query.borrow());
    }
    if input.just_pressed(KeyCode::Right) || input.just_pressed(KeyCode::D) {
        state.right(query.borrow());
    }
    if input.just_pressed(KeyCode::Up) || input.just_pressed(KeyCode::W) {
        state.rotate(query.borrow());
    }
    if input.just_pressed(KeyCode::Down) || input.just_pressed(KeyCode::S) {
        timer.0.set_duration(Duration::from_millis(10));
    }

    state.update(tt, timer, query.borrow_mut());
}

fn in_vec(v: Vec3, data: &Vec<Vec3>) -> bool {
    for datum in data {
        if *datum == v {
            return true;
        }
    }
    false
}


fn camera_startup_system(mut cmd: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 25.0;
    camera.orthographic_projection.bottom = -5.0;
    camera.orthographic_projection.right = 25.0 * RESOLUTION;
    camera.orthographic_projection.left = -5.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    cmd.spawn_bundle(camera);
}

fn insert_table_row(commands: &mut Commands, row: i32) {
    for col in 0..COL {  // 水平轴,x
        let brick_position = Vec2::new(
            col as f32,
            row as f32,
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
            .insert(Name::new(format!("square_x_{:?}_y_{:?}", brick_position.x, brick_position.y)))
            .insert(Square(STATE_EMPTY))
            .insert(TableArea);
    }
}

fn map_startup_system(mut commands: Commands) {
    // 游戏区方块
    for row in 0..ROW {
        insert_table_row(commands.borrow_mut(), row);
    }
    // 预览区
    // for row in 0..5 {
    //     for col in 0..5 {
    //         let brick_position = Vec2::new(
    //             PRE_AREA_X + col as f32 * (0.8),
    //             PRE_AREA_Y + row as f32 * (0.8),
    //         );
    //         let color = sprite::get_color(sprite::SHAPE_PRE);
    //         commands
    //             .spawn()
    //             .insert_bundle(SpriteBundle {
    //                 sprite: Sprite {
    //                     color,
    //                     ..default()
    //                 },
    //                 transform: Transform {
    //                     translation: brick_position.extend(0.0),
    //                     scale: Vec3::new(SCALE * 0.8, SCALE * 0.8, 1.0),
    //                     ..default()
    //                 },
    //                 ..default()
    //             })
    //             .insert(PreviewArea);
    //     }
    // }
}
