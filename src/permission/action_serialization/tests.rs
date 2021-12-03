use super::*;
use serde_json::json;

#[test]
fn deserialize_actions_test() {
    // Test MAX_JSON_DEPTH_ALLOWED reached
    let data_too_much_nesting = json!({"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {}}}}}}}}}}}}}}}}}}}}}});
    match data_too_much_nesting {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(_) => panic!("operation should have panicked for MAX_JSON_DEPTH_ALLOWED exceeded"),
            Err(_) => (),
        },
        _ => panic!("error in test data shoul be a Value::Object",),
    };

    // Test MAX_JSON_DEPTH_ALLOWED reached not starting in 0
    let data_short = json!({"obj": {"obj": {"obj": {}}}});
    match data_short {
        Value::Object(map) => {
            // Should panic and be catched by unwind
            match std::panic::catch_unwind(|| deserialize_actions(17, "", &map)) {
                Ok(_) => {
                    panic!("operation should have panicked for MAX_JSON_DEPTH_ALLOWED exceeded")
                }
                Err(_) => (),
            };

            // Should not panic
            deserialize_actions(16, "", &map);
        }
        _ => panic!("error in test data shoul be a Value::Object",),
    };

    // Test json_string has not valid value in object
    let data_not_valid1 = json!({
        "obj": {
            "obj1": 3
        }
    });
    let data_not_valid2 = json!({"true": []});

    match data_not_valid1 {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(_) => {
                panic!("operation should have panicked for wrong json")
            }
            Err(_) => (),
        },
        _ => panic!("error in test data shoul be a Value::Object",),
    };
    match data_not_valid2 {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(_) => {
                panic!("operation should have panicked for wrong json")
            }
            Err(_) => (),
        },
        _ => panic!("error in test data shoul be a Value::Object",),
    };
}
