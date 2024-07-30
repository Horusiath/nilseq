use crate::index::FractionalIndex;
use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};

mod index;

pub type PeerId = u32;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct LSeq<K, V> {
    peer: PeerId,
    entries: BTreeMap<K, Option<V>>,
}

impl<K, V> LSeq<K, V>
where
    K: FractionalIndex,
{
    pub fn new(peer: PeerId) -> Self {
        LSeq {
            peer,
            entries: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, index: usize, value: V) {
        let mut i = self.entries.keys();
        let left = if index > 0 { i.nth(index - 1) } else { None };
        let right = i.next();
        let key = K::new(self.peer, left, right);

        self.entries.insert(key, Some(value));
    }

    pub fn insert_range<R>(&mut self, index: usize, range: R)
    where
        R: IntoIterator<Item = V>,
    {
        let mut i = self.entries.keys();
        let mut left = if index > 0 {
            i.nth(index - 1).cloned()
        } else {
            None
        };
        let right = i.next().cloned();
        for value in range {
            let key = K::new(self.peer, left.as_ref(), right.as_ref());
            self.entries.insert(key.clone(), Some(value));
            left = Some(key);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<V> {
        let mut i = self.entries.iter_mut();
        let (_, value) = i.nth(index)?;
        value.take()
    }

    pub fn iter(&self) -> impl Iterator<Item = &V> {
        self.entries.iter().filter_map(|(_, v)| v.as_ref())
    }

    pub fn merge(&mut self, other: Self) {
        for (key, value) in other.entries {
            match self.entries.entry(key) {
                Entry::Vacant(e) => {
                    e.insert(value);
                }
                Entry::Occupied(mut e) if value.is_none() => {
                    e.insert(value);
                }
                _ => { /* ignore */ }
            }
        }
    }
}

impl<F> Display for LSeq<F, char> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for (_, value) in self.entries.iter() {
            if let Some(c) = value {
                write!(f, "{}", c)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::index::FractionalIndex;
    use crate::{LSeq, PeerId};
    use std::fmt::Debug;

    const A: PeerId = 1;
    const B: PeerId = 2;

    fn same_peer_sequential_insert<F: FractionalIndex + Debug>() {
        let mut lseq: LSeq<F, char> = LSeq::new(A);
        lseq.insert_range(0, "hello ".chars());
        lseq.insert_range(6, "world".chars());
        assert_eq!(lseq.to_string(), "hello world");
    }

    fn same_peer_mid_insert<F: FractionalIndex + Debug>() {
        let mut lseq: LSeq<F, char> = LSeq::new(A);
        lseq.insert_range(0, "ace".chars());
        lseq.insert(1, 'b');
        lseq.insert(3, 'd');
        assert_eq!(lseq.to_string(), "abcde");
    }

    fn multi_peer_same_position_insert<F: FractionalIndex + Debug>(expected: &str) {
        let mut a: LSeq<F, char> = LSeq::new(A);
        let mut b: LSeq<F, char> = LSeq::new(B);
        a.insert_range(0, "ae".chars());

        // sync
        b.merge(a.clone());

        a.insert_range(1, "bcd".chars());
        b.insert_range(1, "FGH".chars());

        // sync
        a.merge(b.clone());
        b.merge(a.clone());

        assert_eq!(a.to_string(), expected);
        assert_eq!(b.to_string(), expected);
    }

    fn multi_peer_mixed_index_depth<F: FractionalIndex + Debug>() {
        let mut b: LSeq<F, char> = LSeq::new(B);
        let mut a: LSeq<F, char> = LSeq::new(A);
        b.insert_range(0, "ae".chars());
        b.insert(1, 'd');

        // sync
        a.merge(b.clone());

        b.insert(1, 'C');
        a.insert(1, 'b');

        // sync
        a.merge(b.clone());
        b.merge(a.clone());

        assert_eq!(a.to_string(), "abCde");
        assert_eq!(b.to_string(), "abCde");
    }

    #[test]
    fn naive_same_peer_sequential_insert() {
        same_peer_sequential_insert::<crate::index::NaiveFractionalIndex>();
    }

    #[test]
    fn naive_same_peer_mid_insert() {
        same_peer_mid_insert::<crate::index::NaiveFractionalIndex>();
    }

    #[test]
    fn naive_multi_peer_same_position_insert() {
        multi_peer_same_position_insert::<crate::index::NaiveFractionalIndex>("abFcGdHe");
    }

    #[test]
    fn naive_multi_peer_mixed_index_depth() {
        multi_peer_mixed_index_depth::<crate::index::NaiveFractionalIndex>();
    }

    #[test]
    fn non_interleaving_same_peer_sequential_insert() {
        same_peer_sequential_insert::<crate::index::NonInterleavingIndex>();
    }

    #[test]
    fn non_interleaving_same_peer_mid_insert() {
        same_peer_mid_insert::<crate::index::NonInterleavingIndex>();
    }

    #[test]
    fn non_interleaving_multi_peer_same_position_insert() {
        multi_peer_same_position_insert::<crate::index::NonInterleavingIndex>("abcdFGHe");
    }

    #[test]
    fn non_interleaving_multi_peer_mixed_index_depth() {
        multi_peer_mixed_index_depth::<crate::index::NonInterleavingIndex>();
    }
}
