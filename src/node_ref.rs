use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

use stdweb::Reference;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeRef(Reference);

impl NodeRef {
    #[inline]
    pub fn as_raw(&self) -> i32 {
        self.0.as_raw()
    }
}

impl<'a> From<&'a Reference> for NodeRef {
    #[inline]
    fn from(reference: &'a Reference) -> Self {
        NodeRef(reference.clone())
    }
}

impl PartialOrd for NodeRef {
    #[inline]
    fn partial_cmp(&self, other: &NodeRef) -> Option<Ordering> {
        self.as_raw().partial_cmp(&other.as_raw())
    }
}
impl Ord for NodeRef {
    #[inline]
    fn cmp(&self, other: &NodeRef) -> Ordering {
        self.as_raw().cmp(&other.as_raw())
    }
}

impl Hash for NodeRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_raw().hash(state);
    }
}
