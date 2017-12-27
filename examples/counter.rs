#![feature(const_atomic_isize_new)]


extern crate stdweb;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;


use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicIsize, Ordering};

use stdweb::web::document;

use virtual_view::{EventManager, Event, View, Renderer};
use virtual_view_dom::Patcher;


static COUNT: AtomicIsize = AtomicIsize::new(0_isize);
static mut PATCHER: Option<Patcher> = None;
static mut RENDERER: Option<Renderer> = None;
static mut EVENT_MANAGER: Option<Arc<Mutex<EventManager>>> = None;


fn on_add_count(_: &mut Event) {
    println!("ADD");
    COUNT.fetch_add(1, Ordering::Relaxed);
    on_render();
}
fn on_sub_count(_: &mut Event) {
    println!("SUB");
    COUNT.fetch_sub(1, Ordering::Relaxed);
    on_render();
}

fn render(count: isize) -> View {
    View::new("div",
        json!({"class": "Root"}),
        events!(),
        vec![
            text!("Count: {}", count),
            View::new("button",
                json!({"class": "Add"}),
                events!({"click" => on_add_count}),
                vec![text!("Add")]
            ),
            View::new("button",
                json!({"class": "Sub"}),
                events!({"click" => on_sub_count}),
                vec![text!("Sub")]
            )
        ]
    )
}

fn on_render() {
    let patcher = unsafe { PATCHER.as_mut().unwrap() };
    let renderer = unsafe { RENDERER.as_mut().unwrap() };
    let event_manager = unsafe { EVENT_MANAGER.as_ref().unwrap() };
    let count = COUNT.load(Ordering::Relaxed);
    let view = render(count);
    let transaction = renderer.render(view, &mut *event_manager.lock().unwrap());
    patcher.patch(&transaction, event_manager.clone());
}


fn main() {
    stdweb::initialize();

    let patcher = Patcher::new(document().get_element_by_id("app").unwrap().into(), document());
    let renderer = Renderer::new();
    let event_manager = Arc::new(Mutex::new(EventManager::new()));

    unsafe {
        PATCHER = Some(patcher);
        RENDERER = Some(renderer);
        EVENT_MANAGER = Some(event_manager);
    }

    on_render();

    stdweb::event_loop();
}
