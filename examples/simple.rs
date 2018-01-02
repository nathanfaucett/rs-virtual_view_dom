#![feature(const_atomic_usize_new)]


extern crate stdweb;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;


use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use stdweb::web::{document, set_timeout};

use virtual_view::{EventManager, View, Renderer};
use virtual_view_dom::Patcher;


static COUNT: AtomicUsize = AtomicUsize::new(0_usize);
static DIR: AtomicUsize = AtomicUsize::new(1_usize);


fn render(count: usize) -> View {
    virtual_view! {
        <div class="Root">
            { each (0..count).map(|c| {
                let index = count - c;

                virtual_view! {
                    <a key={index} "x-index"={c} style={{ "z-index": index }}>{index}</a>
                }
            }) }
        </div>
    }
}

fn on_render(mut patcher: Patcher, mut renderer: Renderer, event_manager: Arc<Mutex<EventManager>>) {
    let last_count = COUNT.load(Ordering::Relaxed);

    if last_count == 1_usize {
        DIR.store(1_usize, Ordering::Relaxed);
    } else if last_count >= 9_usize {
        DIR.store(0_usize, Ordering::Relaxed);
    }

    let count = if DIR.load(Ordering::Relaxed) == 1_usize {
        COUNT.fetch_add(1_usize, Ordering::Relaxed)
    } else {
        COUNT.fetch_sub(1_usize, Ordering::Relaxed)
    };

    let view = render(count);
    let transaction = renderer.render(view, &mut *event_manager.lock().unwrap());
    patcher.patch(&transaction);
    set_timeout(move || on_render(patcher, renderer, event_manager), 0);
}


fn main() {
    stdweb::initialize();

    let event_manager = Arc::new(Mutex::new(EventManager::new()));
    let patcher = Patcher::new(document().get_element_by_id("app").unwrap().into(), document(), event_manager.clone());
    let renderer = Renderer::new();

    on_render(patcher, renderer, event_manager);

    stdweb::event_loop();
}
