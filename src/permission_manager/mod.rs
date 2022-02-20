use crate::permission::Permission;
use std::collections::HashSet;
use uuid::Uuid;

//// TESTS ////
#[cfg(test)]
mod tests;

/// This is usually what you want to use along with the [`Permission`](crate::Permission) struct.  
/// Allows [`Permission`](crate::Permission) instantiation and validation against a defined reference configuration.
///
/// A [`PermissionManager`](crate::PermissionManager) is created using a reference permission configuration (universe) of the permissions it can hold.  
/// All [`Permission`](crate::Permission)s created by a [`PermissionManager`](crate::PermissionManager) will be validated against the reference [`Permission`](crate::Permission) (universe) used for this
/// [`PermissionManager`](crate::PermissionManager) and can only operate with other [`Permission`](crate::Permission)s belonging to the same [`PermissionManager`](crate::PermissionManager).
///
/// You may want read the [`Permission`](crate::Permission) section in order to clearly understand the [`PermissionManager`](crate::PermissionManager).
#[derive(Debug)]
pub struct PermissionManager {
    universe: Permission,
    id: Uuid,
}

impl PermissionManager {
    /// Creates a new [`PermissionManager`](crate::PermissionManager) with a universe [`Permission`](crate::Permission) containing all the actions specified in the actions set provided.
    ///
    /// # Notes:  
    /// Dots (.) in `universe_actions` elements are used in [`Permission`](crate::Permission) JSON serialization, do not use them unless you know what you are doing.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// // Actions to manage using the PermissionManager
    /// let actions = HashSet::from([
    ///    String::from("create"),
    ///    String::from("view"),
    ///    String::from("edit"),
    ///    String::from("delete"),
    /// ]);
    ///
    /// let manager = PermissionManager::from_actions(actions.clone());
    /// // Should print a HashSet containing the elements 'create', 'view', 'edit' and 'delete'.
    /// println!("Manager universe permission actions: {:#?}", manager.get_universe().get_actions());
    ///
    /// assert_eq!(actions, *manager.get_universe().get_actions());
    /// ```
    pub fn from_actions(universe_actions: HashSet<String>) -> PermissionManager {
        let id = Uuid::new_v4();

        PermissionManager {
            universe: Permission::from_actions_and_uuid(universe_actions, Some(id)),
            id,
        }
    }

    /// Creates a new [`PermissionManager`](crate::PermissionManager) with a universe [`Permission`](crate::Permission) containing all the actions specified in the actions JSON string provided.
    ///
    /// The adecuated format for the actions JSON string is explained [here](crate::Permission::from_json_and_uuid()).
    ///
    /// # Examples:
    ///
    /// Simple actions in JSON equivalent for actions used in [`from_actions` example](Self::from_actions()):
    /// ```
    /// use simple_perm_manager::PermissionManager;
    ///
    /// // Actions JSON equivalent for actions used in `from_actions` example:
    /// let simple_actions_json = String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true,
    ///     "delete": true
    /// }"#);
    ///
    /// let manager = PermissionManager::from_json(&simple_actions_json);
    /// // Should print a HashSet containing the elements 'create', 'view', 'edit' and 'delete'.
    /// println!("Manager universe permission actions resulting from JSON: {:#?}", manager.get_universe().get_actions());
    /// ```
    /// # Panics:
    ///
    /// Panics in the same cases that [Permission::from_json_and_uuid()](crate::Permission::from_json_and_uuid()) does.
    /// - Panics if `universe_actions_json` argument is not valid JSON string.
    /// - Panics if `universe_actions_json` argument is not valid format for Permission actions.
    /// - Panics if `universe_actions_json` argument is s JSON with objects nested to a depth of more than 20.
    pub fn from_json(universe_actions_json: &str) -> PermissionManager {
        let id = Uuid::new_v4();

        PermissionManager {
            universe: Permission::from_json_and_uuid(universe_actions_json, Some(id)),
            id,
        }
    }

    /// Returns the [`PermissionManager`](crate::PermissionManager) universe as a managed [`Permission`](crate::Permission).
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_actions(HashSet::from([
    ///    String::from("create"),
    ///    String::from("view"),
    ///    String::from("edit"),
    ///    String::from("delete"),
    /// ]));
    ///
    /// let universe = manager.get_universe();
    /// // Should print a Permission with an actions HashSet containing the elements 'create', 'view', 'edit' and 'delete'.
    /// println!("Universe permission: {:#?}", universe);
    /// ```
    pub fn get_universe(&self) -> Permission {
        self.universe.clone()
    }

    /// Returns `true` if the [`Permission`](crate::Permission) provided is valid for the [`PermissionManager`](crate::PermissionManager).
    /// A [`Permission`](crate::Permission) is valid if it is managed by this [`PermissionManager`](crate::PermissionManager) and its contained in the [`PermissionManager`](crate::PermissionManager) universe.
    ///
    /// # Examples:
    ///
    /// ```
    /// use simple_perm_manager::PermissionManager;
    /// use simple_perm_manager::Permission;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true
    /// }"#));
    ///
    /// // Create managed Permission from the previous PermissionManager
    /// let managed_perm = manager.perm_from_json(&String::from(r#"{
    ///     "create": true
    /// }"#));
    ///
    /// // Create UNmanaged Permission (actions do not have to be a subset of the manager universe actions)
    /// let unmanaged_perm = Permission::from_json_and_uuid(&String::from(r#"{
    ///     "other_action": true
    /// }"#), None);
    ///
    /// assert!(manager.validate_perm(&managed_perm));
    /// assert!(!manager.validate_perm(&unmanaged_perm));
    /// ```
    pub fn validate_perm(&self, perm: &Permission) -> bool {
        self.universe.has_same_manager(perm) && self.universe.contains(perm)
    }

    /// Returns a managed [`Permission`](crate::Permission) with the actions specified in the actions set provided.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_actions(HashSet::from([
    ///     String::from("create"),
    ///     String::from("view"),
    /// ]));
    ///
    /// // Create managed Permission from the previous PermissionManager
    /// let managed_perm = manager.perm_from_actions(HashSet::from([String::from("create")]));
    /// ```
    ///
    /// # Panics:
    ///
    /// Function panics if actions provided for the Permission are not included in the manager universe
    /// ```rust,should_panic
    /// use std::collections::HashSet;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_actions(HashSet::from([
    ///     String::from("create"),
    ///     String::from("view"),
    /// ]));
    ///
    /// // This line of code panics
    /// let panics = manager.perm_from_actions(HashSet::from([String::from("other_action")]));
    /// ```
    pub fn perm_from_actions(&self, actions: HashSet<String>) -> Permission {
        let perm = Permission::from_actions_and_uuid(actions, Some(self.id));

        if !self.validate_perm(&perm) {
            panic!("Actions for Permission creation not allowed in PermissionManager or Permission id does not correspond to Manager id")
        }

        perm
    }

    /// Returns a managed [`Permission`](crate::Permission) with the actions specified in the actions JSON string provided.
    ///
    /// # Examples:
    ///
    /// ```
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true
    /// }"#));
    ///
    /// // Create managed Permission from the previous PermissionManager
    /// let managed_perm = manager.perm_from_json(&String::from(r#"{"create": true}"#));
    /// ```
    ///
    /// # Panics:
    ///
    /// Function panics if actions provided for the Permission are not included in the manager universe
    /// ```rust,should_panic
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true
    /// }"#));
    ///
    /// // This line of code panics
    /// let panics = manager.perm_from_json(&String::from(r#"{"other_action": true}"#));
    /// ```
    pub fn perm_from_json(&self, actions_json: &str) -> Permission {
        let perm = Permission::from_json_and_uuid(actions_json, Some(self.id));

        if !self.validate_perm(&perm) {
            panic!("Actions for Permission creation not allowed in PermissionManager or Permission id does not correspond to Manager id")
        }

        perm
    }
}
