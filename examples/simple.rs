extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;

use stdweb::web::{document, IEventTarget, Node};
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;

use virtual_view::{Array, Children, Component, Event, EventManager, Instance, Prop, Props,
                   Renderer, Updater, View};
use virtual_view_dom::{Handler, Patcher, TransactionEvent};

struct App;

impl App {
    fn add_todo(updater: &Updater) {
        updater.set_state(|prev| {
            let mut next = prev.clone();
            let next_id = prev.get("next_id").number().unwrap();

            next.update("todos", |todos| {
                if let Some(array) = todos.array_mut() {
                    array.push(prop!({
                        "id": next_id,
                        "completed": false,
                        "text": prev.get("text"),
                    }))
                }
            });
            next.set("next_id", next_id + 1.0);
            next.remove("text");

            next
        });
    }
    fn text_change(updater: &Updater, e: &mut Event) {
        let id = e.target_id();

        if let Some(node) = dom_node(id) {
            let input: InputElement = node.try_into().unwrap();

            updater.set_state(move |prev| {
                let mut next = prev.clone();
                next.set("text", input.value().into_string().unwrap());
                next
            });
        }
    }
    fn remove_todo(updater: &Updater, id: &Prop) {
        let id = id.clone();

        updater.set_state(move |prev| {
            let mut next = prev.clone();

            next.update("todos", |todos| {
                if let Some(array) = todos.array_mut() {
                    if let Some(index) = array.iter().position(|todo| {
                        let todo = todo.object().unwrap();
                        todo.get("id") == &id
                    }) {
                        array.remove(index);
                    }
                }
            });

            next
        });
    }
}

impl Component for App {
    fn initial_state(&self, _: &Props) -> Props {
        props! {
            "text": "Finish me!",
            "next_id": 1,
            "todos": [{"id": 0, "completed": false, "text": "Todo"}]
        }
    }
    fn render(&self, instance: &Instance, _: &Props, _: &Children) -> View {
        view! {
            <div class="Component">
                <input
                    type="text"
                    value={ instance.state.get("text") }
                    oninput={ event {
                        let updater = instance.updater.clone();
                        move |e: &mut Event| App::text_change(&updater, e)
                    } }
                />
                <{AddTodo} add_todo={ event {
                    let updater = instance.updater.clone();
                    move |_: &mut Event| App::add_todo(&updater)
                } } />
                <{VisibleTodoList}
                    todos={ instance.state.get("todos") }
                    app_updater={ instance.updater.clone() }
                />
                <{Footer}/>
            </div>
        }
    }
}

struct AddTodo;

impl Component for AddTodo {
    fn render(&self, _: &Instance, props: &Props, _: &Children) -> View {
        view! {
            <button class="AddTodo" onclick={ props.get("add_todo") }>
                {"Add Todo"}
            </button>
        }
    }
}

struct VisibleTodoList;

impl Component for VisibleTodoList {
    fn render(&self, _: &Instance, props: &Props, _: &Children) -> View {
        view! {
            <div class="VisibleTodoList">
                <{TodoList} todos={ props.get("todos") } app_updater={ props.get("app_updater") }/>
            </div>
        }
    }
}

struct Todo;

impl Component for Todo {
    fn render(&self, _: &Instance, props: &Props, _: &Children) -> View {
        let id = props.take("id").unwrap();
        let completed = props.get("completed").boolean().unwrap_or(false);
        let app_updater = props.take("app_updater").unwrap();

        view! {
            <li class="Todo"
                style={{ "text-decoration": if completed {"line-through"} else {"none"} }}
                onclick={ move |_: &mut Event| {
                    App::remove_todo(app_updater.updater().unwrap(), &id);
                } }
            >
                {props.get("text")}
            </li>
        }
    }
}

struct TodoList;

impl Component for TodoList {
    fn render(&self, _: &Instance, props: &Props, _: &Children) -> View {
        let empty_array = Array::new();
        let todos = props.get("todos").array().unwrap_or(&empty_array);

        view! {
            <ul>
                { each todos.iter().map(|todo| {
                    let todo = todo.object().unwrap();
                    let id = todo.get("id");

                    view! {
                        <{Todo} key={id} ...{todo} app_updater={ props.get("app_updater") }/>
                    }
                }) }
            </ul>
        }
    }
}

struct Footer;

impl Component for Footer {
    fn render(&self, _: &Instance, _: &Props, _: &Children) -> View {
        view! {
            <div class="Footer"></div>
        }
    }
}

fn dom_node(id: &str) -> Option<Node> {
    unsafe { PATCHER.as_ref().unwrap().node(id) }
}
static mut PATCHER: Option<Patcher> = None;

fn main() {
    stdweb::initialize();

    let event_manager = EventManager::new();
    let handler = Handler::new(document());

    let patcher = Patcher::new(
        document().get_element_by_id("app").unwrap().into(),
        document(),
        event_manager.clone(),
    );

    unsafe {
        PATCHER = Some(patcher);
    }

    document().add_event_listener::<TransactionEvent, _>(|e| unsafe {
        PATCHER.as_mut().unwrap().patch(&e.transaction());
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
