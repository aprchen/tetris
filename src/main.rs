mod debug;
mod sprite;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_editor_pls::*;

const RESOLUTION: f32 = 16.0 / 9.0;
const BRICK_COLOR: Color = Color::rgb(0.5, 0.5, 1.0);
const S_COLOR: Color = Color::rgb(0.3, 0.5, 0.3);

const TABLE_AREA_X: f32 = -10.0;
const TABLE_AREA_Y: f32 = -8.0;
const PRE_AREA_X: f32 = 6.0;
const PRE_AREA_Y: f32 = 2.0;

fn main() {
    App::new()
        .insert_resource(SpeedTimer(Timer::from_seconds(1.0, true)))
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
        // //.add_plugin(debug::DebugPlugin)
        .add_startup_system(camera_startup_system)
        .add_startup_system(map_startup_system)
        .add_system(basic_system)
        .add_system(pre_system)
        .run();
}

#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct PreviewArea;
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct TableArea;
#[derive(Component, bevy_inspector_egui::Inspectable)]
pub struct Name(String);

#[derive(Component, Default, Debug, bevy_inspector_egui::Inspectable)]
pub struct Row(i32);
#[derive(Component, Default, Debug, bevy_inspector_egui::Inspectable)]
pub struct Col(i32);
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

struct SpeedTimer(Timer);

#[derive(Default)]
struct CurrentElement {
    col: i32,
    row: i32,
    shape: i32, // 1 _-_ ,2 |
}

fn pre_system(
    mut state: ResMut<CurrentElement>,
    mut query: Query<(&mut Sprite, &Col, &Row), With<PreviewArea>>,
) {
    state.shape = 1;
    for (mut sp, col, row) in query.iter_mut() {
        sp.color = S_COLOR;
        if state.shape == 1 {
            let col1 = 2;
            let row1 = 3;
            if col.0 == col1 && row.0 == row1 {
                sp.color = Color::RED;
            }
            if col.0 == col1 - 1 && row.0 == row1 - 1 {
                sp.color = Color::RED;
            }
            if col.0 == col1 + 1 && row.0 == row1 - 1 {
                sp.color = Color::RED;
            }
            if col.0 == col1 && row.0 == row1 - 1 {
                sp.color = Color::RED;
            }
        }
    }
}

fn basic_system(
    input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Sprite, &Col, &Row), With<TableArea>>,
    tt: Res<Time>,
    mut timer: ResMut<SpeedTimer>,
    mut state: ResMut<CurrentElement>,
) {
    if timer.0.tick(tt.delta()).just_finished() {
        state.col = 5;
        if state.row <= 0 {
            state.row = 16
        }
        state.row -= 1;
        info!("{:?}", state.row);
        for (mut sp, col, row) in query.iter_mut() {
            sp.color = BRICK_COLOR;
            if state.shape == 1 {
                if col.0 == state.col && row.0 == state.row {
                    sp.color = Color::RED;
                }
                if col.0 == state.col - 1 && row.0 == state.row - 1 {
                    sp.color = Color::RED;
                }
                if col.0 == state.col + 1 && row.0 == state.row - 1 {
                    sp.color = Color::RED;
                }
                if col.0 == state.col && row.0 == state.row - 1 {
                    sp.color = Color::RED;
                }
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
    for row in 0..16 {
        for col in 0..12 {
            let brick_position = Vec2::new(
                TABLE_AREA_X + col as f32 * (1.0),
                TABLE_AREA_Y + row as f32 * (1.0),
            );
            commands
                .spawn()
                .insert_bundle(SpriteBundle {
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    transform: Transform {
                        translation: brick_position.extend(0.0),
                        scale: Vec3::new(0.8, 0.8, 1.0),
                        ..default()
                    },
                    ..default()
                })
                .insert(Name("🟦".to_string()))
                .insert(Col(col))
                .insert(Row(row))
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
                .insert(Name("🟩".to_string()))
                .insert(Col(col))
                .insert(Row(row))
                .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                .insert(PreviewArea);
        }
    }
}
