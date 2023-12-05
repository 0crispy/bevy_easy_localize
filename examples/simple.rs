use std::fs::read_to_string;
use bevy::{log::LogPlugin, prelude::*};
use bevy_easy_localize::Localize;
pub fn main() {
    App::new()
        .add_plugins(LogPlugin::default())
        .insert_resource(Localize::from_data(
            &read_to_string("examples/test.csv").unwrap(),
        ))
        .add_systems(Startup, hello)
        .run();
}

fn hello(mut localize: ResMut<Localize>) {
    localize.set_language("German");
    println!("{}", localize.get("hello"));
}
