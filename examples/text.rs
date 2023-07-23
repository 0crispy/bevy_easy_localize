use bevy::prelude::*;
use bevy_easy_localize::{Localize, LocalizeText};

pub fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    asset_folder: "examples/assets".to_string(),
                    watch_for_changes: true,
                })
                .build(),
        )
        .add_plugin(bevy_easy_localize::LocalizePlugin)
        .insert_resource(Localize::from_asset_path("test.csv"))
        .add_startup_system(setup)
        .add_system(change_language)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        TextBundle::from_section(
            "default value",
            TextStyle {
                font: asset_server.load("font.ttf"),
                font_size: 100.0,
                color: Color::WHITE,
            },
        ),
        LocalizeText::from_section("hello"),
    ));
}
fn change_language(keyboard: Res<Input<KeyCode>>, mut localize: ResMut<Localize>) {
    if keyboard.just_pressed(KeyCode::E) {
        localize.set_language("English");
    }
    if keyboard.just_pressed(KeyCode::G) {
        localize.set_language("German");
    }
}
