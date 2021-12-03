use serde_json::{Map, Value};
use std::collections::HashSet;

//// TESTS ////
#[cfg(test)]
mod tests;

const MAX_JSON_DEPTH_ALLOWED: u8 = 20;

pub fn deserialize_actions(
    current_depth: u8,
    prefix: &str,
    json_obj: &Map<String, Value>,
) -> HashSet<String> {
    if current_depth >= MAX_JSON_DEPTH_ALLOWED {
        panic!("wrong format in permission json string - too much nesting")
    }

    let mut actions: HashSet<String> = HashSet::new();

    for (key, value) in json_obj.into_iter() {
        match value {
            Value::Object(map) => {
                let fmt = if prefix.len() == 0 { format!("{}", key) } else { format!("{}.{}", prefix, key) };
                actions = actions
                    .union(&deserialize_actions(
                        current_depth + 1,
                        &fmt,
                        map,
                    ))
                    .cloned()
                    .collect();
            }
            Value::Bool(val) => {
                if *val {
                    // Control trailing first dot with nesting counter
                    actions.insert(format!("{}.{}", prefix, key));
                }
            }
            _ => panic!(
                "wrong format in permission json string - found no object or boolean value, {}-{}",
                key, value
            ),
        };
    }

    actions
}
