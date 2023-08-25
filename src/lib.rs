#![allow(dead_code)]

mod document;

use std::collections::BTreeSet;

pub use crate::document::Document;

pub fn collect_phrases<'a, I: Iterator<Item = (&'a Document, &'a [Document])>>(
    document_set: I,
    min_phrase_len: usize,
    max_phrase_len: usize,
) -> BTreeSet<&'a [u16]> {
    let mut phrases = BTreeSet::new();
    for (document, relevant_documents) in document_set {
        let mut start = 0;
        while start < document.len() {
            let mut query_len = min_phrase_len;
            let mut query = document.get_slice(start, start + query_len);
            let mut found = false;
            'outer: for relevant_document in relevant_documents {
                while relevant_document.contains(query) {
                    found = true;

                    query_len += 1;
                    if query_len > max_phrase_len || start + query_len > document.len() {
                        break 'outer;
                    }

                    query = document.get_slice(start, start + query_len);
                }
            }

            if found {
                phrases.insert(document.get_slice(start, start + query_len - 1));
                start += query_len - 1;
            } else {
                start += 1;
            }
        }
    }
    phrases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect_phrases() {
        let doc1 = Document::from_tokens(vec![0, 1, 2, 3]);
        let doc1_relevant_docs = vec![
            Document::from_tokens(vec![0, 0, 1, 1]),
            Document::from_tokens(vec![0, 2, 3, 3]),
        ];
        let doc2 = Document::from_tokens(vec![4, 5, 6, 7]);
        let doc2_relevant_docs = vec![
            Document::from_tokens(vec![5, 6, 7, 8]),
            Document::from_tokens(vec![4, 5, 6, 7]),
        ];
        let document_set = vec![
            (&doc1, doc1_relevant_docs.as_slice()),
            (&doc2, doc2_relevant_docs.as_slice()),
        ];
        let mut phrases = collect_phrases(document_set.into_iter(), 2, 100);
        assert_eq!(phrases.pop_first().unwrap(), &[0, 1]);
        assert_eq!(phrases.pop_first().unwrap(), &[2, 3]);
        assert_eq!(phrases.pop_first().unwrap(), &[4, 5, 6, 7]);
        assert!(phrases.is_empty());
    }
}
