// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `og` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|og|
//! |**Namespace base IRI**|[<http://ogp.me/ns#>](<http://ogp.me/ns#>)|
//! |**Description**||
//! |**Is defined by**|[<http://ogp.me/ns#>](<http://ogp.me/ns#>)|
//!

use crate::namespace;

namespace!(
    "<http://ogp.me/ns#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `audio`: A relevant audio URL for your object.
    audio, "audio",
    /// `audio album`: [DEPRECATED] An album to which some audio belongs.
    audio:album, "audio:album",
    /// `audio artist`: [DEPRECATED] An artist of some audio.
    audio:artist, "audio:artist",
    /// `audio secure URL`: A relevant, secure audio URL for your object.
    audio:secure_url, "audio:secure_url",
    /// `audio title`: [DEPRECATED] A title for some audio.
    audio:title, "audio:title",
    /// `audio type`: The mime type of an audio file e.g., "application/mp3"
    audio:type, "audio:type",
    /// `country name`: [DEPRECATED] The country name of the resource e.g., "USA"
    country_name, "country-name",
    /// `description`: A one to two sentence description of your object.
    description, "description",
    /// `determiner`: The word to precede the object's title in a sentence (e.g., "the" in "the statue of liberty").  Valid values are "a", "an", "the", "", and "auto".
    determiner, "determiner",
    /// `email`: [DEPRECATED] Email of the contact for your object.
    email, "email",
    /// `fax number`: [DEPRECATED] Fax number of the contact for your object.
    fax_number, "fax_number",
    /// `image`: An image URL which should represent your object within the graph.
    image, "image",
    /// `image height`: The height of an image.
    image:height, "image:height",
    /// `image secure url`: A secure image URL which should represent your object within the graph.
    image:secure_url, "image:secure_url",
    /// `image type`: The mime type of an image.
    image:type, "image:type",
    /// `image width`: The width of an image.
    image:width, "image:width",
    /// `International Standard Book Number`: [DEPRECATED] International Standard Book Number for you object.
    isbn, "isbn",
    /// `latitude`: [DEPRECATED] The latitude of the resource e.g., the latitude of a company.
    latitude, "latitude",
    /// `locale`: A Unix locale in which this markup is rendered.
    locale, "locale",
    /// `locality`: [DEPRECATED] The locality of the resource e.g, "Palo Alto"
    locality, "locality",
    /// `longitude`: [DEPRECATED] The longitude of the resource e.g., the longitude of a company.
    longitude, "longitude",
    /// `phone number`: [DEPRECATED] Phone number of the contact for your object.
    phone_number, "phone_number",
    /// `postal code`: [DEPRECATED] The postal code of the resource e.g., "94304"
    postal_code, "postal-code",
    /// `region`: [DEPRECATED] The region of the resource e.g., "CA"
    region, "region",
    /// `site name`: If your object is part of a larger web site, the name which should be displayed for the overall site. e.g., "IMDb".
    site_name, "site_name",
    /// `street address`: [DEPRECATED] The street address of the resource e.g., "1601 S California Ave".
    street_address, "street-address",
    /// `title`: The title of the object as it should appear within the graph, e.g.,  "The Rock".
    title, "title",
    /// `type`: The type of your object, e.g., "movie".  Depending on the type you specify, other properties may also be required.
    type_, "type",
    /// `universal product code`: [DEPRECATED] Universal Product Code for your object.
    upc, "upc",
    /// `url`: The canonical URL of your object that will be used as its permanent ID in the graph, e.g., "http://www.imdb.com/title/tt0117500/".
    url, "url",
    /// `video`: A relevant video URL for your object.
    video, "video",
    /// `video height`: The height of a video.
    video:height, "video:height",
    /// `video secure URL`: A relevant, secure video URL for your object.
    video:secure_url, "video:secure_url",
    /// `video type`: The mime type of a video e.g., "application/x-shockwave-flash"
    video:type, "video:type",
    /// `video width`: The width of a video.
    video:width, "video:width"
);
