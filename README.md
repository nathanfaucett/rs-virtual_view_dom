virtual_view_dom
=====

a virtual_view transaction renderer for the dom

```rust
#![feature(const_atomic_usize_new)]


extern crate stdweb;
#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;


use std::sync::atomic::{AtomicUsize, Ordering};

use stdweb::web::*;
use virtual_view::*;
use virtual_view_dom::*;


static COUNT: AtomicUsize = AtomicUsize::new(0_usize);
static DIR: AtomicUsize = AtomicUsize::new(1_usize);


fn render(count: usize) -> View {
    let children: Vec<View> = (0..count).map(|c| {
        let index = count - c;
        view!("a", {"key": index, "x-index": c, "style": {"z-index": index}},
            [text!("{}", index)]
        )
    }).collect();
    view!("div", {"class": "Root"}, children)
}

fn on_render(mut patcher: Patcher, mut renderer: Renderer) {
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
    let transaction = renderer.render(view);
    patcher.patch(&transaction);
    set_timeout(move || on_render(patcher, renderer), 100);
}


fn main() {
    stdweb::initialize();

    let patcher = Patcher::new(document().get_element_by_id("app").unwrap().into(), document());
    let renderer = Renderer::new();

    on_render(patcher, renderer);

    stdweb::event_loop();
}
```
