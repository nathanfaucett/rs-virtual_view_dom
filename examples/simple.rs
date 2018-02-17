extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate view;
extern crate view_dom;

use stdweb::web::{document, IEventTarget};
use view::{Children, Component, Event, EventManager, Props, Renderer, Updater, View};
use view_dom::{Handler, Patcher, TransactionEvent};

struct App;

fn on_button_click(updater: &Updater, page: &'static str) {
    updater.update(|prev| {
        let mut next = prev.clone();
        next.insert("page", page);
        next
    });
}

impl Component for App {
    fn name(&self) -> &'static str {
        "App"
    }
    fn initial_state(&self, _: &Props) -> Props {
        props! {
            "page": "index"
        }
    }
    fn render(&self, updater: &Updater, state: &Props, _: &Props, _: &Children) -> View {
        let home_updater = updater.clone();
        let contact_updater = updater.clone();

        view! {
            <div class="App">
                <div>
                    <button onclick={ move |_: &mut Event| {
                        on_button_click(&home_updater, "home") }
                    }>
                        {"Home"}
                    </button>
                    <button onclick={ move |_: &mut Event| {
                        on_button_click(&contact_updater, "contact") }
                    }>
                        {"Contact"}
                    </button>
                </div>
                {
                    match state.get("page").to_string().as_str() {
                        "contact" => view! { <{Contact}/> },
                        _ => view! { <{Home}/> },
                    }
                }
            </div>
        }
    }
}

struct Home;

impl Component for Home {
    fn name(&self) -> &'static str {
        "Home"
    }
    fn render(&self, _: &Updater, _: &Props, _: &Props, _: &Children) -> View {
        view! {
            <div class="Home">
                <h1>{"Home"}</h1>
                <p>{"This is the Home Page."}</p>
            </div>
        }
    }
}

struct Contact;

impl Component for Contact {
    fn name(&self) -> &'static str {
        "Contact"
    }
    fn render(&self, _: &Updater, _: &Props, _: &Props, _: &Children) -> View {
        view! {
            <div class="Contact">
                <h1>{"Contact"}</h1>
                <a href="mailto:nathanfaucett@gmail.com">{"nathanfaucett@gmail.com"}</a>
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
            <{App}/>
        },
        event_manager,
        handler,
    );

    stdweb::event_loop();
}
