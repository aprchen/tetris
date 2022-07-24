use crate::{PreviewArea, TableArea, CurrentElement, Square};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugin(WorldInspectorPlugin::new())
                .register_inspectable::<TableArea>()
                .register_inspectable::<Square>()
                .register_inspectable::<CurrentElement>()
                .register_inspectable::<PreviewArea>();
        }
    }
}
