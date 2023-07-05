// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Workspace Ontology` vocabulary
//!
//! This module requires `ns-pim` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Workspace Ontology|
//! |**Prefix**|pim|
//! |**Namespace base IRI**|<http://www.w3.org/ns/pim/space#>|
//! |**Description**|This ontology is for use in describing Workspaces.  Workspaces are places where data is stored and associated policies of privacy|
//! |**Is defined by**|<http://www.w3.org/ns/pim/space#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/ns/pim/space#",

    terms: [
        /// `ConfigurationFile`:
        (ConfigurationFile, "ConfigurationFile"),
        /// `access controlled storage`: A  storage is a space of URIs in which you can individually control for each resource     who has access to it.
        (ControlledStorage, "ControlledStorage"),
        /// `Master Workspace`: This is a workspace for storing the     information about the other workspaces.     As a user, you normally don't have to worry about it.
        (MasterWorkspace, "MasterWorkspace"),
        /// `personal storage`: A personal storage is a space of URIs in which you and only you have access to data,     you cannot give access to anyone else.
        (PersonalStorage, "PersonalStorage"),
        /// `Preferences workspace`: Aceess may not be open to the public. Contains preferences resources.
        (PreferencesWorkspace, "PreferencesWorkspace"),
        /// `Private workspace`: Access only by the you, the user.
        (PrivateWorkspace, "PrivateWorkspace"),
        /// `public storage`: A public storage is a space of URIs in which you have access to data,     and all data is accessible to anyone without control.
        (PublicStorage, "PublicStorage"),
        /// `Public workspace`: Aceess is open to the public. Anything in a public workspace     can be accesed by anyone.
        (PublicWorkspace, "PublicWorkspace"),
        /// `Shared workspace`: Access is to some but not all people.
        (SharedWorkspace, "SharedWorkspace"),
        /// `storage`: A storage is a space of URIs in which you have access to data.
        (Storage, "Storage"),
        /// `workspace`: Workspaces are place where data is stored, and associated polices of privacy. A given application typically stores information in several different workspaces, some being user private, some shared, and some public.
        (Workspace, "Workspace"),
        /// `master workspace`:
        (masterWorkspace, "masterWorkspace"),
        /// `preferences file`:
        (preferencesFile, "preferencesFile"),
        /// `storage`: The storage in which this workspace is, or the storage which contains this resource, or a storage available to this agent to use.
        (storage, "storage"),
        /// `URI prefix`: URIs which start with this string are in this workspace or storage. This may be used for constructing URIs for new storage resources.
        (uriPrefix, "uriPrefix"),
        /// `workspace`:
        (workspace, "workspace")    ]
);
