use std::{
    fs,
    io::{BufRead, Read},
    path::Path,
};

use anyhow::Result;

use crate::assets::Assets;

pub fn load_stopwords_for_lang(lang: &str) -> Result<Vec<String>> {
    let data = Assets::get_stopwords_for_lang(lang).unwrap();
    read_lines(&*data)
}

fn read_lines<R: Read>(reader: R) -> Result<Vec<String>> {
    let mut lines = Vec::new();
    let reader = std::io::BufReader::new(reader);
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        lines.push(line.to_string());
    }
    Ok(lines)
}
