use serde_json::{Map, Value};
use virtual_view::Event;


#[derive(Debug, Clone)]
pub struct DOMEvent {
    name: String,
    data: Map<String, Value>,
    propagation: bool,
}

impl DOMEvent {
    #[inline(always)]
    pub fn new(name: String, data: Map<String, Value>) -> Self {
        DOMEvent {
            name: name,
            data: data,
            propagation: true,
        }
    }
}

impl Event for DOMEvent {
    #[inline(always)]
    fn name(&self) -> &String { &self.name }
    #[inline(always)]
    fn data(&self) -> &Map<String, Value> { &self.data }
    #[inline(always)]
    fn propagation(&self) -> bool { self.propagation }
    #[inline(always)]
    fn stop_propagation(&mut self) {
        self.propagation = false;
    }
}
