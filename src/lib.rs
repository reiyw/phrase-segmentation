#![allow(dead_code)]

mod document;

use std::sync::Mutex;

use rayon::prelude::*;

pub use crate::document::IndexedDocument;

pub fn collect_phrases<'a>(
    document_set: &'a [(&'a IndexedDocument, Vec<&'a IndexedDocument>)],
    min_phrase_len: usize,
    max_phrase_len: usize,
) -> Vec<Vec<(usize, usize)>> {
    let phrases = Mutex::new(Vec::new());
    document_set
        .par_iter()
        .for_each(|(document, relevant_documents)| {
            let doc_phrases = collect_phrases_per_document(
                document,
                relevant_documents,
                min_phrase_len,
                max_phrase_len,
            );
            let mut lock = phrases.lock().unwrap();
            lock.push(doc_phrases);
        });
    phrases.into_inner().unwrap()
}

fn collect_phrases_per_document<'a>(
    document: &'a IndexedDocument,
    relevant_documents: &'a Vec<&'a IndexedDocument>,
    min_phrase_len: usize,
    max_phrase_len: usize,
) -> Vec<(usize, usize)> {
    let mut phrases = Vec::new();
    let mut start = 0;
    while start < document.len() {
        let mut query_len = min_phrase_len;
        if start + query_len > document.len() {
            break;
        }

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
            phrases.push((start, start + query_len - 1));
            start += query_len - 1;
        } else {
            start += 1;
        }
    }
    phrases
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_collect_phrases() {
        let doc1 = IndexedDocument::from_tokens(vec![0, 1, 2, 3]);
        let doc1_relevant_docs = vec![
            IndexedDocument::from_tokens(vec![0, 0, 1, 1]),
            IndexedDocument::from_tokens(vec![0, 2, 3, 3]),
        ];
        let doc2 = IndexedDocument::from_tokens(vec![4, 5, 6, 7]);
        let doc2_relevant_docs = vec![
            IndexedDocument::from_tokens(vec![5, 6, 7, 8]),
            IndexedDocument::from_tokens(vec![4, 5, 6, 7]),
        ];
        let document_set = vec![
            (&doc1, doc1_relevant_docs.iter().collect::<Vec<_>>()),
            (&doc2, doc2_relevant_docs.iter().collect::<Vec<_>>()),
        ];
        let phrases = collect_phrases(document_set.as_slice(), 2, 100);
        assert_eq!(phrases.len(), 2);
        assert_eq!(phrases[0], vec![(0, 2), (2, 4)]);
        assert_eq!(phrases[1], vec![(0, 4)]);
    }
}
