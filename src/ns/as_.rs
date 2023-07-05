// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Activity Streams 2.0` vocabulary
//!
//! This module requires `ns-as_` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Activity Streams 2.0|
//! |**Prefix**|as|
//! |**Namespace base IRI**|<https://www.w3.org/ns/activitystreams#>|
//! |**Description**|Extended Activity Streams 2.0 Vocabulary|
//! |**Is defined by**|<https://raw.githubusercontent.com/zazuko/activitystreams/owl-fix/vocabulary/activitystreams2.owl>|
//!

use crate::namespace;

namespace!(
    base: "https://www.w3.org/ns/activitystreams#",

    terms: [
        /// `Accept`: Actor accepts the Object
        (Accept, "Accept"),
        /// `Activity`: An Object representing some form of Action that has been taken
        (Activity, "Activity"),
        /// `Add`: To Add an Object or Link to Something
        (Add, "Add"),
        /// `Announce`: Actor announces the object to the target
        (Announce, "Announce"),
        /// `Application`: Represents a software application of any sort
        (Application, "Application"),
        /// `Arrive`: To Arrive Somewhere (can be used, for instance, to indicate that a particular entity is currently located somewhere, e.g. a "check-in")
        (Arrive, "Arrive"),
        /// `Article`: A written work. Typically several paragraphs long. For example, a blog post or a news article.
        (Article, "Article"),
        /// `Audio`: An audio file
        (Audio, "Audio"),
        /// `Block`:
        (Block, "Block"),
        /// `Collection`: An ordered or unordered collection of Objects or Links
        (Collection, "Collection"),
        /// `CollectionPage`: A subset of items from a Collection
        (CollectionPage, "CollectionPage"),
        /// `Create`: To Create Something
        (Create, "Create"),
        /// `Delete`: To Delete Something
        (Delete, "Delete"),
        /// `Dislike`: The actor dislikes the object
        (Dislike, "Dislike"),
        /// `Document`: Represents a digital document/file of any sort
        (Document, "Document"),
        /// `Event`: An Event of any kind
        (Event, "Event"),
        /// `Flag`: To flag something (e.g. flag as inappropriate, flag as spam, etc)
        (Flag, "Flag"),
        /// `Follow`: To Express Interest in Something
        (Follow, "Follow"),
        /// `Group`: A Group of any kind.
        (Group, "Group"),
        /// `Ignore`: Actor is ignoring the Object
        (Ignore, "Ignore"),
        /// `Image`: An Image file
        (Image, "Image"),
        /// `IntransitiveActivity`: An Activity that has no direct object
        (IntransitiveActivity, "IntransitiveActivity"),
        /// `Invite`: To invite someone or something to something
        (Invite, "Invite"),
        /// `Join`: To Join Something
        (Join, "Join"),
        /// `Leave`: To Leave Something
        (Leave, "Leave"),
        /// `Like`: To Like Something
        (Like, "Like"),
        /// `Link`: Represents a qualified reference to another resource. Patterned after the RFC5988 Web Linking Model
        (Link, "Link"),
        /// `Listen`: The actor listened to the object
        (Listen, "Listen"),
        /// `Mention`: A specialized Link that represents an @mention
        (Mention, "Mention"),
        /// `Move`: The actor is moving the object. The target specifies where the object is moving to. The origin specifies where the object is moving from.
        (Move, "Move"),
        /// `Note`: A Short note, typically less than a single paragraph. A "tweet" is an example, or a "status update"
        (Note, "Note"),
        /// `Object`:
        (Object, "Object"),
        /// `Offer`: To Offer something to someone or something
        (Offer, "Offer"),
        /// `OrderedCollection`: A variation of Collection in which items are strictly ordered
        (OrderedCollection, "OrderedCollection"),
        /// `OrderedCollectionPage`: An ordered subset of items from an OrderedCollection
        (OrderedCollectionPage, "OrderedCollectionPage"),
        /// `OrderedItems`: A rdf:List variant for Objects and Links
        (OrderedItems, "OrderedItems"),
        /// `Organization`: An Organization
        (Organization, "Organization"),
        /// `Page`: A Web Page
        (Page, "Page"),
        /// `Person`: A Person
        (Person, "Person"),
        /// `Place`: A physical or logical location
        (Place, "Place"),
        /// `Profile`: A Profile Document
        (Profile, "Profile"),
        /// `Question`: A question of any sort.
        (Question, "Question"),
        /// `Read`: The actor read the object
        (Read, "Read"),
        /// `Reject`: Actor rejects the Object
        (Reject, "Reject"),
        /// `Relationship`: Represents a Social Graph relationship between two Individuals (indicated by the 'a' and 'b' properties)
        (Relationship, "Relationship"),
        /// `Remove`: To Remove Something
        (Remove, "Remove"),
        /// `Service`: A service provided by some entity
        (Service, "Service"),
        /// `TentativeAccept`: Actor tentatively accepts the Object
        (TentativeAccept, "TentativeAccept"),
        /// `TentativeReject`: Actor tentatively rejects the object
        (TentativeReject, "TentativeReject"),
        /// `Tombstone`: A placeholder for a deleted object
        (Tombstone, "Tombstone"),
        /// `Travel`: The actor is traveling to the target. The origin specifies where the actor is traveling from.
        (Travel, "Travel"),
        /// `Undo`: To Undo Something. This would typically be used to indicate that a previous Activity has been undone.
        (Undo, "Undo"),
        /// `Update`: To Update/Modify Something
        (Update, "Update"),
        /// `Video`: A Video document of any kind.
        (Video, "Video"),
        /// `View`: The actor viewed the object
        (View, "View"),
        /// `accuracy`: Specifies the accuracy around the point established by the longitude and latitude
        (accuracy, "accuracy"),
        /// `actor`: Subproperty of as:attributedTo that identifies the primary actor
        (actor, "actor"),
        /// `altitude`: The altitude of a place
        (altitude, "altitude"),
        /// `oneOf`: Describes a possible inclusive answer or option for a question.
        (anyOf, "anyOf"),
        /// `attachment`:
        (attachment, "attachment"),
        /// `attachments`:
        (attachments, "attachments"),
        /// `attributedTo`: Identifies an entity to which an object is attributed
        (attributedTo, "attributedTo"),
        /// `audience`:
        (audience, "audience"),
        /// `author`: Identifies the author of an object. Deprecated. Use as:attributedTo instead
        (author, "author"),
        /// `bcc`:
        (bcc, "bcc"),
        /// `bto`:
        (bto, "bto"),
        /// `cc`:
        (cc, "cc"),
        /// `content`: The content of the object.
        (content, "content"),
        /// `context`: Specifies the context within which an object exists or an activity was performed
        (context, "context"),
        /// `current`:
        (current, "current"),
        /// `deleted`: Specifies the date and time the object was deleted
        (deleted, "deleted"),
        /// `describes`: On a Profile object, describes the object described by the profile
        (describes, "describes"),
        /// `downstreamDuplicates`:
        (downstreamDuplicates, "downstreamDuplicates"),
        /// `duration`: The duration of the object
        (duration, "duration"),
        /// `endTime`: The ending time of the object
        (endTime, "endTime"),
        /// `first`:
        (first, "first"),
        /// `formerType`: On a Tombstone object, describes the former type of the deleted object
        (formerType, "formerType"),
        /// `generator`:
        (generator, "generator"),
        /// `height`: The display height expressed as device independent pixels
        (height, "height"),
        /// `href`: The target URI of the Link
        (href, "href"),
        /// `hreflang`: A hint about the language of the referenced resource
        (hreflang, "hreflang"),
        /// `icon`:
        (icon, "icon"),
        /// `image`:
        (image, "image"),
        /// `inReplyTo`:
        (inReplyTo, "inReplyTo"),
        /// `instrument`: Indentifies an object used (or to be used) to complete an activity
        (instrument, "instrument"),
        /// `items`:
        (items, "items"),
        /// `last`:
        (last, "last"),
        /// `latitude`: The latitude
        (latitude, "latitude"),
        /// `location`:
        (location, "location"),
        /// `longitude`: The longitude
        (longitude, "longitude"),
        /// `mediaType`: The MIME Media Type
        (mediaType, "mediaType"),
        /// `name`:
        (name, "name"),
        /// `next`:
        (next, "next"),
        /// `object`:
        (object, "object"),
        /// `objectType`:
        (objectType, "objectType"),
        /// `oneOf`: Describes a possible exclusive answer or option for a question.
        (oneOf, "oneOf"),
        /// `origin`: For certain activities, specifies the entity from which the action is directed.
        (origin, "origin"),
        /// `partOf`:
        (partOf, "partOf"),
        /// `prev`:
        (prev, "prev"),
        /// `preview`:
        (preview, "preview"),
        /// `provider`:
        (provider, "provider"),
        /// `published`: Specifies the date and time the object was published
        (published, "published"),
        /// `radius`: Specifies a radius around the point established by the longitude and latitude
        (radius, "radius"),
        /// `rating`: A numeric rating (>= 0.0, <= 5.0) for the object
        (rating, "rating"),
        /// `rel`: The RFC 5988 or HTML5 Link Relation associated with the Link
        (rel, "rel"),
        /// `relationship`: On a Relationship object, describes the type of relationship
        (relationship, "relationship"),
        /// `replies`:
        (replies, "replies"),
        /// `result`:
        (result, "result"),
        /// `startIndex`: In a strictly ordered logical collection, specifies the index position of the first item in the items list
        (startIndex, "startIndex"),
        /// `startTime`: The starting time of the object
        (startTime, "startTime"),
        /// `a`: On a Relationship object, identifies the subject. e.g. when saying "John is connected to Sally", 'subject' refers to 'John'
        (subject, "subject"),
        /// `summary`: A short summary of the object
        (summary, "summary"),
        /// `tag`:
        (tag, "tag"),
        /// `tags`:
        (tags, "tags"),
        /// `target`:
        (target, "target"),
        /// `to`:
        (to, "to"),
        /// `totalItems`: The total number of items in a logical collection
        (totalItems, "totalItems"),
        /// `units`: Identifies the unit of measurement used by the radius, altitude and accuracy properties. The value can be expressed either as one of a set of predefined units or as a well-known common URI that identifies units.
        (units, "units"),
        /// `updated`: Specifies when the object was last updated
        (updated, "updated"),
        /// `upstreamDuplicates`:
        (upstreamDuplicates, "upstreamDuplicates"),
        /// `url`: Specifies a link to a specific representation of the Object
        (url, "url"),
        /// `verb`:
        (verb, "verb"),
        /// `width`: Specifies the preferred display width of the content, expressed in terms of device independent pixels.
        (width, "width")    ]
);
