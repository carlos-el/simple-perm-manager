mod action_serialization;
use serde_json::Value;
use std::collections::HashSet;
use uuid::Uuid;

//// TESTS ////
#[cfg(test)]
mod tests;

/// Struct for basic `Permission` instantiation, operation and serialization.  
/// `Permission`s are modeled as a simple sets of actions.  
/// - There are two types of `Permission`s, managed and unmanaged:
///     - Managed permissions:
///         - Tied to a `PermissionManager`.
///         - Can only operate with managed permissions of the same `PermissionManager`.
///         - Created using a `PermissionManager` instance.
///     - Unmanaged permissions:
///         - Not tied to a `PermissionManager`.
///         - Can only operate with any unmanaged permission.
/// 
/// Please note that, although possible, it is not advisable to use unmanaged permissions 
/// unless your permissions are not really restricted by any common structure or management.
/// Instead your should probably use Managed permissions within a `PermissionManager`.
#[derive(Clone, Debug)]
pub struct Permission {
    actions: HashSet<String>,
    manager_id: Option<Uuid>,
}

impl Permission {
    pub fn from_actions(actions: &HashSet<String>, manager_id: &Option<Uuid>) -> Permission {
        Permission {
            actions: actions.clone(),
            manager_id: *manager_id,
        }
    }

    // TODO Not implemented
    // pub fn from_str(actions_str: String, manager_id: &Option<Uuid>) -> Permission;
    // pub fn to_str(&self) -> String;

    pub fn from_json(actions_json: &str, manager_id: &Option<Uuid>) -> Permission {
        let actions_generated: HashSet<String>;
        let actions_value: Value =
            serde_json::from_str(actions_json).expect("wrong format in permission json string");

        match actions_value {
            Value::Object(map) => {
                actions_generated = action_serialization::deserialize_actions(0, "", &map);
            }
            // This will never be reached as Values returned from a serde 'from_str' will always be Object
            _ => panic!("wrong format in permission json string"),
        }

        Permission {
            actions: actions_generated,
            manager_id: *manager_id,
        }
    }

    pub fn to_json(&self) -> String {
        Value::Object(action_serialization::serialize_actions(self.get_actions())).to_string()
    }

    pub fn get_actions(&self) -> &HashSet<String> {
        &self.actions
    }

    pub fn is_managed(&self) -> bool {
        self.manager_id.is_some()
    }

    pub fn has_same_manager(&self, other: &Permission) -> bool {
        self.manager_id.eq(&other.manager_id)
    }

    pub fn can_operate_with(&self, other: &Permission) -> bool {
        self.is_managed() && other.is_managed() && self.has_same_manager(other)
    }

    pub fn union(&self, other: &Permission) -> Permission {
        if !self.can_operate_with(other) {
            panic!("Permissions in union operation do not have same manager");
        }

        let actions_union: HashSet<String> = self
            .get_actions()
            .union(other.get_actions())
            .cloned()
            .collect();

        Permission::from_actions(&actions_union, &self.manager_id)
    }

    pub fn difference(&self, other: &Permission) -> Permission {
        if !self.can_operate_with(other) {
            panic!("Permissions in difference operation do not have same manager");
        }

        let actions_diff: HashSet<String> = self
            .get_actions()
            .difference(other.get_actions())
            .cloned()
            .collect();

        Permission::from_actions(&actions_diff, &self.manager_id)
    }

    pub fn contains(&self, other: &Permission) -> bool {
        if !self.can_operate_with(other) {
            panic!("Permissions in contains operation do not have same manager");
        }

        return self.get_actions().is_superset(other.get_actions());
    }

    pub fn contains_action(&self, action_str: &str) -> bool {
        self.get_actions().contains(action_str)
    }
}
