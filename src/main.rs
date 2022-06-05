mod game_assets;
mod game_plugin;

use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use game_assets::FontAssets;
use game_plugin::GamePlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    AssetLoading,
    MainMenu,
    CharSelect,
    GamePlay,
    GameOver,
}

fn main() {
    let mut app = App::new();

    // Setup the asset loader
    AssetLoader::new(GameState::AssetLoading)
        .continue_to_state(GameState::MainMenu)
        .with_collection::<FontAssets>()
        .build(&mut app);

    app.insert_resource(win_desc())
        .add_state(GameState::AssetLoading)
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

fn win_desc() -> WindowDescriptor {
    WindowDescriptor {
        title: String::from("Character Creator"),
        width: 640.0,
        height: 480.0,
        ..default()
    }
}
