// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Friend of a Friend (FOAF) vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Friend of a Friend (FOAF) vocabulary|
//! |**Prefix**|foaf|
//! |**Namespace base IRI**|[http://xmlns.com/foaf/0.1/](http://xmlns.com/foaf/0.1/)|
//! |**Description**|The Friend of a Friend (FOAF) RDF vocabulary, described using W3C RDF Schema and the Web Ontology Language.|
//! |**Is defined by**|[http://xmlns.com/foaf/0.1/](http://xmlns.com/foaf/0.1/)|
//!

use crate::namespace;

namespace!(
    "http://xmlns.com/foaf/0.1/",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Agent`: An agent (eg. person, group, software or physical artifact).
    Agent, "Agent",
    /// `Document`: A document.
    Document, "Document",
    /// `Group`: A class of Agents.
    Group, "Group",
    /// `Image`: An image.
    Image, "Image",
    /// `Label Property`: A foaf:LabelProperty is any RDF property with texual values that serve as labels.
    LabelProperty, "LabelProperty",
    /// `Online Account`: An online account.
    OnlineAccount, "OnlineAccount",
    /// `Online Chat Account`: An online chat account.
    OnlineChatAccount, "OnlineChatAccount",
    /// `Online E-commerce Account`: An online e-commerce account.
    OnlineEcommerceAccount, "OnlineEcommerceAccount",
    /// `Online Gaming Account`: An online gaming account.
    OnlineGamingAccount, "OnlineGamingAccount",
    /// `Organization`: An organization.
    Organization, "Organization",
    /// `Person`: A person.
    Person, "Person",
    /// `PersonalProfileDocument`: A personal profile RDF document.
    PersonalProfileDocument, "PersonalProfileDocument",
    /// `Project`: A project (a collective endeavour of some kind).
    Project, "Project",
    /// `account`: Indicates an account held by this agent.
    account, "account",
    /// `account name`: Indicates the name (identifier) associated with this online account.
    accountName, "accountName",
    /// `account service homepage`: Indicates a homepage of the service provide for this online account.
    accountServiceHomepage, "accountServiceHomepage",
    /// `age`: The age in years of some agent.
    age, "age",
    /// `AIM chat ID`: An AIM chat ID
    aimChatID, "aimChatID",
    /// `based near`: A location that something is based near, for some broadly human notion of near.
    based_near, "based_near",
    /// `birthday`: The birthday of this Agent, represented in mm-dd string form, eg. '12-31'.
    birthday, "birthday",
    /// `current project`: A current project this person works on.
    currentProject, "currentProject",
    /// `depiction`: A depiction of some thing.
    depiction, "depiction",
    /// `depicts`: A thing depicted in this representation.
    depicts, "depicts",
    /// `DNA checksum`: A checksum for the DNA of some thing. Joke.
    dnaChecksum, "dnaChecksum",
    /// `familyName`: The family name of some person.
    familyName, "familyName",
    /// `family_name`: The family name of some person.
    family_name, "family_name",
    /// `firstName`: The first name of a person.
    firstName, "firstName",
    /// `focus`: The underlying or 'focal' entity associated with some SKOS-described concept.
    focus, "focus",
    /// `funded by`: An organization funding a project or person.
    fundedBy, "fundedBy",
    /// `geekcode`: A textual geekcode for this person, see http://www.geekcode.com/geek.html
    geekcode, "geekcode",
    /// `gender`: The gender of this Agent (typically but not necessarily 'male' or 'female').
    gender, "gender",
    /// `Given name`: The given name of some person.
    givenName, "givenName",
    /// `Given name`: The given name of some person.
    givenname, "givenname",
    /// `account`: Indicates an account held by this agent.
    holdsAccount, "holdsAccount",
    /// `homepage`: A homepage for some thing.
    homepage, "homepage",
    /// `ICQ chat ID`: An ICQ chat ID
    icqChatID, "icqChatID",
    /// `image`: An image that can be used to represent some thing (ie. those depictions which are particularly representative of something, eg. one's photo on a homepage).
    img, "img",
    /// `interest`: A page about a topic of interest to this person.
    interest, "interest",
    /// `is primary topic of`: A document that this thing is the primary topic of.
    isPrimaryTopicOf, "isPrimaryTopicOf",
    /// `jabber ID`: A jabber ID for something.
    jabberID, "jabberID",
    /// `knows`: A person known by this person (indicating some level of reciprocated interaction between the parties).
    knows, "knows",
    /// `lastName`: The last name of a person.
    lastName, "lastName",
    /// `logo`: A logo representing some thing.
    logo, "logo",
    /// `made`: Something that was made by this agent.
    made, "made",
    /// `maker`: An agent that made this thing.
    maker, "maker",
    /// `personal mailbox`: A personal mailbox, ie. an Internet mailbox associated with exactly one owner, the first owner of this mailbox. This is a 'static inverse functional property', in that there is (across time and change) at most one individual that ever has any particular value for foaf:mbox.
    mbox, "mbox",
    /// `sha1sum of a personal mailbox URI name`: The sha1sum of the URI of an Internet mailbox associated with exactly one owner, the first owner of the mailbox.
    mbox_sha1sum, "mbox_sha1sum",
    /// `member`: Indicates a member of a Group
    member, "member",
    /// `membershipClass`: Indicates the class of individuals that are a member of a Group
    membershipClass, "membershipClass",
    /// `MSN chat ID`: An MSN chat ID
    msnChatID, "msnChatID",
    /// `myersBriggs`: A Myers Briggs (MBTI) personality classification.
    myersBriggs, "myersBriggs",
    /// `name`: A name for some thing.
    name, "name",
    /// `nickname`: A short informal nickname characterising an agent (includes login identifiers, IRC and other chat nicknames).
    nick, "nick",
    /// `openid`: An OpenID for an Agent.
    openid, "openid",
    /// `page`: A page or document about this thing.
    page, "page",
    /// `past project`: A project this person has previously worked on.
    pastProject, "pastProject",
    /// `phone`: A phone, specified using fully qualified tel: URI scheme (refs: http://www.w3.org/Addressing/schemes.html#tel).
    phone, "phone",
    /// `plan`: A .plan comment, in the tradition of finger and '.plan' files.
    plan, "plan",
    /// `primary topic`: The primary topic of some page or document.
    primaryTopic, "primaryTopic",
    /// `publications`: A link to the publications of this person.
    publications, "publications",
    /// `schoolHomepage`: A homepage of a school attended by the person.
    schoolHomepage, "schoolHomepage",
    /// `sha1sum (hex)`: A sha1sum hash, in hex.
    sha1, "sha1",
    /// `Skype ID`: A Skype ID
    skypeID, "skypeID",
    /// `status`: A string expressing what the user is happy for the general public (normally) to know about their current activity.
    status, "status",
    /// `Surname`: The surname of some person.
    surname, "surname",
    /// `theme`: A theme.
    theme, "theme",
    /// `thumbnail`: A derived thumbnail image.
    thumbnail, "thumbnail",
    /// `tipjar`: A tipjar document for this agent, describing means for payment and reward.
    tipjar, "tipjar",
    /// `title`: Title (Mr, Mrs, Ms, Dr. etc)
    title, "title",
    /// `topic`: A topic of some page or document.
    topic, "topic",
    /// `topic_interest`: A thing of interest to this person.
    topic_interest, "topic_interest",
    /// `weblog`: A weblog of some thing (whether person, group, company etc.).
    weblog, "weblog",
    /// `work info homepage`: A work info homepage of some person; a page about their work for some organization.
    workInfoHomepage, "workInfoHomepage",
    /// `workplace homepage`: A workplace homepage of some person; the homepage of an organization they work for.
    workplaceHomepage, "workplaceHomepage",
    /// `Yahoo chat ID`: A Yahoo chat ID
    yahooChatID, "yahooChatID"
);
