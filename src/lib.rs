#![recursion_limit="128"]


#[cfg_attr(test, macro_use)]
extern crate serde_json;
#[cfg_attr(test, macro_use)]
extern crate virtual_view;
#[macro_use]
extern crate stdweb;


mod events;
mod utils;
mod patcher;


pub use self::events::{Events, DOMEvent};
pub use self::utils::ToHtmlString;
pub use self::patcher::Patcher;
