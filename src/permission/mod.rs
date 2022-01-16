mod action_serialization;
use serde_json::Value;
use std::collections::HashSet;
use uuid::Uuid;

//// TESTS ////
#[cfg(test)]
mod tests;

/// Struct for basic [`Permission`](crate::Permission) instantiation, operation and serialization.  
/// [`Permission`](crate::Permission)s are modeled as a simple sets of actions that a
/// [`Permission`](crate::Permission) allows for certain subjects or objects (at your choice).  
/// - There are two types of [`Permission`](crate::Permission)s, managed and unmanaged:
///     - Managed permissions:
///         - Tied to a [`PermissionManager`](crate::PermissionManager).
///         - Can only operate with managed permissions of the same [`PermissionManager`](crate::PermissionManager).
///         - Created using a [`PermissionManager`](crate::PermissionManager) instance.
///     - Unmanaged permissions:
///         - Not tied to a [`PermissionManager`](crate::PermissionManager).
///         - Can only operate with any unmanaged permission.
///
/// Please note that, although possible, it is not advisable to use unmanaged permissions
/// unless your permissions are not really restricted by any common structure or management.
/// Instead your should probably use managed permissions within a [`PermissionManager`](crate::PermissionManager).
#[derive(Clone, Debug)]
pub struct Permission {
    actions: HashSet<String>,
    manager_id: Option<Uuid>,
}

impl Permission {
    /// Creates a new [`Permission`](crate::Permission) containing all the actions specified in the actions set provided.
    ///
    /// # Arguments
    ///
    /// * `actions` - HashSet of String elements, each one being a valid action for the [`Permission`](crate::Permission) created.
    /// * `manager_id` - Intended for use only by [`PermissionManager`](crate::PermissionManager) implementation.
    /// Set it to `None` if using [`Permission`](crate::Permission) without a [`PermissionManager`](crate::PermissionManager).
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use simple_perm_manager::Permission;
    ///
    /// let actions = HashSet::from([
    ///    String::from("create"),
    ///    String::from("view"),
    /// ]);
    ///
    /// let perm = Permission::from_actions(&actions, &None);
    /// // Should print a HashSet containing the elements 'create', 'view'.
    /// println!("Permission actions: {:#?}", perm.get_actions());
    ///
    /// assert_eq!(actions, *perm.get_actions());
    /// ```
    pub fn from_actions(actions: &HashSet<String>, manager_id: &Option<Uuid>) -> Permission {
        Permission {
            actions: actions.clone(),
            manager_id: *manager_id,
        }
    }

    /// Creates a new [`Permission`](crate::Permission) containing all the actions specified in the actions JSON string provided.
    ///
    /// # Arguments
    ///
    /// * `actions_json` - HashSet of String elements, each one being a valid action for the [`Permission`](crate::Permission) created.
    /// * `manager_id` - Intended for use only by [`PermissionManager`](crate::PermissionManager) implementation.
    /// Set it to `None` if using [`Permission`](crate::Permission) without a [`PermissionManager`](crate::PermissionManager).
    ///
    /// # JSON actions format:
    ///
    /// Actions for a [`Permission`](crate::Permission) can be modeled using JSON following some rules:  
    /// - Actions can be nested using JSON objects in order to create subgroups of actions.  
    /// - Last actions in a subgroup must always be boolean values.
    /// Set it to `true` to include this particular action in the [`Permission`](crate::Permission) or to `false` in order to exclude it.
    /// This is useful in case you are using a particular configuration and want to exclude actions but keeping the full picture of actions.
    /// - Values for an object can only be:
    ///     - Other object for creating a group.
    ///     - A boolean for the final action.
    /// - Maximum object nesting for JSON objects is set to 20.
    /// - Groups preceding the final word of the full action created are divided by dots.
    ///
    /// JSON simple actions.  
    /// Maps to actions: 'create', 'view', 'edit' and 'delete'.
    /// ```json
    /// {
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true,
    ///     "delete": true,
    /// }
    /// ```
    ///
    /// JSON with nested groups and actions excluded.  
    /// Maps to actions: 'user.create', 'user.edit', 'blog.view', 'blog.edit' and 'blog.comments.delete'.  
    /// This JSON has a nesting depth of 2 (because of blog and comments groups).
    /// ```json
    /// {
    ///     "user": {
    ///         "create": true,
    ///         "edit": true,
    ///         "delete": false
    ///     },
    ///     "blog": {
    ///         "view": true,
    ///         "edit": true,
    ///         "comments":{
    ///             "delete": true
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Examples:
    ///
    /// Simple actions in JSON.
    /// ```
    /// use simple_perm_manager::Permission;
    ///
    /// let simple_actions_json = String::from(r#"{
    ///     "create": true,
    ///     "view": true
    /// }"#);
    ///
    /// let perm = Permission::from_json(&simple_actions_json, &None);
    /// // Should print a HashSet containing the elements 'create', 'view'.
    /// println!("Permission actions: {:#?}", perm.get_actions());
    /// ```
    ///
    /// More complex actions JSON.
    /// ```
    /// use simple_perm_manager::Permission;
    ///
    /// let actions_json = String::from(r#"{
    ///     "user": {
    ///         "create": true,
    ///         "edit": true
    ///     },
    ///     "blog": {
    ///         "view": true
    ///     }
    /// }"#);
    ///
    /// let perm = Permission::from_json(&actions_json, &None);
    /// // Should print a HashSet containing the elements 'user.create', 'user.edit' and 'blog.view'.
    /// println!("Permission actions: {:#?}", perm.get_actions());
    /// ```
    ///
    /// # Panics:
    ///
    /// - Panics if `actions_json` argument is not valid JSON string.
    /// - Panics if `actions_json` argument is not valid format for Permission actions.
    /// - Panics if `actions_json` argument is s JSON with objects nested to a depth of more than 20.
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

    /// Returns [`Permission`](crate::Permission) actions in a JSON formatted string.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use simple_perm_manager::Permission;
    ///
    /// let perm = Permission::from_actions(&HashSet::from([
    ///    String::from("create"),
    ///    String::from("view"),
    /// ]), &None);
    ///
    /// // Should print a JSON string like "{"create": true, "view": true}".
    /// println!("Permission actions as JSON: {:#?}", perm.to_json());
    /// ```
    pub fn to_json(&self) -> String {
        Value::Object(action_serialization::serialize_actions(self.get_actions())).to_string()
    }

    /// Returns the [`Permission`](crate::Permission) actions.
    ///
    /// # Examples:
    ///
    /// ```
    /// use std::collections::HashSet;
    /// use simple_perm_manager::Permission;
    ///
    /// let perm = Permission::from_actions(&HashSet::from([
    ///    String::from("create"),
    ///    String::from("view"),
    /// ]), &None);
    ///
    /// // Should print an HashSet containing the elements 'create', and 'view'.
    /// println!("Permission actions as JSON: {:#?}", perm.get_actions());
    /// ```
    pub fn get_actions(&self) -> &HashSet<String> {
        &self.actions
    }

    /// Returns `true` if the [`Permission`](crate::Permission) is managed.  
    /// A [`Permission`](crate::Permission) has a manager mainly if it has been instantiated by a [`PermissionManager`](crate::PermissionManager).
    ///
    /// # Examples:
    ///
    /// ```
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true
    /// }"#));
    ///
    /// // Create managed Permission
    /// let managed_perm = manager.perm_from_json(&String::from(r#"{
    ///     "create": true
    /// }"#));
    ///
    /// // Create UNmanaged Permission
    /// let unmanaged_perm = Permission::from_json(&String::from(r#"{
    ///     "other_action": true
    /// }"#), &None);
    ///
    /// assert!(managed_perm.is_managed());
    /// assert!(!unmanaged_perm.is_managed());
    /// ```
    pub fn is_managed(&self) -> bool {
        self.manager_id.is_some()
    }

    /// Returns `true` if the [`Permission`](crate::Permission) calling the method has the same
    /// [`PermissionManager`](crate::PermissionManager) as the [`Permission`](crate::Permission) used as argument.
    ///
    /// # Examples:
    ///
    /// ```
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager_foo = PermissionManager::from_json(&String::from(r#"{"create": true}"#));
    /// let manager_bar = PermissionManager::from_json(&String::from(r#"{"create": true}"#));
    ///
    /// // Create 2 managed Permissions by manager_FOO
    /// let managed_perm_foo = manager_foo.perm_from_json(&String::from(r#"{"create": true}"#));
    /// let managed_perm_foo_2 = manager_foo.perm_from_json(&String::from(r#"{"create": true}"#));
    ///
    /// // Create managed Permission by manager_BAR
    /// let managed_perm_bar = manager_bar.perm_from_json(&String::from(r#"{"create": true}"#));
    ///
    /// // Create UNmanaged Permission
    /// let unmanaged_perm = Permission::from_json(&String::from(r#"{"create": true}"#), &None);
    ///
    /// assert!(managed_perm_foo.has_same_manager(&managed_perm_foo_2));
    /// assert!(!managed_perm_foo.has_same_manager(&managed_perm_bar));
    /// assert!(!managed_perm_foo.has_same_manager(&unmanaged_perm));
    /// ```
    pub fn has_same_manager(&self, other: &Permission) -> bool {
        self.manager_id.eq(&other.manager_id)
    }

    /// Returns a [`Permission`](crate::Permission) containing all the actions in the calling
    /// [`Permission`](crate::Permission) and in the [`Permission`](crate::Permission) used as argument.
    ///
    /// Actions in both sets do not created duplicate.
    ///
    /// # Examples:
    ///
    /// ```
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create 2 managed Permissions
    /// let perm_foo = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    /// let perm_bar = manager.perm_from_json(&String::from(r#"{"view": true, "edit": true}"#));
    ///
    /// // Make the union
    /// let perm_foo_bar = perm_foo.union(&perm_bar);
    ///
    /// // Actions in 'perm_foo_bar' should be 'create', 'view' and 'edit'.
    /// println!("'perm_foo_bar' actions as JSON: {:#?}", perm_foo_bar.to_json());
    /// ```
    ///
    /// # Panics:
    ///
    /// Panics if the calling [`Permission`](crate::Permission) and the [`Permission`](crate::Permission)
    /// used as argument do not have the same [`PermissionManager`](crate::PermissionManager).  
    /// A [`Permission`](crate::Permission) being managed and other being unmanaged count has not having the same manager.
    /// ```rust,should_panic
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create managed Permission and UNmanaged one
    /// let managed_perm = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    /// let unmanaged_perm = Permission::from_json(&String::from(r#"{"view": true, "edit": true}"#), &None);
    ///
    /// // This lines of code panics
    /// let panics = managed_perm.union(&unmanaged_perm);
    /// ```
    pub fn union(&self, other: &Permission) -> Permission {
        if !self.has_same_manager(other) {
            panic!("Permissions in union operation do not have same manager");
        }

        let actions_union: HashSet<String> = self
            .get_actions()
            .union(other.get_actions())
            .cloned()
            .collect();

        Permission::from_actions(&actions_union, &self.manager_id)
    }

    /// Returns a [`Permission`](crate::Permission) containing the actions that are in the calling
    /// [`Permission`](crate::Permission) but not in the [`Permission`](crate::Permission) used as argument.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create 2 managed Permissions
    /// let perm_foo_bar = manager.perm_from_json(&String::from(r#"{"create": true, "view": true, "edit": true}"#));
    /// let perm_bar = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    ///
    /// // Make the difference
    /// let perm_foo = perm_foo_bar.difference(&perm_bar);
    ///
    /// // Actions in 'perm_foo' should be just 'edit'.
    /// println!("'perm_foo' actions as JSON: {:#?}", perm_foo.to_json());
    /// ```
    /// 
    /// # Panics:
    ///
    /// Panics if the calling [`Permission`](crate::Permission) and the [`Permission`](crate::Permission)
    /// used as argument do not have the same [`PermissionManager`](crate::PermissionManager).  
    /// A [`Permission`](crate::Permission) being managed and other being unmanaged count has not having the same manager.
    /// ```rust,should_panic
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create managed Permission and UNmanaged one
    /// let managed_perm = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    /// let unmanaged_perm = Permission::from_json(&String::from(r#"{"view": true, "edit": true}"#), &None);
    ///
    /// // This lines of code panics
    /// let panics = managed_perm.difference(&unmanaged_perm);
    /// ```
    pub fn difference(&self, other: &Permission) -> Permission {
        if !self.has_same_manager(other) {
            panic!("Permissions in difference operation do not have same manager");
        }

        let actions_diff: HashSet<String> = self
            .get_actions()
            .difference(other.get_actions())
            .cloned()
            .collect();

        Permission::from_actions(&actions_diff, &self.manager_id)
    }

    /// Returns `true` if the [`Permission`](crate::Permission) calling contains at least 
    /// all the actions in the [`Permission`](crate::Permission) used as argument.
    /// 
    /// # Examples:
    /// 
    /// ```
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create 2 managed Permissions
    /// let perm_foo_bar = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    /// let perm_bar = manager.perm_from_json(&String::from(r#"{"create": true}"#));
    /// 
    /// assert!(perm_foo_bar.contains(&perm_bar));
    /// assert!(!perm_bar.contains(&perm_foo_bar));
    /// ```
    /// 
    /// # Panics:
    ///
    /// Panics if the calling [`Permission`](crate::Permission) and the [`Permission`](crate::Permission)
    /// used as argument do not have the same [`PermissionManager`](crate::PermissionManager).  
    /// A [`Permission`](crate::Permission) being managed and other being unmanaged count has not having the same manager.
    /// ```rust,should_panic
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create managed Permission and UNmanaged one
    /// let managed_perm = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    /// let unmanaged_perm = Permission::from_json(&String::from(r#"{"view": true, "edit": true}"#), &None);
    ///
    /// // This lines of code panics
    /// let panics = managed_perm.contains(&unmanaged_perm);
    /// ```
    pub fn contains(&self, other: &Permission) -> bool {
        if !self.has_same_manager(other) {
            panic!("Permissions in contains operation do not have same manager");
        }

        return self.get_actions().is_superset(other.get_actions());
    }

    /// Returns `true` if the [`Permission`](crate::Permission) calling contains the action used as argument.
    ///
    /// # Examples:
    /// 
    /// ```
    /// use simple_perm_manager::Permission;
    /// use simple_perm_manager::PermissionManager;
    ///
    /// let manager = PermissionManager::from_json(&String::from(r#"{
    ///     "create": true,
    ///     "view": true,
    ///     "edit": true
    /// }"#));
    ///
    /// // Create Permission
    /// let perm = manager.perm_from_json(&String::from(r#"{"create": true, "view": true}"#));
    /// 
    /// assert!(perm.contains_action("create"));
    /// assert!(!perm.contains_action("other"));
    /// ```
    pub fn contains_action(&self, action_str: &str) -> bool {
        self.get_actions().contains(action_str)
    }
}
