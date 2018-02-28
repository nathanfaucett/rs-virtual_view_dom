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
use stdweb::web::{document, INonElementParentNode};
use virtual_view::{Children, Component, EventManager, Instance, Prop, Props, Renderer, Updater,
                   View};
use virtual_view_dom::Patcher;

struct Button;

impl Component for Button {
    fn render(&self, _: &Instance, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ... { props }>{ each children }</button>
        }
    }
}

struct Counter;

impl Counter {
    fn on_add_count(updater: &Updater) -> Prop {
        updater.set_state(|current| {
            let mut next = current.clone();

            next.update("count", |count| {
                if let Some(c) = count.number() {
                    *count = (c + 1.0).into();
                }
            });

            next
        });
        Prop::Null
    }
    fn on_sub_count(updater: &Updater) -> Prop {
        updater.set_state(|current| {
            let mut next = current.clone();

            next.update("count", |count| {
                if let Some(c) = count.number() {
                    *count = (c - 1.0).into();
                }
            });

            next
        });
        Prop::Null
    }
}

impl Component for Counter {
    fn name(&self) -> &'static str {
        "Counter"
    }
    fn initial_state(&self, props: &Props) -> Props {
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let count = instance.state.get("count").number().unwrap_or(0.0);

        view! {
            <div class="Counter">
                <p style={{
                    "color": if count >= 0.0 {"#000"} else {"#f00"},
                }}>{format!("Count {}", count)}</p>
                <{Button} onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| Counter::on_add_count(&updater)
                } }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| Counter::on_sub_count(&updater)
                } }>
                    {"Sub"}
                </{Button}>
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
            <{Counter} count=0/>
        },
        event_manager,
        server,
    );

    PromiseFuture::spawn(future);

    stdweb::event_loop();
}
