mod permission;
mod permission_manager;
use permission::Permission;
use permission_manager::PermissionManager;
use std::collections::HashSet;

fn main() {
  let pm = PermissionManager::from_actions(&HashSet::from([
    String::from("building.create"),
    String::from("building.view"),
    String::from("building.edit"),
    String::from("building.delete"),
    String::from("user.create"),
    String::from("user.view"),
    String::from("user.edit"),
    String::from("user.delete"),
  ]));

  let p1 = pm.perm_from_actions(&HashSet::from([
    String::from("building.create"),
    String::from("building.view"),
    String::from("building.edit"),
  ]));

  let p2 = pm.perm_from_actions(&HashSet::from([
    String::from("building.edit"),
    String::from("building.delete"),
  ]));

  let p3 = pm.perm_from_actions(&HashSet::from([String::from("building.edit")]));

  println!("Universe: {:#?}", pm.get_universe());
  println!("Union: {:#?}", p1.union(&p2));
  println!("Diff: {:#?}", p1.difference(&p2));
  println!("Contains 1: {:#?}", p1.contains(&p2));
  println!("Contains 2: {:#?}", p1.contains(&p3));
  println!("ContAction 1: {:#?}", p1.contains_action("building.edit"));
  println!("ContAction 2: {:#?}", p1.contains_action("building.delete"));

  let data = r#"
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

  let my_perm = Permission::from_json(data, &None);
  println!("From JSON: {:#?}", my_perm);
  println!("To JSON: {:#?}", my_perm.to_json());

  let pm2 = PermissionManager::from_json(
    r#"
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
    }"#,
  );

  let my_perm_json = pm2.perm_from_json(
    r#"
  {
      "user": {
          "delete": true
        }
  }"#,
  );
  println!("From JSON 2: {:#?}", my_perm_json);
}
