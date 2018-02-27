extern crate messenger;
extern crate serde_json;
extern crate stdweb;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;

use std::rc::Rc;
use std::cell::RefCell;

use stdweb::PromiseFuture;
use stdweb::web::{document, INonElementParentNode, Node};
use stdweb::web::html_element::InputElement;
use stdweb::unstable::TryInto;

use virtual_view::{Array, Children, Component, EventManager, Instance, Prop, Props, Renderer,
                   Updater, View};
use virtual_view_dom::Patcher;

struct App;

impl App {
    fn add_todo(updater: &Updater) -> Prop {
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
        Prop::Null
    }
    fn text_change(updater: &Updater, e: &mut Props) -> Prop {
        let id = e.get("component_id").string().unwrap();

        if let Some(node) = dom_node(id) {
            let input: InputElement = node.try_into().unwrap();

            updater.set_state(move |prev| {
                let mut next = prev.clone();
                next.set("text", input.raw_value());
                next
            });
        }
        Prop::Null
    }
    fn remove_todo(updater: &Updater, e: &mut Props) -> Prop {
        let id = e.take("todo_id").unwrap();

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

        Prop::Null
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
                    oninput={ block {
                        let updater = instance.updater.clone();
                        move |e: &mut Props| App::text_change(&updater, e)
                    } }
                />
                <{AddTodo} add_todo={ block {
                    let updater = instance.updater.clone();
                    move |_: &mut Props| App::add_todo(&updater)
                } } />
                <{VisibleTodoList}
                    todos={ instance.state.get("todos") }
                    remove_todo={ block {
                        let updater = instance.updater.clone();
                        move |e: &mut Props| App::remove_todo(&updater, e)
                    } }
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
                <{TodoList} todos={ props.get("todos") } remove_todo={ props.get("remove_todo") }/>
            </div>
        }
    }
}

struct Todo;

impl Component for Todo {
    fn render(&self, _: &Instance, props: &Props, _: &Children) -> View {
        let id = props.take("id").unwrap();
        let completed = props.get("completed").boolean().unwrap_or(false);
        let remove_todo = props.take("remove_todo").unwrap();

        view! {
            <li class="Todo"
                style={{ "text-decoration": if completed {"line-through"} else {"none"} }}
                onclick={ move |e: &mut Props| {
                    e.set("todo_id", &id);
                    remove_todo.call(e).unwrap()
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
                        <{Todo} key={id} ...{todo} remove_todo={ props.get("remove_todo") }/>
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
    unsafe { PATCHER.as_ref().unwrap().borrow().node(id) }
}
static mut PATCHER: Option<Rc<RefCell<Patcher>>> = None;

fn main() {
    stdweb::initialize();

    let (server, client, future) = messenger::unbounded_channel();

    let event_manager = EventManager::new();

    let patcher = Rc::new(RefCell::new(Patcher::new(
        document().get_element_by_id("app").unwrap().into(),
        document(),
        event_manager.clone(),
    )));

    unsafe {
        PATCHER = Some(patcher);
    }

    let _ = client.on("virtual_view.transaction", move |t| unsafe {
        PATCHER.as_ref().unwrap().borrow_mut().patch(t);
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
