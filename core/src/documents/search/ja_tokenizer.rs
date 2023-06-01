use std::{fs, io::BufReader, sync::Arc};

use anyhow::Result;
use tantivy::{
    tokenizer::{
        BoxTokenStream, LowerCaser, RemoveLongFilter, SimpleTokenizer, StopWordFilter, TextAnalyzer, Token,
        TokenFilter, TokenStream,
    },
    Index,
};
use tantivy_vibrato::VibratoTokenizer;

use crate::{
    assets::Assets,
    documents::search::{
        japanese_character_normalizer::JapaneseCharacterTypeNormalizer, stopwords::load_stopwords_for_lang,
    },
};

pub fn setup_tokenizer(index: &Index) -> Result<()> {
    let dict_data = Assets::get_vibrato_dict_data();
    let tokenizer = VibratoTokenizer::from_reader(&*dict_data)?;
    let stopwords = load_stopwords_for_lang("ja")?;
    let analyzer = TextAnalyzer::from(tokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(JapaneseCharacterTypeNormalizer)
        .filter(StopWordFilter::remove(stopwords));

    index.tokenizers().register("lang_ja", analyzer);
    Ok(())
}
