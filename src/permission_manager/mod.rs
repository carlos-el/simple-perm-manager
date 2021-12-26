use crate::permission::Permission;
use std::collections::HashSet;
use uuid::Uuid;

//// TESTS ////
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct PermissionManager {
    universe: Permission,
    id: Uuid,
}

impl PermissionManager {
    pub fn from_actions(universe_actions: &HashSet<String>) -> PermissionManager {
        let id = Uuid::new_v4();

        PermissionManager {
            universe: Permission::from_actions(&universe_actions, &Some(id)),
            id,
        }
    }

    pub fn from_json(actions_json: &str) -> PermissionManager {
        let id = Uuid::new_v4();

        PermissionManager {
            universe: Permission::from_json(actions_json, &Some(id)),
            id,
        }
    }

    pub fn get_universe(&self) -> Permission {
        self.universe.clone()
    }

    pub fn validate_perm(&self, perm: &Permission) -> bool {
        self.universe.has_same_manager(perm) && self.universe.contains(perm)
    }

    pub fn perm_from_actions(&self, actions: &HashSet<String>) -> Permission {
        let perm = Permission::from_actions(&actions, &Some(self.id));

        if !self.validate_perm(&perm) {
            panic!("Actions for Permission creation not allowed in PermissionManager or Permission id does not correspond to Manager id")
        }

        perm
    }

    pub fn perm_from_json(&self, actions_json: &str) -> Permission {
        let perm = Permission::from_json(actions_json, &Some(self.id));

        if !self.validate_perm(&perm) {
            panic!("Actions for Permission creation not allowed in PermissionManager or Permission id does not correspond to Manager id")
        }

        perm
    }
}
