use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use std::fs;
use std::path::Path;

pub struct VectorDatabase {
    index: Index,
    chat_field: Field,
}

impl VectorDatabase {
    pub fn connect() -> tantivy::Result<Self> {
        let mut schema_builder = SchemaBuilder::default();
        let mut chat_field = schema_builder.add_text_field("chat", TEXT | STORED);
        let schema = schema_builder.build();
        if !Path::new("companion_vector").exists() {
            fs::create_dir("companion_vector")?;
        }
        let companion_vector = match Index::open_in_dir("companion_vector") {
            Ok(index) => index,
            Err(_) => Index::create_in_dir("companion_vector", schema.clone())?,
        };
        Ok(VectorDatabase {
            index: companion_vector,
            chat_field: chat_field,
        })
    }

    pub fn add_entry(&self, text: &str) {
        let mut writer = self.index.writer(50_000_000).unwrap();
        writer.add_document(tantivy::doc!(
            self.chat_field => text
        ));
        writer.commit().unwrap();
    }

    pub fn get_matches(&self, query_string: &str) -> Vec<String> {
        let reader = self.index.reader().unwrap();
        let searcher = reader.searcher();
        let qp = QueryParser::for_index(&self.index, vec![self.chat_field]);
        let query = qp.parse_query(query_string).unwrap();
        let matches: Vec<(f32, tantivy::DocAddress)> = searcher.search(&query, &TopDocs::with_limit(2)).unwrap();
        let mut result: Vec<String> = Vec::new();
        for (_, text_addr) in matches {
            let retrieved = searcher.doc(text_addr).unwrap();
            let r = retrieved.get_first(self.chat_field).and_then(|val| val.as_text()).unwrap_or_else(|| "");
            result.push(r.to_string());
        }
        result
    }

    pub fn erase_memory(&self) {
        let mut writer = self.index.writer(50_000_000).unwrap();
        writer.delete_all_documents().unwrap();
        writer.commit().unwrap();
    }
}
