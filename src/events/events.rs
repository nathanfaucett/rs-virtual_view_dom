use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;

use stdweb::{Reference, Value};
use stdweb::web::{Document, Node, EventTarget};
use stdweb::unstable::TryInto;

use serde_json;
use virtual_view::EventManager;

use super::{NodeRef, DOMEvent};


pub struct Events {
    listening: HashMap<String, usize>,
    node_to_ids: Arc<Mutex<HashMap<NodeRef, String>>>,
    event_manager: Arc<Mutex<EventManager>>,
}

impl Events {

    #[inline(always)]
    pub fn new(event_manager: Arc<Mutex<EventManager>>) -> Self {
        Events {
            listening:  HashMap::new(),
            node_to_ids: Arc::new(Mutex::new(HashMap::new())),
            event_manager: event_manager,
        }
    }

    #[inline]
    fn node_to_ids(&self) -> MutexGuard<HashMap<NodeRef, String>> {
        self.node_to_ids.lock().expect("failed to acquire node to ids lock")
    }

    #[inline]
    pub fn listen(&mut self, name: &str, id: &str, node: &Node, document: &Document) {

        self.node_to_ids().insert(node.as_ref().into(), id.into());

        if !self.listening.contains_key(name) {
            self.listening.insert(name.into(), 1);
            self.add_event_listener(name, id, node, document);
        } else {
            self.listening.get_mut(name).map(|count| { *count += 1 });
        }
    }
    #[inline]
    pub fn unlisten(&mut self, name: &str, id: &str, node: &Node, document: &Document) {
        let count = if let Some(count) = self.listening.get_mut(name) {
            *count -= 1;
            Some(*count)
        } else {
            None
        };

        let node_ref: NodeRef = node.as_ref().into();
        self.node_to_ids().remove(&node_ref);

        if count == Some(0) {
            self.listening.remove(name);
            self.remove_event_listener(name, id, node, document);
        }
    }

    #[inline]
    fn handle(node_to_ids: &Arc<Mutex<HashMap<NodeRef, String>>>, event_manager: &Arc<Mutex<EventManager>>, event: Reference) {
        let option: Option<EventTarget> = js! {
            return @{event.as_ref()}.target;
        }.try_into().ok();

        if let Some(target) = option {
            let name: String = js! {
                return @{event.as_ref()}.type;
            }.try_into().unwrap();

            let node_ref: NodeRef = target.as_ref().into();
            let node_to_ids = node_to_ids.lock().expect("failed to acquire node to ids lock");

            if let Some(id) = node_to_ids.get(&node_ref) {
                let map: HashMap<String, Value> = js! {
                    var event = @{event.as_ref()},
                        map = {},
                        key, value, type;

                    for (key in event) {
                        value = event[key];
                        type = typeof(value);

                        if (value != null && type !== "function") {
                            map[key] = value;
                        }
                    }

                    return map;
                }.try_into().unwrap();
                
                let mut data = serde_json::Map::new();
                for (k, v) in map {
                    data.insert(k, serde_json::to_value(v).unwrap());
                }

                let event_manager = event_manager.lock().expect("failed to acquire event manager lock");
                let mut event = DOMEvent::new(name, data);
                event_manager.dispatch(id, &mut event);
            }
        }
    }

    #[inline]
    fn add_event_listener(&mut self, name: &str, _id: &str, _node: &Node, document: &Document) {
        let node_to_ids = self.node_to_ids.clone();
        let event_manager = self.event_manager.clone();
        let listener = move |e| Self::handle(&node_to_ids, &event_manager, e);
        let _listener_reference: Reference = js! {
            var type = @{name},
                document = @{document},
                listener = @{listener};

            document.addEventListener(type, listener);
            return listener;
        }.try_into().unwrap();
    }
    #[inline]
    fn remove_event_listener(&mut self, _name: &str, _id: &str, _node: &Node, _document: &Document) {

    }
}
