// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `acl` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|acl|
//! |**Namespace base IRI**|[http://www.w3.org/ns/auth/acl#](http://www.w3.org/ns/auth/acl#)|
//! |**Description**||
//! |**Is defined by**|[http://www.w3.org/ns/auth/acl#](http://www.w3.org/ns/auth/acl#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/auth/acl#",;
    /// ``: Any kind of access to a resource. Don't use this, use R W and RW
    Access, "Access",
    /// `append`: Append accesses are specific write access which only add information, and do not remove information.<br>    For text files, for example, append access allows bytes to be added onto the end of the file.<br>    For RDF graphs, Append access allows adds triples to the graph but does not remove any.<br>    Append access is useful for dropbox functionality.<br>    Dropbox can be used for link notification, which the information added is a notification<br>    that a some link has been made elsewhere relevant to the given resource.<br>    
    Append, "Append",
    /// `Anyone authenticated`: A class of agents who have been authenticated.<br>In other words, anyone can access this resource, but not anonymously.<br>The social expectation is that the authentication process will provide an<br>identify and a name, or pseudonym.<br>(A new ID should not be minted for every access: the intent is that the user<br>is able to continue to use the ID for continues interactions with peers,<br>and for example to develop a reputation)<br>
    AuthenticatedAgent, "AuthenticatedAgent",
    /// `authorization`: An element of access control,<br>    allowing agent to agents access of some kind to resources or classes of resources
    Authorization, "Authorization",
    /// `control`: Allows read/write access to the ACL for the resource(s)
    Control, "Control",
    /// `Origin`: An Origin is basically a web site<br>        (Note WITHOUT the trailing slash after the domain name and port in its URI)<br>        and is the basis for controlling access to data by web apps<br>        in the Same Origin Model of web security.<br>        All scripts from the same origin are given the same right.
    Origin, "Origin",
    /// `read`: The class of read operations
    Read, "Read",
    /// `write`: 
    Write, "Write",
    /// `access control`: The Access Control file for this information resource.<br>        This may of course be a virtual resource implemented by the access control system.<br>        Note also HTTP's header  Link:  foo.meta ;rel=meta can be used for this.
    accessControl, "accessControl",
    /// `to`: The information resource to which access is being granted.
    accessTo, "accessTo",
    /// `to all in`: A class of information resources to which access is being granted.
    accessToClass, "accessToClass",
    /// `agent`: A person or social entity to being given the right
    agent, "agent",
    /// `agent class`: A class of persons or social entities to being given the right
    agentClass, "agentClass",
    /// `agent group`: A group of persons or social entities to being given the right.<br>          The right is given to any entity which is a vcard:member of the group,<br>          as defined by the document received when the Group is dereferenced.
    agentGroup, "agentGroup",
    /// `default access for things in this`: If a resource has no ACL file (it is 404),<br>        then access to the resource if given by the ACL of the immediately<br>        containing directory, or failing that (404) the ACL of the recursively next<br>        containing directory which has an ACL file.<br>        Within that ACL file,<br>        any Authentication which has that directory as its acl:default applies to the<br>        resource. (The highest directory must have an ACL file.)<br>
    default, "default",
    /// `default access for new things in the object`: THIS IS OBSOLETE AS OF 2017-08-01.   See 'default'.<br>        Was: A directory for which this authorization is used for new files in the directory.
    defaultForNew, "defaultForNew",
    /// `delegates`: Delegates a person or another agent to act on behalf of the agent.<br>    For example, Alice delegates Bob to act on behalf of Alice for ACL purposes.
    delegates, "delegates",
    /// `access mode`: A mode of access such as read or write.
    mode, "mode",
    /// `origin`: A web application, identified by its Origin, such as<br>        <https://scripts.example.com>, being given the right.<br>        When a user of the web application at a certain origin accesses the server,<br>        then the browser sets the Origin: header to warn that a possibly untrusted webapp<br>        is being used.<br>        Then, BOTH the user AND the origin must have the required access.
    origin, "origin",
    /// `owner`: The person or other agent which owns this.<br>    For example, the owner of a file in a filesystem.<br>    There is a sense of right to control.   Typically defaults to the agent who craeted<br>    something but can be changed.
    owner, "owner"
);
