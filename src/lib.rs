#![allow(dead_code)]

use suffix_array::SuffixArray;

struct Document<'a> {
    tokens: &'a [u16],
    sa: SuffixArray<'a>,
}

impl<'a> Document<'a> {
    pub fn new(tokens: &'a [u16]) -> Self {
        let sa = unsafe { SuffixArray::new(tokens.align_to::<u8>().1) };
        Self { tokens, sa }
    }

    /// Tests if it contains the given pattern.
    pub fn contains(&self, pat: &[u16]) -> bool {
        let pat_u8 = unsafe { pat.align_to::<u8>().1 };
        self.sa.contains(pat_u8)
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &[u16] {
        &self.tokens[start..end]
    }
}

pub fn collect_phrases<'a, I: Iterator<Item = (Document<'a>, [Document<'a>])>>(document_set: I) {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let tokens = [0, 255, 256, 65535];
        let doc = Document::new(&tokens);

        let pat = [0];
        assert!(doc.contains(&pat));
        let pat = [255];
        assert!(doc.contains(&pat));
        let pat = [256];
        assert!(doc.contains(&pat));
        let pat = [65535];
        assert!(doc.contains(&pat));

        let pat = [1];
        assert!(!doc.contains(&pat));
        let pat = [65534];
        assert!(!doc.contains(&pat));

        let pat = [0, 255];
        assert!(doc.contains(&pat));
        let pat = [0, 255, 256];
        assert!(doc.contains(&pat));
        let pat = [0, 255, 256, 65535];
        assert!(doc.contains(&pat));

        let pat = [0, 256];
        assert!(!doc.contains(&pat));
        let pat = [0, 65535];
        assert!(!doc.contains(&pat));
        let pat = [255, 65535];
        assert!(!doc.contains(&pat));
    }

    #[test]
    fn test_get_slice() {
        let tokens = [0, 1, 2];
        let doc = Document::new(&tokens);
        assert_eq!(doc.get_slice(0, 1), &[0]);
        assert_eq!(doc.get_slice(0, 2), &[0, 1]);
        assert_eq!(doc.get_slice(0, 3), &[0, 1, 2]);
        assert_eq!(doc.get_slice(1, 2), &[1]);
        assert_eq!(doc.get_slice(1, 3), &[1, 2]);
        assert_eq!(doc.get_slice(2, 3), &[2]);
    }

    #[test]
    fn test_collect_phrases() {
        let tokens = [0, 1, 2];
        let doc = Document::new(&tokens);
    }
}
