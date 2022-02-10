[![Test coverage](https://github.com/carlos-el/simple-perm-manager/actions/workflows/test-coverage.yml/badge.svg)](https://github.com/carlos-el/simple-perm-manager/actions/workflows/test-coverage.yml)
[![codecov](https://codecov.io/gh/carlos-el/simple-perm-manager/branch/main/graph/badge.svg?token=8CCZEX8MMN)](https://codecov.io/gh/carlos-el/simple-perm-manager)

`simple-perm-manager` is a very simple and flexible library for working and operating with permission sets.  
Allows permissions to be attached to a system as required by the developer (i.e: attribute based, per object permission, etc) while maintaining low granularity.

## Usage
Here it is an usage example for managed permissions in a blog based site.
```rust
use simple_perm_manager::PermissionManager

// Create manager with a reference permissions set.
let manager = PermissionManager::from_json(
  r#"{
    "post": {
      "view": true,
      "create": true,
      "edit": true,
      "delete": true,
      "publish": true,
      "comment": {
        "view": true,
        "create": true,
        "delete": true
      }
    },
    "user": {
      "view": true,
      "ban": true
    }
  }"#,
);

// Actions allowed for a unregistered user
let unregistered_user_perm = manager.perm_from_json(
  r#"{
    "post": {
      "view": true,
      "comment": {
        "view": true
      }
    }
  }"#,
);

// Actions allowed for a registered user
let registered_user_perm = manager.perm_from_json(
  r#"{
    "post": {
      "view": true,
      "comment": {
        "view": true,
        "create": true
      }
    },
    "user": {
      "view": true
    }
  }"#
);

// Actions allowed for an admin that can crete an manage posts
let admin_perm = manager.perm_from_json(
  r#"{
    "post": {
      "view": true,
      "create": true,
      "edit": true,
      "delete": true,
      "publish": true,
      "comment": {
        "view": true,
        "create": true
      }
    },
    "user": {
      "view": true
    }
  }"#,
);

// Actions allowed for an super_admin (everything)
let super_admin_perm = manager.perm_from_json(
  r#"{
    "post": {
      "view": true,
      "create": true,
      "edit": true,
      "delete": true,
      "publish": true,
      "comment": {
        "view": true,
        "create": true,
        "delete": true
      }
    },
    "user": {
      "view": true,
      "ban": true
    }
  }"#,
);

// Check access to a certain action like creating a post
assert_eq!(unregistered_user_perm.contains_action("post.create"), false);
assert_eq!(admin_perm.contains_action("post.create"), true);

// Check if super_admin can do everything an admin can.
assert_eq!(super_admin_perm.contains(&admin_perm), true);


// Obtain set of permissions for what a super_admin can do that an admin can not.
let diff = super_admin_perm.difference(&admin_perm);

// Should print actions 'post:comment:delete' and 'user:ban'
println!("super-admin/admin difference: \n{:#?}\n", diff.get_actions());

// This should print the same as JSON (useful for db storage)
println!("super-admin/admin difference as json: \n{:#?}\n", diff.to_json());


// Add permission to an admin for deleting comments in posts
let new_admin_perm = admin_perm
    .union(&manager.perm_from_actions(&HashSet::from([String::from("post.comment.delete")])));
    
// Should print admin actions plus 'post:comment:delete' action
println!("admin with additional perm as json: \n{:#?}", new_admin_perm.get_actions());
```
More examples, operations and info about unmanaged permissions can be found in the docs.

## Docs
Documentation can be generated and open using `cargo doc --open`.

## Future features
 - Role based system for permissions.
 - Node.js support.