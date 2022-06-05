mod char_select;
mod main_menu;

use bevy::prelude::*;

use char_select::CharSelectPlugin;
use main_menu::MainMenuPlugin;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MainMenuPlugin)
            .add_plugin(CharSelectPlugin);
    }
}
