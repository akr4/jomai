use anyhow::Result;
use tantivy::{
    tokenizer::{LowerCaser, StopWordFilter, TextAnalyzer},
    Index,
};

use crate::documents::search::stopwords::load_stopwords_for_lang;

pub fn setup_tokenizer(index: &Index) -> Result<()> {
    let tokenizer = tantivy::tokenizer::NgramTokenizer::all_ngrams(2, 2);
    let mut stopwords = vec![];
    stopwords.extend(load_stopwords_for_lang("en")?);
    stopwords.extend(load_stopwords_for_lang("ja")?);
    let analyzer = TextAnalyzer::from(tokenizer)
        .filter(LowerCaser)
        .filter(StopWordFilter::remove(stopwords));

    index.tokenizers().register("ngram", analyzer);
    Ok(())
}
