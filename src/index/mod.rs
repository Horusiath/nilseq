mod naive;
mod nindex;

use crate::PeerId;
use std::fmt::Debug;

pub use naive::NaiveFractionalIndex;
pub use nindex::NonInterleavingIndex;

pub trait FractionalIndex: Ord + Clone + Debug {
    fn new(peer_id: PeerId, left: Option<&Self>, right: Option<&Self>) -> Self;
}
