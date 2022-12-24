use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::{BoxedFuture, HashMap},
};
use serde::Deserialize;
/// Add this plugin if you are
/// initializing the [`Localize`] resource
/// from an asset handle.
/// Otherwise, the resource will
/// not get initialized.
pub struct LocalizePlugin;
impl Plugin for LocalizePlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_asset::<Translation>()
            .init_asset_loader::<TranslationsAssetLoader>()
            .add_system(init_localize_resource);
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
pub struct Localize{
    set_language:Option<String>,
    current_language_id:usize,
    languages:HashMap<String,usize>,
    words:HashMap<String,Vec<String>>,

    asset_handle:Option<Handle<Translation>>,
}
impl Localize{
    /// Initializes an empty resource
    pub fn empty() -> Self{
        Self{
            set_language:None,
            current_language_id:0,
            languages: HashMap::new(),
            words: HashMap::new(),
            asset_handle: None,
        }
    }
    /// Creates a new resource from 
    /// specified data (in a .csv format)
    pub fn from_data(translations:&str) -> Self{
        let mut localize = Self::empty();
        localize.set_data(translations);
        localize
    }
    /// Sets data for the resource
    pub fn set_data(&mut self, translations:&str){
        let mut languages = HashMap::new();
        let mut words = HashMap::new();
        let records = match Self::get_records(translations){
            Ok(records) => records,
            Err(err) => {
                panic!("Failed to parse translation file: {}", err);
            },
        };
        for (language_id,language) in records[0][2..].into_iter().enumerate(){
            languages.insert(language.to_string(),language_id);
        }
        for record in &records[1..]{
            let keyword = &record[0];
            let translations = record[2..].into_iter().map(|x|x.to_string()).collect();
            words.insert(keyword.to_string(),translations);
        
        }
        self.languages = languages;
        self.words = words;
    }
    /// Sets the asset handle for the resource.
    pub fn set_handle(&mut self, translations:Handle<Translation>){  
        self.asset_handle = Some(translations);
    }
    /// Get a translation for a specified keyword.
    /// 
    /// If there is no translation for the keyword,
    /// it will return an empty string.
    pub fn get(&self, keyword:&str) -> &str{
        match self.words.get(keyword){
            Some(k) => {
                if self.current_language_id < k.len(){
                    &k[self.current_language_id]
                } 
                else{
                    ""
                }
            }
            None => "",
        }
    }
    /// Sets the language for the resource.
    pub fn set_language(&mut self, language:&str){
        if let Some(language_id) = self.languages.get(language){
            self.current_language_id = *language_id;
        }
        else{
            self.set_language = Some(language.to_string());
        }
    }
    fn get_records(translations:&str) -> Result<Vec<Vec<String>>,csv::Error>{
        let mut rdr = csv::Reader::from_reader(translations.as_bytes());
        let mut records:Vec<Vec<_>> = Vec::new();
        records.push(rdr.headers()?.iter().map(|x|x.to_string()).collect::<Vec<_>>());
        for result in rdr.records(){
            records.push(result?.iter().map(|x|x.to_string()).collect());
        }
        Ok(records)
    }
}

fn init_localize_resource(
    localize:Option<ResMut<Localize>>,
    translation_assets:ResMut<Assets<Translation>>,
    mut ev_asset: EventReader<AssetEvent<Translation>>,
){
    if let Some(mut localize) = localize{
        if let Some(asset_handle) = localize.asset_handle.clone(){
            for ev in ev_asset.iter() {
                match ev {
                    AssetEvent::Created { handle } | AssetEvent::Modified { handle }=> {
                        if handle == &asset_handle{
                            let translation = translation_assets.get(&asset_handle).unwrap();
                            localize.set_data(&translation.0);
                        }
                    }
                    _ => {}
                }
            }
        }
        if let Some(language) = &localize.set_language{
            if let Some(language_id) = localize.languages.get(language){
                localize.current_language_id = *language_id;
                localize.set_language = None;
            }
        }
    }
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "30222702-83bc-11ed-a1eb-0242ac120002"]
pub struct Translation(String);

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