// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `ma` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|ma|
//! |**Namespace base IRI**|[http://www.w3.org/ns/ma-ont#](http://www.w3.org/ns/ma-ont#)|
//! |**Description**|Created by Tobias Buerger, Jean Pierre Evain and Pierre-Antoine Champin with the RDFS Taskforce within the W3C Media Annotation Working Group.|
//! |**Is defined by**|[http://www.w3.org/ns/ma-ont#](http://www.w3.org/ns/ma-ont#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/ma-ont#",;
    /// ``: A person or organisation contributing to the media resource.
    Agent, "Agent",
    /// ``: A specialisation of Track for Audio to provide a link to specific data properties such as sampleRate, etc. Specialisation is defined through object properties.
    AudioTrack, "AudioTrack",
    /// ``: Any group of media resource e.g. a series.
    Collection, "Collection",
    /// ``: Ancillary data track e.g. captioning  in addition to video and audio tracks. Specialisation is made through the use of appropriate object properties.
    DataTrack, "DataTrack",
    /// ``: A still image / thumbnail / key frame related to the media resource or being the media resource itself.
    Image, "Image",
    /// ``: A location related to the media resource, e.g. depicted in the resource (possibly fictional) or where the resource was created (shooting location), etc.
    Location, "Location",
    /// ``: A media fragment (spatial, temporal, track...) composing a media resource. In other ontologies fragment is sometimes referred to as a 'part' or 'segment'.
    MediaFragment, "MediaFragment",
    /// ``: An image or an audiovisual media resource, which can be composed of one or more fragment / track.
    MediaResource, "MediaResource",
    /// ``: An organisation or moral agent.
    Organisation, "Organisation",
    /// ``: A physical person.
    Person, "Person",
    /// ``: Information about the rating given to a media resource.
    Rating, "Rating",
    /// ``: Information about The target audience (target region, target audience category but also parental guidance recommendation) for which a media resource is intended.
    TargetAudience, "TargetAudience",
    /// ``: A specialisation of MediaFragment for audiovisual content.
    Track, "Track",
    /// ``: A specialisation of Track for Video to provide a link to specific data properties such as frameRate, etc. Signing is another possible example of video track. Specialisation is defined through object properties.
    VideoTrack, "VideoTrack",
    /// ``: Corresponds to 'title.title' in the Ontology for Media Resources with a 'title.type' meaning "alternative".
    alternativeTitle, "alternativeTitle",
    /// ``: Corresponds to 'averageBitRate' in the Ontology for Media Resources, expressed in kilobits/second.
    averageBitRate, "averageBitRate",
    /// ``: The name by which a collection (e.g. series) is known.
    collectionName, "collectionName",
    /// ``: Corresponds to 'copyright.copyright' in the Ontology for Media Resources.
    copyright, "copyright",
    /// ``: A subproperty of 'hasRelatedLocation" used to specify where material shooting took place.
    createdIn, "createdIn",
    /// ``: Corresponds to 'date.date' in the Ontology for Media Resources with a 'date.type' meaning "creationDate".
    creationDate, "creationDate",
    /// ``: Corresponds to date.date in the ontology for Media Resources. Subproperties can be used to distinguish different values of 'date.type'. The recommended range is 'xsd:dateTime' (for compliance with OWL2-QL and OWL2-RL) but other time-related datatypes may be used (e.g. 'xsd:gYear', 'xsd:date'...).
    date, "date",
    /// ``: A subproperty of 'hasRelatedLocation' used to specify where the action depicted in the media is supposed to take place, as opposed to the location where shooting actually took place (see 'createdIn').
    depictsFictionalLocation, "depictsFictionalLocation",
    /// ``: Corresponds to 'description' in the Ontology for Media Resources. This can be specialised by using sub-properties e.g. 'summary' or 'script'.
    description, "description",
    /// ``: Corresponds to 'duration' in the Ontology for Media Resources.
    duration, "duration",
    /// ``: Corresponds to 'date.date' in the Ontology for Media Resources with a 'date.type' meaning "editDate".
    editDate, "editDate",
    /// ``: Corresponds to 'contributor.contributor' in the Ontology for Media Resources with a 'contributor.role' meaning "actor".
    features, "features",
    /// ``: Corresponds to 'namedFragment.label' in the Ontology for Media Resources.
    fragmentName, "fragmentName",
    /// ``: Corresponds to 'frameSize.height' in the Ontology for Media Resources, measured in frameSizeUnit.
    frameHeight, "frameHeight",
    /// ``: Corresponds to 'frameRate' in the Ontology for Media Resources, in frame per second.
    frameRate, "frameRate",
    /// ``: Corresponds to 'frameSize.unit' in the Ontology for Media Resources.
    frameSizeUnit, "frameSizeUnit",
    /// ``: Corresponds to 'frameSize.width' in the Ontology for Media Resources measured in frameSizeUnit.
    frameWidth, "frameWidth",
    /// ``: Corresponds to 'policy' in the Ontology for Media Resources with a 'policy.type' "access conditions".
    hasAccessConditions, "hasAccessConditions",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources with a 'fragment.role' meaning "audio-description".
    hasAudioDescription, "hasAudioDescription",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources with a 'fragment.role' meaning "captioning". This property can for example point to a spatial fragment, a VideoTrack or a DataTrack. The language of the captioning track can be expressed by attaching a 'hasLanguage' property to the specific track.
    hasCaptioning, "hasCaptioning",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources with a 'fragment.role' meaning "chapter".
    hasChapter, "hasChapter",
    /// ``: Corresponds to 'targetAudience.classification' in the Ontology for Media Resources. This property is used to provide a value characterising the target audience.
    hasClassification, "hasClassification",
    /// ``: Corresponds to 'targetAudience.identifier' in the Ontology for Media Resources. This is used to identify the reference sheme against which the target audience has been characterised.
    hasClassificationSystem, "hasClassificationSystem",
    /// ``: Corresponds to 'compression' in the Ontology for Media Resources.
    hasCompression, "hasCompression",
    /// ``: 
    hasContributedTo, "hasContributedTo",
    /// ``: Corresponds to 'contributor.contributor' in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'contributor.role'.
    hasContributor, "hasContributor",
    /// ``: 
    hasCopyrightOver, "hasCopyrightOver",
    /// ``: 
    hasCreated, "hasCreated",
    /// ``: Corresponds to 'creator.creator' in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'creator.role'. Note that this property is semantically a subproperty of 'hasContributor'.
    hasCreator, "hasCreator",
    /// ``: Corresponds to 'format' in the Ontology for Media Resources.
    hasFormat, "hasFormat",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'fragment.role'.
    hasFragment, "hasFragment",
    /// ``: Corresponds to 'genre' in the Ontology for Media Resources.
    hasGenre, "hasGenre",
    /// ``: Corresponds to 'keyword' in the Ontology for Media Resources.
    hasKeyword, "hasKeyword",
    /// ``: Corresponds to 'language' in the Ontology for Media Resources. The language used in the resource. A controlled vocabulary such as defined in BCP 47 SHOULD be used. This property can also be used to identify the presence of sign language (RFC 5646). By inheritance, the hasLanguage property applies indifferently at the media resource / fragment / track levels.  Best practice recommends to use to best possible level of granularity fo describe the usage of language within a media resource including at fragment and track levels.
    hasLanguage, "hasLanguage",
    /// ``: Corresponds to 'location.coordinateSystem' in the Ontology for Media Resources.
    hasLocationCoordinateSystem, "hasLocationCoordinateSystem",
    /// ``: 
    hasMember, "hasMember",
    /// ``: Corresponds to 'namedFragment' in the Ontology for Media Resources.
    hasNamedFragment, "hasNamedFragment",
    /// ``: Corresponds to 'policy' in the Ontology for Media Resources with a  'policy.type' meaning "permissions".
    hasPermissions, "hasPermissions",
    /// ``: Corresponds to 'policy' in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'policy.type'.
    hasPolicy, "hasPolicy",
    /// ``: 
    hasPublished, "hasPublished",
    /// ``: Corresponds to 'publisher' in the Ontology for Media Resources.
    hasPublisher, "hasPublisher",
    /// ``: Corresponds to 'rating' in the Ontology for Media Resources.
    hasRating, "hasRating",
    /// ``: Corresponds to 'rating.type' in the Ontology for Media Resources.
    hasRatingSystem, "hasRatingSystem",
    /// ``: Corresponds to 'relation' and in the Ontology for Media Resources with a 'relation.type' meaning "related image".
    hasRelatedImage, "hasRelatedImage",
    /// ``: Corresponds to 'location' in the Ontology for Media Resources. Subproperties are provided to specify, when possible, the relation between the media resource and the location.
    hasRelatedLocation, "hasRelatedLocation",
    /// ``: Corresponds to 'relation' and in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'relation.type'.
    hasRelatedResource, "hasRelatedResource",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources with a 'fragment.role' meaning "signing". This property can for example point to a spatial fragment or a VideoTrack. The sign language of the captioning track can be expressed by attaching a 'hasLanguage' property to the specific track.
    hasSigning, "hasSigning",
    /// ``: Corresponds to 'relation' and in the Ontology for Media Resources with a 'relation.type' meaning "source".
    hasSource, "hasSource",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources with a 'fragment.role' meaning "subtitling".
    hasSubtitling, "hasSubtitling",
    /// ``: Corresponds to 'targetAudience' in the Ontology for Media Resources.
    hasTargetAudience, "hasTargetAudience",
    /// ``: Corresponds to 'fragment' in the Ontology for Media Resources with a 'fragment.role' meaning "track".
    hasTrack, "hasTrack",
    /// ``: 
    isCaptioningOf, "isCaptioningOf",
    /// ``: 
    isChapterOf, "isChapterOf",
    /// ``: Corresponds to 'copyright.identifier' in the Ontology for Media Resources.
    isCopyrightedBy, "isCopyrightedBy",
    /// ``: 
    isCreationLocationOf, "isCreationLocationOf",
    /// ``: 
    isFictionalLocationDepictedIn, "isFictionalLocationDepictedIn",
    /// ``: 
    isFragmentOf, "isFragmentOf",
    /// ``: 
    isImageRelatedTo, "isImageRelatedTo",
    /// ``: 
    isLocationRelatedTo, "isLocationRelatedTo",
    /// ``: Corresponds to 'collection' in the Ontology for Media Resources.
    isMemberOf, "isMemberOf",
    /// ``: 
    isNamedFragmentOf, "isNamedFragmentOf",
    /// ``: Corresponds to 'rating.identifier' in the Ontology for Media Resources.
    isProvidedBy, "isProvidedBy",
    /// ``: 
    isRatingOf, "isRatingOf",
    /// ``: 
    isRelatedTo, "isRelatedTo",
    /// ``: 
    isSigningOf, "isSigningOf",
    /// ``: 
    isSourceOf, "isSourceOf",
    /// ``: 
    isTargetAudienceOf, "isTargetAudienceOf",
    /// ``: 
    isTrackOf, "isTrackOf",
    /// ``: Corresponds to 'location.altitude' in the Ontology for Media Resources.
    locationAltitude, "locationAltitude",
    /// ``: Corresponds to 'location.latitude' in the Ontology for Media Resources.
    locationLatitude, "locationLatitude",
    /// ``: Corresponds to 'location.longitude' in the Ontology for Media Resources.
    locationLongitude, "locationLongitude",
    /// ``: Corresponds to 'location.name' in the Ontology for Media Resources.
    locationName, "locationName",
    /// ``: Corresponds to 'locator' in the Ontology for Media Resources.
    locator, "locator",
    /// ``: Corresponds to 'title.title' in the Ontology for Media Resources with a 'title.type' meaning "original".
    mainOriginalTitle, "mainOriginalTitle",
    /// ``: Corresponds to 'numTracks.number' in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'numTracks.type'.
    numberOfTracks, "numberOfTracks",
    /// ``: 
    playsIn, "playsIn",
    /// ``: 
    provides, "provides",
    /// ``: Corresponds to 'rating.max' in the Ontology for Media Resources.
    ratingScaleMax, "ratingScaleMax",
    /// ``: Corresponds to 'rating.min' in the Ontology for Media Resources.
    ratingScaleMin, "ratingScaleMin",
    /// ``: Corresponds to 'rating.value' in the Ontology for Media Resources.
    ratingValue, "ratingValue",
    /// ``: Corresponds to 'date.date' in the Ontology for Media Resources with a 'date.type' meaning "recordDate".
    recordDate, "recordDate",
    /// ``: Corresponds to 'date.date' in the Ontology for Media Resources with a 'date.type' meaning "releaseDate".
    releaseDate, "releaseDate",
    /// ``: Corresponds to 'samplingRate' in the Ontology for Media Resources, in samples per second.
    samplingRate, "samplingRate",
    /// ``: Corresponds to 'title.title' in the Ontology for Media Resources. Subproperties can be used to distinguish different values of 'title.type'.
    title, "title",
    /// ``: Corresponds to 'fragment.name' in the Ontology for Media Resources, for Track fragments.
    trackName, "trackName"
);
