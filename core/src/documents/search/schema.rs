use tantivy::schema::{
    Field, IndexRecordOption, Schema, TextFieldIndexing, TextOptions, FAST, INDEXED, STORED, STRING, TEXT,
};

use crate::documents::search::Language;

pub fn define_schema() -> Schema {
    let en_text: TextOptions = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer("lang_en")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    );
    let ja_text: TextOptions = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer("lang_ja")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    );
    let ngram_text: TextOptions = TextOptions::default().set_indexing_options(
        TextFieldIndexing::default()
            .set_tokenizer("ngram")
            .set_index_option(IndexRecordOption::WithFreqsAndPositions),
    );

    let mut schema_builder = Schema::builder();
    schema_builder.add_u64_field("language", FAST | STORED);

    schema_builder.add_text_field("path", STRING | STORED);
    schema_builder.add_text_field("path_components", STRING | STORED);
    schema_builder.add_text_field("path_ngram", ngram_text.clone() | STORED);

    schema_builder.add_text_field("contents_en", en_text.clone() | STORED);
    schema_builder.add_text_field("contents_ja", ja_text.clone() | STORED);
    schema_builder.add_text_field("contents_ngram", ngram_text.clone() | STORED);

    schema_builder.add_text_field("title_en", en_text.clone() | STORED);
    schema_builder.add_text_field("title_ja", ja_text.clone() | STORED);
    schema_builder.add_text_field("title_ngram", ngram_text.clone() | STORED);

    schema_builder.add_text_field("tag_en", en_text.clone() | STORED);
    schema_builder.add_text_field("tag_ja", ja_text.clone() | STORED);
    schema_builder.add_text_field("tag_ngram", ngram_text.clone() | STORED);

    schema_builder.add_date_field("created_at", STORED | FAST);
    schema_builder.add_date_field("modified_at", STORED | FAST);
    schema_builder.add_i64_field("watch_id", STORED | INDEXED);

    schema_builder.build()
}

pub struct AppSchema(Schema);

impl AppSchema {
    pub fn new(schema: Schema) -> Self {
        AppSchema(schema)
    }

    pub fn path(&self) -> Field {
        self.get_field("path")
    }
    pub fn path_components(&self) -> Field {
        self.get_field("path_components")
    }
    pub fn path_ngram(&self) -> Field {
        self.get_field("path_ngram")
    }

    pub fn language(&self) -> Field {
        self.get_field("language")
    }

    pub fn contents_all_languages(&self) -> Vec<Field> {
        vec![self.get_field("contents_en"), self.get_field("contents_ja")]
    }

    pub fn contents_all(&self) -> Vec<Field> {
        let mut langguage_fileds = self.contents_all_languages();
        let mut fields = Vec::with_capacity(langguage_fileds.len() + 1);
        fields.append(&mut langguage_fileds);
        fields.push(self.contents_ngram());
        fields
    }

    pub fn contents_ngram(&self) -> Field {
        self.get_field("contents_ngram")
    }

    pub fn contents_for_language(&self, language: Language) -> Field {
        match language {
            Language::English => self.get_field("contents_en"),
            Language::Japanese => self.get_field("contents_ja"),
        }
    }

    pub fn title_all_languages(&self) -> Vec<Field> {
        vec![self.get_field("title_en"), self.get_field("title_ja")]
    }

    pub fn title_all(&self) -> Vec<Field> {
        let mut langguage_fileds = self.title_all_languages();
        let mut fields = Vec::with_capacity(langguage_fileds.len() + 1);
        fields.append(&mut langguage_fileds);
        fields.push(self.title_ngram());
        fields
    }

    pub fn title_ngram(&self) -> Field {
        self.get_field("title_ngram")
    }

    pub fn title_for_language(&self, language: Language) -> Field {
        match language {
            Language::English => self.get_field("title_en"),
            Language::Japanese => self.get_field("title_ja"),
        }
    }

    pub fn tag_all_languages(&self) -> Vec<Field> {
        vec![self.get_field("tag_en"), self.get_field("tag_ja")]
    }

    pub fn tag_all(&self) -> Vec<Field> {
        let mut langguage_fileds = self.tag_all_languages();
        let mut fields = Vec::with_capacity(langguage_fileds.len() + 1);
        fields.append(&mut langguage_fileds);
        fields.push(self.tag_ngram());
        fields
    }

    pub fn tag_ngram(&self) -> Field {
        self.get_field("tag_ngram")
    }

    pub fn tag_for_language(&self, language: Language) -> Field {
        match language {
            Language::English => self.get_field("tag_en"),
            Language::Japanese => self.get_field("tag_ja"),
        }
    }

    pub fn all_tag_field_names(&self) -> Vec<&str> {
        vec!["tag_en", "tag_ja", "tag_ngram"]
    }

    pub fn created_at(&self) -> Field {
        self.get_field("created_at")
    }

    pub fn modified_at(&self) -> Field {
        self.get_field("modified_at")
    }

    pub fn watch_id(&self) -> Field {
        self.get_field("watch_id")
    }

    fn get_field(&self, field_name: &str) -> Field {
        self.0
            .get_field(field_name)
            .expect(&format!("field {} not found", field_name))
    }
}
