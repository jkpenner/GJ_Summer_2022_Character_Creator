use bevy::prelude::*;
use bevy_asset_loader::AssetCollection;

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path="fonts/Kenney Future.ttf")]
    pub title: Handle<Font>,
    #[asset(path="fonts/Kenney Future Narrow.ttf")]
    pub general: Handle<Font>,
    #[asset(path="fonts/Kenney Mini.ttf")]
    pub button: Handle<Font>
}