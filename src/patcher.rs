use alloc::btree_map::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use serde_json::Value;
use stdweb::web::{Document, INode, Node};
use virt::{View, Transaction, Patch, get_view_id};

use super::utils::ToHtmlString;


pub struct Patcher {
    root: Node,
    document: Document,
    nodes: BTreeMap<String, Node>,
}

impl Patcher {

    #[inline(always)]
    pub fn new(root: Node, document: Document) -> Self {
        Patcher {
            root: root,
            document: document,
            nodes: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn patch(&mut self, transaction: &Transaction) {
        for (id, patches) in transaction.patches() {
            let node = self.nodes.get(id).map(|n| n.clone());

            for patch in patches {
                self.apply_patch(id, node.as_ref(), patch);
            }
        }
        for (id, view) in transaction.removes() {
            if let Some(node) = self.nodes.get(id) {
                let parent = node.parent_node().expect("node has no parent");
                let _ = parent.remove_child(node);
            }
            self.remove_child_nodes_id(id, &view);
        }
    }

    #[inline]
    fn apply_patch(&mut self, id: &String, node: Option<&Node>, patch: &Patch) {
        match patch {
            &Patch::Mount(ref view) => {
                let new_node = self.create_node(id, view);
                self.root.append_child(&new_node);
            },
            &Patch::Insert(ref child_id, _index, ref view) => {
                let new_node = self.create_node(child_id, view);
                node.unwrap().append_child(&new_node);
            },
            &Patch::Replace(ref _prev_view, ref next_view) => {
                let new_node = self.create_node(id, next_view);
                let old_node = node.expect("node is not any tree");
                let parent = old_node.parent_node().expect("node has no parent");
                parent.replace_child(&new_node, old_node);
            },
            &Patch::Order(ref order) => {
                let parent_node = node.unwrap();
                let child_nodes: Vec<_> = parent_node.child_nodes().iter().collect();
                let mut key_map = BTreeMap::new();

                for &(index, ref key) in order.removes() {
                    let child_node = &child_nodes[index];

                    if let &Some(ref key) = key {
                        key_map.insert(key.clone(), child_node);
                    }
                    let _ = parent_node.remove_child(child_node);
                }

                let mut len = child_nodes.len();
                for &(ref key, index) in order.inserts() {
                    if let &Some(ref key) = key {
                        let node = key_map[key];

                        if index >= len {
                            parent_node.append_child(node);
                        } else {
                            parent_node.insert_before(node, &child_nodes[index]);
                        }
                        len += 1;
                    }
                }
            },
            &Patch::Props(ref prev_props, ref diff_props) => {
                let node = node.unwrap();

                match diff_props {
                    &Value::Object(ref map) => {
                        for (key, value) in map {
                            if value.is_null() {
                                Self::remove_prop(node, key, prev_props);
                            } else if let &Value::Object(_) = value {
                                Self::set_props(node, key, value);
                            } else {
                                let value = value_to_string(value);
                                js! {
                                    var node = @{node};
                                    node.setAttribute(@{key}, @{value});
                                }
                            }
                        }
                    },
                    _ => {},
                }
            },
        }
    }

    fn remove_prop(node: &Node, key: &String, prev_props: &Value) {
        if let &Value::Object(ref prev_props) = prev_props {
            let prev_prop = &prev_props[key];

            if key == "attributes" {
                if let &Value::Object(ref map) = prev_prop {
                    for (attr_key, _) in map {
                        js! {
                            var node = @{node}, attr_key = @{attr_key};
                            node.removeAttribute(attr_key);
                        }
                    }
                }
            } else if key == "style" {
                if let &Value::Object(ref map) = prev_prop {
                    for (attr_key, _) in map {
                        js! {
                            var node = @{node}, attr_key = @{attr_key};
                            node.style[attr_key] = ""
                        }
                    }
                }
            } else if prev_prop.is_string() {
                js! {
                    var node = @{node}, key = @{key};
                    node[key] = "";
                }
            } else {
                js! {
                    var node = @{node}, key = @{key};
                    node[key] = null;
                }
            }
        }
    }
    fn set_props(node: &Node, key: &String, value: &Value) {
        if key == "attributes" {
            if let &Value::Object(ref map) = value {
                for (attr_key, attr_value) in map {
                    let value = value_to_string(attr_value);
                    js! {
                        var node = @{node}, attr_key = @{attr_key}, attr_value = @{value};
                        node.addAttribute(attr_key, attr_value);
                    }
                }
            }
        } else if key == "style" {
            if let &Value::Object(ref map) = value {
                for (attr_key, attr_value) in map {
                    let value = value_to_string(attr_value);
                    js! {
                        var node = @{node}, attr_key = @{attr_key}, attr_value = @{value};
                        node.style[attr_key] = attr_value;
                    }
                }
            }
        } else {
            let value = value_to_string(value);
            js! {
                var node = @{node}, key = @{key}, value = @{value};
                node[key] = value;
            }
        }
    }

    #[inline]
    fn create_node(&mut self, id: &String, view: &View) -> Node {
        match view {
            &View::Text(ref text) => {
                let node: Node = self.document.create_element("span").into();
                node.set_text_content(text);
                self.nodes.insert(id.clone(), node.clone());
                node
            },
            &View::Data { .. } => {
                let tmp = self.document.create_element("div");

                let result = js!{
                    var tmp = @{tmp};
                    tmp.innerHTML = @{view.to_html_string()};
                    return tmp.childNodes[0];
                };

                let node = result.into_reference().unwrap().downcast::<Node>().unwrap();
                self.set_child_nodes_id(&node, id, view);
                node
            }
        }
    }

    #[inline]
    fn set_child_nodes_id(&mut self, node: &Node, id: &String, view: &View) {

        self.nodes.insert(id.clone(), node.clone());

        match view {
            &View::Data { ref children, .. } => {
                let mut index = 0;

                for child_node in node.child_nodes().iter() {
                    let child = &children[index];
                    let child_id = get_view_id(id, child, index);
                    self.set_child_nodes_id(&child_node, &child_id, child);
                    index += 1;
                }
            },
            _ => {},
        }
    }

    #[inline]
    fn remove_child_nodes_id(&mut self, id: &String, view: &View) {
        if let Some(node) = self.nodes.remove(id) {
            match view {
                &View::Data { ref children, .. } => {
                    let mut index = 0;

                    for _child_node in node.child_nodes().iter() {
                        let child = &children[index];
                        let child_id = get_view_id(id, child, index);
                        self.remove_child_nodes_id(&child_id, child);
                        index += 1;
                    }
                },
                _ => {},
            }
        }
    }
}

fn value_to_string(value: &Value) -> String {
    if let &Value::String(ref string) = value {
        string.clone()
    } else {
        value.to_string()
    }
}
