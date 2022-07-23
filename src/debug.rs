use crate::{Name, PreviewArea, TableArea};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<Name>()
                .register_inspectable::<TableArea>()
                .register_inspectable::<PreviewArea>();
        }
    }
}
