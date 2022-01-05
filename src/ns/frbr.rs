// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `frbr` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|frbr|
//! |**Namespace base IRI**|[http://purl.org/vocab/frbr/core#](http://purl.org/vocab/frbr/core#)|
//! |**Description**||
//! |**Is defined by**|[http://purl.org/spar/frbr.nt](http://purl.org/spar/frbr.nt)|
//!

use crate::namespace;

namespace!(
    "http://purl.org/vocab/frbr/core#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `concept`: An abstract notion or idea.  The entity defined as concept encompasses a comprehensive range of abstractions that may be the subject of a work: fields of knowledge, disciplines, schools of thought (philosophies, religions, political ideologies, etc.), theories, processes, techniques, practices, etc.  A concept may be broad in nature or narrowly defined and precise. 
    Concept, "Concept",
    /// `corporate body`: An organization or group of individuals and/or organizations acting as a unit.  The entity defined as corporate body encompasses organizations and groups of individuals and/or organizations that are identified by a particular name, including occasional groups and groups that are constituted as meetings, conferences, congresses, expeditions, exhibitions, festivals, fairs, etc.
    CorporateBody, "CorporateBody",
    /// `endeavour`: It describes different aspects of user interests in the products of intellectual or artistic artifact.
    Endeavour, "Endeavour",
    /// `event`: An action or occurrence.  The entity defined as event encompasses a comprehensive range of actions and occurrences that may be the subject of a work: historical events, epochs, periods of time, etc. 
    Event, "Event",
    /// `expression`: The intellectual or artistic realization of a work in the form of alpha-numeric, musical, or choreographic notation, sound, image, object, movement, etc., or any combination of such forms.  An expression is the specific intellectual or artistic form that a work takes each time it is "realized." Expression encompasses, for example, the specific words, sentences, paragraphs, etc. that result from the realization of a work in the form of a text, or the particular sounds, phrasing, etc. resulting from the realization of a musical work.
    Expression, "Expression",
    /// `item`: A single exemplar of a manifestation.  The entity defined as item is a concrete entity. It is in many instances a single physical object (e.g., a copy of a one-volume monograph, a single audio cassette, etc.). There are instances, however, where the entity defined as item comprises more than one physical object (e.g., a monograph issued as two separately bound volumes, a recording issued on three separate compact discs, etc.). 
    Item, "Item",
    /// `manifestation`: The physical embodiment of an expression of a work.  The entity defined as manifestation encompasses a wide range of materials and formats. As an entity, manifestation represents all the physical objects that bear the same characteristics, in respect to both intellectual content and physical form. 
    Manifestation, "Manifestation",
    /// `object`: A material thing.  The entity defined as object encompasses a comprehensive range of material things that may be the subject of a work: animate and inanimate objects occurring in nature; fixed, movable, and moving objects that are the product of human creation; objects that no longer exist. 
    Object, "Object",
    /// `person`: An individual. The entity defined as person encompasses individuals that are deceased as well as those that are living.
    Person, "Person",
    /// `place`: A location.  The entity defined as place encompasses a comprehensive range of locations: terrestrial and extra-terrestrial; historical and contemporary; geographic features and geo-political jurisdictions. 
    Place, "Place",
    /// `responsible entity`: It represents those responsible for the intellectual or artistic content, the physical production and dissemination, or the custodianship of any endeavour.
    ResponsibleEntity, "ResponsibleEntity",
    /// `subject`: It represents an additional set of entities that serve as the subjects of works.
    Subject, "Subject",
    /// `work`: A distinct intellectual or artistic creation.  A work is an abstract entity; there is no single material object one can point to as the work. We recognize the work through individual realizations or expressions of the work, but the work itself exists only in the commonality of content between and among the various expressions of the work. When we speak of Homer's Iliad as a work, our point of reference is not a particular recitation or text of the work, but the intellectual creation that lies behind all the various expressions of the work. 
    Work, "Work",
    /// `has abridgement`: An expression abridged in another one.  In the abridged expression some content of the previous expression is removed, but the result does not alter the content to the extent that it becomes a new work. The expressions resulting from such modification are generally autonomous in nature (i.e., they do not normally require reference to the prior expression in order to be used or understood). 
    abridgement, "abridgement",
    /// `is abridgement of`: It identifies the entire expression of an abridged one.
    abridgementOf, "abridgementOf",
    /// `has adaption`: A work/expression adapted in another one.  This property describe the modification of an original work that is sufficient in degree to warrant their being considered as new works, rather than simply different expressions of the same work. If there exists a relation of this kind among two different expressions, they always refer to different works.
    adaption, "adaption",
    /// `is adaption of`: It identifies the work/expression of an adapted one.
    adaptionOf, "adaptionOf",
    /// `has alternate`: A manifestation having another one as alternate.  The alternate relationship involves manifestations that effectively serve as alternates for each other. The alternate relationship obtains, for example, when a publication, sound recording, video, etc. is issued in more than one format or when it is released simultaneously by different publishers in different countries.
    alternate, "alternate",
    /// `is alternate of`: It identifies the manifestation of an alternative one.
    alternateOf, "alternateOf",
    /// `has arrangement`: An expression arranged in another one.  In the arranged expression some content of the previous expression is changed in some way, but the result does not alter the content to the extent that it becomes a new work. The expressions resulting from such modification are generally autonomous in nature (i.e., they do not normally require reference to the prior expression in order to be used or understood).
    arrangement, "arrangement",
    /// `is arrangement of`: It identifies the original expression of an arranged one.
    arrangementOf, "arrangementOf",
    /// `has complement`: An expression work/expression having another one as complement.  This property describes works that are intended to be combined with or inserted into the related work. In other words, they are intended to be integrated in some way with the other work, but were not part of the original conception of that prior work. If there exists a relation of this kind among two different expressions, then they always refer to different works.
    complement, "complement",
    /// `is complement of`: It identifies the work/expression of that is a complement of another one.
    complementOf, "complementOf",
    /// `has creator`: A work linked to its creator.
    creator, "creator",
    /// `is creator of`: The creator of a particular work.
    creatorOf, "creatorOf",
    /// `has embodiment`: An expression embodied in a manifestation.
    embodiment, "embodiment",
    /// `is embodiment of`: A manifestation that embodies an expression.
    embodimentOf, "embodimentOf",
    /// `has exemplar`: A manifestation exemplified in an item.
    exemplar, "exemplar",
    /// `is exemplar of`: An item that exemplifies a manifestation.
    exemplarOf, "exemplarOf",
    /// `has imitation`: An work/expression imitated in another one.  This property describes works that are intended to be an imitation another original work that is sufficient in degree to warrant their being considered as new works, rather than simply different expressions of the same work. If there exists a relation of this kind among two different expressions, then they always refer to different works.
    imitation, "imitation",
    /// `is imitation of`: It identifies the work/expression of an imitated one.
    imitationOf, "imitationOf",
    /// `has owner`: An item linked to its owner.
    owner, "owner",
    /// `is owner of`: The owner of a particular item.
    ownerOf, "ownerOf",
    /// `has part`: A part of an endeavour.
    part, "part",
    /// `is part of`: An endeavour incorporating another endeavour.
    partOf, "partOf",
    /// `has producer`: A manifestation linked to its prodecer.
    producer, "producer",
    /// `is producer of`: The producer of a particular manifestation.
    producerOf, "producerOf",
    /// `has realization`: A work realized through an expression.
    realization, "realization",
    /// `is realization of`: An expression that realizes a work.
    realizationOf, "realizationOf",
    /// `has realizer`: An expression linked to its realizer.
    realizer, "realizer",
    /// `is realizer of`: The realizer of a particular expression.
    realizerOf, "realizerOf",
    /// `has reconfiguration`: An item reconfigured in another one.  The reconfiguration relationship is one in which one or more items are changed in such a way that a new item or items result. Most commonly, an item of one manifestation is bound with an item of a different manifestation to make a new item. 
    reconfiguration, "reconfiguration",
    /// `is reconfiguration of`: It identifies the manifestation of a reconfigured one.
    reconfigurationOf, "reconfigurationOf",
    /// `has related endeavour`: 
    relatedEndeavour, "relatedEndeavour",
    /// `has reproduction`: A manifestation/item reproduced in another one.  A reproduction indicates the relationship as it would be drawn from the first manifestation/item in the relationship to the second manifestation/item in the relationship.
    reproduction, "reproduction",
    /// `is reproduction of`: It identifies the manifestation/item of a reproduced one.
    reproductionOf, "reproductionOf",
    /// `has responsible entity`: Any endeavour having a particular entity that is responsible of it.
    responsibleEntity, "responsibleEntity",
    /// `is responsible entity of`: An entity that is resposible for a particular endeavour.
    responsibleEntityOf, "responsibleEntityOf",
    /// `has revision`: An expression revised in another one.  A revision has the intent to alter or update the content of the prior expression, but without changing the content so much that it becomes a new work.
    revision, "revision",
    /// `is revision of`: It identifies the previous expression of a revised one.
    revisionOf, "revisionOf",
    /// `has subject`: A work linked to a particular subject it is talking about.
    subject, "subject",
    /// `is subject of`: A subject a work talks abbout.
    subjectOf, "subjectOf",
    /// `has successor`: An expression work/expression having another one as successor.  The successor type of relationship involves a kind of linear progression of content from one work/expression to the other. In some cases, the content of the successor may be closely connected to the content of the preceding work, which would result in a work that is referential. In others, such as with loosely connected parts of a trilogy, the successor will be autonomous. Serial publications that result from the merger or split of their predecessors and stand on their own without requiring reference to the predecessor are also examples of autonomous works that fall within the successor relationship type. If there exists a relation of this kind among two different expressions, then they always refer to different works.
    successor, "successor",
    /// `is successor of`: It identifies the previous work/expression of a succeeded one.
    successorOf, "successorOf",
    /// `has summarization`: A work/expression summarized in another one.  This property describe the summarization of an original work that is sufficient in degree to warrant their being considered as new works, rather than simply different expressions of the same work. If there exists a relation of this kind among two different expressions, they always refer to different works.
    summarization, "summarization",
    /// `is summarization of`: It identifies the original work/expression of a summarized one.
    summarizationOf, "summarizationOf",
    /// `has supplement`: An expression work/expression having another one as supplement.  The supplement relationship type involves works/expressions that are intended to be used in conjunction with another work/expression. Some of these, such as indices, concordances, teachers' guides, glosses, and instruction manuals for electronic resources will be so closely associated with the content of the related work/expression that they are useless without the other work/expression.
    supplement, "supplement",
    /// `is supplement of`: It identifies the work/expression of a particular supplement of it.
    supplementOf, "supplementOf",
    /// `has transformation`: An work/expression transformed in another one.  This property describes the transformation of an original work or expression into another work or expression that is sufficiently different in degree to warrant the product of the transformation being considered as a new work or expression, rather than simply a different expression of the original work. If there exists a frbr:transformation relation between two different expressions, then they always relate to different works.
    transformation, "transformation",
    /// `is transformation of`: It identifies the original work/expression of a trasformed one.
    transformationOf, "transformationOf",
    /// `has translation`: An expression translated in another one.  It allows to refer to a literal translation, in which the intent is to render the intellectual content of the previous expression as accurately as possible.
    translation, "translation",
    /// `is translation of`: It identifies the original expression of a translated one.
    translationOf, "translationOf"
);
