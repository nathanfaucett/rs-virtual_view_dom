use std::sync::{Arc, Mutex};

use stdweb::Reference;
use stdweb::web::{Document, Node};
use stdweb::unstable::TryInto;
use serde_json::{self, Map, Value};
use fnv::FnvHashMap;
use virtual_view::EventManager;

use super::super::NodesIds;
use super::DOMEvent;


pub struct Events {
    listening: FnvHashMap<String, usize>,
    listening_handlers: FnvHashMap<String, FnvHashMap<String, Reference>>,
    event_manager: Arc<Mutex<EventManager>>,
}

impl Events {
    #[inline(always)]
    pub fn new(event_manager: Arc<Mutex<EventManager>>) -> Self {
        Events {
            listening: FnvHashMap::default(),
            listening_handlers: FnvHashMap::default(),
            event_manager: event_manager,
        }
    }

    #[inline]
    pub fn listen(
        &mut self,
        name: &str,
        id: &str,
        node: &Node,
        nodes: &Arc<Mutex<NodesIds>>,
        document: &Document
    ) {
        if !self.listening.contains_key(name) {
            self.listening.insert(name.into(), 1);
            self.add_event_listener(name, id, node, nodes, document);
        } else {
            self.listening.get_mut(name).map(|count| { *count += 1 });
        }
    }
    #[inline]
    pub fn unlisten(
        &mut self,
        name: &str,
        id: &str,
        node: &Node,
        nodes: &Arc<Mutex<NodesIds>>,
        document: &Document
    ) {
        let count = if let Some(count) = self.listening.get_mut(name) {
            *count -= 1;
            Some(*count)
        } else {
            None
        };
        if count == Some(0) {
            self.listening.remove(name);
            self.remove_event_listener(name, id, node, nodes, document);
        }
    }

    #[inline]
    fn event_to_json(event: &Reference) -> Map<String, Value> {
        let object = js! {
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
        };

        match serde_json::to_value(object).unwrap() {
            Value::Object(json) => json,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn handle(
        event_manager: &Arc<Mutex<EventManager>>,
        nodes_ids: &Arc<Mutex<NodesIds>>,
        event: Reference
    ) {
        let target: Node = js! {
            return @{event.as_ref()}.target;
        }.try_into().unwrap();

        if let Some(id) = nodes_ids.lock().expect("failed to acquire nodes_ids lock").get_id(&target) {
            let name: String = js! {
                return @{event.as_ref()}.type;
            }.try_into().unwrap();

            let mut event = DOMEvent::new(name, Self::event_to_json(&event));
            event_manager.lock().expect("failed to acquire event_manager lock")
                .dispatch(id, &mut event);
        }
    }

    #[inline]
    fn add_event_listener(
        &mut self,
        name: &str,
        id: &str,
        _node: &Node,
        nodes_ids: &Arc<Mutex<NodesIds>>,
        document: &Document
    ) {
        let event_manager = self.event_manager.clone();
        let nodes_ids = nodes_ids.clone();
        let listener = move |e| Self::handle(&event_manager, &nodes_ids, e);
        let listener_reference: Reference = js! {
            var type = @{name},
                document = @{document},
                listener = @{listener};

            document.addEventListener(type, listener);
            return listener;
        }.try_into().unwrap();

        self.listening_handlers
            .entry(name.into()).or_insert(FnvHashMap::default())
            .insert(id.into(), listener_reference);

    }
    #[inline]
    fn remove_event_listener(
        &mut self,
        name: &str,
        id: &str,
        _node: &Node,
        _nodes_ids: &Arc<Mutex<NodesIds>>,
        document: &Document
    ) {
        let listener_reference = self.listening_handlers
            .entry(name.into()).or_insert(FnvHashMap::default())
            .get(id);

        js! {
            var type = @{name},
                document = @{document},
                listener = @{listener_reference.as_ref()};

            document.removeEventListener(type, listener);
        };
    }
}
