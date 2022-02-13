use serde_json::{Map, Value};
use std::collections::HashSet;

//// TESTS ////
#[cfg(test)]
mod tests;

// Constant for defining maximun nesting allowed in a json_object when deserializing
// As the deserializing function is recursive, this prevent overflows.
const MAX_JSON_DEPTH_ALLOWED: u8 = 20;
// Contant for defining the character that acts as a divider for action subgroups in
// serialization and deserialization.
const ACTION_DIVIDER: char = ':';

#[doc(hidden)]
/// Deserializes a JSON object into a HashSet of string actions.
/// The function is recursive and thus uses some parameters to control this recursivity.
///
/// # Arguments
///
/// * `current_depth` - Current recursivity depth of the function. Should be set to cero (0) everytime the function is called explicitly.
/// Can be set to a greter value if you want to reduce max recursivity allowed by the `MAX_JSON_DEPTH_ALLOWED` constant.
/// * `prefix` - Should be set to cero (0) everytime the function is called explicitly.
/// Represents the action key acumulated value from previous function calls.
/// * `json_obj` - serde_json::Map containing the JSON object with the actions that need to be deserialized into action strings.
///
/// # Panics
///
/// - Panics if `current_depth` is greater or equal to `MAX_JSON_DEPTH_ALLOWED` value.
/// This usually means that max recursivity has been reached.
/// - Panics if `json_obj` is not correctly formed and for any key contains a
/// value different from a serde_json::Map or a serde_json::Bool.
pub fn deserialize_actions(
    current_depth: u8,
    prefix: &str,
    json_obj: &Map<String, Value>,
) -> HashSet<String> {
    // If we have already reached max recursivity nesting allowed then panic.
    if current_depth >= MAX_JSON_DEPTH_ALLOWED {
        panic!("wrong format in permission json string - too much nesting")
    }

    let mut actions: HashSet<String> = HashSet::new();

    // Iterate json map object
    for (key, value) in json_obj.into_iter() {
        // Get the string representing the action correctly formatted (Control trailing first colon with prefix value)
        let action_value = if prefix.is_empty() {
            key.to_string()
        } else {
            format!("{}{}{}", prefix, ACTION_DIVIDER, key)
        };

        match value {
            // If the value for a key is a json map again then call this function recursively passing current depth
            // and the current key value as prefix for subsequent actions.
            Value::Object(map) => {
                actions = actions
                    .union(&deserialize_actions(current_depth + 1, &action_value, map))
                    .cloned()
                    .collect();
            }
            // If the value is a boolean then we have reached the end of the action definition and
            // can include the action in the actions HashSet.
            Value::Bool(val) => {
                if *val {
                    actions.insert(action_value);
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

#[doc(hidden)]
/// Serializes a HashSet of string actions into a serde_json::Map which represents valid JSON.
pub fn serialize_actions(actions: &HashSet<String>) -> Map<String, Value> {
    let mut map: Map<String, Value> = Map::new();
    // Fo each action
    for action in actions.iter() {
        // Declare a mutable pointer to the map start
        let mut map_pointer = &mut map;
        // Get objects in an action string
        let objects: Vec<&str> = action.split(ACTION_DIVIDER).collect();

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
                        "unexpected error in map creation, value should had been a Value::Object"
                    ),
                }
            } else {
                // If it is NOT in the current map
                // Add the object to the map as a new empty map
                map_pointer.insert(String::from(*obj), Value::Object(Map::new()));
                // Move map pointer to the map stored for the current word
                map_pointer = match map_pointer.get_mut(&String::from(*obj)) {
                    Some(Value::Object(x)) => x,
                    _ => panic!(
                        "unexpected error in map creation, value should had been a Value::Object"
                    ),
                };
            }
        }
    }

    map
}
