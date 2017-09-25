#![feature(alloc)]
#![no_std]


#[macro_use]
extern crate alloc;
extern crate serde_json;
extern crate virt;
#[macro_use]
extern crate stdweb;


mod utils;
mod patcher;


pub use self::utils::ToHtmlString;
pub use self::patcher::Patcher;
