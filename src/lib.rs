use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    utils::{BoxedFuture, HashMap},
};
/// Add this plugin if you are
/// initializing the [`Localize`] resource
/// from an asset handle.
/// Otherwise, the resource will
/// not get initialized.
pub struct LocalizePlugin;
impl Plugin for LocalizePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_asset::<Translation>()
            .init_asset_loader::<TranslationsAssetLoader>()
            .add_systems(Update, update);
    }
}
/// You can use this resource in two ways:
///
/// 1. Load from file
/// ```
/// app.insert_resource(Localize::from_data(&std::fs::read_to_string("test.csv").unwrap()));
/// ```
/// This way makes sure that the
/// resource will be completely initialized
/// and ready to translate.
///
/// 2. Insert it empty, then load from asset handle
/// ```
/// //first insert it as empty
/// app.insert_resource(Localize::empty());
/// //then, in a startup system set the handle
/// fn setup(asset_server:Res<AssetServer>, mut localize:ResMut<Localize>){
///     localize.set_handle(asset_server.load("test.csv"));
/// }
/// ```
/// Using it this way will result in a slight
/// delay until it gets initialized.
#[derive(Resource)]
pub struct Localize {
    is_initialized: bool,
    set_language: Option<String>,
    current_language_id: usize,
    languages: HashMap<String, usize>,
    words: HashMap<String, Vec<String>>,

    asset_handle_path: Option<String>,
    asset_handle: Option<Handle<Translation>>,
}
impl Localize {
    /// Initializes an empty resource
    pub fn empty() -> Self {
        Self {
            is_initialized: false,
            set_language: None,
            current_language_id: 0,
            languages: HashMap::new(),
            words: HashMap::new(),
            asset_handle_path: None,
            asset_handle: None,
        }
    }
    /// Creates a new resource from
    /// specified data (in a .csv format)
    pub fn from_data(translations: &str) -> Self {
        let mut localize = Self::empty();
        localize.set_data(translations);
        localize
    }
    /// Creates a new resource from
    /// specified asset path.
    pub fn from_asset_path(path: &str) -> Self {
        let mut localize = Self::empty();
        localize.asset_handle_path = Some(path.to_string());
        localize
    }
    /// Sets data for the resource
    pub fn set_data(&mut self, translations: &str) {
        let mut languages = HashMap::new();
        let mut words = HashMap::new();

        let mut data = csv::Reader::from_reader(translations.as_bytes());
        let mut records: Vec<Vec<_>> = Vec::new();
        if let Ok(headers) = data.headers() {
            records.push(headers.iter().map(|field| field.to_string()).collect());
        }
        for result in data.records() {
            if let Ok(record) = result {
                records.push(record.iter().map(|field| field.to_string()).collect());
            }
        }
        for (language_id, language) in records[0][2..].into_iter().enumerate() {
            languages.insert(language.to_string(), language_id);
        }
        for record in &records[1..] {
            let keyword = &record[0];
            let translations = record[2..].into_iter().map(|x| x.to_string()).collect();
            words.insert(keyword.to_string(), translations);
        }
        self.languages = languages;
        self.words = words;
        self.is_initialized = true;
    }
    /// Get a translation for a specified keyword.
    ///
    /// If there is no translation for the keyword,
    /// it will return an empty string.
    pub fn get(&self, keyword: &str) -> &str {
        match self.words.get(keyword) {
            Some(k) => {
                if self.current_language_id < k.len() {
                    &k[self.current_language_id]
                } else {
                    ""
                }
            }
            None => "",
        }
    }
    /// Sets the language for the resource.
    pub fn set_language(&mut self, language: &str) {
        if let Some(language_id) = self.languages.get(language) {
            self.current_language_id = *language_id;
        } else {
            self.set_language = Some(language.to_string());
        }
    }
}
/// Translates text.
/// Use it with the [`Text`] component.
/// ```
/// commands.spawn((
///     TextBundle::from_section(
///        "default value",
///        TextStyle {
///            font: asset_server.load("font.ttf"),
///            font_size: 100.0,
///            color: Color::WHITE,
///        },
///     ),
///     //The first section of the text will be
///     //automatically translated
///     //using the specified keyword
///     LocalizeText::from_section("your_keyword")
/// ));
/// ```
#[derive(Component)]
pub struct LocalizeText {
    sections: Vec<String>,
    translated_language: Option<usize>,
}
impl LocalizeText {
    /// The first section
    /// of the text will be translated
    /// using the specified keyword
    pub fn from_section(keyword: impl Into<String>) -> Self {
        Self {
            sections: vec![keyword.into()],
            translated_language: None,
        }
    }
    /// All sections
    /// of the text will be translated
    /// using the specified keywords
    pub fn from_sections(keywords: impl IntoIterator<Item = String>) -> Self {
        Self {
            sections: keywords.into_iter().collect(),
            translated_language: None,
        }
    }
}

fn update(
    localize: Option<ResMut<Localize>>,
    translation_assets: ResMut<Assets<Translation>>,
    mut ev_asset: EventReader<AssetEvent<Translation>>,
    asset_server: Res<AssetServer>,
    mut text: Query<(&mut Text, &mut LocalizeText)>,
) {
    if let Some(mut localize) = localize {
        if let Some(asset_handle_path) = localize.asset_handle_path.clone() {
            localize.asset_handle_path = None;
            localize.asset_handle = Some(asset_server.load(asset_handle_path));
        }
        if let Some(asset_handle) = localize.asset_handle.clone() {
            for ev in ev_asset.iter() {
                match ev {
                    AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                        if handle == &asset_handle {
                            let translation = translation_assets.get(&asset_handle).unwrap();
                            localize.set_data(&translation.0);
                        }
                    }
                    _ => {}
                }
            }
        }

        if let Some(language) = localize.set_language.clone() {
            localize.set_language(&language);
        }
        if localize.is_initialized {
            for (mut text, mut localize_text) in &mut text {
                if localize_text.translated_language.is_none()
                    || localize_text.translated_language.unwrap_or(0)
                        != localize.current_language_id
                {
                    localize_text.translated_language = Some(localize.current_language_id);
                    for (id, keyword) in localize_text.sections.iter().enumerate() {
                        text.sections[id].value = localize.get(&keyword).to_string();
                    }
                }
            }
        }
    }
}

#[derive(Debug, TypeUuid, TypePath)]
#[uuid = "30222702-83bc-11ed-a1eb-0242ac120002"]
pub struct Translation(pub String);

#[derive(Default)]
struct TranslationsAssetLoader;
impl AssetLoader for TranslationsAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = Translation(std::str::from_utf8(bytes).unwrap().to_string());
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["csv"]
    }
}
