extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;

use stdweb::web::{document, IEventTarget, INonElementParentNode};
use virtual_view::{Children, Component, EventManager, Instance, Prop, Props, Renderer, Updater,
                   View};
use virtual_view_dom::{Handler, Patcher, TransactionEvent};

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

    let event_manager = EventManager::new();
    let handler = Handler::new(document());

    let mut patcher = Patcher::new(
        document().get_element_by_id("app").unwrap().into(),
        document(),
        event_manager.clone(),
    );

    document().add_event_listener::<TransactionEvent, _>(move |e| {
        patcher.patch(&e.transaction());
    });

    let _renderer = Renderer::new(
        view! {
            <{Counter} count=0/>
        },
        event_manager,
        handler,
    );

    stdweb::event_loop();
}
