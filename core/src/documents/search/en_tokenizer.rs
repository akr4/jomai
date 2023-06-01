use std::{fs, io::BufReader, sync::Arc};

use anyhow::Result;
use tantivy::{
    tokenizer::{Language, LowerCaser, RemoveLongFilter, SimpleTokenizer, Stemmer, StopWordFilter, TextAnalyzer},
    Index,
};
use tantivy_vibrato::VibratoTokenizer;

use crate::{assets::Assets, documents::search::stopwords::load_stopwords_for_lang};

pub fn setup_tokenizer(index: &Index) -> Result<()> {
    let stopwords = load_stopwords_for_lang("en")?;
    let analyzer = TextAnalyzer::from(SimpleTokenizer)
        .filter(RemoveLongFilter::limit(40))
        .filter(LowerCaser)
        .filter(StopWordFilter::remove(stopwords))
        .filter(Stemmer::new(Language::English));

    index.tokenizers().register("lang_en", analyzer);
    Ok(())
}
