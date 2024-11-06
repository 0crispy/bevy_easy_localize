use bevy::prelude::*;
use bevy_easy_localize::{Localize, LocalizeText};

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
        .insert_resource(
            Localize::from_asset_path("test.csv")
            .with_default_language({
                match sys_locale::get_locale().unwrap().as_str(){
                    "en-GB" | "en-US" => "English",
                    "de-DE" => "German",
                    _ => "Unsupported system language!"
                }
            }))
        .add_systems(Startup, setup)
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