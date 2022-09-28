// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Solid Terms` vocabulary
//!
//! This module requires `ns-solid` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Solid Terms|
//! |**Prefix**|solid|
//! |**Namespace base IRI**|<http://www.w3.org/ns/solid/terms#>|
//! |**Description**|The Solid Terms vocabulary defines terms referenced in Solid specifications.|
//! |**Is defined by**|<http://www.w3.org/ns/solid/terms>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/solid/terms#",;
    /// `Solid Terms`: The Solid Terms vocabulary defines terms referenced in Solid specifications.
    NAMESPACE_BASE, "",
    /// `Account`: A Solid account.
    Account, "Account",
    /// `Inbox`: A resource containing notifications.
    Inbox, "Inbox",
    /// `Insert/delete patch`: A class of patch expressing insertions, deletions, and conditional modifications to a resource that has an RDF-based representation.
    InsertDeletePatch, "InsertDeletePatch",
    /// `Listed Type Index`: Listed Type Index is a registry of resources that are publicly discoverable by outside users and applications.
    ListedDocument, "ListedDocument",
    /// `Notification`: A notification resource.
    Notification, "Notification",
    /// `Patch`: A patch expresses conditional modifications to a resource that has an RDF-based representation.
    Patch, "Patch",
    /// `Timeline`: A resource containing time ordered items and sub-containers.  Sub-containers may be desirable in file based systems to split the timeline into logical components e.g. /yyyy-mm-dd/ as used in ISO 8061.
    Timeline, "Timeline",
    /// `Type index`: A index of type registries for resources. Applications can register the RDF type they use and list them in the index resource.
    TypeIndex, "TypeIndex",
    /// `Type Registration`: The registered types that map a RDF classes/types to their locations using either `instance` or `instanceContainer` property.
    TypeRegistration, "TypeRegistration",
    /// `Unlisted Type Index`: Unlisted Type Index is a registry of resources that are private to the user and their apps, for types that are not publicly discoverable.
    UnlistedDocument, "UnlistedDocument",
    /// `account`: A solid account belonging to an Agent.
    account, "account",
    /// `deletes`: The triple patterns this patch removes from the document.
    deletes, "deletes",
    /// `registry class`: A class that is used to map an listed or unlisted type index.
    forClass, "forClass",
    /// `inbox (deprecated)`: Deprecated pointer to a Linked Data Notifications inbox; please use http://www.w3.org/ns/ldp#inbox instead.
    inbox, "inbox",
    /// `inserts`: The triple patterns this patch adds to the document.
    inserts, "inserts",
    /// `instance`: Maps a type to an individual resource, typically an index or a directory listing resource.
    instance, "instance",
    /// `instance container`: Maps a type to a container which the client would have to list to get the instances of that type.
    instanceContainer, "instanceContainer",
    /// `loginEndpoint`: The login URI of a given server.
    loginEndpoint, "loginEndpoint",
    /// `logoutEndpoint`: The logout URI of a given server.
    logoutEndpoint, "logoutEndpoint",
    /// `notification`: Notification resource for an inbox.
    notification, "notification",
    /// `OIDC issuer`: The preferred OpenID Connect issuer URI for a given WebID.
    oidcIssuer, "oidcIssuer",
    /// `patches`: The document to which this patch applies.
    patches, "patches",
    /// `private type index`: Points to an unlisted type index resource.
    privateTypeIndex, "privateTypeIndex",
    /// `public type index`: Points to a listed type index resource.
    publicTypeIndex, "publicTypeIndex",
    /// `read`: Indicates if a message has been read or not. This property should have a boolean datatype.
    read, "read",
    /// `Non-volatile memory quota`: The quota of non-volatile memory that is available for the account (in bytes)
    storageQuota, "storageQuota",
    /// `Non-volatile memory usage`: The amount of non-volatile memory that the account have used (in bytes)
    storageUsage, "storageUsage",
    /// `timeline`: Timeline for a given resource.
    timeline, "timeline",
    /// `type index`: Points to a TypeIndex resource.
    typeIndex, "typeIndex",
    /// `where`: The conditions the document and the inserted and deleted triple patterns need to satisfy in order for the patch to be applied.
    where_, "where"
);
