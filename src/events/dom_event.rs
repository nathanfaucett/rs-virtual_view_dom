use serde_json::Value;
use virtual_view::Event;


#[derive(Debug, Clone)]
pub struct DOMEvent {
    name: String,
    data: Value,
    propagation: bool,
}

impl DOMEvent {
    #[inline(always)]
    pub fn new(name: String, data: Value) -> Self {
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
    fn data(&self) -> &Value { &self.data }
    #[inline(always)]
    fn propagation(&self) -> bool { self.propagation }
    #[inline(always)]
    fn stop_propagation(&mut self) {
        self.propagation = false;
    }
}
