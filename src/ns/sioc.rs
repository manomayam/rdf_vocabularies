// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `SIOC Core Ontology Namespace` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|SIOC Core Ontology Namespace|
//! |**Prefix**|sioc|
//! |**Namespace base IRI**|[http://rdfs.org/sioc/ns#](http://rdfs.org/sioc/ns#)|
//! |**Description**|SIOC (Semantically-Interlinked Online Communities) is an ontology for describing the information in online communities.  This information can be used to export information from online communities and to link them together. The scope of the application areas that SIOC can be used for includes (and is not limited to) weblogs, message boards, mailing lists and chat channels.|
//! |**Is defined by**|[http://rdfs.org/sioc/ns#](http://rdfs.org/sioc/ns#)|
//!

use crate::namespace;

namespace!(
    "http://rdfs.org/sioc/ns#",;
    /// `Community`: Community is a high-level concept that defines an online community and what it consists of.
    Community, "Community",
    /// `Container`: An area in which content Items are contained.
    Container, "Container",
    /// `Forum`: A discussion area on which Posts or entries are made.
    Forum, "Forum",
    /// `Item`: An Item is something which can be in a Container.
    Item, "Item",
    /// `Post`: An article or message that can be posted to a Forum.
    Post, "Post",
    /// `Role`: A Role is a function of a UserAccount within a scope of a particular Forum, Site, etc.
    Role, "Role",
    /// `Site`: A Site can be the location of an online community or set of communities, with UserAccounts and Usergroups creating Items in a set of Containers. It can be thought of as a web-accessible data Space.
    Site, "Site",
    /// `Space`: A Space is a place where data resides, e.g. on a website, desktop, fileshare, etc.
    Space, "Space",
    /// `Thread`: A container for a series of threaded discussion Posts or Items.
    Thread, "Thread",
    /// `User`: UserAccount is now preferred. This is a deprecated class for a User in an online community site.
    User, "User",
    /// `User Account`: A user account in an online community site.
    UserAccount, "UserAccount",
    /// `Usergroup`: A set of UserAccounts whose owners have a common purpose or interest. Can be used for access control purposes.
    Usergroup, "Usergroup",
    /// `about`: Specifies that this Item is about a particular resource, e.g. a Post describing a book, hotel, etc.
    about, "about",
    /// `account of`: Refers to the foaf:Agent or foaf:Person who owns this sioc:UserAccount.
    account_of, "account_of",
    /// `addressed to`: Refers to who (e.g. a UserAccount, e-mail address, etc.) a particular Item is addressed to.
    addressed_to, "addressed_to",
    /// `administrator of`: A Site that the UserAccount is an administrator of.
    administrator_of, "administrator_of",
    /// `attachment`: The URI of a file attached to an Item.
    attachment, "attachment",
    /// `avatar`: An image or depiction used to represent this UserAccount.
    avatar, "avatar",
    /// `container of`: An Item that this Container contains.
    container_of, "container_of",
    /// `content`: The content of the Item in plain text format.
    content, "content",
    /// `content encoded`: The encoded content of the Post, contained in CDATA areas.
    content_encoded, "content_encoded",
    /// `created at`: When this was created, in ISO 8601 format.
    created_at, "created_at",
    /// `creator of`: A resource that the UserAccount is a creator of.
    creator_of, "creator_of",
    /// `delivered at`: When this was delivered, in ISO 8601 format.
    delivered_at, "delivered_at",
    /// `description`: The content of the Post.
    description, "description",
    /// `discussion of`: The Item that this discussion is about.
    discussion_of, "discussion_of",
    /// `earlier version`: Links to a previous (older) revision of this Item or Post.
    earlier_version, "earlier_version",
    /// `email`: An electronic mail address of the UserAccount.
    email, "email",
    /// `email sha1`: An electronic mail address of the UserAccount, encoded using SHA1.
    email_sha1, "email_sha1",
    /// `embeds knowledge`: This links Items to embedded statements, facts and structured content.
    embeds_knowledge, "embeds_knowledge",
    /// `feed`: A feed (e.g. RSS, Atom, etc.) pertaining to this resource (e.g. for a Forum, Site, UserAccount, etc.).
    feed, "feed",
    /// `first name`: First (real) name of this User. Synonyms include given name or christian name.
    first_name, "first_name",
    /// `follows`: Indicates that one UserAccount follows another UserAccount (e.g. for microblog posts or other content item updates).
    follows, "follows",
    /// `function of`: A UserAccount that has this Role.
    function_of, "function_of",
    /// `generator`: A URI for the application used to generate this Item.
    generator, "generator",
    /// `group of`: 
    group_of, "group_of",
    /// `has administrator`: A UserAccount that is an administrator of this Site.
    has_administrator, "has_administrator",
    /// `has container`: The Container to which this Item belongs.
    has_container, "has_container",
    /// `has creator`: This is the UserAccount that made this resource.
    has_creator, "has_creator",
    /// `has discussion`: A discussion that is related to this Item. The discussion can be anything, for example, a sioc:Forum or sioc:Thread, a sioct:WikiArticle or simply a foaf:Document.
    has_discussion, "has_discussion",
    /// `has function`: A Role that this UserAccount has.
    has_function, "has_function",
    /// `has group`: 
    has_group, "has_group",
    /// `has host`: The Site that hosts this Container.
    has_host, "has_host",
    /// `has member`: A UserAccount that is a member of this Usergroup.
    has_member, "has_member",
    /// `has moderator`: A UserAccount that is a moderator of this Forum.
    has_moderator, "has_moderator",
    /// `has modifier`: A UserAccount that modified this resource (e.g. Item, Container, Space).
    has_modifier, "has_modifier",
    /// `has owner`: A UserAccount that this resource is owned by.
    has_owner, "has_owner",
    /// `has parent`: A Container or Forum that this Container or Forum is a child of.
    has_parent, "has_parent",
    /// `has part`: An resource that is a part of this subject.
    has_part, "has_part",
    /// `has reply`: Points to an Item or Post that is a reply or response to this Item or Post.
    has_reply, "has_reply",
    /// `has scope`: A resource that this Role applies to.
    has_scope, "has_scope",
    /// `has space`: A data Space which this resource is a part of.
    has_space, "has_space",
    /// `has subscriber`: A UserAccount that is subscribed to this Container.
    has_subscriber, "has_subscriber",
    /// `has usergroup`: Points to a Usergroup that has certain access to this Space.
    has_usergroup, "has_usergroup",
    /// `host of`: A Container that is hosted on this Site.
    host_of, "host_of",
    /// `id`: An identifier of a SIOC concept instance. For example, a user ID. Must be unique for instances of each type of SIOC concept within the same site.
    id, "id",
    /// `ip address`: The IP address used when creating this Item, UserAccount, etc. This can be associated with a creator. Some wiki articles list the IP addresses for the creator or modifiers when the usernames are absent.
    ip_address, "ip_address",
    /// `last activity date`: The date and time of the last activity associated with a SIOC concept instance, and expressed in ISO 8601 format. This could be due to a reply Post or Comment, a modification to an Item, etc.
    last_activity_date, "last_activity_date",
    /// `last item date`: The date and time of the last Post (or Item) in a Forum (or a Container), in ISO 8601 format.
    last_item_date, "last_item_date",
    /// `last name`: Last (real) name of this user. Synonyms include surname or family name.
    last_name, "last_name",
    /// `last reply date`: The date and time of the last reply Post or Comment, which could be associated with a starter Item or Post or with a Thread, and expressed in ISO 8601 format.
    last_reply_date, "last_reply_date",
    /// `later version`: Links to a later (newer) revision of this Item or Post.
    later_version, "later_version",
    /// `latest version`: Links to the latest revision of this Item or Post.
    latest_version, "latest_version",
    /// `likes`: Used to indicate some form of endorsement by a UserAccount or Agent of an Item, Container, Space, UserAccount, etc.
    likes, "likes",
    /// `link`: A URI of a document which contains this SIOC object.
    link, "link",
    /// `links to`: Links extracted from hyperlinks within a SIOC concept, e.g. Post or Site.
    links_to, "links_to",
    /// `member of`: A Usergroup that this UserAccount is a member of.
    member_of, "member_of",
    /// `mentions`: Refers to a UserAccount that a particular Item mentions.
    mentions, "mentions",
    /// `moderator of`: A Forum that a UserAccount is a moderator of.
    moderator_of, "moderator_of",
    /// `modified at`: When this was modified, in ISO 8601 format.
    modified_at, "modified_at",
    /// `modifier of`: A resource that this UserAccount has modified.
    modifier_of, "modifier_of",
    /// `name`: The name of a SIOC concept instance, e.g. a username for a UserAccount, group name for a Usergroup, etc.
    name, "name",
    /// `next by date`: Next Item or Post in a given Container sorted by date.
    next_by_date, "next_by_date",
    /// `next version`: Links to the next revision of this Item or Post.
    next_version, "next_version",
    /// `note`: A note associated with this resource, for example, if it has been edited by a UserAccount.
    note, "note",
    /// `num authors`: The number of unique authors (UserAccounts and unregistered posters) who have contributed to this Item, Thread, Post, etc.
    num_authors, "num_authors",
    /// `num items`: The number of Posts (or Items) in a Forum (or a Container).
    num_items, "num_items",
    /// `num replies`: The number of replies that this Item, Thread, Post, etc. has. Useful for when the reply structure is absent.
    num_replies, "num_replies",
    /// `num threads`: The number of Threads (AKA discussion topics) in a Forum.
    num_threads, "num_threads",
    /// `num views`: The number of times this Item, Thread, UserAccount profile, etc. has been viewed.
    num_views, "num_views",
    /// `owner of`: A resource owned by a particular UserAccount, for example, a weblog or image gallery.
    owner_of, "owner_of",
    /// `parent of`: A child Container or Forum that this Container or Forum is a parent of.
    parent_of, "parent_of",
    /// `part of`: A resource that the subject is a part of.
    part_of, "part_of",
    /// `previous by date`: Previous Item or Post in a given Container sorted by date.
    previous_by_date, "previous_by_date",
    /// `previous version`: Links to the previous revision of this Item or Post.
    previous_version, "previous_version",
    /// `read at`: When this was read, in ISO 8601 format.
    read_at, "read_at",
    /// `reference`: Links either created explicitly or extracted implicitly on the HTML level from the Post.
    reference, "reference",
    /// `related to`: Related resources for this resource, e.g. for Posts, perhaps determined implicitly from topics or references.
    related_to, "related_to",
    /// `reply of`: Links to an Item or Post which this Item or Post is a reply to.
    reply_of, "reply_of",
    /// `respond to`: For the reply-to address set in email messages, IMs, etc. The property name was chosen to avoid confusion with has_reply/reply_of (the reply graph).
    respond_to, "respond_to",
    /// `scope of`: A Role that has a scope of this resource.
    scope_of, "scope_of",
    /// `shared by`: For shared Items where there is a certain creator_of and an intermediary who shares or forwards it (e.g. as a sibling Item).
    shared_by, "shared_by",
    /// `sibling`: An Item may have a sibling or a twin that exists in a different Container, but the siblings may differ in some small way (for example, language, category, etc.). The sibling of this Item should be self-describing (that is, it should contain all available information).
    sibling, "sibling",
    /// `space of`: A resource which belongs to this data Space.
    space_of, "space_of",
    /// `subject`: Keyword(s) describing subject of the Post.
    subject, "subject",
    /// `subscriber of`: A Container that a UserAccount is subscribed to.
    subscriber_of, "subscriber_of",
    /// `title`: This is the title (subject line) of the Post. Note that for a Post within a threaded discussion that has no parents, it would detail the topic thread.
    title, "title",
    /// `topic`: A topic of interest, linking to the appropriate URI, e.g. in the Open Directory Project or of a SKOS category.
    topic, "topic",
    /// `usergroup of`: A Space that the Usergroup has access to.
    usergroup_of, "usergroup_of"
);
