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
                let fmt = if prefix.is_empty() {
                    key.to_string()
                } else {
                    format!("{}.{}", prefix, key)
                };
                actions = actions
                    .union(&deserialize_actions(current_depth + 1, &fmt, map))
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

pub fn serialize_actions(actions: &HashSet<String>) -> Map<String, Value> {
    let mut map: Map<String, Value> = Map::new();
    // Fo each action
    for action in actions.iter() {
        // Declare a mutable pointer to the map start
        let mut map_pointer = &mut map;
        // Get objects in an action string
        let objects: Vec<&str> = action.split('.').collect();

        // For each object
        for obj in &objects {
            if obj == &objects[objects.len() - 1] {
                // If it is in the last object
                // Add the object to the map as boolean with true as value
                map_pointer.insert(String::from(*obj), Value::Bool(true));
            } else if map_pointer.contains_key(&String::from(*obj)) {
                // If it is the current map
                // Move map pointer to the map stored for the current word
                map_pointer = match map_pointer.get_mut(&String::from(*obj)) {
                    Some(Value::Object(x)) => x,
                    _ => panic!(
                        "1unexpected error in map creation, value should had been a Value::Object"
                    ),
                }
            } else {
                // If it is NOT in the current map
                // Add the object to the map as a new empty map
                map_pointer.insert(String::from(*obj), Value::Object(Map::new()));
                // Move map pointer to the map stored for the current word
                map_pointer = match map_pointer.get_mut(&String::from(*obj)) {
                    Some(Value::Object(x)) => x,
                    Some(y) => panic!(
                        "3unexpected error in map creation, value should had been a Value::Object,{}", y
                    ),
                    None => panic!(
                        "4unexpected error in map creation, value should had been a Value::Object"
                    ),
                };
            }
        }
    }

    map
}
