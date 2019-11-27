use regex::Regex;

use serde_json::{self, Value};

fn main() {
    println!("Testing param_parser");
}

fn get_regex() -> Regex {
    //Regex::new(r"sentry(\s*\[([a-zA-Z0-9_]*)\]\s*)*").unwrap()
    Regex::new(r"^sentry(?:\[[a-zA-Z0-9_. ]*\])*=\w*$").unwrap()
}

pub fn update_value<'a>(obj: &'a mut Value, path: &[&str], val: &str) {
    update_value_internal(obj, path, val, false)
}

fn update_value_internal<'a>(
    obj: &'a mut Value,
    path: &[&str],
    val: &str,
    recursion_protection: bool,
) {
    if path.len() == 0 {
        return;
    }
    if let Value::Object(the_map) = obj {
        if path.len() == 1 {
            the_map.insert(path[0].into(), Value::String(val.into()));
        } else {
            match the_map.get_mut(path[0]) {
                Some(inner) => {
                    if inner.is_object() {
                        // we have a member at the specified index and it is an object (we can insert at path)
                        update_value_internal(inner, &path[1..], val, false);
                    }
                }
                None => {
                    if recursion_protection {
                        //this is a bug we should NEVER be here
                        log::error!(
                            "Bug in update_value_internal, infinite recursion detection triggered"
                        );
                    }
                    //nothing yet at the specified path create an object
                    the_map.insert(path[0].into(), Value::Object(serde_json::Map::new()));
                    // now we should have an object at the path, try again
                    update_value_internal(obj, path, val, true);
                }
            }
        }
    }
}

pub fn merge_vals(a: &mut Value, b: Value) {
    match (a, b) {
        //recursively merge dicts
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge_vals(a.entry(k).or_insert(Value::Null), v);
            }
        }
        //fill in missing left values
        (a @ &mut Value::Null, b) => *a = b,
        //do not override existing values that are not maps
        (a, b) => {} //*a = b,
    }
}

enum IndexingState {
    LookingForLeftParan,
    Accumulating(usize),
}

fn get_indexes(full_string: &str) -> Vec<&str> {
    let mut ret_vals = vec![];
    let mut state = IndexingState::LookingForLeftParan;
    //first iterate by byte (so we can get correct offsets)
    for (idx, by) in full_string.as_bytes().iter().enumerate() {
        match state {
            IndexingState::LookingForLeftParan => {
                if by == &b'[' {
                    state = IndexingState::Accumulating(idx + 1);
                }
            }
            IndexingState::Accumulating(start_idx) => {
                if by == &b']' {
                    let slice = &full_string[start_idx..idx];
                    ret_vals.push(slice);
                    state = IndexingState::LookingForLeftParan;
                }
            }
        }
    }
    ret_vals
}
fn use_map(x: &[i32]) {
    let ww = &[1, 2, 3];
    let w = ww.map(|x| x + 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_parser() {
        let examples: &[(&str, &[&str])] = &[
            ("fafdasd[a][b][33]", &["a", "b", "33"]),
            ("[23a][234][abc123]", &["23a", "234", "abc123"]),
            ("sentry[abc][123][]=SomeVal", &["abc", "123", ""]),
            ("sentry[Grüße][Jürgen][❤]", &["Grüße", "Jürgen", "❤"]),
            ("[农22历][新年][b新年c]", &["农22历", "新年", "b新年c"]),
            ("[ὈΔΥΣΣΕΎΣ][abc]", &["ὈΔΥΣΣΕΎΣ", "abc"]),
        ];

        for &(example, expected_result) in examples {
            let indexes = get_indexes(example);
            assert_eq!(&indexes[..], expected_result)
        }
    }

    #[test]
    fn test_update_value() {
        let mut val = Value::Object(serde_json::Map::new());

        update_value(&mut val, &["x", "y", "z"], "xx");

        insta::assert_json_snapshot!(val, @r###"
       ⋮{
       ⋮  "x": {
       ⋮    "y": {
       ⋮      "z": "xx"
       ⋮    }
       ⋮  }
       ⋮}
        "###);

        update_value(&mut val, &["x", "y", "k"], "kk");
        update_value(&mut val, &["w", ""], "w");
        update_value(&mut val, &["z1"], "val1");
        insta::assert_json_snapshot!(val, @r###"
       ⋮{
       ⋮  "w": {
       ⋮    "": "w"
       ⋮  },
       ⋮  "x": {
       ⋮    "y": {
       ⋮      "k": "kk",
       ⋮      "z": "xx"
       ⋮    }
       ⋮  },
       ⋮  "z1": "val1"
       ⋮}
        "###);
    }

    #[test]
    fn test_merge() {
        let mut a = serde_json::json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities":[ "london", "paris" ]
        });

        let b = serde_json::json!({
            "title": "This is another title",
            "person" : {
                "firstName" : "Jane",
                "middleName": "Helen"
            },
            "cities":[ "colombo" ]
        });

        merge_vals(&mut a, b);
        insta::assert_json_snapshot!(a, @r###"
       ⋮{
       ⋮  "cities": [
       ⋮    "london",
       ⋮    "paris"
       ⋮  ],
       ⋮  "person": {
       ⋮    "firstName": "John",
       ⋮    "lastName": "Doe",
       ⋮    "middleName": "Helen"
       ⋮  },
       ⋮  "title": "This is a title"
       ⋮}
        "###);
    }
}
