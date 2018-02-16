use fnv::FnvHashMap;
use stdweb::web::Node;

use super::NodeRef;

pub struct NodesIds {
    nodes: FnvHashMap<String, Node>,
    ids: FnvHashMap<NodeRef, String>,
}

impl NodesIds {
    #[inline]
    pub fn new() -> Self {
        NodesIds {
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
    pub fn get_node(&self, id: &str) -> Option<&Node> {
        self.nodes.get(id)
    }
    #[inline]
    pub fn get_id(&self, node: &Node) -> Option<&String> {
        let node_ref: NodeRef = node.as_ref().into();
        self.ids.get(&node_ref)
    }
}
