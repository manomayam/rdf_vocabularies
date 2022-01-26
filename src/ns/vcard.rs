// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Ontology for vCard` vocabulary
//!
//! This module requires `ns-vcard` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Ontology for vCard|
//! |**Prefix**|vcard|
//! |**Namespace base IRI**|<http://www.w3.org/2006/vcard/ns#>|
//! |**Description**|Ontology for vCard based on RFC6350|
//! |**Is defined by**|<http://www.w3.org/2006/vcard/ns#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2006/vcard/ns#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Acquaintance`: 
    Acquaintance, "Acquaintance",
    /// `Address`: To specify the components of the delivery address for the  object
    Address, "Address",
    /// `Agent`: 
    Agent, "Agent",
    /// `BBS`: This class is deprecated
    BBS, "BBS",
    /// `Car`: This class is deprecated
    Car, "Car",
    /// `Cell`: Also called mobile telephone
    Cell, "Cell",
    /// `Child`: 
    Child, "Child",
    /// `Colleague`: 
    Colleague, "Colleague",
    /// `Contact`: 
    Contact, "Contact",
    /// `Coresident`: 
    Coresident, "Coresident",
    /// `Coworker`: 
    Coworker, "Coworker",
    /// `Crush`: 
    Crush, "Crush",
    /// `Date`: 
    Date, "Date",
    /// `Dom`: This class is deprecated
    Dom, "Dom",
    /// `Email`: To specify the electronic mail address for communication with the object the vCard represents. Use the hasEmail object property.
    Email, "Email",
    /// `Emergency`: 
    Emergency, "Emergency",
    /// `Fax`: 
    Fax, "Fax",
    /// `Female`: 
    Female, "Female",
    /// `Friend`: 
    Friend, "Friend",
    /// `Gender`: Used for gender codes. The URI of the gender code must be used as the value for Gender.
    Gender, "Gender",
    /// `Group`: Object representing a group of persons or entities.  A group object will usually contain hasMember properties to specify the members of the group.
    Group, "Group",
    /// `Home`: This implies that the property is related to an individual's personal life
    Home, "Home",
    /// `ISDN`: This class is deprecated
    ISDN, "ISDN",
    /// `Individual`: An object representing a single person or entity
    Individual, "Individual",
    /// `Internet`: This class is deprecated
    Internet, "Internet",
    /// `Intl`: This class is deprecated
    Intl, "Intl",
    /// `Kin`: 
    Kin, "Kin",
    /// `Kind`: The parent class for all objects
    Kind, "Kind",
    /// `Label`: This class is deprecated
    Label, "Label",
    /// `Location`: An object representing a named geographical place
    Location, "Location",
    /// `Male`: 
    Male, "Male",
    /// `Me`: 
    Me, "Me",
    /// `Met`: 
    Met, "Met",
    /// `Modem`: This class is deprecated
    Modem, "Modem",
    /// `Msg`: This class is deprecated
    Msg, "Msg",
    /// `Muse`: 
    Muse, "Muse",
    /// `Name`: To specify the components of the name of the object
    Name, "Name",
    /// `Neighbor`: 
    Neighbor, "Neighbor",
    /// `None`: 
    None, "None",
    /// `Organization`: An object representing an organization.  An organization is a single entity, and might represent a business or government, a department or division within a business or government, a club, an association, or the like. 
    Organization, "Organization",
    /// `Other`: 
    Other, "Other",
    /// `PCS`: This class is deprecated
    PCS, "PCS",
    /// `Pager`: 
    Pager, "Pager",
    /// `Parcel`: This class is deprecated
    Parcel, "Parcel",
    /// `Parent`: 
    Parent, "Parent",
    /// `Postal`: This class is deprecated
    Postal, "Postal",
    /// `Pref`: This class is deprecated
    Pref, "Pref",
    /// `Relation Type`: Used for relation type codes. The URI of the relation type code must be used as the value for the Relation Type.
    RelatedType, "RelatedType",
    /// `Sibling`: 
    Sibling, "Sibling",
    /// `Spouse`: 
    Spouse, "Spouse",
    /// `Sweetheart`: 
    Sweetheart, "Sweetheart",
    /// `Tel`: This class is deprecated. Use the hasTelephone object property.
    Tel, "Tel",
    /// `Phone`: Used for telephone type codes. The URI of the telephone type code must be used as the value for the Telephone Type.
    TelephoneType, "TelephoneType",
    /// `Text`: Also called sms telephone
    Text, "Text",
    /// `Text phone`: 
    TextPhone, "TextPhone",
    /// `Type`: Used for type codes. The URI of the type code must be used as the value for Type.
    Type, "Type",
    /// `Unknown`: 
    Unknown, "Unknown",
    /// `VCard`: The vCard class is  equivalent to the new Kind class, which is the parent for the four explicit types of vCards (Individual, Organization, Location, Group)
    VCard, "VCard",
    /// `Video`: 
    Video, "Video",
    /// `Voice`: 
    Voice, "Voice",
    /// `Work`: This implies that the property is related to an individual's work place
    Work, "Work",
    /// `X400`: This class is deprecated
    X400, "X400",
    /// `additional name`: The additional name associated with the object
    additional_name, "additional-name",
    /// `address`: This object property has been mapped
    adr, "adr",
    /// `agent`: This object property has been deprecated
    agent, "agent",
    /// `anniversary`: The date of marriage, or equivalent, of the object
    anniversary, "anniversary",
    /// `birth date`: To specify the birth date of the object
    bday, "bday",
    /// `category`: The category information about the object, also known as tags
    category, "category",
    /// `class`: This data property has been deprecated
    class, "class",
    /// `country name`: The country name associated with the address of the object
    country_name, "country-name",
    /// `email`: This object property has been mapped
    email, "email",
    /// `extended address`: This data property has been deprecated
    extended_address, "extended-address",
    /// `family name`: The family name associated with the object
    family_name, "family-name",
    /// `formatted name`: The formatted text corresponding to the name of the object
    fn_, "fn",
    /// `geo`: This object property has been mapped
    geo, "geo",
    /// `given name`: The given name associated with the object
    given_name, "given-name",
    /// `has additional name`: Used to support property parameters for the additional name data property
    hasAdditionalName, "hasAdditionalName",
    /// `has address`: To specify the components of the delivery address for the object
    hasAddress, "hasAddress",
    /// `has calendar busy`: To specify the busy time associated with the object. (Was called FBURL in RFC6350)
    hasCalendarBusy, "hasCalendarBusy",
    /// `has calendar link`: To specify the calendar associated with the object. (Was called CALURI in RFC6350)
    hasCalendarLink, "hasCalendarLink",
    /// `has calendar request`: To specify the calendar user address to which a scheduling request be sent for the object. (Was called CALADRURI in RFC6350)
    hasCalendarRequest, "hasCalendarRequest",
    /// `has category`: Used to support property parameters for the category data property
    hasCategory, "hasCategory",
    /// `has country name`: Used to support property parameters for the country name data property
    hasCountryName, "hasCountryName",
    /// `has email`: To specify the electronic mail address for communication with the object
    hasEmail, "hasEmail",
    /// `has formatted name`: Used to support property parameters for the formatted name data property
    hasFN, "hasFN",
    /// `has family name`: Used to support property parameters for the family name data property
    hasFamilyName, "hasFamilyName",
    /// `has gender`: To specify  the sex or gender identity of the object. URIs are recommended to enable interoperable sex and gender codes to be used.
    hasGender, "hasGender",
    /// `has geo`: To specify information related to the global positioning of the object. May also be used as a property parameter.
    hasGeo, "hasGeo",
    /// `has given name`: Used to support property parameters for the given name data property
    hasGivenName, "hasGivenName",
    /// `has honorific prefix`: Used to support property parameters for the honorific prefix data property
    hasHonorificPrefix, "hasHonorificPrefix",
    /// `has honorific suffix`: Used to support property parameters for the honorific suffix data property
    hasHonorificSuffix, "hasHonorificSuffix",
    /// `has messaging`: To specify the instant messaging and presence protocol communications with the object. (Was called IMPP in RFC6350)
    hasInstantMessage, "hasInstantMessage",
    /// `has key`: To specify a public key or authentication certificate associated with the object
    hasKey, "hasKey",
    /// `has language`: Used to support property parameters for the language data property
    hasLanguage, "hasLanguage",
    /// `has locality`: Used to support property parameters for the locality data property
    hasLocality, "hasLocality",
    /// `has logo`: To specify a graphic image of a logo associated with the object 
    hasLogo, "hasLogo",
    /// `has member`: To include a member in the group this object represents. (This property can only be used by Group individuals)
    hasMember, "hasMember",
    /// `has name`: To specify the components of the name of the object
    hasName, "hasName",
    /// `has nickname`: Used to support property parameters for the nickname data property
    hasNickname, "hasNickname",
    /// `has note`: Used to support property parameters for the note data property
    hasNote, "hasNote",
    /// `has organization name`: Used to support property parameters for the organization name data property
    hasOrganizationName, "hasOrganizationName",
    /// `has organization unit name`: Used to support property parameters for the organization unit name data property
    hasOrganizationUnit, "hasOrganizationUnit",
    /// `has photo`: To specify an image or photograph information that annotates some aspect of the object
    hasPhoto, "hasPhoto",
    /// `has postal code`: Used to support property parameters for the postal code data property
    hasPostalCode, "hasPostalCode",
    /// `has region`: Used to support property parameters for the region data property
    hasRegion, "hasRegion",
    /// `has related`: To specify a relationship between another entity and the entity represented by this object
    hasRelated, "hasRelated",
    /// `has role`: Used to support property parameters for the role data property
    hasRole, "hasRole",
    /// `has sound`: To specify a digital sound content information that annotates some aspect of the object
    hasSound, "hasSound",
    /// `has source`: To identify the source of directory information of the object
    hasSource, "hasSource",
    /// `has street address`: Used to support property parameters for the street address data property
    hasStreetAddress, "hasStreetAddress",
    /// `has telephone`: To specify the telephone number for telephony communication with the object
    hasTelephone, "hasTelephone",
    /// `has title`: Used to support property parameters for the title data property
    hasTitle, "hasTitle",
    /// `has uid`: To specify a value that represents a globally unique identifier corresponding to the object
    hasUID, "hasUID",
    /// `has url`: To specify a uniform resource locator associated with the object
    hasURL, "hasURL",
    /// `has value`: Used to indicate the resource value of an object property that requires property parameters
    hasValue, "hasValue",
    /// `honorific prefix`: The honorific prefix of the name associated with the object
    honorific_prefix, "honorific-prefix",
    /// `honorific suffix`: The honorific suffix of the name associated with the object
    honorific_suffix, "honorific-suffix",
    /// `key`: This object property has been mapped
    key, "key",
    /// `label`: This data property has been deprecated
    label, "label",
    /// `language`: To specify the language that may be used for contacting the object. May also be used as a property parameter.
    language, "language",
    /// `latitude`: This data property has been deprecated. See hasGeo
    latitude, "latitude",
    /// `locality`: The locality (e.g. city or town) associated with the address of the object
    locality, "locality",
    /// `logo`: This object property has been mapped
    logo, "logo",
    /// `longitude`: This data property has been deprecated. See hasGeo
    longitude, "longitude",
    /// `mailer`: This data property has been deprecated
    mailer, "mailer",
    /// `name`: This object property has been mapped
    n, "n",
    /// `nickname`: The nick name associated with the object
    nickname, "nickname",
    /// `note`: A note associated with the object
    note, "note",
    /// `organization`: This object property has been mapped. Use the organization-name data property.
    org, "org",
    /// `organization name`: To specify the organizational name associated with the object
    organization_name, "organization-name",
    /// `organizational unit name`: To specify the organizational unit name associated with the object
    organization_unit, "organization-unit",
    /// `photo`: This object property has been mapped
    photo, "photo",
    /// `post office box`: This data property has been deprecated
    post_office_box, "post-office-box",
    /// `postal code`: The postal code associated with the address of the object
    postal_code, "postal-code",
    /// `product id`: To specify the identifier for the product that created the object
    prodid, "prodid",
    /// `region`: The region (e.g. state or province) associated with the address of the object
    region, "region",
    /// `revision`: To specify revision information about the object
    rev, "rev",
    /// `role`: To specify the function or part played in a particular situation by the object
    role, "role",
    /// `sort as`: To specify the string to be used for national-language-specific sorting. Used as a property parameter only.
    sort_string, "sort-string",
    /// `sound`: This object property has been mapped
    sound, "sound",
    /// `street address`: The street address associated with the address of the object
    street_address, "street-address",
    /// `telephone`: This object property has been mapped
    tel, "tel",
    /// `title`: To specify the position or job of the object
    title, "title",
    /// `time zone`: To indicate time zone information that is specific to the object. May also be used as a property parameter.
    tz, "tz",
    /// `url`: This object property has been mapped
    url, "url",
    /// `value`: Used to indicate the literal value of a data property that requires property parameters
    value, "value"
);
