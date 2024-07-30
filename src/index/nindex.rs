use crate::index::FractionalIndex;
use crate::PeerId;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct NonInterleavingIndex(Vec<Segment>);

impl FractionalIndex for NonInterleavingIndex {
    fn new(peer_id: PeerId, left: Option<&Self>, right: Option<&Self>) -> Self {
        let lo = left.map(|l| l.0.as_slice()).unwrap_or(&[Segment::MIN]);
        let hi = right.map(|r| r.0.as_slice()).unwrap_or(&[Segment::MAX]);
        let mut sequence = Vec::new();
        let mut i = 0;
        let mut diffed = false;
        while i < lo.len() && i < hi.len() {
            let l = lo[i];
            let r = hi[i];
            let mut n = Segment::new(peer_id, l.seq_no + 1);
            if r > n {
                if n.peer != peer_id {
                    sequence.push(l);
                } else {
                    sequence.push(n);
                    diffed = true;
                }
                break;
            } else {
                sequence.push(l);
            }
            i += 1;
        }
        while !diffed {
            let l = lo.get(i).unwrap_or(&Segment::MIN);
            let r = hi.get(i).unwrap_or(&Segment::MAX);
            let n = Segment::new(peer_id, l.seq_no + 1);
            if r > &n {
                sequence.push(n);
                diffed = true;
            } else {
                sequence.push(*l);
            }
            i += 1;
        }
        Self(sequence)
    }
}

impl Debug for NonInterleavingIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = self.0.iter();
        if let Some(segment) = i.next() {
            write!(f, "{}", segment)?;
        }
        while let Some(segment) = i.next() {
            write!(f, ".{}", segment)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Ord, PartialOrd, Hash, Default)]
struct Segment {
    peer: PeerId,
    seq_no: u32,
}

impl Segment {
    pub const MIN: Self = Self { peer: 0, seq_no: 0 };

    pub const MAX: Self = Self {
        peer: u32::MAX,
        seq_no: u32::MAX,
    };

    pub fn new(peer: PeerId, seq_no: u32) -> Self {
        Self { peer, seq_no }
    }
}

impl Display for Segment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.peer, self.seq_no)
    }
}
