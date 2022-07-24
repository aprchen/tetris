use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::reflect::struct_partial_eq;
use bevy::render::camera::ScalingMode;
use bevy_editor_pls::*;

mod debug;
mod sprite;

const RESOLUTION: f32 = 16.0 / 9.0;
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const S_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);

const TABLE_AREA_X: f32 = -10.0;
const TABLE_AREA_Y: f32 = -8.0;
const PRE_AREA_X: f32 = 6.0;
const PRE_AREA_Y: f32 = 2.0;

fn main() {
    App::new()
        .insert_resource(SpeedTimer(Timer::from_seconds(0.5, true)))
        .insert_resource(CurrentElement::default())
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
       // .add_system(pre_system)
        .run();
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct PreviewArea;

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct TableArea;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

struct SpeedTimer(Timer);

#[derive(Component,Default, bevy_inspector_egui::Inspectable)]
pub struct Square{
    central_location: Vec2,
    state: i32, // 0 空置,1 占位 2 元素过度
}

const STATE_EMPTY:i32=0;
const STATE_WALKING:i32=2;
const STATE_TAKE:i32=1;


impl Square {
    fn new(loc: Vec2) -> Square {
        Square{central_location:loc, state: STATE_EMPTY }
    }
    fn update_state(&mut self, state: i32){
        if self.state != STATE_TAKE {
            self.state = state;
        }
    }

    fn is_took(&self) -> bool {
        self.state == STATE_TAKE
    }

    fn can_update(&self) -> bool {
        if self.state != STATE_EMPTY {
            return false;
        }
        return true;
    }
}

#[derive(Default, bevy_inspector_egui::Inspectable)]
pub struct CurrentElement {
    central_location:Vec2,
    shape: i32,
    direction : i32,
    state:i32, // 0 下行中,-1无法下行
}



fn is_against_wall_x (tar:f32) -> bool {
    if tar== 0. || tar >= 11.0{
        return true;
    }
    return false;
}

impl CurrentElement {
    fn new(central_location:Vec2, shape: i32,direction:i32) -> CurrentElement {
        CurrentElement { central_location, shape ,direction,state:0}
    }
    fn left(&mut self) {
        self.central_location.x -= 1.;
    }
    fn right(&mut self) {
        self.central_location.x += 1.;
    }

    fn down(&mut self) {
        if self.central_location.y <= 0. {
            self.central_location.y = 16.;
            return;
        }
        if self.state == -1 &&  self.central_location.y !=0.{
            info!("状态初始化");
            self.central_location.y = 16.;
            self.state = 0;
            return;
        }
        self.central_location.y -= 1.0;
    }
    // 旋转
    fn rotate(&mut self) {
        self.direction +=1;
        if self.direction >3 {
            self.direction = 0;
        }
    }

    fn square_match(&mut self, square:&Square) -> bool {
        let cy = self.central_location.y.clone();
        // 判断下一行是否到底
        // 判断下一行是否可以占用
        if sprite::is_against_bottom_wall(cy,self.shape,self.direction) {
            info!("已被占用1");
            self.state = -1; //标记当前已不可以下行,需要占用当前位置
        }
        // 判断下一行是否存在被占用的
        let nc = self.central_location.clone();
        if sprite::convex_up_match(Vec2::new(nc.x,nc.y-1.),square.central_location.clone(),self.direction) && square.state == STATE_TAKE {
            info!("已被占用2");
            self.state = -1; //标记当前已不可以下行,需要占用当前位置
        }
        if self.shape == sprite::CONVEX {
           return  sprite::convex_up_match(self.central_location.clone(),square.central_location.clone(),self.direction);
        }
        return false;
    }
}


// fn pre_system(
//     mut state: ResMut<CurrentElement>,
//     mut query: Query<(&mut Sprite, &mut Square), With<PreviewArea>>,
// ) {
//     state.shape = CONVEX;
//     for (mut sp, mut sq) in query.iter_mut() {
//         sp.color = S_COLOR;
//         if state.shape == CONVEX {
//             let col1 = 2;
//             let row1 = 3;
//         }
//     }
// }

fn basic_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Sprite, &mut Square), With<TableArea>>,
    tt: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
    mut state: ResMut<CurrentElement>,
    mut is_initialized: Local<bool>,
) {
    if !*is_initialized {
        state.central_location = Vec2::new(6.,15.);
        *is_initialized = true;
    }
    if input.just_pressed(KeyCode::Left) {
        state.left();
    }
    if input.just_pressed(KeyCode::Right) {
        state.right();
    }
    if input.just_pressed(KeyCode::Up){
        state.rotate();
    }
    if timer.0.tick(tt.delta()).just_finished() {
        state.down();
        for (mut sp, mut sq) in query.iter_mut() {
            sq.update_state(STATE_EMPTY);
            if state.square_match(&sq) {
                // 如果当前元素状态为暂停了,且匹配到了对应方块
                if state.state == -1{
                    sq.update_state(STATE_TAKE);
                }else {
                    sq.update_state(STATE_WALKING);
                }
                if sq.state == STATE_WALKING || sq.state == STATE_TAKE {
                    if sq.central_location == state.central_location {
                        sp.color = Color::GOLD;
                    }else {
                        sp.color = Color::RED;
                    }
                }
            }
        }
        for (mut sp, mut sq) in query.iter_mut() {
            if sq.state == STATE_EMPTY {
                sp.color = BRICK_COLOR;
            }
        }
    }
}

fn camera_startup_system(mut cmd: Commands) {
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.orthographic_projection.top = 10.0;
    camera.orthographic_projection.bottom = -10.0;
    camera.orthographic_projection.right = 10.0 * RESOLUTION;
    camera.orthographic_projection.left = -10.0 * RESOLUTION;
    camera.orthographic_projection.scaling_mode = ScalingMode::None;
    cmd.spawn_bundle(camera);
}

fn map_startup_system(mut commands: Commands) {
    // 游戏区方块
    for row in 0..16 { //垂直轴,y
        for col in 0..12 {  // 水平轴,x
            let brick_position = Vec2::new(
                TABLE_AREA_X + col as f32 * (1.0),
                TABLE_AREA_Y + row as f32 * (1.0),
            );
            let mut color = BRICK_COLOR;
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(0.8, 0.8, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(format!("square_row_{:?}_col_{:?}", row, col)))
                .insert(Square::new(Vec2::new(col as f32,row as f32)))
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
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: S_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(0.6, 0.6, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name::new(format!("square_pre_row_{:?}_col_{:?}", row, col)))
                .insert(Square::new(Vec2::new(col as f32,row as f32)))
                .insert(PreviewArea);
        }
    }
}
