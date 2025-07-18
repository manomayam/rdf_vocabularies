// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `rss` vocabulary
//!
//! This module requires `ns-rss` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|rss|
//! |**Namespace base IRI**|<http://purl.org/rss/1.0/>|
//! |**Description**||
//! |**Is defined by**|<http://purl.org/rss/1.0/schema.rdf>|
//!

use crate::namespace;

namespace!(
    base: "http://purl.org/rss/1.0/",

    terms: [
        /// `Channel`: An RSS information channel.
        (channel, "channel"),
        /// `Description`: A short text description of the subject.
        (description, "description"),
        /// `Image`: An RSS image.
        (image, "image"),
        /// `Item`: An RSS item.
        (item, "item"),
        /// `Items`: Points to a list of rss:item elements that are members of the subject channel.
        (items, "items"),
        /// `Link`: The URL to which an HTML rendering of the subject will link.
        (link, "link"),
        /// `Name`: The text input field's (variable) name.
        (name, "name"),
        /// `Text Input`: An RSS text input.
        (textinput, "textinput"),
        /// `Title`: A descriptive title for the channel.
        (title, "title"),
        /// `URL`: The URL of the image to used in the 'src' attribute of the channel's image tag when rendered as HTML.
        (url, "url")    ]
);
