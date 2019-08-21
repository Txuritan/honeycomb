extern crate comb;
use comb::{atoms::rec, language, language::token_is, Parser};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn boolean() -> Parser<JsonValue> {
    (token_is("true") - |_| JsonValue::Bool(true))
        | (token_is("false") - |_| JsonValue::Bool(false))
}

fn string() -> Parser<JsonValue> {
    language::string() - JsonValue::Str
}

fn number() -> Parser<JsonValue> {
    language::number() - |s| JsonValue::Num(s.parse::<f64>().unwrap())
}

fn null() -> Parser<JsonValue> {
    token_is("null") - |_| JsonValue::Null
}

fn array() -> Parser<JsonValue> {
    language::array("[", json(), "]") - JsonValue::Array
}

fn object() -> Parser<JsonValue> {
    language::array("{", string() << token_is(":") & rec(json), "}")
        - (|v: Vec<(JsonValue, JsonValue)>| -> JsonValue {
            let mut result = HashMap::new();
            for (key, value) in v {
                if let JsonValue::Str(s) = key {
                    result.insert(s, value);
                }
            }
            JsonValue::Object(result)
        })
}

fn json() -> Parser<JsonValue> {
    null() | boolean() | number() | string() | rec(array) | rec(object)
}

fn main() {
    println!(
        "{:#?}",
        json().parse(
            r#"
{
    abc: 1,
    "testing" : null,
    "recursion" : {
        "WOW": 1.2345
    },
    "array": [1, 2, {"test": "123"}, 4],
    "test": "testing"
}
"#
        )
    );
}
