extern crate messenger;
extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;

use std::rc::Rc;
use std::cell::RefCell;

use serde_json::{from_value, Value};
use stdweb::PromiseFuture;
use stdweb::web::{document, set_timeout, INonElementParentNode};
use virtual_view::{Children, Component, EventManager, Instance, Props, Renderer, Updater, View};
use virtual_view_dom::Patcher;

struct App;

impl App {
    fn app_update(updater: &Updater) {
        updater.set_state(|prev| {
            let mut next = prev.clone();

            let mut direction = next.get("direction").number().unwrap_or(1.0);
            let mut count = next.get("count").number().unwrap_or(1.0);

            if direction == 1.0 {
                count += 1.0;

                if count >= 9.0 {
                    direction = -1.0;
                }
            } else {
                count -= 1.0;

                if count <= 1.0 {
                    direction = 1.0;
                }
            }

            next.insert("direction", direction);
            next.insert("count", count);

            next
        });

        let set_timeout_updater = updater.clone();
        set_timeout(move || App::app_update(&set_timeout_updater), 16);
    }
}

impl Component for App {
    fn name(&self) -> &'static str {
        "App"
    }
    fn initial_state(&self, _: &Props) -> Props {
        props! {
            "direction": 1,
            "count": 1,
        }
    }
    fn will_mount(&self, instance: &Instance) {
        let set_timeout_updater = instance.updater.clone();
        set_timeout(move || App::app_update(&set_timeout_updater), 16);
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let count = instance.state.get("count").number().unwrap_or(1.0) as isize;

        view! {
            <div class="App">
                { each (0..count).map(|c| {
                    let index = count - c;

                    view! {
                        <a key={c} style={{ "z-index": index }}>{index}</a>
                    }
                }) }
            </div>
        }
    }
}

fn main() {
    stdweb::initialize();

    let (server, client, future) = messenger::unbounded_channel();

    let event_manager = EventManager::new();

    let patcher = Rc::new(RefCell::new(Patcher::new(
        document().get_element_by_id("app").unwrap().into(),
        document(),
        event_manager.clone(),
    )));

    let _ = client.on("virtual_view.transaction", move |t: &Value| {
        let transaction = from_value(t.clone()).unwrap();
        patcher.borrow_mut().patch(&transaction);
        None
    });

    let _renderer = Renderer::new(
        view! {
            <{App}/>
        },
        event_manager,
        server,
    );

    PromiseFuture::spawn(future);

    stdweb::event_loop();
}
