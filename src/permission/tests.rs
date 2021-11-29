use super::*;

#[test]
fn from_actions_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let id = Uuid::new_v4();
    let p = Permission::from_actions(&actions, &Some(id));
    let p2 = Permission::from_actions(&actions, &None);

    assert_eq!(actions, p.actions);
    assert_eq!(Some(id), p.manager_id);
    assert_eq!(None, p2.manager_id);
}

#[test]
fn get_actions_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let id = Uuid::new_v4();
    let p = Permission::from_actions(&actions, &Some(id));

    assert_eq!(actions, *p.get_actions());
}

#[test]
fn is_managed_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let id = Uuid::new_v4();
    let managed = Permission::from_actions(&actions, &Some(id));
    let not_managed = Permission::from_actions(&actions, &None);

    assert_eq!(managed.is_managed(), true);
    assert_eq!(not_managed.is_managed(), false);
}

#[test]
fn has_same_manager_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(&actions, &Some(id));
    let p2 = Permission::from_actions(&actions, &Some(id));
    let p3 = Permission::from_actions(&actions, &Some(Uuid::new_v4()));
    let p4 = Permission::from_actions(&actions, &None);

    assert_eq!(p1.has_same_manager(&p2), true);
    assert_eq!(p1.has_same_manager(&p3), false);
    assert_eq!(p1.has_same_manager(&p4), false);
}

#[test]
fn can_operate_with_test() {
    let actions = HashSet::from([String::from("view"), String::from("create")]);
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(&actions, &Some(id));
    let p2 = Permission::from_actions(&actions, &Some(id));
    let p3 = Permission::from_actions(&actions, &Some(Uuid::new_v4()));
    let not_managed = Permission::from_actions(&actions, &None);

    assert_eq!(p1.can_operate_with(&p2), true);
    assert_eq!(p1.can_operate_with(&p3), false);
    assert_eq!(p1.can_operate_with(&not_managed), false);
}

#[test]
fn union_test_no_duplicated() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let copy_p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    assert_eq!(
        *p1.union(&copy_p1).get_actions(),
        HashSet::from([String::from("view"), String::from("create"),])
    );
}

#[test]
fn union_test_overlap() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let overlap_p1 = Permission::from_actions(
        &HashSet::from([String::from("create"), String::from("delete")]),
        &Some(id),
    );

    assert_eq!(
        *p1.union(&overlap_p1).get_actions(),
        HashSet::from([
            String::from("view"),
            String::from("create"),
            String::from("delete")
        ])
    );
}

#[test]
fn union_test_empty() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let empty = Permission::from_actions(&HashSet::from([]), &Some(id));

    assert_eq!(
        *p1.union(&empty).get_actions(),
        HashSet::from([String::from("view"), String::from("create")])
    );
}

#[test]
fn union_test_diff_manager_catch_unwind() {
    // Test func panics if perms do not have same manager_id (using catch_unwind - know of duplicated test)
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let diff_id = Permission::from_actions(&HashSet::from([]), &Some(Uuid::new_v4()));
    let none_id = Permission::from_actions(&HashSet::from([]), &None);

    match std::panic::catch_unwind(|| {
        assert_eq!(
            *p1.union(&diff_id).get_actions(),
            HashSet::from([String::from("view"), String::from("create")])
        );
    }) {
        Ok(_) => panic!("operation with perm with different manager_id should have panicked"),
        Err(_) => (),
    }

    match std::panic::catch_unwind(|| {
        assert_eq!(
            *p1.union(&none_id).get_actions(),
            HashSet::from([String::from("view"), String::from("create")])
        );
    }) {
        Ok(_) => panic!("operation with perm with different manager_id should have panicked"),
        Err(_) => (),
    }
}

#[test]
#[should_panic]
fn union_test_diff_manager() {
    // Test func panics if perms do not have same manager_id (using should_panic)
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let diff_id = Permission::from_actions(&HashSet::from([]), &Some(Uuid::new_v4()));

    assert_eq!(
        *p1.union(&diff_id).get_actions(),
        HashSet::from([String::from("view"), String::from("create")])
    );
}

#[test]
#[should_panic]
fn union_test_none_manager() {
    // Test func panics if perms do not have same manager_id (using should_panic)
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let none_id = Permission::from_actions(&HashSet::from([]), &None);

    assert_eq!(
        *p1.union(&none_id).get_actions(),
        HashSet::from([String::from("view"), String::from("create")])
    );
}

#[test]
fn difference_test_from_empty() {
    let id = Uuid::new_v4();
    let empty = Permission::from_actions(&HashSet::from([]), &Some(id));
    let full = Permission::from_actions(&HashSet::from([String::from("view")]), &Some(id));

    assert_eq!(*empty.difference(&full).get_actions(), HashSet::from([]));
}

#[test]
fn difference_test_perm_empty() {
    let id = Uuid::new_v4();
    let empty = Permission::from_actions(&HashSet::from([]), &Some(id));
    let full = Permission::from_actions(&HashSet::from([String::from("view")]), &Some(id));

    assert_eq!(
        *full.difference(&empty).get_actions(),
        HashSet::from([String::from("view")])
    );
}

#[test]
fn difference_test_diff_items() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let p2 = Permission::from_actions(
        &HashSet::from([String::from("delete"), String::from("edit")]),
        &Some(id),
    );

    assert_eq!(
        *p1.difference(&p2).get_actions(),
        HashSet::from([String::from("view"), String::from("create")])
    );
}

#[test]
fn difference_test_overlap() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let p2 = Permission::from_actions(
        &HashSet::from([String::from("create"), String::from("edit")]),
        &Some(id),
    );

    assert_eq!(
        *p1.difference(&p2).get_actions(),
        HashSet::from([String::from("view")])
    );
}

#[test]
fn difference_test_perm_diff_manager() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(&HashSet::from([String::from("view")]), &Some(id));
    let diff_id = Permission::from_actions(&HashSet::from([]), &Some(Uuid::new_v4()));
    let none_id = Permission::from_actions(&HashSet::from([]), &None);

    match std::panic::catch_unwind(|| {
        assert_eq!(
            *p1.union(&diff_id).get_actions(),
            HashSet::from([String::from("view"), String::from("create")])
        );
    }) {
        Ok(_) => panic!("operation with perm with different manager_id should have panicked"),
        Err(_) => (),
    }

    match std::panic::catch_unwind(|| {
        assert_eq!(
            *p1.union(&none_id).get_actions(),
            HashSet::from([String::from("view"), String::from("create")])
        );
    }) {
        Ok(_) => panic!("operation with perm with different manager_id should have panicked"),
        Err(_) => (),
    }
}

#[test]
fn contains_test() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([
            String::from("view"),
            String::from("create"),
            String::from("edit"),
        ]),
        &Some(id),
    );
    let p2 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let p3 = Permission::from_actions(
        &HashSet::from([
            String::from("view"),
            String::from("create"),
            String::from("delete"),
        ]),
        &Some(id),
    );
    let p4 = Permission::from_actions(&HashSet::from([String::from("delete")]), &Some(id));

    assert_eq!(p1.contains(&p2), true);
    assert_eq!(p1.contains(&p3), false);
    assert_eq!(p1.contains(&p4), false);
}

#[test]
fn contains_test_from_empty() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(&HashSet::from([]), &Some(id));
    let p2 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );

    assert_eq!(p1.contains(&p2), false);
}

#[test]
fn contains_test_perm_empty() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(
        &HashSet::from([String::from("view"), String::from("create")]),
        &Some(id),
    );
    let p2 = Permission::from_actions(&HashSet::from([]), &Some(id));

    assert_eq!(p1.contains(&p2), true);
}

#[test]
fn contains_test_perm_diff_manager() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(&HashSet::from([String::from("view")]), &Some(id));
    let diff_id = Permission::from_actions(&HashSet::from([]), &Some(Uuid::new_v4()));
    let none_id = Permission::from_actions(&HashSet::from([]), &None);

    match std::panic::catch_unwind(|| {
        assert_eq!(p1.contains(&diff_id), false);
    }) {
        Ok(_) => panic!("operation with perm with different manager_id should have panicked"),
        Err(_) => (),
    }
    match std::panic::catch_unwind(|| {
        assert_eq!(p1.contains(&none_id), false);
    }) {
        Ok(_) => panic!("operation with perm with different manager_id should have panicked"),
        Err(_) => (),
    }
}

#[test]
fn contains_action_test() {
    let id = Uuid::new_v4();
    let p1 = Permission::from_actions(&HashSet::from([String::from("view"),String::from("create")]), &Some(id));

    assert_eq!(p1.contains_action(&String::from("view")), true);
    assert_eq!(p1.contains_action(&String::from("delete")), false);
    assert_eq!(p1.contains_action(&String::from("other")), false);
    assert_eq!(p1.contains_action(&String::from("")), false);
}