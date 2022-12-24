use bevy::prelude::*;
use bevy_easy_localize::Localize;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin{
            asset_folder: "examples/assets".to_string(),
            watch_for_changes: true,
        }).build())
        .add_plugin(bevy_easy_localize::LocalizePlugin)
        .insert_resource(Localize::empty())
        .add_startup_system(init_localize)
        .add_system(press_space)
        .run();
}

fn init_localize(
    asset_server:Res<AssetServer>,
    mut localize:ResMut<Localize>,
){
    localize.set_handle(asset_server.load("test.csv"));
}

fn press_space(
    keyboard:Res<Input<KeyCode>>,
    mut localize:ResMut<Localize>,
){
    localize.set_language("German");
    if keyboard.just_pressed(KeyCode::Space){
        println!("{}",localize.get("start_game"));
    }
}