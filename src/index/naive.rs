use crate::index::FractionalIndex;
use crate::PeerId;
use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct NaiveFractionalIndex {
    sequence: Vec<u8>,
    peer_id: PeerId,
}

impl FractionalIndex for NaiveFractionalIndex {
    fn new(peer_id: PeerId, left: Option<&Self>, right: Option<&Self>) -> Self {
        let lo = left.map(|l| l.sequence.as_slice()).unwrap_or(&[0]);
        let hi = right.map(|r| r.sequence.as_slice()).unwrap_or(&[255]);
        let mut sequence = Vec::new();
        let mut i = 0;
        let mut diffed = false;
        while i < lo.len() && i < hi.len() {
            let lo_byte = lo[i];
            let hi_byte = hi[i];
            if hi_byte > lo_byte + 1 {
                sequence.push(lo_byte + 1);
                diffed = true;
                break;
            } else {
                sequence.push(lo_byte);
            }
            i += 1;
        }
        while !diffed {
            let lo_byte = lo.get(i).copied().unwrap_or(0x01);
            let hi_byte = hi.get(i).copied().unwrap_or(0xff);
            if hi_byte > lo_byte + 1 {
                sequence.push(lo_byte + 1);
                diffed = true;
            } else {
                sequence.push(lo_byte);
            }
            i += 1;
        }
        Self { sequence, peer_id }
    }
}

impl Debug for NaiveFractionalIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut i = self.sequence.iter();
        if let Some(byte) = i.next() {
            write!(f, "{:02x}", byte)?;
        }
        while let Some(byte) = i.next() {
            write!(f, ".{:02x}", byte)?;
        }
        write!(f, "@{}", self.peer_id)
    }
}

impl Display for NaiveFractionalIndex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
