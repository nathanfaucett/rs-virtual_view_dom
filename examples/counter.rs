extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate view;
extern crate view_dom;

use stdweb::web::{document, IEventTarget};
use view::{Children, Component, Event, EventManager, Props, Renderer, Updater, View};
use view_dom::{Handler, Patcher, TransactionEvent};

struct Button;

impl Component for Button {
    #[inline]
    fn name(&self) -> &'static str {
        "Button"
    }
    #[inline]
    fn render(&self, _: &Updater, _: &Props, props: &Props, children: &Children) -> View {
        view! {
            <button class="Button" ... { props }>{ each children }</button>
        }
    }
}

struct Counter;

#[inline]
fn on_add_count(updater: &Updater, _: &mut Event) {
    updater.update(|current| {
        let mut next = current.clone();

        next.update("count", |count| {
            if let Some(c) = count.number() {
                *count = (c + 1.0).into();
            }
        });

        next
    });
}

#[inline]
fn on_sub_count(updater: &Updater, _: &mut Event) {
    updater.update(|current| {
        let mut next = current.clone();

        next.update("count", |count| {
            if let Some(c) = count.number() {
                *count = (c - 1.0).into();
            }
        });

        next
    });
}

impl Component for Counter {
    #[inline]
    fn name(&self) -> &'static str {
        "Counter"
    }
    #[inline]
    fn initial_state(&self, props: &Props) -> Props {
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    #[inline]
    fn render(&self, updater: &Updater, state: &Props, _: &Props, _: &Children) -> View {
        let count = state.get("count");

        let add_updater = updater.clone();
        let sub_updater = updater.clone();

        view! {
            <div class="Counter">
                <p>{format!("Count {}", count)}</p>
                <{Button} onclick={ move |e: &mut Event| on_add_count(&add_updater, e) }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ move |e: &mut Event| on_sub_count(&sub_updater, e) }>
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
