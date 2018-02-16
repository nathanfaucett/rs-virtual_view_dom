#![recursion_limit = "128"]

extern crate fnv;
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[macro_use]
extern crate stdweb;
#[cfg_attr(test, macro_use)]
extern crate view;

mod events;
mod utils;
mod handler;
mod node_ref;
mod nodes_ids;
mod patcher;

pub use self::events::{DOMEvent, Events};
pub use self::utils::ToHtmlString;
pub use self::handler::{Handler, TransactionEvent};
pub use self::node_ref::NodeRef;
pub use self::nodes_ids::NodesIds;
pub use self::patcher::Patcher;
