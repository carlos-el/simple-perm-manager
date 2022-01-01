//! Simple and flexible management and operation of permissions.
//!
//! Permissions can be created freely from a set of actions in order to easily operate, serialize, store or associate them.
//! This allows fine grained customization such as per object or per subject permission association.
//!
//! Also, permissions can be tied to a PermissionManager with a reference permission configuration for validation, instantiation and operations between linked permissions. This potentially grants more control and security to your permission management strategy.

mod permission;
pub use permission::Permission;
mod permission_manager;
pub use permission_manager::PermissionManager;