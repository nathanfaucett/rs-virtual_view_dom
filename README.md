virtual_view_dom
=====

a virtual view transaction renderer for the dom

### Build Examples
```bash
$ cargo install -f cargo-web
```
```bash
$ make
```

### Counter Example

```rust
extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;

use stdweb::web::{document, IEventTarget};
use virtual_view::{Children, Component, Event, Instance, EventManager, Props, Renderer, Updater, View};
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
}

impl Component for App {
    #[inline]
    fn name(&self) -> &'static str {
        "App"
    }
    #[inline]
    fn initial_state(&self, props: &Props) -> Props {
        props! {
            "count": props.take("count").unwrap_or(0.into())
        }
    }
    #[inline]
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        let count = instance.state.get("count");

        view! {
            <div class="Counter">
                <p>{format!("Count {}", count)}</p>
                <{Button} onclick={ instance.wrap(App::on_add_count) }>
                    {"Add"}
                </{Button}>
                <{Button} onclick={ instance.wrap(App::on_sub_count) }>
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
```
