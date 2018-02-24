use std::collections::HashMap;

use stdweb;
use stdweb::unstable::TryInto;
use virtual_view::{Array, Prop, Props};

#[inline]
pub fn js_value_to_prop(value: stdweb::Value) -> Prop {
    match value {
        stdweb::Value::Undefined => Prop::Null,
        stdweb::Value::Null => Prop::Null,
        stdweb::Value::Bool(v) => Prop::Boolean(v),
        stdweb::Value::Number(v) => Prop::Number(v.try_into().unwrap()),
        stdweb::Value::Symbol(_) => Prop::Null,
        stdweb::Value::String(v) => Prop::String(v),
        stdweb::Value::Reference(v) => {
            let ref_type: String = js! {
                return typeof(@{v.as_ref()});
            }.try_into()
                .unwrap();

            if ref_type == "object" {
                Prop::Object(js_value_to_props(v.try_into().unwrap()))
            } else if ref_type == "array" {
                Prop::Array(js_value_to_array(v.try_into().unwrap()))
            } else {
                Prop::Object(Props::new())
            }
        }
    }
}

#[inline]
pub fn js_value_to_array(array: stdweb::Array) -> Array {
    let array: Vec<stdweb::Value> = array.try_into().unwrap();
    array.into_iter().map(|v| js_value_to_prop(v)).collect()
}

#[inline]
pub fn js_value_to_props(object: stdweb::Object) -> Props {
    let map: HashMap<String, stdweb::Value> = object.try_into().unwrap();
    map.into_iter()
        .map(|(k, v)| (k, js_value_to_prop(v)))
        .collect()
}
