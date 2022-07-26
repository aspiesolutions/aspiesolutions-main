use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
// "object" referrs to the current user, machine, etc
pub enum Action {
    Create,
    Read,
    Update,
    Delete,
}
// whether to allow or deny access
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum Access {
    Allow,
    Deny,
}
// allows to differentiate between a machine performing the request
// and the user asking the machine to perform a request on its behalf
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum AccessType {
    // when a machine is performing an action on behalf of another object while the object is not currently loged in
    Offline,
    OnlineOnly,
}
// The default action is to deny.
impl std::default::Default for Access {
    fn default() -> Self {
        Self::Deny
    }
}

// pub struct ExternalGrant {
//     // the subject allowing/denying the action
//     grantor: crate::Id,
//     // the subject that is allowed to perform the action,
//     grantee: crate::Id,

//     action: Action,
//     access_type:AccessType,
//     access:Access
// }

// records the combination of what this is referring to,
pub struct Permission {
    id: crate::Id,
    subject: crate::Id,
    // if the action referrs to another object, we need it here
    external_subject: Option<crate::Id>,
    action: Action,
    access: Access,
    access_type: AccessType,
}
pub struct Permissions(Vec<Permission>);
/// a trait used by the datastore to get the owner of this struct
#[async_trait]
pub trait GetOwnerAsync {
    async fn get_owner_async(object_id: &crate::Id) -> Result<&crate::Id, crate::Error>;
}
#[async_trait]
pub trait GetPermissionsAsync {
    async fn get_permissions_async(id: &crate::Id) -> Result<Vec<Permission>, crate::Error>;
}
// i guess this is an ACL?
// S for subject O for the object
pub async fn enforce_permissions_async<Subject: GetPermissionsAsync, Object: GetOwnerAsync>(
    // the object that the request is referring to
    object_id: &crate::Id,
    // the object (user, machine) who is requesting this action
    subject: &crate::Id,
    requested_action: &Action,
    // whether or not we want
    access_type: &AccessType,
) -> Result<Access, crate::Error> {
    // get the permissions for this subject
    let owner = Object::get_owner_async(object_id).await?;
    let sub_permissions = Subject::get_permissions_async(subject).await?;

    // if the owner is the same as the subject,
    // and there is an explicit "DENY" entry present in the list
    // macthing the above inputs. deny access.
    if owner == subject
        && sub_permissions
            .iter()
            .find(|p| {
                p.subject == *subject
                    && p.action == *requested_action
                    && p.access_type == *access_type
                    && p.access == Access::Deny
            })
            .is_some()
    {
        // deny access whenever there is an explicit deny entry
        return Ok(Access::Deny);
    } else if owner == subject {
        return Ok(Access::Deny);
    }
    let owner_permissions = Subject::get_permissions_async(owner).await?;
    // if the subject is not the owner of this object
    // search the permissions list of the owner of this object
    // for an explicit Deny
    if owner_permissions
        .iter()
        .find(|p| {
            p.subject == *owner
                && p.action == *requested_action
                && p.access_type == *access_type
                && p.access == Access::Deny
        })
        .is_some()
    {
        // deny in this case
        return Ok(Access::Deny);
    } else if owner_permissions
        .iter()
        .find(|p| {
            p.subject == *owner
                && p.action == *requested_action
                && p.access_type == *access_type
                && p.access == Access::Allow
        })
        .is_some()
    {
        // allow in this case
        return Ok(Access::Allow);
    } else {
        // when no permission is specified
        return Ok(Access::default());
    }

    // let permission =
    // Ok(Access::default())
}
