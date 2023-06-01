#[derive(rust_embed::RustEmbed)]
#[folder = "assets"]
pub struct Assets;

impl Assets {
    pub fn get_vibrato_dict_data() -> Box<[u8]> {
        Box::from(Assets::get("system.dic").unwrap().data)
    }

    pub fn get_stopwords_for_lang(lang: &str) -> Option<Box<[u8]>> {
        Assets::get(format!("stopwords/stopwords-{}.txt", lang).as_str()).map(|x| Box::from(x.data))
    }
}
