use super::*;

#[test]
fn from_actions_test() {
    let actions = HashSet::from([
        String::from("view"),
        String::from("create"),
        String::from("edit"),
        String::from("delete"),
    ]);
    let pm = PermissionManager::from_actions(actions.clone());

    assert_eq!(*pm.get_universe().get_actions(), actions);
}

#[test]
fn from_json_test() {
    // Create 2 equals sets of actions in different formats
    let actions = HashSet::from([
        String::from("building.view"),
        String::from("building.meter.create"),
        String::from("building.room.edit"),
        String::from("user.delete"),
    ]);

    let actions_json = r#"
    {
        "building": {
          "view": true,
          "meter": {
            "create": true
          },
          "room": {
            "edit": true
          }
        },
        "user": {
            "delete": true
          }
    }"#;

    // Create 2 manager using the actions sets
    let pm = PermissionManager::from_json(actions_json);
    let pm2 = PermissionManager::from_actions(actions);

    // Manager actions must be the same as set actions are equal
    assert_eq!(*pm.get_universe().get_actions(), *pm2.get_universe().get_actions());
}

#[test]
fn get_universe_test() {
    let actions = HashSet::from([
        String::from("view"),
        String::from("create"),
        String::from("edit"),
        String::from("delete"),
    ]);
    let pm = PermissionManager::from_actions(actions.clone());

    assert_eq!(*pm.get_universe().get_actions(), actions);
}

#[test]
fn validate_perm_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let actions2 = HashSet::from([
        String::from("view"),
        String::from("create"),
        String::from("edit"),
        String::from("delete"),
    ]);
    let pm = PermissionManager::from_actions(actions.clone());
    let pm_second = PermissionManager::from_actions(actions.clone());

    let managed_perm = pm.perm_from_actions(actions.clone());
    let managed_perm_diff_manager = pm_second.perm_from_actions(actions.clone());
    let unmanaged = Permission::from_actions(&actions, &None);
    let unmanaged_diff_actions = Permission::from_actions(&actions2, &None);
    // Only possible to create this perm in tests as PermissionManager id (pm.id) should not be obtained in code
    let false_managed_perm_diff_actions = Permission::from_actions(&actions2, &Some(pm.id));

    assert_eq!(pm.validate_perm(&managed_perm), true);
    assert_eq!(pm.validate_perm(&managed_perm_diff_manager), false);
    assert_eq!(pm.validate_perm(&unmanaged), false);
    assert_eq!(pm.validate_perm(&unmanaged_diff_actions), false);
    assert_eq!(pm.validate_perm(&false_managed_perm_diff_actions), false);
}

#[test]
fn perm_from_actions_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let pm = PermissionManager::from_actions(actions.clone());

    match std::panic::catch_unwind(|| {
        pm.perm_from_actions(HashSet::from([String::from("other")]));
    }) {
        Ok(_) => panic!("actions supplied for creating a permission are not present in permssion manager universe"),
        Err(_) => (),
    }
}
