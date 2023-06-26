# `bevy_easy_localize`
[![Crates.io](https://img.shields.io/crates/v/bevy_easy_localize)](https://crates.io/crates/bevy_easy_localize)

A simple crate to localize your game using .csv files.
## Features
- Loading from `.csv` files
- Loading the translation file from the asset folder
- Automatically updating text components
- Hot reloading
- Lightweight
## Upcoming features
- Per-language fonts
- More flexible and customizable `.csv` file loading
## How to use
The `.csv` file currently must be arranged in this order:

|Keyword|Comments|Language_0|Language_1|...|
|---|---|---|---|---|
|word|comment|translation0|translation1|...|

![image](https://user-images.githubusercontent.com/50209404/209450226-0362a4b5-4b26-47ad-adc0-90fa2f902ef3.png)

In your project:
```rust
use bevy::prelude::*;
use bevy_easy_localize::Localize;
pub fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(bevy_easy_localize::LocalizePlugin)
        //Insert the resource from an asset path
        .insert_resource(Localize::from_asset_path("translations.csv"))
        .add_system(translate)
        .run();
}
fn translate(
    keyboard:Res<Input<KeyCode>>,
    mut localize:ResMut<Localize>,
){
    //Easily set the language
    localize.set_language("German");
    if keyboard.just_pressed(KeyCode::Space){
        //Use the get() method to get a translated word for the specified keyword
        println!("{}",localize.get("start_game"));
    }
}
```
Using the `LocalizeText` component:
```rust
commands.spawn((
    TextBundle::from_section(
        "default value",
        TextStyle {
            font: asset_server.load("font.ttf"),
            font_size: 100.0,
            color: Color::WHITE,
        },
    ),
    //add this component to automatically translate text
    LocalizeText::from_section("my_keyword")
));
```
## Examples
- [`simple`](examples/simple.rs) – Reading from a file to initialize the resource.
- [`asset`](examples/asset.rs) – Using asset handles to initialize the resource.
- [`text`](examples/text.rs) – Using the `LocalizeText` component to update text.
## Bevy Compatibility
|bevy|bevy_easy_localize|
|---|---|
|0.10|0.2|
|0.9|0.1|
## About
I made this crate for my personal projects. 
The obvious alternative is [`bevy_fluent`](https://github.com/kgv/bevy_fluent), but my goal is to just translate some text and 
I don't need all of the fancy features it offers.
I will definitely be updating this crate and if you want to add a feature, please submit a pull request.
