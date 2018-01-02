virtual_view_dom
=====

a virtual view transaction renderer for the dom

# Build Examples
```bash
$ cargo install cargo-web
```
```bash
$ make
```

```rust
#![feature(const_atomic_isize_new)]


extern crate stdweb;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;


use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicIsize, Ordering};

use stdweb::web::{document, set_timeout};

use virtual_view::{EventManager, Event, View, Renderer};
use virtual_view_dom::Patcher;


static COUNT: AtomicIsize = AtomicIsize::new(0_isize);
static mut PATCHER: Option<Patcher> = None;
static mut RENDERER: Option<Renderer> = None;
static mut EVENT_MANAGER: Option<Arc<Mutex<EventManager>>> = None;


fn on_add_count(_: &mut Event) {
    COUNT.fetch_add(1, Ordering::Relaxed);
    enqueue_render();
}
fn on_sub_count(_: &mut Event) {
    COUNT.fetch_sub(1, Ordering::Relaxed);
    enqueue_render();
}

fn counter_render(count: isize) -> View {
    virtual_view! {
        <div class="Root">
            <p style={{ "color": if count < 0 {"#F00"} else {"#000"} }}>
                {format!("Count: {}", count)}
            </p>
            <button class="Add" style={{ "color": "#000", "background-color": "#FFF" }} click => on_add_count>
                {"Add"}
            </button>
            <button class="Sub" style={{ "color": "#000", "background-color": "#FFF" }} click => on_sub_count>
                {"Sub"}
            </button>
        </div>
    }
}

fn render() {
    let patcher = unsafe { PATCHER.as_mut().unwrap() };
    let renderer = unsafe { RENDERER.as_mut().unwrap() };
    let event_manager = unsafe { EVENT_MANAGER.as_ref().unwrap() };
    let count = COUNT.load(Ordering::Relaxed);
    let view = counter_render(count);
    let transaction = renderer.render(view, &mut *event_manager.lock().expect("failed to acquire event_manager lock"));
    patcher.patch(&transaction);
}

fn enqueue_render() {
    set_timeout(render, 0);
}


fn main() {
    stdweb::initialize();

    let event_manager = Arc::new(Mutex::new(EventManager::new()));
    let patcher = Patcher::new(document().get_element_by_id("app").unwrap().into(), document(), event_manager.clone());
    let renderer = Renderer::new();

    unsafe {
        PATCHER = Some(patcher);
        RENDERER = Some(renderer);
        EVENT_MANAGER = Some(event_manager);
    }

    enqueue_render();

    stdweb::event_loop();
}
```
