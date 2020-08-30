use bevy::app::Plugin;

mod board_element;
mod board_materials;

pub use board_element::*;
pub use board_materials::*;

pub struct GameComponentsPlugin;

impl Plugin for GameComponentsPlugin {
    fn build(&self, app: &mut bevy::prelude::AppBuilder) {
        app.init_resource::<BoardMaterials>();
    }
}
