use alloc::string::{String, ToString};
use alloc::vec::Vec;

use serde_json::Value;
use virt::View;


pub trait ToHtmlString {
    fn to_html_string(&self) -> String;
}


impl ToHtmlString for View {
    #[inline]
    fn to_html_string(&self) -> String {
        match self {
            &View::Text(ref string) => format!("<span>{}</span>", string),
            &View::Data { ref kind, ref props, ref children, .. } => format!(
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
fn props_to_html_string(props: &Value) -> String {
    match props {
        &Value::Object(ref props) => {
            let mut out = String::new();
            let mut index = props.len();

            if index != 0 {
                out.push(' ');
            }

            for (k, v) in props {
                out.push_str(k);
                out.push('=');
                out.push('"');
                out.push_str(&prop_to_html_string(v));
                out.push('"');

                index -= 1;
                if index >= 1 {
                    out.push(' ');
                }
            }

            out
        },
        _ => "".into()
    }
}

#[inline]
fn prop_to_html_string(prop: &Value) -> String {
    match prop {
        &Value::Null => "null".to_string(),
        &Value::Bool(ref value) => value.to_string(),
        &Value::Number(ref value) => value.to_string(),
        &Value::String(ref value) => value.clone(),
        &Value::Array(ref array) => array.iter().map(|v| prop_to_html_string(v)).collect(),
        &Value::Object(ref map) => {
            let mut out = String::new();

            for (k, v) in map {
                out.push_str(k);
                out.push(':');
                out.push(' ');
                out.push_str(&prop_to_html_string(v));
                out.push(';');
            }

            out
        },
    }
}

#[inline]
fn children_to_html_string(children: &Vec<View>) -> String {
    let mut out = String::new();

    for child in children {
        out.push_str(&child.to_html_string());
    }

    out
}
