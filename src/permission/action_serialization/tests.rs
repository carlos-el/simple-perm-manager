use super::*;
use serde_json::json;
use std::collections::HashSet;

#[test]
fn deserialize_actions_test() {
    // Test json returns correct actions
    let data_simple = json!({"building": { "view": true, "meter": {"create":true}}, "user": {"edit": true}, "simple_action": true});
    match data_simple {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(actions) => {
                assert_eq!(
                    actions,
                    HashSet::from([
                        String::from("building.view"),
                        String::from("building.meter.create"),
                        String::from("user.edit"),
                        String::from("simple_action")
                    ])
                )
            }
            Err(_) => panic!("operation should not return error"),
        },
        _ => panic!("error in test data should be a Value::Object",),
    };

    // Test json is very simple, just object with values
    let data_simple = json!({"view": true, "create": true});
    match data_simple {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(actions) => {
                assert_eq!(
                    actions,
                    HashSet::from([String::from("view"), String::from("create")])
                )
            }
            Err(_) => panic!("operation should not return error"),
        },
        _ => panic!("error in test data should be a Value::Object",),
    };

    // Test MAX_JSON_DEPTH_ALLOWED reached
    let data_too_much_nesting = json!({"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {"obj": {}}}}}}}}}}}}}}}}}}}}}});
    match data_too_much_nesting {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(_) => panic!("operation should have panicked for MAX_JSON_DEPTH_ALLOWED exceeded"),
            Err(_) => (),
        },
        _ => panic!("error in test data should be a Value::Object",),
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
        _ => panic!("error in test data should be a Value::Object",),
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
        _ => panic!("error in test data should be a Value::Object",),
    };
    match data_not_valid2 {
        Value::Object(map) => match std::panic::catch_unwind(|| deserialize_actions(0, "", &map)) {
            Ok(_) => {
                panic!("operation should have panicked for wrong json")
            }
            Err(_) => (),
        },
        _ => panic!("error in test data should be a Value::Object",),
    };
}

#[test]
fn serialize_actions_test() {
    // Test normal actions to map
    let actions: HashSet<String> = HashSet::from([
        String::from("building.view"),
        String::from("building.create"),
        String::from("building.meter.view"),
        String::from("building.meter.readings.view"),
        String::from("building.meter.readings.edit"),
        String::from("user.view"),
    ]);

    let result_map = json!({
        "building": {
            "view": true,
            "create": true,
            "meter": {
                "view": true,
                "readings": {
                    "view": true,
                    "edit": true,
                },
            },
        },
        "user": {
            "view": true,
        },
    });

    assert_eq!(Value::Object(serialize_actions(&actions)), result_map);

    // Test empty actions to map
    let actions: HashSet<String> = HashSet::from([]);
    let result_map = json!({});

    assert_eq!(Value::Object(serialize_actions(&actions)), result_map);

    // Test simple actions to map
    let actions: HashSet<String> = HashSet::from([
        String::from("building_view"),
        String::from("building_create"),
        String::from("user_view"),
    ]);
    let result_map = json!({
        "building_view": true,
        "building_create": true,
        "user_view": true,
    });

    assert_eq!(Value::Object(serialize_actions(&actions)), result_map);
}
