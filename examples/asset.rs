use std::time::Duration;

use bevy::{asset::ChangeWatcher, prelude::*};
use bevy_easy_localize::Localize;
pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    asset_folder: "examples/assets".to_string(),
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                })
                .build(),
        )
        .add_plugins(bevy_easy_localize::LocalizePlugin)
        .insert_resource(Localize::from_asset_path("test.csv"))
        .add_systems(Update, press_space)
        .run();
}

fn press_space(keyboard: Res<Input<KeyCode>>, mut localize: ResMut<Localize>) {
    localize.set_language("German");
    if keyboard.just_pressed(KeyCode::Space) {
        println!("{}", localize.get("start_game"));
    }
}
