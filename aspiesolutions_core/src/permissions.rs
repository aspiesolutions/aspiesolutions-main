use async_trait::async_trait;
use const_format::concatcp;

use serde::{Deserialize, Serialize};

// i want to emulate the linux permission system
// Owner group

/// used to seperate and combine scope segments
pub const SCOPE_SEPERATOR: & str = ":";
///  used to separate and combine lists of scopes;
pub const SCOPE_LIST_SEPERATOR: & str = " ";
/// the label given for access equivelent to 'root' permissions
pub const SUPERUSER_SCOPE_PREFIX: & str = "superuser";
pub const ACTION_READ_STR: & str = "read";
pub const ACTION_UPDATE_STR: & str = "update";
pub const ACTION_CREATE_STR: & str = "create";
pub const ACTION_DELETE_STR: & str = "delete";
/// A special case in the auth system to allow full READ access.
///
/// USE CAREFULLY AND REQUIRE ADDITIONAL PROTECTION FOR SUPERUSERS
pub const SUPERUSER_READ_SCOPE: & str =
    concatcp!(SUPERUSER_SCOPE_PREFIX, SCOPE_SEPERATOR, ACTION_READ_STR);
/// A special case in the auth system to allow full UPDATE access.
///
/// USE CAREFULLY AND REQUIRE ADDITIONAL PROTECTION FOR SUPERUSERS
pub const SUPERUSER_UPDATE_SCOPE: & str =
    concatcp!(SUPERUSER_SCOPE_PREFIX, SCOPE_SEPERATOR, ACTION_UPDATE_STR);
/// A special case in the auth system to allow full DELETE access.
///
/// USE CAREFULLY AND REQUIRE ADDITIONAL PROTECTION FOR SUPERUSERS
pub const SUPERUSER_DELETE_SCOPE: & str =
    concatcp!(SUPERUSER_SCOPE_PREFIX, SCOPE_SEPERATOR, ACTION_DELETE_STR);
/// A special case in the auth system to allow full CREATE access.
///
/// USE CAREFULLY AND REQUIRE ADDITIONAL PROTECTION FOR SUPERUSERS
pub const SUPERUSER_CREATE_SCOPE: & str =
    concatcp!(SUPERUSER_SCOPE_PREFIX, SCOPE_SEPERATOR, ACTION_CREATE_STR);
pub const OFFLINE_ACCESS_SCOPE: & str = "offline_access";
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
// what actions the user can perform

pub enum Action {
    Create,
    Read,
    Update,
    Delete,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Read => write!(f, "{}", ACTION_READ_STR),
            Self::Create => write!(f, "{}", ACTION_CREATE_STR),
            Self::Update => write!(f, "{}", ACTION_UPDATE_STR),
            Self::Delete => write!(f, "{}", ACTION_DELETE_STR),
        }
    }
}

impl std::str::FromStr for Action {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            ACTION_CREATE_STR => Ok(Self::Create),
            ACTION_READ_STR => Ok(Self::Read),
            ACTION_UPDATE_STR => Ok(Self::Update),
            ACTION_DELETE_STR => Ok(Self::Delete),
            _ => Err(Self::Err::FromStrError(format!(
                "'{}' is not a valid identifier for type Action",
                &s
            ))),
        }
    }
}

// whether to allow or deny access
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Access {
    Allow,
    Deny,
}
impl std::fmt::Display for Access {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Allow => write!(f, "allow"),
            Self::Deny => write!(f, "deny"),
        }
    }
}
// allows to differentiate between a machine performing the request
// and the user asking the machine to perform a request on its behalf
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AccessType {
    // when a machine is performing an action on behalf of another object while the object is not currently loged in
    Offline,
    Online,
}
impl std::fmt::Display for AccessType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Offline => write!(f, "offline"),
            Self::Online => write!(f, "online"),
        }
    }
}

// The default action is to deny.
impl std::default::Default for Access {
    fn default() -> Self {
        Self::Deny
    }
}

pub struct Permission {
    #[allow(unused)]
    id: crate::Id,
    subject: crate::Id,
    // if the action referrs to another object, we need it here
    struct_name: String,
    action: Action,
    access: Access,
    access_type: AccessType,
}
pub struct Permissions(Vec<Permission>);
/// a trait used by the datastore to get the owner of this struct
#[async_trait]
pub trait GetOwnerAsync<Subject> {
    async fn get_owner_async(&self) -> Result<&Subject, crate::Error>;
}
#[async_trait]
pub trait GetMetaPermissionsAsync {
    async fn get_meta_permissions_async(
        &self,
    ) -> Result<std::collections::HashMap<crate::Id, Vec<Permission>>, crate::Error>;
}
pub trait GetUniqueIdentifer {
    fn get_unique_identifer(&self) -> crate::Id;
}
pub trait GetStructName {
    fn get_struct_name() -> &'static str;
}

pub async fn enforce_access_async<
    Subject: GetMetaPermissionsAsync + GetUniqueIdentifer + GetStructName,
    Object: GetMetaPermissionsAsync + GetOwnerAsync<Subject> + GetUniqueIdentifer + GetStructName,
>(
    subject: &Subject,
    object: &Object,
    action: &Action,
    // access_type: &AccessType,
    scopes: &str,
    meta_permissions: std::collections::HashMap<crate::Id, Vec<Permission>>,
) -> Result<Access, crate::Error> {
    let access_type = {
        if scopes.contains(OFFLINE_ACCESS_SCOPE) {
            AccessType::Offline
        } else {
            AccessType::Online
        }
    };
    // special cases for superusers. will be logged;
    if (*action == Action::Create && scopes.contains(SUPERUSER_CREATE_SCOPE))
        || (*action == Action::Read && scopes.contains(SUPERUSER_READ_SCOPE))
        || (*action == Action::Update && scopes.contains(SUPERUSER_UPDATE_SCOPE))
        || (*action == Action::Delete && scopes.contains(SUPERUSER_DELETE_SCOPE))
    {
        log::info!("Subject has superuser permissions. granting access");
        return Ok(Access::Allow);
    }
    let subject_id = subject.get_unique_identifer();
    let owner_id: crate::Id = object.get_owner_async().await?.get_unique_identifer();
    let object_id: crate::Id = object.get_unique_identifer();
    let action_str = action.to_string();
    let subject_struct_name: &'static str = Subject::get_struct_name();
    let object_struct_name: &'static str = Object::get_struct_name();
    log::debug!(
        "subject: '{}' with id '{subject_id}' requesting action '{action}' with access type '{access_type}' on object type '{}' with id {object_id} ",
        subject_struct_name,
        object_struct_name
    );
    let scope_string = format!("{}{}{}", subject_struct_name, SCOPE_SEPERATOR, action_str);
    log::debug!("scope: '{scope_string}'");

    // actions will be allowed if the owner of this object is the same as the subject requesting it only if there is not an explicit
    // deny entry
    log::info!("Subject is not a superuser.");
    if owner_id == subject_id
        && meta_permissions.contains_key(&subject_id)
        && meta_permissions
            .get(&subject_id)
            .unwrap()
            .iter()
            .any(|p| {
                p.subject == subject_id
                    && p.access_type == access_type
                    && p.struct_name == subject_struct_name
                    && p.action == *action
                    && p.access == Access::Deny
            })

    {
        log::info!("Subject has an explicit deny entry in the meta-permissions. Denying access");
        return Ok(Access::Deny);
    }
    if owner_id == subject_id {
        log::info!("Subject is the owner of the current Object. Granting access");
        return Ok(Access::Allow);
    }

    if scopes.contains(&scope_string)
        && meta_permissions.contains_key(&owner_id)
        && meta_permissions
            .get(&owner_id)
            .unwrap()
            .iter()
            .any(|p| {
                p.subject == subject_id
                    && p.struct_name == subject_struct_name
                    && p.access_type == access_type
                    && p.action == *action
                    && p.access == Access::Allow
            })
    {
        log::info!("The owner '{owner_id}' of the current object '{object_id}' has granted access to the current object. allowing access");
        return Ok(Access::Allow);
    }
    let default_access = Access::default();
    log::info!("No access conditions were met. access result '{default_access}'");
    Ok(default_access)
}

// // i guess this is an ACL?
// // S for subject O for the object
// pub async fn enforce_permissions_list_async<Subject: GetPermissionsAsync, Object: GetOwnerAsync>(
//     // the object that the request is referring to
//     object_id: &crate::Id,
//     // the object (user, machine) who is requesting this action
//     subject: &crate::Id,
//     requested_action: &Action,
//     // whether or not we want
//     access_type: &AccessType,
// ) -> Result<Access, crate::Error> {
//     // get the permissions for this subject
//     let owner = Object::get_owner_async(object_id).await?;
//     let sub_permissions = Subject::get_permissions_async(subject).await?;

//     // if the owner is the same as the subject,
//     // and there is an explicit "DENY" entry present in the list
//     // macthing the above inputs. deny access.
//     if owner == subject
//         && sub_permissions
//             .iter()
//             .find(|p| {
//                 p.subject == *subject
//                     && p.action == *requested_action
//                     && p.access_type == *access_type
//                     && p.access == Access::Deny
//             })
//             .is_some()
//     {
//         // deny access whenever there is an explicit deny entry
//         return Ok(Access::Deny);
//     } else if owner == subject {
//         // otherwise allow access
//         return Ok(Access::Deny);
//     }
//     let owner_permissions = Subject::get_permissions_async(owner).await?;
//     // if the subject is not the owner of this object
//     // search the permissions list of the owner of this object
//     // for an explicit Deny
//     if owner_permissions
//         .iter()
//         .find(|p| {
//             p.subject == *owner
//                 && p.action == *requested_action
//                 && p.access_type == *access_type
//                 && p.access == Access::Deny
//         })
//         .is_some()
//     {
//         // deny in this case
//         return Ok(Access::Deny);
//     } else if owner_permissions
//         .iter()
//         .find(|p| {
//             p.subject == *owner
//                 && p.action == *requested_action
//                 && p.access_type == *access_type
//                 && p.access == Access::Allow
//         })
//         .is_some()
//     {
//         // allow in this case
//         return Ok(Access::Allow);
//     } else {
//         // when no permission is specified
//         return Ok(Access::default());
//     }

//     // let permission =
//     // Ok(Access::default())
// }
