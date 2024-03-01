use bevy::prelude::*;
use bevy_easy_localize::Localize;
pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "examples/assets".to_string(),
                    ..Default::default()
                })
                .build(),
        )
        .add_plugins(bevy_easy_localize::LocalizePlugin)
        .insert_resource(Localize::from_asset_path("test.csv"))
        .add_systems(Update, press_space)
        .run();
}

fn press_space(keyboard: Res<ButtonInput<KeyCode>>, mut localize: ResMut<Localize>) {
    localize.set_language("German");
    if keyboard.just_pressed(KeyCode::Space) {
        println!("{}", localize.get("start_game"));
    }
}
