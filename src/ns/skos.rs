// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `SKOS Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|SKOS Vocabulary|
//! |**Prefix**|skos|
//! |**Namespace base IRI**|[http://www.w3.org/2004/02/skos/core#](http://www.w3.org/2004/02/skos/core#)|
//! |**Description**|An RDF vocabulary for describing the basic structure and content of concept schemes such as thesauri, classification schemes, subject heading lists, taxonomies, 'folksonomies', other types of controlled vocabulary, and also concept schemes embedded in glossaries and terminologies.|
//! |**Is defined by**|[http://www.w3.org/2004/02/skos/core#](http://www.w3.org/2004/02/skos/core#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2004/02/skos/core#",;
    /// `Collection`: A meaningful collection of concepts.
    Collection, "Collection",
    /// `Concept`: An idea or notion; a unit of thought.
    Concept, "Concept",
    /// `Concept Scheme`: A set of concepts, optionally including statements about semantic relationships between those concepts.
    ConceptScheme, "ConceptScheme",
    /// `Ordered Collection`: An ordered collection of concepts, where both the grouping and the ordering are meaningful.
    OrderedCollection, "OrderedCollection",
    /// `alternative label`: The range of skos:altLabel is the class of RDF plain literals.
    altLabel, "altLabel",
    /// `has broader match`: skos:broadMatch is used to state a hierarchical mapping link between two conceptual resources in different concept schemes.
    broadMatch, "broadMatch",
    /// `has broader`: Broader concepts are typically rendered as parents in a concept hierarchy (tree).
    broader, "broader",
    /// `has broader transitive`: skos:broaderTransitive is a transitive superproperty of skos:broader.
    broaderTransitive, "broaderTransitive",
    /// `change note`: A note about a modification to a concept.
    changeNote, "changeNote",
    /// `has close match`: skos:closeMatch is used to link two concepts that are sufficiently similar that they can be used interchangeably in some information retrieval applications. In order to avoid the possibility of "compound errors" when combining mappings across more than two concept schemes, skos:closeMatch is not declared to be a transitive property.
    closeMatch, "closeMatch",
    /// `definition`: A statement or formal explanation of the meaning of a concept.
    definition, "definition",
    /// `editorial note`: A note for an editor, translator or maintainer of the vocabulary.
    editorialNote, "editorialNote",
    /// `has exact match`: skos:exactMatch is disjoint with each of the properties skos:broadMatch and skos:relatedMatch.
    exactMatch, "exactMatch",
    /// `example`: An example of the use of a concept.
    example, "example",
    /// `has top concept`: Relates, by convention, a concept scheme to a concept which is topmost in the broader/narrower concept hierarchies for that scheme, providing an entry point to these hierarchies.
    hasTopConcept, "hasTopConcept",
    /// `hidden label`: The range of skos:hiddenLabel is the class of RDF plain literals.
    hiddenLabel, "hiddenLabel",
    /// `history note`: A note about the past state/use/meaning of a concept.
    historyNote, "historyNote",
    /// `is in scheme`: Relates a resource (for example a concept) to a concept scheme in which it is included.
    inScheme, "inScheme",
    /// `is in mapping relation with`: These concept mapping relations mirror semantic relations, and the data model defined below is similar (with the exception of skos:exactMatch) to the data model defined for semantic relations. A distinct vocabulary is provided for concept mapping relations, to provide a convenient way to differentiate links within a concept scheme from links between concept schemes. However, this pattern of usage is not a formal requirement of the SKOS data model, and relies on informal definitions of best practice.
    mappingRelation, "mappingRelation",
    /// `has member`: Relates a collection to one of its members.
    member, "member",
    /// `has member list`: For any resource, every item in the list given as the value of the<br>      skos:memberList property is also a value of the skos:member property.
    memberList, "memberList",
    /// `has narrower match`: skos:narrowMatch is used to state a hierarchical mapping link between two conceptual resources in different concept schemes.
    narrowMatch, "narrowMatch",
    /// `has narrower`: Narrower concepts are typically rendered as children in a concept hierarchy (tree).
    narrower, "narrower",
    /// `has narrower transitive`: skos:narrowerTransitive is a transitive superproperty of skos:narrower.
    narrowerTransitive, "narrowerTransitive",
    /// `notation`: A notation, also known as classification code, is a string of characters such as "T58.5" or "303.4833" used to uniquely identify a concept within the scope of a given concept scheme.
    notation, "notation",
    /// `note`: A general note, for any purpose.
    note, "note",
    /// `preferred label`: The range of skos:prefLabel is the class of RDF plain literals.
    prefLabel, "prefLabel",
    /// `has related`: skos:related is disjoint with skos:broaderTransitive
    related, "related",
    /// `has related match`: skos:relatedMatch is used to state an associative mapping link between two conceptual resources in different concept schemes.
    relatedMatch, "relatedMatch",
    /// `scope note`: A note that helps to clarify the meaning and/or the use of a concept.
    scopeNote, "scopeNote",
    /// `is in semantic relation with`: Links a concept to a concept related by meaning.
    semanticRelation, "semanticRelation",
    /// `is top concept in scheme`: Relates a concept to the concept scheme that it is a top level concept of.
    topConceptOf, "topConceptOf"
);
