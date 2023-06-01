use std::{
    collections::HashMap,
    fmt::Formatter,
    fs,
    path::{Component, Path},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use serde::Serialize;
use tantivy::{
    collector::{Collector, Count, Fruit, TopDocs},
    doc,
    query::{AllQuery, Query, QueryParser, RegexQuery, TermQuery},
    schema::*,
    DocAddress, Index, IndexReader, IndexWriter, ReloadPolicy, Searcher, Snippet, SnippetGenerator,
};

use crate::{
    documents::{file::get_file_metadata, markdown, search::schema::AppSchema},
    path_string_normalization::PathStringNormalizationExt,
    WatchId,
};

mod en_tokenizer;
pub mod index_writer;
mod ja_tokenizer;
mod japanese_character_normalizer;
mod ngram_tokenizer;
mod schema;
mod stopwords;

const RESULT_COUNT: usize = 10;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum Language {
    English = 1,
    Japanese = 2,
}

impl From<lingua::Language> for Language {
    fn from(x: lingua::Language) -> Self {
        match x {
            lingua::Language::Japanese => Language::Japanese,
        }
    }
}

impl From<u64> for Language {
    fn from(x: u64) -> Self {
        match x {
            1 => Language::English,
            2 => Language::Japanese,
            _ => panic!("Invalid language value {}", x),
        }
    }
}

#[derive(Debug)]
pub enum Sort {
    Relevance,
    Date,
}

#[derive(Serialize, Debug)]
pub struct SearchResults {
    pub count: usize,
    pub documents: Vec<SearchResultDocument>,
}

#[derive(Serialize, Debug)]
pub struct SearchResultDocument {
    pub path: String,
    pub title: Option<String>,
    pub tags: Vec<String>,
    pub watch_id: i64,
    pub highlight: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "modifiedAt")]
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Clone)]
pub struct Search {
    index: Index,
    index_reader: IndexReader,
}

impl std::fmt::Debug for Search {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Search").field("index", &self.index).finish()
    }
}

impl Search {
    fn new(index: Index) -> Result<(Self, IndexWriter)> {
        let index_reader = index
            .reader_builder()
            .reload_policy(ReloadPolicy::OnCommit)
            .try_into()?;
        let index_writer = index.writer(100_000_000)?;
        Ok((Self { index, index_reader }, index_writer))
    }

    pub fn open_index<P: AsRef<Path>>(index_dir: P) -> Result<(Self, IndexWriter)> {
        let index_dir = index_dir.as_ref();
        let index =
            Index::open_in_dir(index_dir).or_else(|_| Index::create_in_dir(index_dir, schema::define_schema()))?;
        en_tokenizer::setup_tokenizer(&index)?;
        ja_tokenizer::setup_tokenizer(&index)?;
        ngram_tokenizer::setup_tokenizer(&index)?;
        // index.writer(50_000_000)?.commit()?;
        Self::new(index)
    }

    pub(super) fn make_document<P: AsRef<Path>>(&self, path: P, watch_id: WatchId) -> Result<tantivy::Document> {
        let path = path.as_ref();
        let schema = AppSchema::new(self.index.schema());
        let contents = read_file_content(path)?;
        let (frontmatter, body) = match frontmatter::parse(&contents) {
            Ok(frontmatter::ParseResult { frontmatter, body }) => (frontmatter, body),
            Err(e) => {
                tracing::warn!("failed to parse markdown file: {}", path.display());
                return Err(e);
            }
        };
        let file_metadata = get_file_metadata(path)?;
        let title = get_title(&frontmatter, body, path);

        let detector = lingua::LanguageDetectorBuilder::from_all_languages().build();
        let language = detector.detect_language_of(body).map(|x| x.into());
        let language = language.unwrap_or(Language::English);

        let mut document = Document::new();
        let normalized_path = path.to_normalized_path_string();
        document.add_text(schema.path(), &normalized_path);
        for text in path_component_strings(path) {
            document.add_text(schema.path_components(), text);
        }
        document.add_text(schema.path_ngram(), &normalized_path);
        document.add_u64(schema.language(), language as u64);
        document.add_text(schema.title_ngram(), &title);
        document.add_text(schema.title_for_language(language), &title);
        document.add_text(schema.contents_ngram(), &body);
        document.add_text(schema.contents_for_language(language), &body);
        document.add_date(
            schema.created_at(),
            tantivy::DateTime::from_timestamp_secs(file_metadata.created_at.timestamp()),
        );
        document.add_date(
            schema.modified_at(),
            tantivy::DateTime::from_timestamp_secs(file_metadata.modified_at.timestamp()),
        );
        document.add_i64(schema.watch_id(), watch_id.0);

        if let Some(frontmatter) = frontmatter {
            if let Some(tags) = frontmatter.tags {
                for tag in tags {
                    document.add_text(schema.tag_for_language(language), &tag);
                    document.add_text(schema.tag_ngram(), &tag);
                }
            }
        }

        Ok(document)
    }

    pub(super) fn search_document_with_tags(
        &self,
        query: &str,
        tags: &[&str],
        sort: Sort,
        offset: usize,
        limit: usize,
    ) -> Result<SearchResults> {
        let query = query.trim();
        let tag_query = self.make_tag_query(tags)?;
        let mut parts = vec![];
        if !query.is_empty() {
            parts.push(query);
        }
        parts.push(&tag_query);
        let q = parts.join(" AND ");
        self.search_document(&q, sort, offset, limit)
    }

    fn make_tag_query(&self, tags: &[&str]) -> Result<String> {
        let schema = AppSchema::new(self.index.schema());
        let queries: Vec<String> = tags
            .iter()
            .map(|tag| {
                let fields: Vec<String> = schema
                    .all_tag_field_names()
                    .iter()
                    .map(|field| format!(r#"{field}:"{tag}""#, field = field, tag = tag))
                    .collect();
                format!("({})", fields.join(" OR "))
            })
            .collect();
        Ok(queries.join(" AND "))
    }

    pub(super) fn search_document(
        &self,
        query: &str,
        sort: Sort,
        offset: usize,
        limit: usize,
    ) -> Result<SearchResults> {
        let query = self.build_query(query)?;
        match sort {
            Sort::Relevance => self.do_query_and_build_results(query, TopDocs::with_limit(limit).and_offset(offset)),
            Sort::Date => self.do_query_and_build_results(
                query,
                TopDocs::with_limit(limit)
                    .and_offset(offset)
                    .order_by_fast_field::<tantivy::DateTime>(AppSchema::new(self.index.schema()).modified_at()),
            ),
        }
    }

    fn build_query(&self, query: &str) -> Result<Box<dyn Query>> {
        let schema = AppSchema::new(self.index.schema());

        let mut fields = vec![schema.path(), schema.path_components(), schema.path_ngram()];
        fields.extend_from_slice(&schema.contents_all());
        fields.extend_from_slice(&schema.title_all());
        fields.extend_from_slice(&schema.tag_all());
        let mut query_parser = QueryParser::for_index(&self.index, fields);
        query_parser.set_conjunction_by_default();

        const BOOST_LITTLE: f32 = 0.5;
        const BOOST_NORMAL: f32 = 1.0;
        const BOOST_MUCH: f32 = 5.0;
        const BOOST_MORE: f32 = 8.0;
        const BOOST_MOST: f32 = 10.0;

        query_parser.set_field_boost(schema.path(), BOOST_MOST);
        query_parser.set_field_boost(schema.path_components(), BOOST_MORE);
        query_parser.set_field_boost(schema.path_ngram(), BOOST_LITTLE);
        query_parser.set_field_boost(schema.path_components(), BOOST_MOST);
        for field in schema.title_all_languages() {
            query_parser.set_field_boost(field, BOOST_MOST);
        }
        query_parser.set_field_boost(schema.title_ngram(), BOOST_MUCH);
        for field in schema.contents_all_languages() {
            query_parser.set_field_boost(field, BOOST_NORMAL);
        }
        query_parser.set_field_boost(schema.contents_ngram(), BOOST_LITTLE);
        for field in schema.tag_all_languages() {
            query_parser.set_field_boost(field, BOOST_MORE);
        }
        query_parser.set_field_boost(schema.tag_ngram(), BOOST_MUCH);
        // Ok(query_parser.parse_query(&escape_query(query))?)
        Ok(query_parser.parse_query(query)?)
    }

    fn do_query_and_build_results<C, F, K>(&self, query: Box<dyn Query>, doc_collector: C) -> Result<SearchResults>
    where
        C: Collector<Fruit = F>,
        F: Fruit + IntoIterator<Item = (K, DocAddress)>,
    {
        let schema = AppSchema::new(self.index.schema());

        let searcher = self.index_reader.searcher();
        let (count, top_docs) = searcher.search(&query, &(Count, doc_collector))?;

        let mut snippet_generator_map: HashMap<Language, SnippetGenerator> = HashMap::new();

        let snippet_generator_ngram = SnippetGenerator::create(&searcher, &*query, schema.contents_ngram())?;

        let mut documents = Vec::new();

        for (_, doc_address) in top_docs {
            let doc = searcher.doc(doc_address)?;
            let language: Language = doc
                .get_first(schema.language())
                .map(|x| x.as_u64().unwrap().into())
                .unwrap_or(Language::English);

            let snippet = get_snippet(
                &schema,
                &searcher,
                &*query,
                &mut snippet_generator_map,
                &snippet_generator_ngram,
                &doc,
                language,
            );
            let document = populate_document(&schema, Some(snippet.to_html()), &doc)?;
            documents.push(document);
        }

        Ok(SearchResults { count, documents })
    }

    pub(super) fn get_all_documents(&self, offset: usize, limit: usize) -> Result<SearchResults> {
        let schema = AppSchema::new(self.index.schema());
        let modified_at_field = schema.modified_at();
        self.do_query_and_build_results(
            Box::new(AllQuery),
            TopDocs::with_limit(limit)
                .and_offset(offset)
                .order_by_fast_field::<tantivy::DateTime>(modified_at_field),
        )
    }

    pub(super) fn count_documents_under_path<P: AsRef<Path>>(&self, path: P) -> Result<u32> {
        let path = path.as_ref();
        let schema = AppSchema::new(self.index.schema());
        let path_field = schema.path();
        let searcher = self.index_reader.searcher();
        let pattern = format!(r"{}/.*", path.to_normalized_path_string());
        let query = RegexQuery::from_pattern(&pattern, path_field)?;
        let count = searcher.search(&query, &Count)? as u32;
        Ok(count)
    }

    pub(super) fn count_documents_by_path<P: AsRef<Path>>(&self, path: P) -> Result<u32> {
        let path = path.as_ref();
        let schema = AppSchema::new(self.index.schema());
        let path_field = schema.path();
        let searcher = self.index_reader.searcher();
        let query = TermQuery::new(
            Term::from_field_text(path_field, &path.to_normalized_path_string()),
            IndexRecordOption::Basic,
        );
        let count = searcher.search(&query, &Count)? as u32;
        Ok(count)
    }
}

fn populate_document(schema: &AppSchema, highlight: Option<String>, doc: &Document) -> Result<SearchResultDocument> {
    let path = doc.get_first(schema.path()).unwrap().as_text().unwrap().to_string();
    let watch_id = doc.get_first(schema.watch_id()).unwrap().as_i64().unwrap();
    let title = doc
        .get_first(schema.title_ngram())
        .map(|x| x.as_text().unwrap().to_string());
    let tag = doc
        .get_all(schema.tag_ngram())
        .map(|x| x.as_text().unwrap().to_string())
        .collect::<Vec<_>>();
    let created_at = doc.get_first(schema.created_at()).unwrap().as_date().unwrap();
    let created_at = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(created_at.into_timestamp_secs(), 0)
            .ok_or(anyhow::anyhow!("timestamp is out of range"))?,
        chrono::Utc,
    );
    let modified_at = doc.get_first(schema.modified_at()).unwrap().as_date().unwrap();
    let modified_at = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp_opt(modified_at.into_timestamp_secs(), 0)
            .ok_or(anyhow::anyhow!("timestamp is out of range"))?,
        chrono::Utc,
    );

    Ok(SearchResultDocument {
        path,
        title,
        tags: tag,
        watch_id,
        highlight,
        created_at,
        modified_at,
    })
}

fn read_file_content(path: &Path) -> Result<String> {
    Ok(fs::read_to_string(path)?)
}

fn get_title(frontmatter: &Option<frontmatter::Frontmatter>, body: &str, path: &Path) -> String {
    fn get_title_from_frontmatter(frontmatter: &Option<frontmatter::Frontmatter>) -> Option<String> {
        frontmatter.as_ref().and_then(|x| x.title).map(|x| x.to_string())
    }

    fn get_title_from_body(body: &str) -> Option<String> {
        markdown::infer_title(body).ok().flatten()
    }

    fn get_title_from_path(path: &Path) -> String {
        path.file_stem()
            .expect("if path ends with ..")
            .to_normalized_path_string()
    }

    get_title_from_frontmatter(frontmatter)
        .or_else(|| get_title_from_body(body))
        .unwrap_or_else(|| get_title_from_path(path))
}

fn get_snippet(
    schema: &AppSchema,
    searcher: &Searcher,
    query: &dyn Query,
    snippet_generator_map: &mut HashMap<Language, SnippetGenerator>,
    snippet_generator_ngram: &SnippetGenerator,
    doc: &Document,
    language: Language,
) -> Snippet {
    let snippet_generator_language = snippet_generator_map.entry(language).or_insert_with(|| {
        SnippetGenerator::create(&searcher, &*query, schema.contents_for_language(language)).unwrap()
    });

    let snippet_language = snippet_generator_language.snippet_from_doc(&doc);
    if snippet_language.is_empty() {
        snippet_generator_ngram.snippet_from_doc(&doc)
    } else {
        snippet_language
    }
}

const SPECIAL_CHARACTERS: &str = r#"+^`:{}"[]()~!\*"#;

/// Escape special characters
/// https://quickwit.io/docs/reference/query-language/#escaping-special-characters
fn escape_query(query: &str) -> String {
    query
        .trim()
        .chars()
        .map(|c| {
            if SPECIAL_CHARACTERS.contains(c) {
                format!("\\{}", c)
            } else {
                c.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("")
}

fn path_component_strings<P: AsRef<Path>>(path: P) -> Vec<String> {
    let mut results = vec![];
    for c in path.as_ref().components() {
        if let Component::Normal(x) = c {
            results.push(x.to_normalized_path_string());
        }
    }
    results
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_escape_query() {
        assert_eq!(super::escape_query("a+b"), "a\\+b");
        assert_eq!(super::escape_query("a^b"), "a\\^b");
        assert_eq!(super::escape_query("a:b"), "a\\:b");
        assert_eq!(super::escape_query("a`b"), "a\\`b");
        assert_eq!(super::escape_query("a{b"), "a\\{b");
        assert_eq!(super::escape_query("a}b"), "a\\}b");
        assert_eq!(super::escape_query("a\"b"), "a\\\"b");
        assert_eq!(super::escape_query("a[b"), "a\\[b");
        assert_eq!(super::escape_query("a]b"), "a\\]b");
        assert_eq!(super::escape_query("a(b"), "a\\(b");
        assert_eq!(super::escape_query("a)b"), "a\\)b");
        assert_eq!(super::escape_query("a~b"), "a\\~b");
        assert_eq!(super::escape_query("a!b"), "a\\!b");
        assert_eq!(super::escape_query("a*b"), "a\\*b");
        assert_eq!(super::escape_query("a\\b"), "a\\\\b");
    }
}
