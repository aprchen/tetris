use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_editor_pls::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::{CurrentElement, PreviewArea, Square, TableArea};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(FrameTimeDiagnosticsPlugin)
                .add_plugin(EntityCountDiagnosticsPlugin)
                .add_plugin(EditorPlugin)
                .add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<TableArea>()
                .register_inspectable::<Square>()
                .register_inspectable::<CurrentElement>()
                .register_inspectable::<PreviewArea>();
        }
    }
}
