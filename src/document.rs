#![allow(clippy::future_not_send)]

use ouroboros::self_referencing;
use suffix_array::SuffixArray;

#[self_referencing]
pub struct Document {
    tokens: Vec<u16>,
    #[borrows(tokens)]
    #[covariant]
    sa: SuffixArray<'this>,
}

impl Document {
    pub fn from_tokens(tokens: Vec<u16>) -> Self {
        DocumentBuilder {
            tokens,
            sa_builder: |tokens: &Vec<u16>| unsafe { SuffixArray::new(tokens.align_to::<u8>().1) },
        }
        .build()
    }

    /// Tests if it contains the given pattern.
    pub fn contains(&self, pat: &[u16]) -> bool {
        let pat_u8 = unsafe { pat.align_to::<u8>().1 };
        self.borrow_sa().contains(pat_u8)
    }

    pub fn get_slice(&self, start: usize, end: usize) -> &[u16] {
        &self.borrow_tokens()[start..end]
    }

    pub fn len(&self) -> usize {
        self.borrow_tokens().len()
    }

    pub fn is_empty(&self) -> bool {
        self.borrow_tokens().is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_contains() {
        let doc = Document::from_tokens(vec![0, 255, 256, 65535]);

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
        let doc = Document::from_tokens(vec![0, 1, 2]);
        assert_eq!(doc.get_slice(0, 1), &[0]);
        assert_eq!(doc.get_slice(0, 2), &[0, 1]);
        assert_eq!(doc.get_slice(0, 3), &[0, 1, 2]);
        assert_eq!(doc.get_slice(1, 2), &[1]);
        assert_eq!(doc.get_slice(1, 3), &[1, 2]);
        assert_eq!(doc.get_slice(2, 3), &[2]);
    }
}
