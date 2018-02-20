use serde_json::{Map, Value};
use virtual_view::{RawView, View};

pub trait ToHtmlString {
    fn to_html_string(&self) -> String;
}

impl ToHtmlString for View {
    #[inline]
    fn to_html_string(&self) -> String {
        let raw_view: RawView = self.into();
        raw_view.to_html_string()
    }
}

impl ToHtmlString for RawView {
    #[inline]
    fn to_html_string(&self) -> String {
        match self {
            &RawView::Text(ref string) => format!("<span>{}</span>", string),
            &RawView::Data {
                ref kind,
                ref props,
                ref children,
                ..
            } => format!(
                "<{}{}>{}</{}>",
                kind,
                props_to_html_string(props),
                children_to_html_string(children),
                kind
            ),
        }
    }
}

#[inline]
fn props_to_html_string(props: &Map<String, Value>) -> String {
    let mut out = String::new();

    for (k, v) in props {
        out.push(' ');
        out.push_str(k);
        out.push('=');
        out.push('"');
        out.push_str(&prop_to_html_string(v));
        out.push('"');
    }

    out
}

#[inline]
fn prop_to_html_string(prop: &Value) -> String {
    match prop {
        &Value::Null => "null".to_string(),
        &Value::Bool(ref value) => value.to_string(),
        &Value::Number(ref value) => value.to_string(),
        &Value::String(ref value) => value.clone(),
        &Value::Array(ref array) => {
            let mut out = String::new();

            for v in array {
                out.push_str(&prop_to_html_string(v));
                out.push(',');
            }

            out
        }
        &Value::Object(ref map) => {
            let mut out = String::new();

            for (k, v) in map {
                out.push_str(k);
                out.push(':');
                out.push_str(&prop_to_html_string(v));
                out.push(';');
            }

            out
        }
    }
}

#[inline]
fn children_to_html_string(children: &Vec<RawView>) -> String {
    let mut out = String::new();

    for child in children {
        out.push_str(&child.to_html_string());
    }

    out
}

#[test]
fn test_to_html_string() {
    let view = view! {
        <div class="Root" style={{"font-size": "32px", "color": "#F00"}} array={[0, 1, 2]}>
            {"Hello, world!"}
        </div>
    };
    assert_eq!(
        view.to_html_string(),
        "<div array=\"0,1,2,\" class=\"Root\" style=\"color:#F00;font-size:32px;\"><span>Hello, world!</span></div>"
    );
}
