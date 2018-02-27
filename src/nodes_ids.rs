use std::rc::Rc;
use std::cell::RefCell;

use fnv::FnvHashMap;
use stdweb::web::Node;

use super::NodeRef;

pub struct NodesIdsInner {
    nodes: FnvHashMap<String, Node>,
    ids: FnvHashMap<NodeRef, String>,
}

impl NodesIdsInner {
    #[inline]
    pub fn new() -> Self {
        NodesIdsInner {
            nodes: FnvHashMap::default(),
            ids: FnvHashMap::default(),
        }
    }

    #[inline]
    pub fn insert(&mut self, id: String, node: Node) {
        let node_ref: NodeRef = node.as_ref().into();
        self.nodes.insert(id.clone(), node);
        self.ids.insert(node_ref, id);
    }

    #[inline]
    pub fn remove_node(&mut self, node: &Node) -> Option<String> {
        let node_ref: NodeRef = node.as_ref().into();
        let id_option = self.ids.remove(&node_ref);

        if let Some(id) = id_option {
            self.nodes.remove(&id);
            Some(id)
        } else {
            None
        }
    }
    #[inline]
    pub fn remove_id(&mut self, id: &str) -> Option<Node> {
        let node_option = self.nodes.remove(id);

        if let Some(node) = node_option {
            let node_ref: NodeRef = node.as_ref().into();
            self.ids.remove(&node_ref);
            Some(node)
        } else {
            None
        }
    }

    #[inline]
    pub fn node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }
    #[inline]
    pub fn id(&self, node: &Node) -> Option<&String> {
        let node_ref: NodeRef = node.as_ref().into();
        self.ids.get(&node_ref)
    }
}

#[derive(Clone)]
pub struct NodesIds(Rc<RefCell<NodesIdsInner>>);

impl NodesIds {
    #[inline]
    pub fn new() -> Self {
        NodesIds(Rc::new(RefCell::new(NodesIdsInner::new())))
    }

    #[inline]
    pub fn insert(&self, id: String, node: Node) {
        self.0.borrow_mut().insert(id, node);
    }

    #[inline]
    pub fn remove_node(&mut self, node: &Node) -> Option<String> {
        self.0.borrow_mut().remove_node(node)
    }
    #[inline]
    pub fn remove_id(&mut self, id: &str) -> Option<Node> {
        self.0.borrow_mut().remove_id(id)
    }

    #[inline]
    pub fn node(&self, id: &str) -> Option<Node> {
        self.0.borrow().node(id).map(Clone::clone)
    }
    #[inline]
    pub fn id(&self, node: &Node) -> Option<String> {
        self.0.borrow().id(node).map(Clone::clone)
    }
}
