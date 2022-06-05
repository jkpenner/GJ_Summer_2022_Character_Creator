mod main_menu;

use bevy::prelude::*;

use main_menu::MainMenuPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin);        
    }
}

