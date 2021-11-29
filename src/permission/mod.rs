use std::collections::HashSet;
use uuid::Uuid;

// Struct for managing permissions. Permissions are modeled as a simple set of actions
// Operations can be done between them. 
// There are two types of permissions, managed and unmanaged:
//  - Managed permissions: 
//      - Have a manager_id different from None.
//      - Can only operate with other managed permissions with same manager_id.
//  - Unmanaged permissions: 
//      - Its manager_id is None.
//      - Can only operate with any other unmanaged permission.
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
    // pub fn from_json(actions_json: String, manager_id: &Option<Uuid>) -> Permission;
    // pub fn to_str(&self) -> String;
    // pub fn to_json(&self) -> String;

    pub fn get_actions(&self) -> &HashSet<String> {
        &self.actions
    }

    pub fn is_managed(&self) -> bool {
        !self.manager_id.is_none()
    }

    pub fn has_same_manager(&self, other: &Permission) -> bool {
        self.manager_id.eq(&other.manager_id)
    }

    pub fn can_operate_with(&self, other: &Permission) -> bool {
        self.is_managed() && other.is_managed() && self.has_same_manager(other)
    }

    pub fn union(&self, other: &Permission) -> Permission {
        if self.can_operate_with(other) {
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
        if self.can_operate_with(other) {
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
        if self.can_operate_with(other) {
            panic!("Permissions in contains operation do not have same manager");
        }

        return self.get_actions().is_superset(other.get_actions());
    }

    pub fn contains_action(&self, action_str: &str) -> bool {
        self.get_actions().contains(action_str)
    }
}
