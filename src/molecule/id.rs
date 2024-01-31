#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct EdgeID(pub usize);
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeID(pub usize);

impl EdgeID { pub const fn into_inner(&self) -> usize { self.0 } }
impl NodeID { pub const fn into_inner(&self) -> usize { self.0 } }
