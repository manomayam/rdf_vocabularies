// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-ebucore")))]
//! This module provides terms for `EBUCore - the Dublin Core for media` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|EBUCore - the Dublin Core for media|
//! |**Prefix**|ebucore|
//! |**Namespace base IRI**|<http://www.ebu.ch/metadata/ontologies/ebucore/ebucore#>|
//! |**Description**|Guidelines: for the purpose of flexibility and interoperability with a wider range of implementations, some properties purposefully do not have a range and accept either a resource or a literal.  When a resource is used, it is recommended to reuse objects defined in the model (e.g. pair hasEvent/Event or hasRole/Role). Example 1: x hasRole 'actor'. Example 2: x hasRole _:Role_1 (a reference to the Concept identifier from a SKOS Role vocabulary defined in the ontology).|
//! |**Is defined by**|<https://www.ebu.ch/metadata/ontologies/ebucore/ebucore.rdf>|
//!

use crate::namespace;

namespace!(
    "http://www.ebu.ch/metadata/ontologies/ebucore/ebucore#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Access conditions`: The conditions under which content can be accessed.
    AccessConditions, "AccessConditions",
    /// `Action`: A class to log Actions.
    Action, "Action",
    /// `Action type`: To define a type of Action.
    Action_Type, "Action_Type",
    /// `Active format descriptor code`: To define an active format code.
    ActiveFormatDescriptorCode, "ActiveFormatDescriptorCode",
    /// `Affiliation`: An Organisation to which a Contact is affiliated (with period of validity).
    Affiliation, "Affiliation",
    /// `Agent`: A person / contact or organisation.
    Agent, "Agent",
    /// `Ancillary data`: Any ancillary data provided with the content             other than captioning and subtitling.
    AncillaryData, "AncillaryData",
    /// `Ancillary data format`: To define the format of AncillaryData such as             legacy data used to be carried in vertical blanking intervals. This is provided as free             text in an annotation label or as an identifier pointing to a term in a classification             scheme.
    AncillaryDataFormat, "AncillaryDataFormat",
    /// `Animal`: To identify an animal.
    Animal, "Animal",
    /// `Animal breed code`: To provide a breed code for an animal..
    AnimalBreedCode, "AnimalBreedCode",
    /// `Animal colour code`: To provide a colour code for an animal..
    AnimalColourCode, "AnimalColourCode",
    /// `Annotation`: A class used to annotate Assets.
    Annotation, "Annotation",
    /// `Annotation type`: To define a type of Annotation.
    Annotation_Type, "Annotation_Type",
    /// `Artefact`: To identify and describe artefacts used in a production (on and behind the stage).
    Artefact, "Artefact",
    /// `Artefact type`: To define a type of artefact.
    Artefact_Type, "Artefact_Type",
    /// `Asset`: The Class "Asset" is an             object to which an identifier will be associated at commissioning. It will serve as a             central reference point to manage rights associated to EditorialObjects, Resources,             MediaResources or Essences, and PublicationEvents (distribution and exploitation             conditions).
    Asset, "Asset",
    /// `Asset type`: To define a type of asset.
    Asset_Type, "Asset_Type",
    /// `Atmosphere`: To describe a feeling summarising the atmosphere.
    Atmosphere, "Atmosphere",
    /// `Target audience`: The target audience (target region, target             audience category but also parental guidance recommendation) for which the media             resource is intended.
    AudienceLevel, "AudienceLevel",
    /// `Audience rating`: The audience by which the Resource can be             seen according to ratings like MPAA  (http://en.wikipedia.org/wiki/Motion_picture_rating_system) or other organisational / national / local standards.
    AudienceRating, "AudienceRating",
    /// `Audience score recording technique`: To define the technique use to measure an audience score.
    AudienceScoreRecordingTechnique, "AudienceScoreRecordingTechnique",
    /// `Audio channel function`: To define the function of an AudioChannel.
    AudioChannelFunction, "AudioChannelFunction",
    /// `Audio channel purpose`: To define the purpose of an AudioChannel.
    AudioChannelPurpose, "AudioChannelPurpose",
    /// `Audio codec`: To provide information about an audio codec.
    AudioCodec, "AudioCodec",
    /// `Audio content`: An audioContent defines one component of a programme (e.g. background 				music), its association with an audioGroup (e.g. a 2.0 audioPackFormat of 				audioChannelFormats for stereo reproduction), its association with an 				audioStreamFormat, and its set of loudness parameters.
    AudioContent, "AudioContent",
    /// `Audio content type`: to define a type of AudioContent.
    AudioContent_Type, "AudioContent_Type",
    /// `Audio description`: A Track containing audio description.
    AudioDescription, "AudioDescription",
    /// `Audio encoding format`: The encoding format for the audio.
    AudioEncodingFormat, "AudioEncodingFormat",
    /// `Audio format`: To define an AudioFormat.
    AudioFormat, "AudioFormat",
    /// `Audio object`: To define an audio object in reference to the Audio Definition Model (ADM)
    AudioObject, "AudioObject",
    /// `Audio programme`: A set of one or more audioContent that derive from the same material, 				i.e. an audioMultiplex, and the definition of its multiplexed audioContents (e.g. 				foreground and commentary, background music).
    AudioProgramme, "AudioProgramme",
    /// `Audio programme type`: to define a type of AudioProgramme.
    AudioProgramme_Type, "AudioProgramme_Type",
    /// `Audio stream`: An audioStreamFormat describes a decodable signal - PCM signal or a Dolby E stream for example. It is composed of one or more AudioTracks.
    AudioStream, "AudioStream",
    /// `Audio track`: An audioTrack is the basic audio data container of a medium. Attribute is 				an unambiguous reference to this container in a given medium.
    AudioTrack, "AudioTrack",
    /// `Audio track purpose`: To describe the purpose of an AudioTrack e.g. dubbing.
    AudioTrackPurpose, "AudioTrackPurpose",
    /// `Award`: To describe an Award and associated information.
    Award, "Award",
    /// `Award type`: To define a type of Award.
    Award_Type, "Award_Type",
    /// `BMContent`: The FIMS BMContent.
    BMContent, "BMContent",
    /// `BMEssence`: The FIMS Essence
    BMEssence, "BMEssence",
    /// `BMTemplate`: A template describe as a BMEssence.
    BMTemplate, "BMTemplate",
    /// `Bibliographical object`: Documents of various nature.
    BibliographicalObject, "BibliographicalObject",
    /// `Biography`: To record a biography.
    Biography, "Biography",
    /// `Brand`: A group of EditorialObjects having a Brand as a             common denominator.
    Brand, "Brand",
    /// `Breaking news item`: To describe a breaking news.
    BreakingNewsItem, "BreakingNewsItem",
    /// `Business Object`: An image, a document, an annotation             (descriptive textual metadata or audio/video tag), a tag (time related in audiovisual             media resources), or an audiovisual media resource (optionally composed of one or more             fragment / part and / or audio, video data tracks). Other types of BusinessObjects may             be defined as subclasses.
    BusinessObject, "BusinessObject",
    /// `Business object type`: To define a type of business object.
    BusinessObject_Type, "BusinessObject_Type",
    /// `Captioning`: To signal the presence of hard of hearing             captioning.
    Captioning, "Captioning",
    /// `Captioning format`: To define the format of captioning.             Captioning's main use isfor hard of hearing transcription. This is provided as             free text in an annotation label or as an identifier pointing to a term in a             classification scheme.
    CaptioningFormat, "CaptioningFormat",
    /// `Cast member`: A member of the cast list (a list of performers/actors and associated fictitious             characters).
    Cast, "Cast",
    /// `Character`: E.g. a fictitious contact / person.
    Character, "Character",
    /// `City code`: To allocate a city code.
    CityCode, "CityCode",
    /// `Clip`: For use in models where Clip is common.
    Clip, "Clip",
    /// `Closed caption`: Closed captioning is provided as separate             content.
    ClosedCaptions, "ClosedCaptions",
    /// `Closed subtitling`: Closed subtitles are provided as separate             content.
    ClosedSubtitling, "ClosedSubtitling",
    /// `Codec`: To provide information on a codec.
    Codec, "Codec",
    /// `Collection`: A group of EditorialObjects. There can be many             types of collections for which specific sub-classes should be defined. In the worl of             archives, A collection corresponds to all items belonging to an individual /             collector.
    Collection, "Collection",
    /// `Colour space`: The CoulourSpace of a VideoResource. A             ColourSpace is defined as free text in an annotation label or as an identifier pointing             to a term in a classification scheme such as             http://www.ebu.ch/metadata/ontologies/skos/ebu_ColourCodeCS.rdf.
    ColourSpace, "ColourSpace",
    /// `Commercial code`: To identify a type of commercial content.
    CommercialCode, "CommercialCode",
    /// `Component`: A component e.g. audio, video, data or else or a MediaResource or Essence.
    Component, "Component",
    /// `Contact`: A physical person.
    Contact, "Contact",
    /// `Container codec`: To identify an container codec, e.g. MXF
    ContainerCodec, "ContainerCodec",
    /// `Container encoding format`: To define the conatiner encoding format.
    ContainerEncodingFormat, "ContainerEncodingFormat",
    /// `Container Mime type`: The definition of the container if available as             a MIME type. This is provided as free text in an annotation label or as an identifier             pointing to a term in a classification scheme. For more information:             http://www.iana.org/assignments/media-types/application/index.html.
    ContainerMimeType, "ContainerMimeType",
    /// `Content alert`: To provide information about a particular type of content potentially sensitive.
    ContentAlert, "ContentAlert",
    /// `Editorial code`: To define a code of EditorialFormat
    ContentEditorialCode, "ContentEditorialCode",
    /// `Editorial format`: To define an EditorialFormat
    ContentEditorialFormat, "ContentEditorialFormat",
    /// `Contract type`: To define a type of contract.
    ContractType, "ContractType",
    /// `Copyright`: To provide a copyright             statement.
    Copyright, "Copyright",
    /// `Costume`: To identify and describe Costumes used in productions.
    Costume, "Costume",
    /// `Costume type`: To define a costume type.
    CostumeType, "CostumeType",
    /// `Country code`: To identify a country by its ISO code.
    CountryCode, "CountryCode",
    /// `Coverage restrictions`: To provide information on possible restrictions             regarding the temporal and spatial coverage for publication.
    CoverageRestrictions, "CoverageRestrictions",
    /// `Creative commons`: A set of creative commons rights.
    CreativeCommons, "CreativeCommons",
    /// `Crew member`: A member of the Crew.
    Crew, "Crew",
    /// `Cuisine style`: To identify a style of Cuisine.
    CuisineStyle, "CuisineStyle",
    /// `Currency code`: To identify a currency by its ISO code.
    CurrencyCode, "CurrencyCode",
    /// `DID`: The Data Identifier word (along with the SDID,             if used), indicates the type of ancillary data that the packet corresponds             to.
    DID, "DID",
    /// `Data format`: To provide addtional technical information on             the characteristics of data streams in a MediaResource including but not limited to             AncillaryData, Subtilting and Captioning. Additional specific data format may be defined             as subclasses of DataFormat.
    DataFormat, "DataFormat",
    /// `Data track`: Ancillary data track e.g. Â¨captioning"             or "subtitling" in addition to video and audio tracks.
    DataTrack, "DataTrack",
    /// `Department`: A department within and             organisation.
    Department, "Department",
    /// `Depicted Event`: A DepictedEVent is fictitious or historical or             other sort of Event that the content of the BusinessObject or resource relates             to.
    DepictedEvent, "DepictedEvent",
    /// `Disclaimer`: To provide a disclaimer of any             form.
    Disclaimer, "Disclaimer",
    /// `Document`: To describe a publication in the form of a             document e.g. a html webpage (news item) or a pdf document e.g. a script.
    Document, "Document",
    /// `Document format`: To provide technical information about the             format of a document such as the orientation. This is provided as free text in an             annotation label or as an identifier pointing to a term in a classification             scheme.
    DocumentFormat, "DocumentFormat",
    /// `Dopesheet`: Provides additional information about a NewsItem, e.g. date and place, subject.
    Dopesheet, "Dopesheet",
    /// `Editorial Object`: In the audiovisual domain, the Class             EditorialObject transforms a commissioned concept into an editorial definition of a             MediaResource before fabrication (in the Production Domain) and Distribution (in the             Distribution Domain). An EditorialObject is a set of descriptive metadata summarising             e.g. editing decisions. An EditorialObject can also be a part of an EditorialObject,             which is defined by its start time and duration. An EditorialObject can also be a group             of EditorialObjects. For example a series composed of episodes is defined as an             EditorialObject.
    EditorialObject, "EditorialObject",
    /// `Editorial object type`: To define a type of editorial object.
    EditorialObject_Type, "EditorialObject_Type",
    /// `Emotion`: A class to log Emotions.
    Emotion, "Emotion",
    /// `Emotion type`: To define a type of emotion.
    Emotion_Type, "Emotion_Type",
    /// `Encoding`: To provide a definition of the encoding format             for audio and video. This is provided as free text in an annotation label or as an             identifier pointing to a term in a classification scheme e.g.             http://www.ebu.ch/metadata/ontologies/skos/ebu_AudioCompressionCodeCS.rdf or             http://www.ebu.ch/metadata/ontologies/skos/ebu_VideoCompressionCodeCS.rdf.
    EncodingFormat, "EncodingFormat",
    /// `Episode`: To describe an episode in a series.
    Episode, "Episode",
    /// `Essence`: Essence is content ready for distribution. Essence can become a MediaResource in further production processes.
    Essence, "Essence",
    /// `Event`: Additional types of event shall be defined as             new sub-classes of event.
    Event, "Event",
    /// `Event type`: To define a type of event.
    EventType, "EventType",
    /// `Exclusivity type`: To define a type of exclusity rights.
    ExclusivityType, "ExclusivityType",
    /// `Exploitation issues`: To highlight potential exploitation             issues.
    ExploitationIssues, "ExploitationIssues",
    /// `Feature`: The editorial object for a feature film.
    Feature, "Feature",
    /// `Fictional event`: To describe a fictional Event.
    FictionalEvent, "FictionalEvent",
    /// `Fictional location`: To describe a fictional Location.
    FictionalLocation, "FictionalLocation",
    /// `Fictional organisation`: To define a fictional Organisation.
    FictionalOrganisation, "FictionalOrganisation",
    /// `Fictional person`: To describe a fictional Person, e.g. a character in a drama.
    FictionalPerson, "FictionalPerson",
    /// `File format`: A file format for Resources other than             audiovisual resources. The format is defined as free text or pointing at a term in a             classification scheme e.g.             http://www.ebu.ch/metadata/ontologies/skos/ebu_FileFormatCS.rdf.
    FileFormat, "FileFormat",
    /// `Food`: To describe Food shown or consumed in productions.
    Food, "Food",
    /// `Food style`: To define a style of food.
    FoodStyle, "FoodStyle",
    /// `Format`: The format provides technical information on             the format of a Resource. A BusinessObject can be instantiated in a variety of Resources             each in a particular Format. Other specific data formats may be defined as subclasses of             format.
    Format, "Format",
    /// `Generation`: Identifies the generation of a version of a resource, i.e. master, edit master, distribution copy, etc.
    Generation, "Generation",
    /// `Genre`: This class shall be used to provide information             on the genre of the BusinessObject or Resource. This is provided as free text in an             annotation label or as an identifier pointing to a term in a classification scheme e.g.             http://www.ebu.ch/metadata/ontologies/skos/ebu_ContentGenreCS.rdf or             http://www.ebu.ch/metadata/ontologies/skos/ebu_EditorialFormatCodeCS.rdf.
    Genre, "Genre",
    /// `Group`: To define a collection / group of media             resources, for example a series made of episodes.
    Group, "Group",
    /// `IPR restrictions`: To provide information on intellectual             property.
    IPRRestrictions, "IPRRestrictions",
    /// `Identifier`: To support the use of structured identifiers.
    Identifier, "Identifier",
    /// `Identifier type`: To define a type of identifier.
    IdentifierType, "IdentifierType",
    /// `Image`: A still image / thumbnail / key frame / logo             related to the media resource or being the media resource itself.
    Image, "Image",
    /// `Image codec`: to identify a codec for images
    ImageCodec, "ImageCodec",
    /// `Image format`: To provide technical information about the             format of an image such as the orientation. This is provided as free text in an             annotation label or as an identifier pointing to a term in a classification             scheme.
    ImageFormat, "ImageFormat",
    /// `Intention code`: To indicate the purpose for which content was created.
    IntentionCode, "IntentionCode",
    /// `Item`: An item e.g. newsItem or sportItem
    Item, "Item",
    /// `Key career event`: To describe a key career Event of a Contact.
    KeyCareerEvent, "KeyCareerEvent",
    /// `Key event`: To describe a significant event.
    KeyEvent, "KeyEvent",
    /// `Key personal event`: A key personal Event of a Contact.
    KeyPersonalEvent, "KeyPersonalEvent",
    /// `key frame`: A key frame is a frame extarcted from video,             e.g. representative of a part of a MediaResource.
    Keyframe, "Keyframe",
    /// `Keyword`: To proivde keywords and define key concepts             illustrating the content of the Resource or EditorialObject. This is provided as free             text in an annotation label or as an identifier pointing to a term in a classification             scheme.
    Keyword, "Keyword",
    /// `Language`: To provide information on languages present in             the BusinessObject and its purpose. This is provided as free text in an annotation label             or as an identifier pointing to a term in a classification scheme.Other language             specific types may be added as subclasses of language.
    Language, "Language",
    /// `Licensing`: To define the licensing terms associated with an Asset.
    Licensing, "Licensing",
    /// `Link`: To define a custom link.
    Link, "Link",
    /// `Location`: A location related to the media resource, e.g.             depicted in the resource (possibly fictional) or where the resource was created             (shooting location), etc.
    Location, "Location",
    /// `Location code.`: A code given to a Location.
    LocationCode, "LocationCode",
    /// `Location time type`: To define a type of time at a location.
    LocationTimeType, "LocationTimeType",
    /// `Location type`: To define a type of location.
    LocationType, "LocationType",
    /// `Locator`: Custom attributes are to be associated by implementers.
    Locator, "Locator",
    /// `Logo`: A Logo allows to visually identify an             organisation, publicationService, publicationChannel, or ratings /             parentalGuidance
    Logo, "Logo",
    /// `Media Fragment`: A MediaFragment is a temporal or spatial segment of a resource identified by a MediaGragment URI (http://www.w3.org/2008/WebVideo/Fragments/WD-media-fragments-spec/).
    MediaFragment, "MediaFragment",
    /// `Media Resource`: The use of MediaResource is reserved to             audiovisual content. In a production process, several MediaResources can be edited and assembled to realsie an Essence ready for distribution (see IMF package and OPL)
    MediaResource, "MediaResource",
    /// `Media resource type`: To define a type of MediaResource.
    MediaResource_Type, "MediaResource_Type",
    /// `Media type`: To provide additional information on the type of media.
    MediaType, "MediaType",
    /// `Medium`: To provide information on the medium formats in             which the resource is available. This is provided as free text in an annotation label or             as an identifier pointing to a term in a classification scheme.
    Medium, "Medium",
    /// `Metadata track`: A Track on which metadata is embedded (e.g. MXF).
    MetadataTrack, "MetadataTrack",
    /// `Mime type`: The definition of the container if available as             a MIME type. This is provided as free text in an annotation label or as an identifier             pointing to a term in a classification scheme. For more information:             http://www.iana.org/assignments/media-types/index.html.
    MimeType, "MimeType",
    /// `News Item`: A NewsItem aggregates all information about a particular news event.
    NewsItem, "NewsItem",
    /// `Object type`: To specify the type of BusinessObject e.g. and             EditorialObject of type "programme" or clip". This is             provided as free text in an annotation label or as an identifier pointing to a term in a             classification scheme e.g.             http://www.ebu.ch/metadata/ontologies/skos/ebu_ObjectTypeCodeCS.rdf.
    ObjectType, "ObjectType",
    /// `Open captions`: Open Captions are burned in the             image.
    OpenCaptions, "OpenCaptions",
    /// `Open subtitling`: Open subtitles are burned in the             image.
    OpenSubtitling, "OpenSubtitling",
    /// `Organisation`: An organisation (business, corporation, federation, etc.) or moral agent (government body).
    Organisation, "Organisation",
    /// `Language`: The original language in which the             BusinessObject or Resource has been created and released. This is provided as free text             in an annotation label or as an identifier pointing to a term in a classification             scheme.
    OriginalLanguage, "OriginalLanguage",
    /// `Part, Fragment, Segment`: A Fragment is a particular section of a             MediaResource identified by a start and end time or duration. Fragment can also be             called segment or part.
    Part, "Part",
    /// `Part type`: To define a type or part.
    Part_Type, "Part_Type",
    /// `Party`: To identify a Party intervening in a transaction or contractual agreement.
    Party, "Party",
    /// `Person`: To describe a Person.
    Person, "Person",
    /// `Physical resource`: To describe a physical resource e.g. a tape.
    PhysicalResource, "PhysicalResource",
    /// `Pictogram`: A visual / graphical representation of a concept.
    Pictogram, "Pictogram",
    /// `Picture`: A photography, a logo, a pictogram, etc.
    Picture, "Picture",
    /// `Picture display format code`: To define a picture display format code.
    PictureDisplayFormat, "PictureDisplayFormat",
    /// `Platform`: A platform like a network or operator platform.
    Platform, "Platform",
    /// `Programme`: An EditorialObject corresponding to a             MediaResource ready for publication.
    Programme, "Programme",
    /// `Props`: To identify and describe Props used in productions (e.g. vehicles, objects of various shapes and brand and purpose, etc.).
    Props, "Props",
    /// `Provenance`: 
    Provenance, "Provenance",
    /// `Publication Channel`: The name of the channel through which a             Resource has been published as a PublicationEvent. A PublicationChannel can use a             variety of medias e.g. broadcast or online.
    PublicationChannel, "PublicationChannel",
    /// `Publication channel type`: To define a type of publication channel.
    PublicationChannel_Type, "PublicationChannel_Type",
    /// `Publication Event`: To describe any manifestation of a media             resource on any media (live, on demand, catch-up TV, etc.) and the appropriate             PublciationChannel.
    PublicationEvent, "PublicationEvent",
    /// `Publication event type`: To define a type of publication event.
    PublicationEvent_Type, "PublicationEvent_Type",
    /// `Publication History`: A collection of PublicationEvents through which             a resource has been published.
    PublicationHistory, "PublicationHistory",
    /// `Publication medium`: 
    PublicationMedium, "PublicationMedium",
    /// `Publication History`: A collection of PublicationEvents organised as a PublicationPlanning.
    PublicationPlan, "PublicationPlan",
    /// `Publication plan type`: To define a type of publication plan.
    PublicationPlan_Type, "PublicationPlan_Type",
    /// `Radio Programme`: A programme for distribution on radio             channels.
    RadioProgramme, "RadioProgramme",
    /// `Rating`: All the information about the rating/evaluation             given to a media resource by an Agent i.e. a person/Contact or             Organisation.
    Rating, "Rating",
    /// `Record`: The record the description of an Asset.
    Record, "Record",
    /// `Region code`: To define a region.(@en}
    RegionCode, "RegionCode",
    /// `Relation`: To define links and relations.
    Relation, "Relation",
    /// `Relation type`: To specify a type of relation.
    Relation_Type, "Relation_Type",
    /// `Resource`: To describe a Resource.
    Resource, "Resource",
    /// `Resource type`: To define a type of resource.
    Resource_Type, "Resource_Type",
    /// `Review`: To provide a Review.
    Review, "Review",
    /// `Rights`: To provide information on the rights, including             intellectual property, related to a BusinessObject or Resource.
    Rights, "Rights",
    /// `Rights Clearance`: To signal that rights have been cleared (or             not)
    RightsClearance, "RightsClearance",
    /// `Rights type`: To define a type of Rights.
    RightsType, "RightsType",
    /// `Role`: To define the role / action of an agent. This             is provided as free text in an annotation label or as an identifier pointing to a term             in a classification scheme.
    Role, "Role",
    /// `SDID`: Secondary data identification word for             ancillary data. Send mode identifier. An identifier which indicates the transmission             timing for closed caption data.
    SDID, "SDID",
    /// `Scene`: A specifc type of Part.
    Scene, "Scene",
    /// `Season`: A series can be composed of one or more seasons             clustering a certain number of episodes. Fro this reason, seasons are related to series             using the isRelatedTo property.
    Season, "Season",
    /// `Series`: Series is a particular type of collection. TV             or Radio Series are composed of Episodes.
    Series, "Series",
    /// `Service`: A service is the umbrella under which one or             more PublicationChannel is operated.
    Service, "Service",
    /// `Service type`: To define a type of service.
    Service_Type, "Service_Type",
    /// `Shot`: A specifc type of Part.
    Shot, "Shot",
    /// `Sign language code`: To identify a sign language by its code.
    SignLanguageCode, "SignLanguageCode",
    /// `Signing`: To signal the presence of Signing for hard of             hearing users. The type of Signing (e.g. incursted in or else) or language of Signing             can be specified using the appropriate properties.
    Signing, "Signing",
    /// `Signing format`: To provide additional information on the             signing format. This is provided as free text in an annotation label or as an identifier             pointing to a term in a classification scheme.
    SigningFormat, "SigningFormat",
    /// `Sport item`: A SportItem aggregates all information about a sport event.
    SportItem, "SportItem",
    /// `Staff member.`: A member of Staff.
    Staff, "Staff",
    /// `Standard`: identifies the technical video standard of a resource, i.e. NTSC or PAL.
    Standard, "Standard",
    /// `Sticker`: A sticker associated with a Costume.
    Sticker, "Sticker",
    /// `Storage type`: The type of storage used for the repository.             This is provided as free text in an annotation label or as an identifier pointing to a             term in a classification scheme.
    Storage_Type, "Storage_Type",
    /// `Stream`: A continuous stream of bits.
    Stream, "Stream",
    /// `Subject`: A term describing the topic covered by the             BusinessObject or resource. This is provided as free text in an annotation label or as             an identifier pointing to a term in a classification scheme.
    Subject, "Subject",
    /// `Subtitling`: To signal the presence of subtitles for             translation in alternative languages.
    Subtitling, "Subtitling",
    /// `Subtitling format`: To define the format of subtitling.             subtitling's main use isfor translation. This is provided as free text in an             annotation label  or as an identifier pointing to a term in a classification             scheme.
    SubtitlingFormat, "SubtitlingFormat",
    /// `TV Programme`: A programme for distribution on television             channels.
    TVProgramme, "TVProgramme",
    /// `Target audience`: To identify the audience for which the content was created.
    TargetAudience, "TargetAudience",
    /// `Target Platform`: To specify a target platform.
    TargetPlatform, "TargetPlatform",
    /// `Team`: To define a Team.
    Team, "Team",
    /// `Template`: An Essence defined as a Template with all associated technical parameters.
    Template, "Template",
    /// `Territory code`: To identify a territory e.g. by its UN code.
    TerritoryCode, "TerritoryCode",
    /// `Text Annotation`: A class specific to the annotation of a text or portions of text.
    TextAnnotation, "TextAnnotation",
    /// `Text line`: To provide lines of text extracted from or additional to the resource.
    TextLine, "TextLine",
    /// `Text line type`: To define a TextLine type.
    TextLine_Type, "TextLine_Type",
    /// `Text usage type`: To specify the usage of a text.
    TextUsageType, "TextUsageType",
    /// `Theme`: To define a Theme associated with an Asset.
    Theme, "Theme",
    /// `Thumbnail`: A thumbnail is a low resolution picture that             can be associated with EditorialObjects or e.g. MediaResources or             Contacts.
    Thumbnail, "Thumbnail",
    /// `Timecode track`: A track with timecode information e.g. in MXF.
    TimecodeTrack, "TimecodeTrack",
    /// `Timed text authoring technique`: To define a timed text authoring technique.
    TimedTextAuthoringTechnique, "TimedTextAuthoringTechnique",
    /// `Timed text content type`: To define a type of timed text.
    TimedTextContentType, "TimedTextContentType",
    /// `Timed text subtitle target format`: To define a timed text subtitle format.
    TimedTextSubtitleTargetFormat, "TimedTextSubtitleTargetFormat",
    /// `Timeline track`: To define a time sequence of EditorialObjects.
    TimelineTrack, "TimelineTrack",
    /// `Timeline track type`: To specify a type or TimelineTrack.
    TimelineTrack_Type, "TimelineTrack_Type",
    /// `Topic`: A type subject for use in some contexts. This             is provided as free text in an annotation label or as an identifier pointing to a term             in a classification scheme.
    Topic, "Topic",
    /// `Track`: Audiovisual content can be composed of audio,             video and data Tracks (including captioning and subtitling).
    Track, "Track",
    /// `Track purpose`: To define the prupose of a track.
    TrackPurpose, "TrackPurpose",
    /// `Track type`: To define a type of track.
    Track_Type, "Track_Type",
    /// `Type`: An expression of type in textual form or as a term from a classification scheme.
    Type, "Type",
    /// `Usage restrictions`: To define a set of UsageRestrictions.
    UsageRestrictions, "UsageRestrictions",
    /// `Usage rights`: Usage rights associated with content.
    UsageRights, "UsageRights",
    /// `Version`: To specifically identify a Version of an EditorialObject.
    Version, "Version",
    /// `Video codec`: To provide information about a video codec.
    VideoCodec, "VideoCodec",
    /// `Video encoding format`: The encoding format of the video.
    VideoEncodingFormat, "VideoEncodingFormat",
    /// `Video format`: To define an VideoFormat.
    VideoFormat, "VideoFormat",
    /// `Video stream`: A decodable video stream of bits.
    VideoStream, "VideoStream",
    /// `Video track`: A specialisation of Track for Video to provide             a link to specific data properties such as frameRate, etc. Signing is another possible             example of video track. Specific VideoTracks such as Signing can be defined as sub             VideoTracks.. In advanced systems, different VideoTracks can be used to provide e.g.             different viewing angles.
    VideoTrack, "VideoTrack",
    /// `Wrapping type`: To define a type of wrapping.
    WrappingType, "WrappingType",
    /// `Abridged title`: An abridged title.
    abrigedTitle, "abrigedTitle",
    /// `Abstract`: To provide an abstract.
    abstract_, "abstract",
    /// `Action description`: The description of an Action.
    actionDescription, "actionDescription",
    /// `Action identifier`: An identifier attributed to an Action.
    actionId, "actionId",
    /// `Action name`: The name of an Action.
    actionName, "actionName",
    /// `Action timestamp`: The time when the Action occurs.
    actionTimestamp, "actionTimestamp",
    /// `Action edit unit number`: The edit unit number at which the Action occurs.
    actionTimestampEditUnits, "actionTimestampEditUnits",
    /// `Action normal play time`: The normal play time at which the Action occurs.
    actionTimestampNormalPlayTime, "actionTimestampNormalPlayTime",
    /// `Action timecode`: The timecode at which the Action occurs.
    actionTimestampTimecode, "actionTimestampTimecode",
    /// `Action timecode (dropframe)`: The timecode (dropframe) at which the Action occurs.
    actionTimestampTimecodeDropFrame, "actionTimestampTimecodeDropFrame",
    /// `Action type`: A type of Action.
    actionType, "actionType",
    /// `Activity end date`: To provide the end date of activity of an Organisation.
    activityEndDate, "activityEndDate",
    /// `Activity start date`: To provide the start date of activity of an Organisation.
    activityStartDate, "activityStartDate",
    /// `Adult content flag`: A flag to indiucate adult content.
    adultContent, "adultContent",
    /// `Affiliation end date`: The date of end of Affiliation.
    affiliationEndDate, "affiliationEndDate",
    /// `Affiliation start date`: The date of Affiliation.
    affiliationStartDate, "affiliationStartDate",
    /// `Age`: The age of a Contact/Person.
    age, "age",
    /// `DBPedia`: A link to a DBPedia page.
    agentDbpedia, "agentDbpedia",
    /// `Description`: To provide a description of an Agent.
    agentDescription, "agentDescription",
    /// `email`: To provide an email address.
    agentEmailAddress, "agentEmailAddress",
    /// `Facebook`: 
    agentFacebook, "agentFacebook",
    /// `Agent fee`: The fee of an Agent.
    agentFee, "agentFee",
    /// `Flickr`: 
    agentFlickr, "agentFlickr",
    /// `Agent identifier`: An identifier attributed to an Agent.
    agentId, "agentId",
    /// `Wikidata`: A link to an imdb page.
    agentImdb, "agentImdb",
    /// `Instagram`: 
    agentInstagram, "agentInstagram",
    /// `Agent linked data`: Range: a URL or URI.
    agentLinkedData, "agentLinkedData",
    /// `LinkedIn`: 
    agentLinkedIn, "agentLinkedIn",
    /// `Mobile`: To provide the mobile telephone number of an             Agent (Contact/Person or organisation)
    agentMobileTelephoneNumber, "agentMobileTelephoneNumber",
    /// `Name`: To provide a name of an Agent.
    agentName, "agentName",
    /// `Nickname`: To provide a nickname of a Contact/Person.
    agentNickname, "agentNickname",
    /// `Previous name`: To provide the previous name of a Contact/Person.
    agentPreviousName, "agentPreviousName",
    /// `Related information link`: To provide a link to a web resource containing             information related to an Agent (Contact/Person or Organisation).
    agentRelatedInformationLink, "agentRelatedInformationLink",
    /// `Related link`: To provide a link to e.g. a web resource related to an Agent.
    agentRelatedLink, "agentRelatedLink",
    /// `Related press link`: To provide a link to a web resource containing             information related to an Agent (Contact/Person or Organisation).
    agentRelatedPressLink, "agentRelatedPressLink",
    /// `Socail media`: Links to an Agent's social media.
    agentSocialMedia, "agentSocialMedia",
    /// `Telephone`: To provide the telephone number of an Agent             (Contact/Person or Organisation).
    agentTelephoneNumber, "agentTelephoneNumber",
    /// `Twitter`: 
    agentTwitter, "agentTwitter",
    /// `Agent type`: Range: string or anyURI or Concept.
    agentType, "agentType",
    /// `Homepage`: To provide the address of the webpage of an             Agent (Contact/Person or Organisation).
    agentWebHomepage, "agentWebHomepage",
    /// `Wikidata`: A link to a wikidata page.
    agentWikidata, "agentWikidata",
    /// `Wikipedia`: 
    agentWikipedia, "agentWikipedia",
    /// `Alternative title.`: To provide an alternative title.
    alternativeTitle, "alternativeTitle",
    /// `Animal birth year`: To year of birth of an animal.
    animalBirthYear, "animalBirthYear",
    /// `Animal character name`: To associate a fictitious character name with an animal.
    animalCharacterName, "animalCharacterName",
    /// `Animal code`: To associate a code with an animal.
    animalCode, "animalCode",
    /// `Animal description`: To describe an animal.
    animalDescription, "animalDescription",
    /// `Animal gender`: To give the gender of an animal.
    animalGender, "animalGender",
    /// `Animal Id`: To associate an Id with an animal.
    animalId, "animalId",
    /// `Animal name`: To name an animal.
    animalName, "animalName",
    /// `Animal passport`: To replicate the passport of an animal.
    animalPassport, "animalPassport",
    /// `Annotation confidence`: To estimate the confidence in an Annotation.
    annotationConfidence, "annotationConfidence",
    /// `Annotation curation date & time`: To provide the date and time when an Annotation has been reviewed.
    annotationCurationDateTime, "annotationCurationDateTime",
    /// `Annotation description`: To describe an Annotation.
    annotationDescription, "annotationDescription",
    /// `Annotation Id`: To identify an Annotation.
    annotationId, "annotationId",
    /// `Annotation name`: To name an Annotation.
    annotationName, "annotationName",
    /// `Annotation saliency`: To estimate the saliency of an Annotation.
    annotationSaliency, "annotationSaliency",
    /// `Annotation type`: Range: string, anyURI or Concept.
    annotationType, "annotationType",
    /// `Exclusion area`: Range: string or CountryCode.
    appliesOutOf, "appliesOutOf",
    /// `Media resource`: To identify the media resource to which the Rating applies.
    appliesTo, "appliesTo",
    /// `Asset`: The Asset to which Rights apply.
    applyTo, "applyTo",
    /// `Agent`: Range: Agent or string
    approvedBy, "approvedBy",
    /// `Artefact availability flag`: To flag the availability of an Artefact.
    artefactAvailability, "artefactAvailability",
    /// `Artefact box height.`: The height of the box containing the Artefact.
    artefactBoxHeight, "artefactBoxHeight",
    /// `Artefact box top left corner Y position.`: The coordinates on a vertical axis of the position of the top left corner of the box containing the Artefact.
    artefactBoxTopLeftCornerLineNumber, "artefactBoxTopLeftCornerLineNumber",
    /// `Artefact box top left corner X position.`: The coordinates on an horizontal axis of the position of the top left corner of the box containing the Artefact.
    artefactBoxTopLeftCornerPixelNumber, "artefactBoxTopLeftCornerPixelNumber",
    /// `Artefact box width.`: The width of the box containing the Artefact.
    artefactBoxWidth, "artefactBoxWidth",
    /// `Artefact brand`: To specify the brand of an Artefact.
    artefactBrand, "artefactBrand",
    /// `Artefact colour(s)`: To provide the clour(s) of an Artefact.
    artefactColour, "artefactColour",
    /// `Artefact comment`: To provide a contextual comment about an Artefact.
    artefactComment, "artefactComment",
    /// `Artefact date of purchase`: The date when an Artefact was purchased. .
    artefactDateOfPurchase, "artefactDateOfPurchase",
    /// `Artefact date of sell`: The date when an Artefact was sold.
    artefactDateOfSell, "artefactDateOfSell",
    /// `Artefact description`: A description of an Artefact.
    artefactDescription, "artefactDescription",
    /// `Artefact Identifier`: Range: string or Identifier.
    artefactId, "artefactId",
    /// `Artefact model`: To specify a model of an Artefact.
    artefactModel, "artefactModel",
    /// `Artefact name`: A name associated with an Artefact.
    artefactName, "artefactName",
    /// `Artefact period`: To specify the period associated with an Artefact.
    artefactPeriod, "artefactPeriod",
    /// `Artefact price`: To specifythe price of an Artefact.
    artefactPriceAmount, "artefactPriceAmount",
    /// `Artefact reference`: To specify a reference of an Artefact.
    artefactReference, "artefactReference",
    /// `Artefact style`: To specify the style associated with an Artefact.
    artefactStyle, "artefactStyle",
    /// `Artefact type`: Range: string or Artefact_type.
    artefactType, "artefactType",
    /// `Artefact usage history`: To provide information on the usage history of an Artefact.
    artefactUsageHistory, "artefactUsageHistory",
    /// `Artefact website`: To specify a website where more 	information can be found on the Artefact.
    artefactWebsite, "artefactWebsite",
    /// `Aspect ratio`: To specify the aspect ratio.
    aspectRatio, "aspectRatio",
    /// `Description`: To provide a description of an Asset.
    assetDescription, "assetDescription",
    /// `Asset identifier`: An identifier attributed to an Asset.
    assetId, "assetId",
    /// `Name`: To provide a name of an Asset.
    assetName, "assetName",
    /// `Asset type`: Range: string or anyURI or Concept.
    assetType, "assetType",
    /// `Audio bitrate`: The audio bitrate.
    audioBitRate, "audioBitRate",
    /// `Audio bitrate`: The max audio bitrate.
    audioBitRateMax, "audioBitRateMax",
    /// `Audio bitrate mode`: The audio bitrate mode.
    audioBitRateMode, "audioBitRateMode",
    /// `Audio channel number`: The total number of audio channels contained in             the MediaResource.
    audioChannelNumber, "audioChannelNumber",
    /// `Audio encoding level`: The encoding level as defined in specifications.
    audioEncodingLevel, "audioEncodingLevel",
    /// `Audio encoding profile`: The encoding profile as defined in specifications.
    audioEncodingProfile, "audioEncodingProfile",
    /// `Audio track configuration`: The configuration of audio tracks contained in             the MediaResource.
    audioTrackConfiguration, "audioTrackConfiguration",
    /// `Audio track number`: The total number of audio tracks contained in             the MediaResource.
    audioTrackNumber, "audioTrackNumber",
    /// `Award ceremony`: To provide an Award ceremony name.
    awardCeremony, "awardCeremony",
    /// `Award date`: To provide an date when an Award was delivered.
    awardDate, "awardDate",
    /// `Award description`: To provide a description for an Award.
    awardDescription, "awardDescription",
    /// `Award identifier`: Range: string or Identifier
    awardId, "awardId",
    /// `Award name`: To provide the name of an Award.
    awardName, "awardName",
    /// `Award type`: Range: string or Award_Type
    awardType, "awardType",
    /// `Bit depth`: To provide the bitdepth at which the             MediaResource has been encoded.
    bitDepth, "bitDepth",
    /// `Bitrate`: To provide the bitrate at which the             MediaResource can be played in bits/second. Current bitrate if constant, and average bitrate if variable.
    bitRate, "bitRate",
    /// `Maximum bitrate`: The maximum bitrate when variable, in bits per second.
    bitRateMax, "bitRateMax",
    /// `Bitrate mode`: A flag to indicate if the bit rate is fixed or             variable.
    bitRateMode, "bitRateMode",
    /// `Overall bitrate`: To provide the overall bitrate at which the             MediaResource can be played in bits/second. Current bitrate if constant, and average bitrate if variable.
    bitRateOverall, "bitRateOverall",
    /// `Bookmark`: To provide a bookmark.
    bookmark, "bookmark",
    /// `Description`: To provide a description of an BusinessObject.
    businessObjectDescription, "businessObjectDescription",
    /// `BusinessObject identifier`: An identifier attributed to an BusinessObject.
    businessObjectId, "businessObjectId",
    /// `Name`: To provide a name of an BusinessObject.
    businessObjectName, "businessObjectName",
    /// `Business Object type`: A type attributed to a BusinessObject.
    businessObjectType, "businessObjectType",
    /// `Character description`: To provide the description of a Character.
    characterDescription, "characterDescription",
    /// `Annotation character start index`: To identify the end character index of the portion of text to which the Annotation applies.
    characterEndIndex, "characterEndIndex",
    /// `Character name.`: To specify the name of a Character.
    characterName, "characterName",
    /// `Annotation text character start index`: To identify the start character index of the portion of text to which the Annotation applies.
    characterStartIndex, "characterStartIndex",
    /// `Cloned to`: Identifies relationship between a digital instantiation of a Resource and its direct copy, with no generational loss.
    clonedTo, "clonedTo",
    /// `Codec family`: To provide information on the product family of the Codec.
    codecFamily, "codecFamily",
    /// `Codec Identifier`: Range: string or Identifier.
    codecId, "codecId",
    /// `Codec name`: To provide a name for the Codec, e.g. a product name.
    codecName, "codecName",
    /// `Codec version`: To provide information on the version of the Codec.
    codecVersion, "codecVersion",
    /// `Comments`: To provide a comment.
    comments, "comments",
    /// `Costume gender`: To specify the gender associated with a Costume.
    costumeGender, "costumeGender",
    /// `Costume size information`: To collect all information available useful to determine the size of a Costume.
    costumeSizeInformation, "costumeSizeInformation",
    /// `Costume texture`: Range: a string or a Concept code from a vocabulary, e.g. Getty
    costumeTexture, "costumeTexture",
    /// `Costume type`: To specify a type of Costume.
    costumeType, "costumeType",
    /// `Date`: A date associated to an Asset.
    date, "date",
    /// `Archiving date`: The date when the Asset was archived.
    dateArchived, "dateArchived",
    /// `Broadcast date`: The date when the Asset was first broadcast publicly on television or radio or via streaming.
    dateBroadcast, "dateBroadcast",
    /// `Creation date/time`: The date of creation of the Asset.
    dateCreated, "dateCreated",
    /// `Deletion date`: The date when the Resource was deleted.
    dateDeleted, "dateDeleted",
    /// `Digitisation date`: The date when the Resource was digitised.
    dateDigitised, "dateDigitised",
    /// `Distribution date`: The date when the Resource was first made available to the public for purchase, download, or online access.
    dateDistributed, "dateDistributed",
    /// `Ingest date`: The date when the Resource was ingested/acquired in institutional holdings.
    dateIngested, "dateIngested",
    /// `Archiving date`: The date when the Asset was issued.
    dateIssued, "dateIssued",
    /// `Migration date`: The date when the Resource was copied or converted from an obsolete or endangered original format to a more updated format for preservation.
    dateMigrated, "dateMigrated",
    /// `Modification date/time`: To indicate the date at which the Asset has been modified.
    dateModified, "dateModified",
    /// `Normalization date`: The date when the Resource was converted from its original format into a format pre-selected by the institution for preservation.
    dateNormalized, "dateNormalized",
    /// `Date of birth`: The date when a Contact/Person is born.
    dateOfBirth, "dateOfBirth",
    /// `Date of death`: The date when a Contact/Person has passed away.
    dateOfDeath, "dateOfDeath",
    /// `Date of retirement`: The date when a Contact/Person has retired.
    dateOfRetirement, "dateOfRetirement",
    /// `production date`: The date of production of the Asset.
    dateProduced, "dateProduced",
    /// `Release date`: The date when the Resource was first made available to the public for purchase, download, or online access.
    dateReleased, "dateReleased",
    /// `Transfer date`: The date when the Asset was moved from one digital or physical location to another.
    dateTransferred, "dateTransferred",
    /// `Validation date`: The most recent date when the Resource was confirmed to be valid through manual or digital QC.
    dateValidated, "dateValidated",
    /// `Licence end date`: The date when the licence for the Asset ends.
    datelicenseEnd, "datelicenseEnd",
    /// `Licence start date`: The date when the licence for the Asset begins.
    datelicensed, "datelicensed",
    /// `Derivation target`: To identify a new version derived from the original.
    derivedTo, "derivedTo",
    /// `Description`: This can be specialised by using sub-properties             like defined in http://www.ebu.ch/metadata/cs/web/ebu_DescriptionTypeCodeCS_p.xml.htm             implemented as examples as e.g. 'summary' or             'script'.
    description, "description",
    /// `Dimensions`: Describes the physical dimensions of a MediaResource, with units of measure concatenated to become part of the value.
    dimensions, "dimensions",
    /// `Dish description`: The description of a Dish.
    dishDescription, "dishDescription",
    /// `Dish name`: The name of a Dish.
    dishName, "dishName",
    /// `Display aspect ratio`: The aspect ratio when displayed.
    displayAspectRatio, "displayAspectRatio",
    /// `Display order`: The order in which an Agent appears in a scene.
    displayOrder, "displayOrder",
    /// `Dubbed to`: Range: string or Language.
    dubbedTo, "dubbedTo",
    /// `Duration`: To provide information on the duration of an EditorialObject or MediaResource.
    duration, "duration",
    /// `Duration (edit units)`: To provide a duration in edit units.
    durationEditUnits, "durationEditUnits",
    /// `Duration (time)`: To provide a duration as normal             time.
    durationNormalPlayTime, "durationNormalPlayTime",
    /// `Published Duration`: To provide information on the published / announced duration of an EditorialObject.
    durationPublished, "durationPublished",
    /// `Published duration (play time)`: To provide a published duration as normal play time.
    durationPublishedNormalPlayTime, "durationPublishedNormalPlayTime",
    /// `Duration (timecode)`: The duration expressed as a             timecode.
    durationTimecode, "durationTimecode",
    /// `Duration (timecode, drop frame)`: The duration expressed as a             timecode with drop frames.
    durationTimecodeDropFrame, "durationTimecodeDropFrame",
    /// `Edit unit`: The edit unit is e.g. the inverse of the audio             sample rate or video frame rate.
    editUnit, "editUnit",
    /// `Description`: To provide a description of an EditorialObject.
    editorialObjectDescription, "editorialObjectDescription",
    /// `Editorial object identifier`: An identifier attributed to an EditorialObject.
    editorialObjectId, "editorialObjectId",
    /// `Editorial Object name`: A name attributed to an EditorialObject.
    editorialObjectName, "editorialObjectName",
    /// `Editorial Object type`: A type attributed to an EditorialObject.
    editorialObjectType, "editorialObjectType",
    /// `Education`: To provide information on the education.
    education, "education",
    /// `Emotion description`: The description of an Emotion.
    emotionDescription, "emotionDescription",
    /// `Emotion edit unit number`: The edit unit number at which the Emotion occurs.
    emotionEditUnit, "emotionEditUnit",
    /// `Emotion identifier`: An identifier attributed to an Emotion.
    emotionId, "emotionId",
    /// `Emotion name`: A name attributed to an Emotion.
    emotionName, "emotionName",
    /// `Emotion normal play time`: The normal play time at which the Emotion occurs.
    emotionNormalPlayTime, "emotionNormalPlayTime",
    /// `Emotion timecode`: The timecode at which the Emotion occurs.
    emotionTimecode, "emotionTimecode",
    /// `Emotion timecode (dropframe)`: The timecode (dropframe) at which the Emotion occurs.
    emotionTimecodeDropFrame, "emotionTimecodeDropFrame",
    /// `Emotion timestamp`: To identify a timestamp at which an Emotion can be seen.
    emotionTimestamp, "emotionTimestamp",
    /// `Emotion type`: A type of Emotion.
    emotionType, "emotionType",
    /// `Encoding level`: To define an encoding level.
    encodingLevel, "encodingLevel",
    /// `Encoding profile`: The encoding profile as defined in specifications.
    encodingProfile, "encodingProfile",
    /// `End`: To define an end timestamp, e.g. the end point of a MediaResource.
    end, "end",
    /// `End time (edit units)`: The end time in edit units.
    endEditUnits, "endEditUnits",
    /// `End time (time)`: The end time expressed using a time             expression.
    endNormalPlayTime, "endNormalPlayTime",
    /// `Offset end time (edit units)`: The offset end time in edit units.
    endOffsetEditUnits, "endOffsetEditUnits",
    /// `Offset end time (normal play time)`: The offset end time in normal play time.
    endOffsetNormalPlayTime, "endOffsetNormalPlayTime",
    /// `Offset end timecode`: The offset end timecode.
    endOffsetTimecode, "endOffsetTimecode",
    /// `Offset end timecode (dropframe)`: The offset end timecode dropframe.
    endOffsetTimecodedropframe, "endOffsetTimecodedropframe",
    /// `End time (timecode)`: An end time expressed as             timecode.
    endTimecode, "endTimecode",
    /// `End time (timecode dropframe)`: An end time expressed as             timecode with drop frames.
    endTimecodeDropFrame, "endTimecodeDropFrame",
    /// `Episode number`: The Episode Number
    episodeNumber, "episodeNumber",
    /// `Episode number in season`: The Episode Number in a season
    episodeNumberInSeason, "episodeNumberInSeason",
    /// `Episode number in series`: The Episode Number in a series
    episodeNumberInSeries, "episodeNumberInSeries",
    /// `Event description`: To provide a description for an Event.
    eventDescription, "eventDescription",
    /// `Duration`: The duration of an Event.
    eventDuration, "eventDuration",
    /// `Event end date`: The end date of an Event.
    eventEndDate, "eventEndDate",
    /// `Event end date & time`: The end date and time of an Event.
    eventEndDateTime, "eventEndDateTime",
    /// `Event end time`: The end time of an Event.
    eventEndTime, "eventEndTime",
    /// `Event identifier`: An identifier attributed to an Event.
    eventId, "eventId",
    /// `Event name`: To provide a name for an Event.
    eventName, "eventName",
    /// `Event period`: The period of time during which an Event has occured.
    eventPeriod, "eventPeriod",
    /// `Event start date`: The start date of an Event.
    eventStartDate, "eventStartDate",
    /// `Event start date & time`: The start date and time of an Event.
    eventStartDateTime, "eventStartDateTime",
    /// `Event start time`: The start time of an Event.
    eventStartTime, "eventStartTime",
    /// `Event type`: Range: Event or string
    eventType, "eventType",
    /// `Family information`: To provide information on the family of a Person.
    familyInformation, "familyInformation",
    /// `Family name`: The family name of a Person.
    familyName, "familyName",
    /// `File size`: Provides the size of a MediaResource in bytes.
    fileSize, "fileSize",
    /// `First showing flag`: To flag this is a first showing PublicationEvent.
    firstShowing, "firstShowing",
    /// `First showing on service flag`: To falg this is a first showing  PublicationEvent on this service.
    firstShowingThisService, "firstShowingThisService",
    /// `Folksonomy`: Provides a user/audience-generated description, tag, or label for resource content.
    folksonomy, "folksonomy",
    /// `Preceding`: A link to an Asset precedinging the current Asset in an ordered sequence.
    followsInSequence, "followsInSequence",
    /// `Food category`: To define a category of Food.
    foodCategory, "foodCategory",
    /// `Food ingredient`: The Food ingredients or Food items.
    foodIngredient, "foodIngredient",
    /// `Format version identifier`: A version identifier attributed to a Format.
    formatId, "formatId",
    /// `Format version identifier`: A version identifier attributed to a Format.
    formatVersionId, "formatVersionId",
    /// `Frame height`: The height of a video frame.
    frameHeight, "frameHeight",
    /// `Frame height unit`: The unit used to measure the height of a frame.
    frameHeightUnit, "frameHeightUnit",
    /// `Frame rate`: The unit used to express the frame rate of a MediaResource in frames/second.
    frameRate, "frameRate",
    /// `Frame size unit`: The unit used to express the frame width or             height. The unit by default is 'pixel'.
    frameSizeUnit, "frameSizeUnit",
    /// `Frame width`: The width of a video frame.
    frameWidth, "frameWidth",
    /// `Frame width unit`: The unit used to measure the width of a frame.
    frameWidthUnit, "frameWidthUnit",
    /// `Free access`: A flag to indicate that the access to the PublicationEvent is 'free'.
    free, "free",
    /// `Gender`: The gender of a Person e.g. male or female.
    gender, "gender",
    /// `Geographical blocking`: To identify geographical areas where content cannor be accessed.
    geoBlocking, "geoBlocking",
    /// `Given name`: The given name of a Person.
    givenName, "givenName",
    /// `Group description`: A textual description of a             Group.
    groupDescription, "groupDescription",
    /// `Group identifier`: An identifier attributed to a Group.
    groupId, "groupId",
    /// `Group name`: The name attributed to a Group.
    groupName, "groupName",
    /// `Group identifier`: A type attributed to a Group.
    groupType, "groupType",
    /// `Access conditions`: Range: string or AccessConditions.
    hasAccessConditions, "hasAccessConditions",
    /// `Action related agent`: Range: string or Agent.
    hasActionRelatedAgent, "hasActionRelatedAgent",
    /// `Action related scene`: Range: string or Scene.
    hasActionRelatedScene, "hasActionRelatedScene",
    /// `Affiliation`: A property to establish the relation between a             Contact/Person and an Organisation.
    hasAffiliation, "hasAffiliation",
    /// `Biography`: Range: a string or an anyURI (e,g, a URL to a webpage) or a Biography.
    hasAgentBiography, "hasAgentBiography",
    /// `Country of residence`: Range: string or CountryCode
    hasAgentCountryOfResidence, "hasAgentCountryOfResidence",
    /// `Language`: Range: a string or Language.
    hasAgentLanguage, "hasAgentLanguage",
    /// `Agent member`: Range: string or Agent.
    hasAgentMember, "hasAgentMember",
    /// `Nationality`: Range: a string or CountryCode.
    hasAgentNationality, "hasAgentNationality",
    /// `Place of residence`: Range: string or Location
    hasAgentPlaceOfResidence, "hasAgentPlaceOfResidence",
    /// `Related picture`: 
    hasAgentRelatedPicture, "hasAgentRelatedPicture",
    /// `Ancillary data`: To identify ancillary data in the media resource.
    hasAncillaryData, "hasAncillaryData",
    /// `Ancillary data format`: Range: string or AncillaryDataFormat
    hasAncillaryDataFormat, "hasAncillaryDataFormat",
    /// `Animal breed code`: To associate a breed code with an animal.
    hasAnimalBreedCode, "hasAnimalBreedCode",
    /// `Animal colour code`: To associate a colour code with an animal.
    hasAnimalColourCode, "hasAnimalColourCode",
    /// `Animal role`: To identify the role of an animal.
    hasAnimalRole, "hasAnimalRole",
    /// `Annotation body`: To define the Annotation has a string or instance of an EBUCore class.
    hasAnnotationBody, "hasAnnotationBody",
    /// `Annotation confidence`: To define the purpose of an Annotation.
    hasAnnotationPurpose, "hasAnnotationPurpose",
    /// `Annotation related agent`: To identify an Agent subject of the Annotation.
    hasAnnotationRelatedAgent, "hasAnnotationRelatedAgent",
    /// `Annotation related artefact`: To identify an Artefact subject of the Annotation.
    hasAnnotationRelatedArtefact, "hasAnnotationRelatedArtefact",
    /// `Annotation related event`: To identify an Event subject of the Annotation.
    hasAnnotationRelatedEvent, "hasAnnotationRelatedEvent",
    /// `Annotation related location`: To identify a Location subject of the Annotation.
    hasAnnotationRelatedLocation, "hasAnnotationRelatedLocation",
    /// `Annotation target`: To define the target object to which the Annotation applies.
    hasAnnotationTarget, "hasAnnotationTarget",
    /// `Buyer`: Range: string or Agent
    hasArtefactBuyer, "hasArtefactBuyer",
    /// `Creator`: Range: string or Agent
    hasArtefactCreator, "hasArtefactCreator",
    /// `Location`: Range: string or Location
    hasArtefactLocation, "hasArtefactLocation",
    /// `Owner`: Range: string or Agent
    hasArtefactOwner, "hasArtefactOwner",
    /// `Artefact price currency`: Range: string or CurrencyCode
    hasArtefactPriceCurrency, "hasArtefactPriceCurrency",
    /// `Associated agent`: Range: string or Agent
    hasArtefactRelatedAgent, "hasArtefactRelatedAgent",
    /// `Associated editorial object`: Range: string or EditorialObject
    hasArtefactRelatedEditorialObject, "hasArtefactRelatedEditorialObject",
    /// `Associated location`: Range: string or Location
    hasArtefactRelatedLocation, "hasArtefactRelatedLocation",
    /// `Associated physical resource`: Range: string or PhysicalResource
    hasArtefactRelatedPhysicalResource, "hasArtefactRelatedPhysicalResource",
    /// `Associated resource`: Range: string or Resource
    hasArtefactRelatedResource, "hasArtefactRelatedResource",
    /// `Retailer`: Range: string or Agent
    hasArtefactRetailer, "hasArtefactRetailer",
    /// `Supplier`: Range: string or Agent
    hasArtefactSupplier, "hasArtefactSupplier",
    /// `Related business object`: To associate a BusinessObject with an Asset.
    hasAssetRelatedBusinessObject, "hasAssetRelatedBusinessObject",
    /// `Related editorial object`: To associate an EditorialObject with an Asset.
    hasAssetRelatedEditorialObject, "hasAssetRelatedEditorialObject",
    /// `Asset related media resource`: To identify a related MediaResource.
    hasAssetRelatedMediaResource, "hasAssetRelatedMediaResource",
    /// `Asset related resource`: To identify a related Resource.
    hasAssetRelatedResource, "hasAssetRelatedResource",
    /// `Related Artefact`: 
    hasAssociatedArtefact, "hasAssociatedArtefact",
    /// `Associated asset`: To identify an associated asset.
    hasAssociatedAsset, "hasAssociatedAsset",
    /// `Relation`: To define a Relation.
    hasAssociatedRelation, "hasAssociatedRelation",
    /// `Audience score recording technique`: Range: string or AudienceScorerecordingTechnique.
    hasAudienceScoreRecordingTechnique, "hasAudienceScoreRecordingTechnique",
    /// `Audio codec`: Range:string or AudioCodec
    hasAudioCodec, "hasAudioCodec",
    /// `Audio content type`: Range:string or AudioContent_Type.
    hasAudioContentType, "hasAudioContentType",
    /// `Audio description`: To signal the presence of             AudioDescription.
    hasAudioDescription, "hasAudioDescription",
    /// `Audio encoding format`: Range: string or AudioEncodingFormat
    hasAudioEncodingFormat, "hasAudioEncodingFormat",
    /// `Audio programme type.`: Range:string or AudioProgramme_Type
    hasAudioProgrammeType, "hasAudioProgrammeType",
    /// `Audio track`: To identify AudioTracks in the Resource.
    hasAudioTrack, "hasAudioTrack",
    /// `Agent`: Range: string or Agent.
    hasAwardRelatedAgent, "hasAwardRelatedAgent",
    /// `Business object`: To link a BusinessObject to an Award.
    hasAwardRelatedBusinessObject, "hasAwardRelatedBusinessObject",
    /// `Event`: An Event e.g. a ceremony, associated to an Award.
    hasAwardRelatedEvent, "hasAwardRelatedEvent",
    /// `Agent`: Range: a string or an Award.
    hasBeenAwarded, "hasBeenAwarded",
    /// `Captioning`: To signal the presence of             Captioning.
    hasCaptioning, "hasCaptioning",
    /// `Captioning format`: Range: string or CaptioningFormat
    hasCaptioningFormat, "hasCaptioningFormat",
    /// `Captioning source`: Range: string or Agent
    hasCaptioningSource, "hasCaptioningSource",
    /// `Cast member`: A member of the cast.
    hasCastMember, "hasCastMember",
    /// `Cast role`: Range: a string or a Role/Concept from a controlled vocabulary.
    hasCastRole, "hasCastRole",
    /// `Channel logo`: The logo of a Publication Channel
    hasChannelLogo, "hasChannelLogo",
    /// `Publication event`: To associate PublicationEvents with             PublicationChannels.
    hasChannelPublicationEvent, "hasChannelPublicationEvent",
    /// `Character`: Range: a string or a "fictitious" person - Character.
    hasCharacter, "hasCharacter",
    /// `Codec`: Range: string or Codec
    hasCodec, "hasCodec",
    /// `Codec vendor`: Range: string or Agent.
    hasCodecVendor, "hasCodecVendor",
    /// `Colour space`: Range: string or Concept
    hasColourSpace, "hasColourSpace",
    /// `Contact`: Range: a link to a Contact or a string.
    hasContact, "hasContact",
    /// `Container codec`: Range: string or ContainerCodec
    hasContainerCodec, "hasContainerCodec",
    /// `Container encoding format`: Range: string or ContainerEncodingFormat
    hasContainerEncodingFormat, "hasContainerEncodingFormat",
    /// `Mime type`: Range: string or MimeType
    hasContainerMimeType, "hasContainerMimeType",
    /// `Editorial format`: Range: string or ContentEditorialFormat.
    hasContentEditorialFormat, "hasContentEditorialFormat",
    /// `Contributor`: Range: string or Agent
    hasContributor, "hasContributor",
    /// `Copyright`: Range: string or Copyright.
    hasCopyright, "hasCopyright",
    /// `Costume type`: Range: a string or Costume_type e.g. a Concept code from a vocabulary, e.g. Getty.
    hasCostumeType, "hasCostumeType",
    /// `Country of birth`: Range: string or CountryCode
    hasCountryOfBirth, "hasCountryOfBirth",
    /// `Country of death`: Range: string or CountryCode
    hasCountryOfDeath, "hasCountryOfDeath",
    /// `Coverage`: Range: string or Event or Location
    hasCoverage, "hasCoverage",
    /// `Coverage restrictions`: Range: string or CoverageRestrictions.
    hasCoverageRestrictions, "hasCoverageRestrictions",
    /// `Creation location`: Range: string or Location.
    hasCreationLocation, "hasCreationLocation",
    /// `Creative Commons`: Range: string or Creative Commons.
    hasCreativeCommons, "hasCreativeCommons",
    /// `Creator`: Range: string or Agent.
    hasCreator, "hasCreator",
    /// `Crew member`: A member of the crew.
    hasCrewMember, "hasCrewMember",
    /// `Crew role`: Range: a string or a Role/Concept from a controlled vocabulary.
    hasCrewRole, "hasCrewRole",
    /// `Cuisine origin`: Range: a string or CountryCode
    hasCuisineOrigin, "hasCuisineOrigin",
    /// `Cuisine style`: Range: a string or CuisineStyle
    hasCuisineStyle, "hasCuisineStyle",
    /// `Data format`: Range: string or DataFormat
    hasDataFormat, "hasDataFormat",
    /// `Data track`: To identify DataTracks in the Resource.
    hasDataTrack, "hasDataTrack",
    /// `Department`: Range: string or Department.
    hasDepartment, "hasDepartment",
    /// `Disclaimer`: Range: string or Disclaimer.
    hasDisclaimer, "hasDisclaimer",
    /// `Document format`: Range: string or Document format
    hasDocumentFormat, "hasDocumentFormat",
    /// `Dopesheet`: Range: string or Dopesheet.
    hasDopesheet, "hasDopesheet",
    /// `Dubbed language`: Range: string or Language.
    hasDubbedLanguage, "hasDubbedLanguage",
    /// `EIDR`: Range: string or Identifier.
    hasEidrIdentifier, "hasEidrIdentifier",
    /// `Emotion related agent`: Range: String or Agent
    hasEmotionRelatedAgent, "hasEmotionRelatedAgent",
    /// `Emotion related scene`: Range: string or Scene
    hasEmotionRelatedScene, "hasEmotionRelatedScene",
    /// `Encoding format`: Range: string or Encoding format
    hasEncodingFormat, "hasEncodingFormat",
    /// `Episode`: Range: string or Episode.
    hasEpisode, "hasEpisode",
    /// `Agent`: An Agent relates to an Event.
    hasEventRelatedAgent, "hasEventRelatedAgent",
    /// `Artefact`: An artefact related to an Event.
    hasEventRelatedArtefact, "hasEventRelatedArtefact",
    /// `Business object`: A BusinessObject relates to an Event.
    hasEventRelatedBusinessObject, "hasEventRelatedBusinessObject",
    /// `Event`: An Event relates to an Event.
    hasEventRelatedEvent, "hasEventRelatedEvent",
    /// `Location`: Range: a Location or a string
    hasEventRelatedLocation, "hasEventRelatedLocation",
    /// `Resource`: A Resource relates to an Event.
    hasEventRelatedResource, "hasEventRelatedResource",
    /// `Agent`: Range: string or EventType
    hasEventType, "hasEventType",
    /// `Exploitation Issues`: Range: string or Exploitation Issues.
    hasExploitationIssues, "hasExploitationIssues",
    /// `File format`: Range: string or FileFormat.
    hasFileFormat, "hasFileFormat",
    /// `Food style`: Range: string or FoodStyle.
    hasFoodStyle, "hasFoodStyle",
    /// `Format`: Range: string, Format or any Format-related Concept
    hasFormat, "hasFormat",
    /// `Format identifier`: An identifier attributed to a Format.
    hasFormatId, "hasFormatId",
    /// `Generation`: Identifies the generation of a version of a resource, i.e. master,       edit master, distribution copy, etc.
    hasGeneration, "hasGeneration",
    /// `Genre`: To define a Genre/category associated to the             BusinesssObject.
    hasGenre, "hasGenre",
    /// `IMedia Id`: Range: string or Identifier.
    hasIMediaIdentifier, "hasIMediaIdentifier",
    /// `IPR restrictions`: Range: string or IPR Restrictions.
    hasIPRRestrictions, "hasIPRRestrictions",
    /// `Identification picture`: A locator / URI or a Picture.
    hasIdPicture, "hasIdPicture",
    /// `Identifier`: Range: string or Identifier.
    hasIdentifier, "hasIdentifier",
    /// `Identifier type`: Range: Concept or string
    hasIdentifierType, "hasIdentifierType",
    /// `Image codec`: Range: string or Codec
    hasImageCodec, "hasImageCodec",
    /// `Image format`: Range: string or ImageFormat
    hasImageFormat, "hasImageFormat",
    /// `ISAN`: Range: string or Identifier.
    hasIsanIdentifier, "hasIsanIdentifier",
    /// `Career event`: Range: string or KeyCareerEvent
    hasKeyCareerEvent, "hasKeyCareerEvent",
    /// `Personal event`: Range: string or KeyPersonalEvent
    hasKeyPersonalEvent, "hasKeyPersonalEvent",
    /// `Keyword`: Range: Keyword or string or any URI from a controlled vocabulary
    hasKeyword, "hasKeyword",
    /// `Language`: Range: string or Language.
    hasLanguage, "hasLanguage",
    /// `Licensing`: Range: string or Licensing.
    hasLicensing, "hasLicensing",
    /// `Locationcode`: Range: string or LocationCode.
    hasLocationCode, "hasLocationCode",
    /// `Picture`: A picture associated with a Location.
    hasLocationPicture, "hasLocationPicture",
    /// `Artefact`: Range: a string or an Artefact.
    hasLocationRelatedArtefact, "hasLocationRelatedArtefact",
    /// `Event`: Range: a string or an Event.
    hasLocationRelatedEvent, "hasLocationRelatedEvent",
    /// `Related resource`: To identify a Resource associated with a Location.
    hasLocationRelatedResource, "hasLocationRelatedResource",
    /// `Locationcode`: Range: string or LocationType.
    hasLocationType, "hasLocationType",
    /// `Locator`: A locator from where the MediaResource can be accessed.
    hasLocator, "hasLocator",
    /// `Logo`: Logos can be used in a variety of contexts.             Logo can be associated with an Organisation or a Service or a PublicationChannel.
    hasLogo, "hasLogo",
    /// `Manifestation`: A manifestation is the physical embodiment of work e.g. a tape, a file...
    hasManifestation, "hasManifestation",
    /// `Master`: To identify the master of a Resource
    hasMaster, "hasMaster",
    /// `Fragment`: To define relation to MediaFragments             withiin a MediaResource.
    hasMediaFragment, "hasMediaFragment",
    /// `Medium`: Range: string or Medium
    hasMedium, "hasMedium",
    /// `Member.`: To establish group/collection relationship between EditorialObjects.
    hasMember, "hasMember",
    /// `Publication  plan member`: To identify a PublicationPlan that forms part of another PublicationPlan.
    hasMemberPublicationPlan, "hasMemberPublicationPlan",
    /// `Metadata track`: To identify MetadataTracks in the Resource.
    hasMetadataTrack, "hasMetadataTrack",
    /// `Mime type`: Range: string or MimeType
    hasMimeType, "hasMimeType",
    /// `Object/asset type`: Range: string or ObjectType.
    hasObjectType, "hasObjectType",
    /// `Organisation logo`: The logo representing an Organisation
    hasOrganisationLogo, "hasOrganisationLogo",
    /// `Staff`: Range: string or Staff.
    hasOrganisationStaff, "hasOrganisationStaff",
    /// `Original language`: Range: string or Language.
    hasOriginalLanguage, "hasOriginalLanguage",
    /// `Parent editorial object`: To link a EditorialOject to a parent.
    hasParentEditorialObject, "hasParentEditorialObject",
    /// `Parent resource`: To link a MediaResource to a parent.
    hasParentMediaResource, "hasParentMediaResource",
    /// `Part`: To define Parts (segments, fragments, shots, etc.)             within an EditorialObject.
    hasPart, "hasPart",
    /// `Part type`: A type of Part.
    hasPartType, "hasPartType",
    /// `Participating agent`: To identify participating Agents.
    hasParticipatingAgent, "hasParticipatingAgent",
    /// `Pictogram`: Range: a locator/URI or a Picture.
    hasPictogram, "hasPictogram",
    /// `Birth place`: To identify the place of birth.
    hasPlaceOfBirth, "hasPlaceOfBirth",
    /// `Death place`: To identify the place of death.
    hasPlaceOfDeath, "hasPlaceOfDeath",
    /// `Producer`: Range: string or Agent.
    hasProducer, "hasProducer",
    /// `Production location`: Range: a Location or string
    hasProductionLocation, "hasProductionLocation",
    /// `Provenance`: Range: string, anyURI or Concept.
    hasProvenance, "hasProvenance",
    /// `Provenance target`: The instance of an object sourced by the Provenance.
    hasProvenanceTarget, "hasProvenanceTarget",
    /// `Publication event`: To associate PublicationEvents with             PublicationChannels or as elements of a PublicationHistory or PublicationPlanning.
    hasPublicationEvent, "hasPublicationEvent",
    /// `Publication history`: To provide the history of publication of an EditorailObject or MediaResource.
    hasPublicationHistory, "hasPublicationHistory",
    /// `Publication medium`: Range: string or PublicationMedium.
    hasPublicationMedium, "hasPublicationMedium",
    /// `Publication plan member.`: To identify a subplan of a publication plan.
    hasPublicationPlanMember, "hasPublicationPlanMember",
    /// `Publication plan type`: Range: string or PublicationPlan_type.
    hasPublicationPlanType, "hasPublicationPlanType",
    /// `Publication region`: Range: string or Location
    hasPublicationRegion, "hasPublicationRegion",
    /// `Publisher`: To identify an Agent involved in the publication of the Resource or BusinessObject.
    hasPublisher, "hasPublisher",
    /// `Rating`: Range: a string or a Rating.
    hasRating, "hasRating",
    /// `Rating source / agent`: Range: string or Agent.
    hasRatingProvider, "hasRatingProvider",
    /// `Rating source / agent`: Range: string or Agent.
    hasRatingSource, "hasRatingSource",
    /// `Related animal`: To identify animals associate with an Asset.
    hasRelatedAnimal, "hasRelatedAnimal",
    /// `Related artefact`: Range: string or Artefact.
    hasRelatedArtefact, "hasRelatedArtefact",
    /// `Related asset`: To identify related Assets.
    hasRelatedAsset, "hasRelatedAsset",
    /// `Audio content`: To identify related Audio Content
    hasRelatedAudioContent, "hasRelatedAudioContent",
    /// `Audio object`: To identify related Audio Objects
    hasRelatedAudioObject, "hasRelatedAudioObject",
    /// `Audio programme`: A related audio programme
    hasRelatedAudioProgramme, "hasRelatedAudioProgramme",
    /// `Audio track`: To identify related Audio Tracks
    hasRelatedAudioTrack, "hasRelatedAudioTrack",
    /// `Related award`: Range: string or Award.
    hasRelatedAward, "hasRelatedAward",
    /// `Related editorial object`: To identify related EditorialObjects.
    hasRelatedEditorialObject, "hasRelatedEditorialObject",
    /// `Related essence`: To establish a relation between a MediaResource and an Essence.
    hasRelatedEssence, "hasRelatedEssence",
    /// `Event`: A property to identify the              Events, all real or fictional, covered by the              EditorialObject.
    hasRelatedEvent, "hasRelatedEvent",
    /// `Image`: To associate an Image with a BusinessObject.
    hasRelatedImage, "hasRelatedImage",
    /// `Location`: A property to identify the              Locations, all real or fictional, covered by the              EditorialObject.
    hasRelatedLocation, "hasRelatedLocation",
    /// `Media fragment`: To associate a Part of an Asset with a MediaFragment within the association MediaResource instantiating the Asset.
    hasRelatedMediaFragment, "hasRelatedMediaFragment",
    /// `Related media resource`: To identify a MediaResource associated with an Asset or a BusinessObject or a PublicationEvent or another Resource.
    hasRelatedMediaResource, "hasRelatedMediaResource",
    /// `Picture`: To associate a Picture with a BusinessObject or a Resource.
    hasRelatedPicture, "hasRelatedPicture",
    /// `Publication channel`: Range: string or PublicationChannel
    hasRelatedPublicationChannel, "hasRelatedPublicationChannel",
    /// `Publication event`: To identify the PublicationEvent associated with a MediaResource (manifestation of an EditorialObject).
    hasRelatedPublicationEvent, "hasRelatedPublicationEvent",
    /// `Related record`: Range, a string a URI or a Record.
    hasRelatedRecord, "hasRelatedRecord",
    /// `Related resource`: To identify a Resource associated with an Asset or a BusinessObject or a PublicationEvent or another Resource.
    hasRelatedResource, "hasRelatedResource",
    /// `Related Service`: Range: string or Service.
    hasRelatedService, "hasRelatedService",
    /// `Related text line`: A TextLine or free text related to an EditorialObject.
    hasRelatedTextLine, "hasRelatedTextLine",
    /// `Relation source`: Range: string or Agent.
    hasRelationSource, "hasRelationSource",
    /// `Locator`: A locator from where the Resource can be accessed.
    hasResourceLocator, "hasResourceLocator",
    /// `Review`: Range: string or Review.
    hasReview, "hasReview",
    /// `Rights clearance`: Range: string or Rights Clearance.
    hasRightsClearance, "hasRightsClearance",
    /// `Contact`: Range: a string or a Contact.
    hasRightsContact, "hasRightsContact",
    /// `Rights holder`: Range: a string or an Agent.
    hasRightsHolder, "hasRightsHolder",
    /// `Role`: Range: a string or a Role/Concept from a controlled vocabulary.
    hasRole, "hasRole",
    /// `Season`: To identiify Seasons in a Series.
    hasSeason, "hasSeason",
    /// `Service genre`: Range: string or Genre
    hasServiceGenre, "hasServiceGenre",
    /// `Service logo`: The Logo characterising a Service
    hasServiceLogo, "hasServiceLogo",
    /// `Shooting location`: Range: Location or string
    hasShootingLocation, "hasShootingLocation",
    /// `Accessibility - signing`: A locator/URI to a resource or a Signing resource.
    hasSigning, "hasSigning",
    /// `Signing format`: Range: string or SigningFormat.
    hasSigningFormat, "hasSigningFormat",
    /// `Signing source`: Range: string or Agent.
    hasSigningSource, "hasSigningSource",
    /// `Source`: To identify the source of a MediaResource.
    hasSource, "hasSource",
    /// `member of Staff`: Range: string or Staff.
    hasStaffMember, "hasStaffMember",
    /// `Staff role`: Range: a string or a Role/Concept from a controlled vocabulary.
    hasStaffRole, "hasStaffRole",
    /// `Publication plan stakeholder`: An Agent related to the PublicationPlan.
    hasStakeholder, "hasStakeholder",
    /// `Standard`: Identifies the technical video standard of a MediaResource, i.e. NTSC or PAL.
    hasStandard, "hasStandard",
    /// `Storage identifier`: Range: Identifier, anyURI, string
    hasStorageId, "hasStorageId",
    /// `Storage type`: Range:; string or Storage_Type
    hasStorageType, "hasStorageType",
    /// `Subject`: Range: string, anyURI or Subject
    hasSubject, "hasSubject",
    /// `Subtitling`: Range: string or Subtitling
    hasSubtitling, "hasSubtitling",
    /// `Subtitling format`: Range: string or SubtitlingFormat
    hasSubtitlingFormat, "hasSubtitlingFormat",
    /// `Subtitling source`: Range: a string or an Agent.
    hasSubtitlingSource, "hasSubtitlingSource",
    /// `Target audience`: Range: string or TargetAudience.
    hasTargetAudience, "hasTargetAudience",
    /// `Target platform`: Range: string or TargetPlatform.
    hasTargetPlatform, "hasTargetPlatform",
    /// `Team member`: Range: a Person or a string
    hasTeamMember, "hasTeamMember",
    /// `Text line identifier.`: Range: string or Identifier.
    hasTextLineId, "hasTextLineId",
    /// `Text line related agent`: Range: string or Agent.
    hasTextLineRelatedAgent, "hasTextLineRelatedAgent",
    /// `Text line related character`: Range: string or Character.
    hasTextLineRelatedCharacter, "hasTextLineRelatedCharacter",
    /// `Text line related scene`: Range: string or Scene.
    hasTextLineRelatedScene, "hasTextLineRelatedScene",
    /// `Text line source`: Range: string or Agent.
    hasTextLineSource, "hasTextLineSource",
    /// `Text line type`: Range: string or TextLine_Type
    hasTextLineType, "hasTextLineType",
    /// `Theme`: Range: a Concept, anyURI or a string
    hasTheme, "hasTheme",
    /// `Timecode track`: To identify a timecode track with a MediaResource.
    hasTimecodeTrack, "hasTimecodeTrack",
    /// `Timeline track`: To associate a TimelineTrack with an EditorialObject
    hasTimelineTrack, "hasTimelineTrack",
    /// `Timeline track part`: To associate an EditorialObject to a TimelineTrackPart
    hasTimelineTrackPart, "hasTimelineTrackPart",
    /// `Timeline track type`: Range: string or anyURI or TimelineTrack_Type.
    hasTimelineTrackType, "hasTimelineTrackType",
    /// `Topic`: Range: a Topic, anyURI or a string
    hasTopic, "hasTopic",
    /// `Track`: To associate audio/data/video tracks with a MediaResource.
    hasTrack, "hasTrack",
    /// `Track part source`: An element to identify a part of a track by a title, a start time and an end time in both the media source and media destinationn.
    hasTrackPart, "hasTrackPart",
    /// `Track purpose`: Range: string or TrackPurpose.
    hasTrackPurpose, "hasTrackPurpose",
    /// `Asset type`: An type of Asset.
    hasType, "hasType",
    /// `Usage restrictions`: Range: string or UsageRestrictions.
    hasUsageRestrictions, "hasUsageRestrictions",
    /// `Usage rights`: Range: string or UsageRights.
    hasUsageRights, "hasUsageRights",
    /// `Version`: To identify another version of an Asset, BusinessObject or Resource.
    hasVersion, "hasVersion",
    /// `Video codec`: Range: string or VideoCodec
    hasVideoCodec, "hasVideoCodec",
    /// `Audio encoding format`: Range: string or VideoEncodingFormat
    hasVideoEncodingFormat, "hasVideoEncodingFormat",
    /// `Video track`: To identify VideoTracks in the Resource.
    hasVideoTrack, "hasVideoTrack",
    /// `Wrapping type`: Range: string or WrappingType.
    hasWrappingType, "hasWrappingType",
    /// `Hash code`: The hash value associated to a Resource. There             are different methods / algorithms to calculate hash values, which can be defined as             subproperties.
    hashValue, "hashValue",
    /// `Height`: The height of e.g. a video frame typically             expressed as a number of lines or the height of a picture/image expressed in millimeters             or else.
    height, "height",
    /// `Height unit`: To specify a unit to express height.
    heightUnit, "heightUnit",
    /// `Highlights`: To provide highlights.
    highlights, "highlights",
    /// `Hobbies`: The hobbies of a Person.
    hobbies, "hobbies",
    /// `I-frame/Gop size`: The distance between 2 I-frames also known as the gop size.
    iFrameSize, "iFrameSize",
    /// `Date of creation`: The date when the identifier was generated.
    idDateOfCreation, "idDateOfCreation",
    /// `Identifier value`: Range: string or anyURI.
    identifierValue, "identifierValue",
    /// `Inches per second`: Identifies the inches per second at which an analog audio tape should be played back for human consumption.
    inchesPerSecond, "inchesPerSecond",
    /// `Business object`: To link a particular manifestation of a             BusinessObject to the corresponding Resource.
    instantiates, "instantiates",
    /// `isAgent`: Range: string or Agent.
    isAgent, "isAgent",
    /// `Animal groom`: To identify the groom / care taker of an animal.
    isAnimalGroom, "isAnimalGroom",
    /// `Animal owner`: To identify the owner of an animal.
    isAnimalOwner, "isAnimalOwner",
    /// `Media resource`: To link an Annotation to a MediaResource.
    isAnnotatedMediaResource, "isAnnotatedMediaResource",
    /// `Agent`: To link an Annotation to an Agent who created it.
    isAnnotationBy, "isAnnotationBy",
    /// `Provenance target`: Tassociate an Agent with a Provenance instance.
    isAttributedTo, "isAttributedTo",
    /// `Brand`: Range: a string or Brand
    isBrand, "isBrand",
    /// `Fictional character.`: Range: string or Agent.
    isCharacter, "isCharacter",
    /// `Parent`: To link a BusinessOject or Resource to a parent.
    isChildOf, "isChildOf",
    /// `Clone source`: To identify the source of a clone Editorial Object or Resource
    isClonedFrom, "isClonedFrom",
    /// `Media Resource`: To identify mediaResources used to compose an Essence.
    isComposedOf, "isComposedOf",
    /// `Rights`: Range: a link to Rights or open text (string).
    isCoveredBy, "isCoveredBy",
    /// `Derived from`: Identifies a content-based relationship between two resources.
    isDerivedFrom, "isDerivedFrom",
    /// `Platform/Service/PublicationChannel`: Range: Service or string.
    isDistributedOn, "isDistributedOn",
    /// `Dubbed from`: the origin of a dubbed MediaResource.
    isDubbedFrom, "isDubbedFrom",
    /// `Same editorial format`: To identify an Editorial Object based on the same Editorial format
    isEditorialFormatOf, "isEditorialFormatOf",
    /// `Parent season / series`: Range: Series or string.
    isEpisodeOf, "isEpisodeOf",
    /// `Parent season / series`: Range: string or Season.
    isEpisodeOfSeason, "isEpisodeOfSeason",
    /// `Parent season / series`: Range: string or Series.
    isEpisodeOfSeries, "isEpisodeOfSeries",
    /// `Fictitious contact`: Range: a string or a FictitiousPerson.
    isFictitiousPerson, "isFictitiousPerson",
    /// `Media Resource`: To identify a MediaResource instantiating an EditorialObject.
    isInstantiatedBy, "isInstantiatedBy",
    /// `Issuer`: Range: Agent or String
    isIssuedBy, "isIssuedBy",
    /// `Derived media resource`: To identify the master of a derived media resource.
    isMasterOf, "isMasterOf",
    /// `Media fragment source`: To identify the Media Resource to which a Media Fragment belongs to
    isMediaFragmentOf, "isMediaFragmentOf",
    /// `Member of`: Range: string or Group.
    isMemberOf, "isMemberOf",
    /// `Parent publication  plan`: To identify a parent Publication Plan
    isMemberOfPublicationPlan, "isMemberOfPublicationPlan",
    /// `Next`: A link to an Asset following the current Asset in an ordered sequence.
    isNextInSequence, "isNextInSequence",
    /// `Operator, owner`: Range: string or Service.
    isOperatedBy, "isOperatedBy",
    /// `Owner`: Range: string or Agent.
    isOwnedBy, "isOwnedBy",
    /// `Child`: To link a Asset to a parent Asset.
    isParentOf, "isParentOf",
    /// `Editorial object`: To identify the editorial object to which belongs a part.
    isPartOf, "isPartOf",
    /// `Identification picture locator`: Range: e.g. a string, URL or Locator.
    isPictureIdLocator, "isPictureIdLocator",
    /// `Rated business object`: To identify the BusinessObject associated with a Rating.
    isRatingRelatedToBusinessObject, "isRatingRelatedToBusinessObject",
    /// `Rated Resource`: To identify the resource associated with a Rating.
    isRatingRelatedToResource, "isRatingRelatedToResource",
    /// `Reference source`: To described references between assets.
    isReferencedBy, "isReferencedBy",
    /// `Service`: Range: Service or string
    isReleasedBy, "isReleasedBy",
    /// `Replacement`: To identify substitutions.
    isReplacedBy, "isReplacedBy",
    /// `Required`: To express strong relations between Assets, BusinessObjects or Resources.
    isRequiredBy, "isRequiredBy",
    /// `Publication event`: To associatre a PublicationEvent with an EditorialObject.
    isScheduledOn, "isScheduledOn",
    /// `Series`: Range: Series or string.
    isSeasonOf, "isSeasonOf",
    /// `Brand`: Range: Brand or string.
    isSeriesOf, "isSeriesOf",
    /// `Editorial Object`: To associate an EditorialObject with a part of the TimelineTrack.
    isTimelineTrackPartOf, "isTimelineTrackPartOf",
    /// `Track part source`: An element to identify a part of a track by a title, a start time and an end time in both the media source and media destination.
    isTrackPartOf, "isTrackPartOf",
    /// `Version of`: To identify related versions.
    isVersionOf, "isVersionOf",
    /// `Line number`: To provide the number of the line on which             ancillary data is being carried and the equivalent in the digital domain.
    lineNumber, "lineNumber",
    /// `Link to logo`: Range: string or Logo
    linkToLogo, "linkToLogo",
    /// `Link to Sticker`: Range: anyURI or Sticker.
    linkToSticker, "linkToSticker",
    /// `live`: A flag to signal that content is live
    live, "live",
    /// `Local familiy name`: To provide a family name in its local expression.
    localFamiliyName, "localFamiliyName",
    /// `Local given name`: To provide a given name in its local expression.
    localGivenName, "localGivenName",
    /// `Address`: To provide the address of a            Location.
    locationAddress, "locationAddress",
    /// `Area`: To provide the Area part of an             Adrress.
    locationAddressArea, "locationAddressArea",
    /// `Country`: To provide the country name and or country             code.
    locationAddressCountry, "locationAddressCountry",
    /// `Adress line`: To provide an address line.
    locationAddressLine, "locationAddressLine",
    /// `Locality`: To provide the name of a city, village,             etc.
    locationAddressLocality, "locationAddressLocality",
    /// `Postal code`: To provide an address postal             code.
    locationAddressPostalCode, "locationAddressPostalCode",
    /// `Altitude`: To define the altitude of a Location in             meters.
    locationAltitude, "locationAltitude",
    /// `Coordinate system`: To specify the name of the gps coordinate             system used for the Location.
    locationCoordinateSystemName, "locationCoordinateSystemName",
    /// `Location description`: To provide a description of a particular Location.
    locationDescription, "locationDescription",
    /// `Location identifier`: An identifier attributed to a Location.
    locationId, "locationId",
    /// `Latitude`: The latitude of the Location.
    locationLatitude, "locationLatitude",
    /// `Longitude`: To define the longitude of the             Location.
    locationLongitude, "locationLongitude",
    /// `Location name`: To provide a namefor a particular Location.
    locationName, "locationName",
    /// `Region`: Range: string or RegionCode
    locationRegion, "locationRegion",
    /// `Location type`: A type of time at a Location.
    locationTimeType, "locationTimeType",
    /// `Location type`: A type of Location.
    locationType, "locationType",
    /// `Locator target information`: Information on the locator target.
    locatorTargetInformation, "locatorTargetInformation",
    /// `Log`: To log everything in the content following predefined rules and criterias, as a neutral sequence of (possibly timed) textual descriptions.
    log, "log",
    /// `Integrated loudness`: The value for integrated loudness measured at AudioProgramme or AudioContent level.
    loudnessIntegratedLoudness, "loudnessIntegratedLoudness",
    /// `Max momentary loudness`: The value for maximum momentary loudness measured at AudioProgramme or AudioContent level.
    loudnessMaxMomentary, "loudnessMaxMomentary",
    /// `Max short term loudness`: The value for maximum max short term loudness measured at AudioProgramme or AudioContent level.
    loudnessMaxShortTerm, "loudnessMaxShortTerm",
    /// `Max true peak loudness`: The value for maximum true peak loudness measured at AudioProgramme or AudioContent level.
    loudnessMaxTruepeak, "loudnessMaxTruepeak",
    /// `Loudness method`: The method for loudness measurement at AudioProgramme or AudioContent level.
    loudnessMethod, "loudnessMethod",
    /// `Loudness parameters`: To provide loudness parameters.
    loudnessParameters, "loudnessParameters",
    /// `Loudness range`: The loudness range measured at AudioProgramme or AudioContent level.
    loudnessRange, "loudnessRange",
    /// `Main title`: Specifies the main title or name given to the             EditorialObject.
    mainTitle, "mainTitle",
    /// `Marital Status`: To identify the marital status of a Person.
    maritalStatus, "maritalStatus",
    /// `MediaResource description`: A description of a MediaResource.
    mediaResourceDescription, "mediaResourceDescription",
    /// `Media resource Id`: Range: Identifier or string
    mediaResourceId, "mediaResourceId",
    /// `Media resource type`: Range: MediaResource_Type or string
    mediaResourceType, "mediaResourceType",
    /// `Midroll ad allowed`: A flag to indicate whether it is allowed to insert ad breaks in mid-roll.
    midRollAdAllowed, "midRollAdAllowed",
    /// `Middle name`: To provide one or more middle names for a Person.
    middleName, "middleName",
    /// `Nickname`: The nickname of a Person.
    nickName, "nickName",
    /// `Noise filter`: A flag to signal that a noise filter has been             used.
    noiseFilter, "noiseFilter",
    /// `Not rated`: A flag to indicate that the EditorialObejct has not been rated.
    notRated, "notRated",
    /// `Number of audio tracks`: To provide the number of audio tracks.
    numberOfAudioTracks, "numberOfAudioTracks",
    /// `Number of tracks`: The number of Tracks composing the MediaResource.
    numberOfTracks, "numberOfTracks",
    /// `Number of video tracks`: To provide the number of video tracks.
    numberOfVideoTracks, "numberOfVideoTracks",
    /// `Occupation`: The job / occupation name of a Person.
    occupation, "occupation",
    /// `PublicationEvent`: To identify the PublicationEvents provided through a Service.
    offers, "offers",
    /// `Office email`: To provide the professional/office email             address of an Agent (Contact/Person or Organisation).
    officeEmailAddress, "officeEmailAddress",
    /// `Telephone (private)`: To provide the office mobile telephone number of an             Agent (Contact/Person).
    officeMobileTelephoneNumber, "officeMobileTelephoneNumber",
    /// `Telephone (private)`: To provide the office telephone number of an             Agent (Contact/Person).
    officeTelephoneNumber, "officeTelephoneNumber",
    /// `Ordered flag`: A flag to indicate that a EditorialObject is member of an ordered group or is an ordered group (e.g. Series)
    orderedFlag, "orderedFlag",
    /// `Organisation description`: To provide a description of an Organisation.
    organisationDescription, "organisationDescription",
    /// `Organisation identifier`: Range: string or Identifier
    organisationId, "organisationId",
    /// `Organisation name`: To provide the full name of an Organisation.
    organisationName, "organisationName",
    /// `Asset type`: Range: string or anyURI or Concept.
    organisationType, "organisationType",
    /// `Orientation`: The orientation of a Document or an Image i.e. landscape or             portrait.
    orientation, "orientation",
    /// `Original title.`: The original title used to identify the work.
    originalTitle, "originalTitle",
    /// `Package size (in bytes)`: The size of a media package in             Bytes.
    packageByteSize, "packageByteSize",
    /// `Package name`: The name of a media package in             Bytes.
    packageName, "packageName",
    /// `Part definition`: A definition associated with the Part.
    partDefinition, "partDefinition",
    /// `Part definition`: A description associated with the Part.
    partDescription, "partDescription",
    /// `Part identifier`: Range: a string or Identifier
    partId, "partId",
    /// `Part name`: A name associated with the Part.
    partName, "partName",
    /// `Part number`: The number associated to a Part as one among             many.
    partNumber, "partNumber",
    /// `Part total number`: The total number of Parts associated with an EditorialObject.
    partTotalNumber, "partTotalNumber",
    /// `Description`: To provide a description of a Person.
    personDescription, "personDescription",
    /// `Person height`: To indicate the height of a person.
    personHeight, "personHeight",
    /// `Person identifier`: An identifier attributed to a Person.
    personId, "personId",
    /// `Person name`: To provide e.g. compound names.
    personName, "personName",
    /// `Person type`: A type attributed to a Person.
    personType, "personType",
    /// `Person weight`: To indicate the weight of a person.
    personWeight, "personWeight",
    /// `Playback speed`: Identifies the rate of units against time at which the resource should be played back for human consumption.  If the unit of measure is known, use sub-properties framesPerSecond or inchesPerSecond.
    playbackSpeed, "playbackSpeed",
    /// `Playlist`: To provide a playlist.
    playlist, "playlist",
    /// `Essence`: To identify the Essence used in a PublicationEvent
    playsOut, "playsOut",
    /// `Position`: To indicate the position of an EditorialObject in an ordered       group.
    position, "position",
    /// `Private email`: To provide the private email address of an             Agent (Contact/Person)
    privateEmailAddress, "privateEmailAddress",
    /// `Homepage (private)`: To provide an private web homepage of an Agent             (Contact/Person).
    privateHomepage, "privateHomepage",
    /// `Telephone (private)`: To provide the private mobile telephone number of an             Agent (Contact/Person).
    privateMobileTelephoneNumber, "privateMobileTelephoneNumber",
    /// `Telephone (private)`: To provide the private telephone number of an             Agent (Contact/Person).
    privateTelephoneNumber, "privateTelephoneNumber",
    /// `Production synopsis`: A synopsis or summary provided by the producer at the time of production.
    productionSynopsis, "productionSynopsis",
    /// `Promotional information`: To provide textual promotional information.
    promotionalInformation, "promotionalInformation",
    /// `Provenance creation date & time`: The date of creation of a Provenance instance.
    provenanceDateCreated, "provenanceDateCreated",
    /// `Provenance modification date & time`: The date of modification of a Provenance instance.
    provenanceDateModified, "provenanceDateModified",
    /// `Provenance description`: To describe a Provenance.
    provenanceDescription, "provenanceDescription",
    /// `Provenance Id`: To identify a Provenance.
    provenanceId, "provenanceId",
    /// `Provenance name`: To name a Provenance.
    provenanceName, "provenanceName",
    /// `Provenance type`: Range: string, anyURI or Concept.
    provenanceType, "provenanceType",
    /// `Publication status`: To indicate a publication status.
    pubStatus, "pubStatus",
    /// `PublicationChannel description`: To provide a description of a PublicationChannel e.g. a TV channel or website.
    publicationChannelDescription, "publicationChannelDescription",
    /// `Publication channel identifier`: An identifier attributed to a PublicationChannel.
    publicationChannelId, "publicationChannelId",
    /// `PublicationChannel name`: To provide a name to a PublicationChannel e.g. a TV channel or website.
    publicationChannelName, "publicationChannelName",
    /// `Publication Channel type`: Range: string or PublicationChannel_Type.
    publicationChannelType, "publicationChannelType",
    /// `PublicationEvent duration`: The actual duration of a PublicationEvent.
    publicationDuration, "publicationDuration",
    /// `PublicationEvent end date & time`: The actual end date and time of a PublicationEvent.
    publicationEndDateTime, "publicationEndDateTime",
    /// `PublicationEvent abstract`: To provide an abstract for a PublicationEvent.
    publicationEventAbstract, "publicationEventAbstract",
    /// `PublicationEvent  description`: To provide the description of a PublicationEvent.
    publicationEventDescription, "publicationEventDescription",
    /// `Publication event identifier`: An identifier attributed to a PublicationEvent.
    publicationEventId, "publicationEventId",
    /// `PublicationEvent name`: To provide a name to a PublicationEvent.
    publicationEventName, "publicationEventName",
    /// `PublicationEvent title`: To provide a title for a PublicationEvent.
    publicationEventTitle, "publicationEventTitle",
    /// `Publication event type`: A type of PublicationEvent.
    publicationEventType, "publicationEventType",
    /// `PublicationPlan description`: A description of a PublicationPlan.
    publicationPlanDescription, "publicationPlanDescription",
    /// `PublicationPlan end date`: The end date of a PublicationPlan
    publicationPlanEndDate, "publicationPlanEndDate",
    /// `Publication plan identifier`: An identifier attributed to a PublicationPlan.
    publicationPlanId, "publicationPlanId",
    /// `Publication plan name`: A name attributed to a PublicationPlan.
    publicationPlanName, "publicationPlanName",
    /// `PublicationPlan start date`: The start date of a PublicationPlan
    publicationPlanStartDate, "publicationPlanStartDate",
    /// `PublicationPlan status`: To provide a status regarding the PublicationPlan.
    publicationPlanStatus, "publicationPlanStatus",
    /// `Schedule date`: To express specifically the schedule date to which a PublicationEvent is related in particular if the broacdast time is after midnight. For example, the schedule date would be May 29th and the programme is published at 1 am on May 30th, while still associated in the schedule with the night of May 29th.
    publicationScheduleDate, "publicationScheduleDate",
    /// `Publication start date & time`: The actual start date and time of a PublicationEvent.
    publicationStartDateTime, "publicationStartDateTime",
    /// `Publication end date & time`: The end date and time of a PublicationEvent as             scheduled.
    publishedEndDateTime, "publishedEndDateTime",
    /// `Publication start date & time`: The start date and time of a PublicationEvent             as scheduled.
    publishedStartDateTime, "publishedStartDateTime",
    /// `Published title.`: The title used to identify the work at publication time.
    publishedTitle, "publishedTitle",
    /// `Editorial object`: The editorial object associated to a PublicationEvent.
    publishes, "publishes",
    /// `Rating name`: To associate a description with a Rating.
    ratingDescription, "ratingDescription",
    /// `Rating Id`: Range: string or anyURI or Identifier.
    ratingId, "ratingId",
    /// `Rating name`: To associate a name with a Rating.
    ratingName, "ratingName",
    /// `Rating scale (top value)`: The maximum value of the scale used for the Rating             of a MediaResource.
    ratingScaleMax, "ratingScaleMax",
    /// `Rating scale (min. value)`: The minimum value of the scale used for rating             a MediaResource.
    ratingScaleMin, "ratingScaleMin",
    /// `Rating environment`: To identify the environment in which rating applies.
    ratingSystemEnvironment, "ratingSystemEnvironment",
    /// `Rating system`: To identify a Rating system by its name.
    ratingSystemName, "ratingSystemName",
    /// `Rating type`: Range: string or anyURI or Concept.
    ratingType, "ratingType",
    /// `Rating`: To express a free text Rating value defined in             a rating classification scheme.
    ratingValue, "ratingValue",
    /// `Ready for publication`: A flag to indicate that the Essence is ready for publication.
    readyForPublication, "readyForPublication",
    /// `Reason`: A reason given for a rating.
    reason, "reason",
    /// `References`: To express a reference between Assets, BusinessObjects or Resources.
    references, "references",
    /// `Region delimiter (x-axis)`: To define the top left corner of a zone on             the x-axis. If present with regionDelimy, the zone definition is complemented by the             associated values of the height and width.
    regionDelimX, "regionDelimX",
    /// `Region delimiter (y-axis)`: To define the bottom right corner of a zone on             the y-axis. If present with regionDelimX, the zone definition is complemented by the             associated values of the height and width.
    regionDelimY, "regionDelimY",
    /// `Relation Type`: Range: string or Identifier.
    relationIdentifier, "relationIdentifier",
    /// `Link`: To define a link in a Relation.
    relationLink, "relationLink",
    /// `Relation Note`: A note to provide additional information about a Relation.
    relationNote, "relationNote",
    /// `Relation Ordered group flag`: A boolean to define if a Relation is defined within and ordered group.
    relationOrderedGroupFlag, "relationOrderedGroupFlag",
    /// `Relation Running Order Number`: The order number in a list.
    relationRunningOrderNumber, "relationRunningOrderNumber",
    /// `Total number of group members.`: Total number of group members in a Relation.
    relationTotalNumberOfGroupMembers, "relationTotalNumberOfGroupMembers",
    /// `Relation Type`: Range: string or Relation_Type.
    relationType, "relationType",
    /// `Replaces`: To identify substitution.
    replaces, "replaces",
    /// `Related asset`: To establish a relation between a BusinessObject and an Asset.
    represents, "represents",
    /// `Requires`: To express dependency.
    requires, "requires",
    /// `Resolution`: To define the resolution of an Asset e.g. video, image...
    resolution, "resolution",
    /// `Resource description`: A desciprtion of a Resource.
    resourceDescription, "resourceDescription",
    /// `File size`: Provides the size of a Resource in bytes.
    resourceFileSize, "resourceFileSize",
    /// `File name`: The name of the file containing the             Resource.
    resourceFilename, "resourceFilename",
    /// `Resource id`: An identifier associated to a Resource.
    resourceId, "resourceId",
    /// `Resource id`: Range: Resource_type or anyURI or string
    resourceIdType, "resourceIdType",
    /// `Locator target information`: Information on the Resource locator target.
    resourceLocatorTargetInformation, "resourceLocatorTargetInformation",
    /// `Resource name`: The name given to a Resource.
    resourceName, "resourceName",
    /// `Resource offset`: The start offset within a Resource.
    resourceOffset, "resourceOffset",
    /// `Resource offset normal playtime`: The resource offset in normal play time
    resourceOffsetNormalPlaytime, "resourceOffsetNormalPlaytime",
    /// `Resource offset number edit units`: The resource offset in edit units
    resourceOffsetNumberEditUnit, "resourceOffsetNumberEditUnit",
    /// `Resource offset timecode.`: The resource offset in timecode
    resourceOffsetTimecode, "resourceOffsetTimecode",
    /// `Resource offset timecode (dropframe).`: The resource offset in timecode dropframe
    resourceOffsetTimecodedropframe, "resourceOffsetTimecodedropframe",
    /// `Rights clearance flag`: A flag to indicate that righst have been cleared
    rightsClearanceFlag, "rightsClearanceFlag",
    /// `Rights duration`: To define the duration of the period when Rights are applicable.
    rightsDuration, "rightsDuration",
    /// `Rights end date time`: To define the end time until when Rights are applicable.
    rightsEndDateTime, "rightsEndDateTime",
    /// `Rights`: To express an expression of Rights.
    rightsExpression, "rightsExpression",
    /// `Rights identifier`: An identifier attributed to a set of Rights.
    rightsId, "rightsId",
    /// `Rights web resource`: A link to e.g. a webpage where an expression of             the rights can be found and consulted.
    rightsLink, "rightsLink",
    /// `Rights start date time`: To define the start time since when Rights are applicable.
    rightsStartDateTime, "rightsStartDateTime",
    /// `Excluded territories`: A list of country or region codes to which Rights do not apply.
    rightsTerritoryExcludes, "rightsTerritoryExcludes",
    /// `Included territories`: A list of country or region codes to which Rights apply.
    rightsTerritoryIncludes, "rightsTerritoryIncludes",
    /// `Rights type`: Range: a string or a Rights_Type or a ContractType.
    rightsType, "rightsType",
    /// `role Id`: Range: string or anyURI.
    roleId, "roleId",
    /// `Role type`: To define a type of Role (not the Role itself).
    roleType, "roleType",
    /// `Salutation title`: To provide a salutation title e.g M. Ms, Dr, Pr.
    salutationTitle, "salutationTitle",
    /// `Sample Rate`: The frequency at which audio is sampled per second. Also called sampling rate.
    sampleRate, "sampleRate",
    /// `Sample size`: The size of an audio sample in             bits. Also called bit depth.
    sampleSize, "sampleSize",
    /// `Sample type`: The type of audio sample.
    sampleType, "sampleType",
    /// `Scanning format`: To define the scanning format for a             MediaResource. For video, the two main values are "interlaced" or             "progressive".
    scanningFormat, "scanningFormat",
    /// `Script`: To provide a script.
    script, "script",
    /// `Season number`: To provide a Season number.
    seasonNumber, "seasonNumber",
    /// `Service description`: A description of the Service.
    serviceDescription, "serviceDescription",
    /// `Service identiifier`: Range: string or Identifier.
    serviceId, "serviceId",
    /// `Service name`: The name of the Service.
    serviceName, "serviceName",
    /// `Service type`: Range: string or Service_type
    serviceType, "serviceType",
    /// `Shot log`: Provides a shot-by-shot description of a MediaResource.
    shotLog, "shotLog",
    /// `Start time`: Start timestamp e.g. the start time for a MediaResource.
    start, "start",
    /// `Start time (edit units)`: A start time expressed as a number of edit             units.
    startEditUnits, "startEditUnits",
    /// `Start time (normal play time)`: A start time expressed as a normal play time.
    startNormalPlayTime, "startNormalPlayTime",
    /// `Startoffset  time (edit units)`: A start offset time expressed as a number of edit             units.
    startOffsetEditUnit, "startOffsetEditUnit",
    /// `Startoffset  time (edit units)`: A start offset time expressed as normal play time.
    startOffsetNormalPlayTime, "startOffsetNormalPlayTime",
    /// `Start offset time (timecode)`: A start offset time expressed as             timecode.
    startOffsetTimecode, "startOffsetTimecode",
    /// `Start offset time (timecode, drop frames)`: A start offset time expressed as             timecode with drop frames.
    startOffsetTimecodeDropFrame, "startOffsetTimecodeDropFrame",
    /// `Start time (timecode)`: A start time expressed as             timecode.
    startTimecode, "startTimecode",
    /// `Start time (timecode, drop frames)`: A start time expressed as             timecode with drop frames.
    startTimecodeDropFrame, "startTimecodeDropFrame",
    /// `Subtitle`: A complementary subtitle.
    subtitle, "subtitle",
    /// `Suffix`: To provide a suffix associated with a Person name e.g. Jr.
    suffix, "suffix",
    /// `Summary`: To provide a summary.
    summary, "summary",
    /// `Synopsis`: To provide a summary.
    synopsis, "synopsis",
    /// `Table of content`: to provide a table of content.
    tableOfContent, "tableOfContent",
    /// `Tag`: To provide a list of tags.
    tag, "tag",
    /// `Target audience system`: To define the system used to provide a TargetAudience.
    targetAudienceSystem, "targetAudienceSystem",
    /// `Text line box height.`: The height of the text box containing the TextLine.
    textLineBoxHeight, "textLineBoxHeight",
    /// `Text line box top left corner Y position.`: The coordinates on a vertical axis of the position of the top left corner of the text box containing the TextLine.
    textLineBoxTopLeftCornerLineNumber, "textLineBoxTopLeftCornerLineNumber",
    /// `Text line box top left Coner X position.`: The coordinates on an horizontal axis of the position of the top left corner of the text box containing the TextLine.
    textLineBoxTopLeftCornerPixelNumber, "textLineBoxTopLeftCornerPixelNumber",
    /// `Text line box width.`: The width of the text box containing the TextLine.
    textLineBoxWidth, "textLineBoxWidth",
    /// `Text line`: To provide the content of a text line.
    textLineContent, "textLineContent",
    /// `Text line end in edit units`: The end time of a TextLine expressed as a number of edit units.
    textLineEndEditUnits, "textLineEndEditUnits",
    /// `Text line end in normal play time`: The end time of a TextLine expressed as a normal play time.
    textLineEndNormalPlayTime, "textLineEndNormalPlayTime",
    /// `Text line end time`: The end time point of a TextLine in a Scene.
    textLineEndTime, "textLineEndTime",
    /// `Text line end timecode`: The end time of a TextLine expressed as timecode.
    textLineEndTimecode, "textLineEndTimecode",
    /// `Text line end timecode drop frames`: The end time of a TextLine expressed as timecode with drop frames.
    textLineEndTimecodeDropFrame, "textLineEndTimecodeDropFrame",
    /// `Text line order`: The order in which a text line can be found e.g. in a scene.
    textLineOrder, "textLineOrder",
    /// `Text line start in edit units`: The start time of a TextLine expressed as a number of edit units.
    textLineStartEditUnits, "textLineStartEditUnits",
    /// `Text line start in normal play time`: The start time of a TextLine expressed as a normal play time.
    textLineStartNormalPlayTime, "textLineStartNormalPlayTime",
    /// `Text line start time`: The start time point of a TextLine in a Scene.
    textLineStartTime, "textLineStartTime",
    /// `Text line start timecode`: The start time of a TextLine expressed as timecode.
    textLineStartTimecode, "textLineStartTimecode",
    /// `Text line start timecode drop frames`: The start time of a TextLine expressed as timecode with drop frames.
    textLineStartTimecodeDropFrame, "textLineStartTimecodeDropFrame",
    /// `Time created.`: the tie of creation of an Asset.
    timeCreated, "timeCreated",
    /// `TimelineTrack duration`: To express the duration of a TimelineTrack.
    timelineTrackDuration, "timelineTrackDuration",
    /// `Duration (edit unit)`: To provide a duration as a number of edit units.
    timelineTrackDurationEditUnits, "timelineTrackDurationEditUnits",
    /// `Duration (time)`: To provide a duration as normal             time.
    timelineTrackDurationNormalPlayTime, "timelineTrackDurationNormalPlayTime",
    /// `Duration (timecode)`: The duration expressed as a             timecode.
    timelineTrackDurationTimecode, "timelineTrackDurationTimecode",
    /// `Duration (timecode, drop frame)`: The duration expressed as a             timecode with drop frames.
    timelineTrackDurationTimecodeDropFrame, "timelineTrackDurationTimecodeDropFrame",
    /// `Title`: All value of the EBU title status             classification scheme             (http://www.ebu.ch/metadata/cs/web/ebu_TitleStatusCodeCS_p.xml.htm) are candidates             subproperties of the title property as implemented for an example with             alternativeTitle.
    title, "title",
    /// `Total number of episodes`: To provide the total number of episodes in a Series or a Season.
    totalNumberOfEpisodes, "totalNumberOfEpisodes",
    /// `Total number of Group members`: To provide the total number of members in a Group.
    totalNumberOfGroupMembers, "totalNumberOfGroupMembers",
    /// `Definition`: To provide a definition associated to a             Track.
    trackDefinition, "trackDefinition",
    /// `Track identifier`: An Identifier attributed to a Track.
    trackId, "trackId",
    /// `Track name`: To provide  name of a             Track.
    trackName, "trackName",
    /// `Track name`: Range: string or Track_Type
    trackType, "trackType",
    /// `Translation title`: A translated version of the title.
    translationTitle, "translationTitle",
    /// `Username`: The username by which a Person is             known e.g. when attributing a rating value.
    username, "username",
    /// `Version`: To provide information on the current version of an EditorialObject.
    version, "version",
    /// `Version title`: An alternative title specific to a verison of content.
    versionTitle, "versionTitle",
    /// `Video bitrate`: The video bitrate. To provide the bitrate at which the MediaResource can be played           in bits/second. Current bitrate if constant, and average bitrate if           variable.
    videoBitRate, "videoBitRate",
    /// `Video bitrate max`: The maximum video bitrate.
    videoBitRateMax, "videoBitRateMax",
    /// `Video bitrate mode`: The video bitrate mode.
    videoBitRateMode, "videoBitRateMode",
    /// `Video encoding level`: The encoding level as defined in specifications.
    videoEncodingLevel, "videoEncodingLevel",
    /// `Video encoding profile`: The encoding level as defined in specifications.
    videoEncodingProfile, "videoEncodingProfile",
    /// `Width`: The width of e.g. a video frame typically             expressed as a number of pixels, or picture/image in millimeters.
    width, "width",
    /// `Width unit`: The unit used to measure a width e.g. in pixels             or number of lines or millimeters or else.
    widthUnit, "widthUnit",
    /// `Word count`: The number of words contained in a             document.
    wordCount, "wordCount",
    /// `Working title`: A title used while content is not complete.
    workingTitle, "workingTitle"
);
