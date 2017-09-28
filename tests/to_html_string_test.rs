#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate virtual_view;
extern crate virtual_view_dom;


use virtual_view::View;
use virtual_view_dom::ToHtmlString;


#[test]
fn to_html_string_test() {
    let view = View::new("div", json!({"class": "Root", "style": {"font-size": "32px"}}), events!(),
        vec![text!("Hello, world!")]
    );
    assert_eq!(view.to_html_string(), "<div class=\"Root\" style=\"font-size: 32px;\"><span>Hello, world!</span></div>");
}
