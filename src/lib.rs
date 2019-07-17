use std::f32;
use std::fmt;
use std::str::Chars;

use std::collections::HashMap;

#[derive(Debug)]
pub struct JSON {
    elements: Vec<Value>,
}

impl JSON {
    pub fn new() -> JSON {
        JSON {
            elements: Vec::new()
        }
    }
    pub fn parse(raw: String) -> JSON {
        tokenize(raw)
    }
}

#[derive(Debug)]
pub enum Value {
    JSONObject(HashMap<String, Value>),
    JSONArray(Vec<Value>),
    String(String),
    Number(f32),
}

impl fmt::Display for JSON {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<String> for JSON {
    fn from(s: String) -> JSON {
        JSON::parse(s)
    }
}

impl From<&str> for JSON {
    fn from(s: &str) -> JSON {
        JSON::parse(s.to_string())
    }
}

pub fn tokenize(raw: String) -> JSON {
    let mut tree = JSON { elements: vec![] };
    let mut char_array = raw.chars();
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            "{" => {
                tree.elements.push(parse_object(&mut char_array));
            },
            "[" => {
                tree.elements.push(parse_array(&mut char_array));
            },
            _ => panic!("invalid JSON")
        }
    }
    tree
}

pub fn parse_object(char_array: &mut Chars) -> Value {
    let mut object: HashMap<String, Value> = HashMap::new();
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            "\"" => {
                let (key, value) = parse_member(char_array);
                object.insert(key, value);
            },
            "}" => {
                return Value::JSONObject(object);
            },
            " " | "\n" | "\t" => {
                // we don't care about whitespaces here.
            },
            e => {
                eprint!("{}", e);
                panic!("invalid JSON")
            }
        }
    }
    Value::JSONObject(object)
}

pub fn parse_member(char_array: &mut Chars) -> (String, Value) {
    // always starts with a string, the key of the member
    let key = parse_string(char_array);
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            ":" => {
                match parse_value(char_array){
                    value => {
                        return (key, value);
                    }
                };
            },
            " " | "\n" | "\t" => {
                // we don't care about whitespaces here.
            },
            _ => panic!("invalid JSON")
        }
    }
    panic!("invalid JSON")
}

pub fn parse_string(char_array: &mut Chars) -> String {
    let mut accumulator: Vec<char> = Vec::new();
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            "\"" => {
                return accumulator.into_iter().collect();
            },
            _ => {
                accumulator.push(c);
            }
        }
    }
    panic!("could not parse string!")

}

pub fn parse_value(char_array: &mut Chars) -> Value {
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            "\"" => {
                return Value::String(parse_string(char_array));
            },
            "{" => {
                return parse_object(char_array);
            },
            "[" => {
                return parse_array(char_array);
            },
            " " | "\n" | "\t" => {
                // we don't care about whitespaces here.
            },
            "\\" | "/" => {
                // escaped character, ignore for now.
            },
            "." => {
                // fraction, should be part of number loop
            },
            "E" | "e" => {
                // exponents, should be part of number loop
            },
            "-" | "+" => {
                // sign, parts of exponents loop
            },
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                // number loop
            },
            e => {
                eprintln!("{}", e);
                panic!("invalid JSON")
            }
        }
    }
    panic!("invalid JSON")
}

pub fn parse_value_first_item_array(char_array: &mut Chars) -> Result<Value, ()> {
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            "]" => {
                return Err(());
            },
            " " | "\n" | "\t" => {
                // we don't care about whitespaces here.
            },
            "\"" => {
                return Ok(Value::String(parse_string(char_array)));
            },
            "}" => {
                return Ok(Value::JSONObject(HashMap::new()));
            },
            "{" => {
                return Ok(parse_object(char_array));
            },
            "[" => {
                return Ok(parse_array(char_array));
            },
            "\\" | "/" => {
                // escaped character, ignore for now.
            },
            "." => {
                // fraction, should be part of number loop
            },
            "E" | "e" => {
                // exponents, should be part of number loop
            },
            "-" | "+" => {
                // sign, parts of exponents loop
            },
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                // number loop
            },
            e => {
                eprintln!("{}", e);
                panic!("invalid JSON")
            }
        }
    }
    panic!("invalid JSON")
}

pub fn parse_array(char_array: &mut Chars) -> Value {
    let mut accumulator: Vec<Value> = Vec::new();
    if let Ok(item) = parse_value_first_item_array(char_array) {
        accumulator.push(item);
    } else {
        return Value::JSONArray(Vec::new());
    }
    while let Some(c) = char_array.next() {
        match c.to_string().as_ref() {
            "," => {
                accumulator.push(parse_value(char_array));
            },
            "]" => {
                return Value::JSONArray(accumulator);
            },
            e => {
                eprintln!("{}", e);
                panic!("invalid JSON")
            }
        }
    }
    return Value::JSONArray(accumulator)
}

// pub fn stringify(json: JSON) -> String {
//     json
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_obj() {
        let input = String::from("{}");

        print!("value: {:?}", JSON::from(input));

        assert_eq!(
            "{}",
            "{}"
        );
    }
}
