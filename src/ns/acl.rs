// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-acl")))]
//! This module provides terms for `acl` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|acl|
//! |**Namespace base IRI**|<http://www.w3.org/ns/auth/acl#>|
//! |**Description**||
//! |**Is defined by**|<http://www.w3.org/ns/auth/acl#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/auth/acl#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Access`: Any kind of access to a resource. Don't use this, use R W and RW
    Access, "Access",
    /// `append`: Append accesses are specific write access which only add information, and do not remove information.     For text files, for example, append access allows bytes to be added onto the end of the file.     For RDF graphs, Append access allows adds triples to the graph but does not remove any.     Append access is useful for dropbox functionality.     Dropbox can be used for link notification, which the information added is a notification     that a some link has been made elsewhere relevant to the given resource.     
    Append, "Append",
    /// `Anyone authenticated`: A class of agents who have been authenticated. In other words, anyone can access this resource, but not anonymously. The social expectation is that the authentication process will provide an identify and a name, or pseudonym. (A new ID should not be minted for every access: the intent is that the user is able to continue to use the ID for continues interactions with peers, and for example to develop a reputation) 
    AuthenticatedAgent, "AuthenticatedAgent",
    /// `authorization`: An element of access control,     allowing agent to agents access of some kind to resources or classes of resources
    Authorization, "Authorization",
    /// `control`: Allows read/write access to the ACL for the resource(s)
    Control, "Control",
    /// `Origin`: An Origin is basically a web site         (Note WITHOUT the trailing slash after the domain name and port in its URI)         and is the basis for controlling access to data by web apps         in the Same Origin Model of web security.         All scripts from the same origin are given the same right.
    Origin, "Origin",
    /// `read`: The class of read operations
    Read, "Read",
    /// `write`: 
    Write, "Write",
    /// `access control`: The Access Control file for this information resource.         This may of course be a virtual resource implemented by the access control system.         Note also HTTP's header  Link:  foo.meta ;rel=meta can be used for this.
    accessControl, "accessControl",
    /// `to`: The information resource to which access is being granted.
    accessTo, "accessTo",
    /// `to all in`: A class of information resources to which access is being granted.
    accessToClass, "accessToClass",
    /// `agent`: A person or social entity to being given the right
    agent, "agent",
    /// `agent class`: A class of persons or social entities to being given the right
    agentClass, "agentClass",
    /// `agent group`: A group of persons or social entities to being given the right.           The right is given to any entity which is a vcard:member of the group,           as defined by the document received when the Group is dereferenced.
    agentGroup, "agentGroup",
    /// `default access for things in this`: If a resource has no ACL file (it is 404),         then access to the resource if given by the ACL of the immediately         containing directory, or failing that (404) the ACL of the recursively next         containing directory which has an ACL file.         Within that ACL file,         any Authentication which has that directory as its acl:default applies to the         resource. (The highest directory must have an ACL file.) 
    default, "default",
    /// `default access for new things in the object`: THIS IS OBSOLETE AS OF 2017-08-01.   See 'default'.         Was: A directory for which this authorization is used for new files in the directory.
    defaultForNew, "defaultForNew",
    /// `delegates`: Delegates a person or another agent to act on behalf of the agent.     For example, Alice delegates Bob to act on behalf of Alice for ACL purposes.
    delegates, "delegates",
    /// `label`: 
    label, "label",
    /// `access mode`: A mode of access such as read or write.
    mode, "mode",
    /// `origin`: A web application, identified by its Origin, such as         <https://scripts.example.com>, being given the right.         When a user of the web application at a certain origin accesses the server,         then the browser sets the Origin: header to warn that a possibly untrusted webapp         is being used.         Then, BOTH the user AND the origin must have the required access.
    origin, "origin",
    /// `owner`: The person or other agent which owns this.     For example, the owner of a file in a filesystem.     There is a sense of right to control.   Typically defaults to the agent who craeted     something but can be changed.
    owner, "owner"
);
