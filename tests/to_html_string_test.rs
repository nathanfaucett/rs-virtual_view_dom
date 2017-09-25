#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate virt;
extern crate virt_dom;


use virt::*;
use virt_dom::ToHtmlString;


#[test]
fn test_simple() {
    let view = view!("div", {"class": "Root", "style": {"font-size": "32px"}}, [text!("Hello, world!")]);
    assert_eq!(view.to_html_string(), "<div class=\"Root\" style=\"font-size: 32px;\"><span>Hello, world!</span></div>");
}
