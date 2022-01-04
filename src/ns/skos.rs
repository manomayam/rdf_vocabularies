// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `SKOS Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|SKOS Vocabulary|
//! |**Prefix**|skos|
//! |**Namespace base IRI**|[<http://www.w3.org/2004/02/skos/core#>](<http://www.w3.org/2004/02/skos/core#>)|
//! |**Description**|An RDF vocabulary for describing the basic structure and content of concept schemes such as thesauri, classification schemes, subject heading lists, taxonomies, 'folksonomies', other types of controlled vocabulary, and also concept schemes embedded in glossaries and terminologies.|
//! |**Is defined by**|[<http://www.w3.org/2004/02/skos/core#>](<http://www.w3.org/2004/02/skos/core#>)|
//!

use crate::namespace;

namespace!(
    "<http://www.w3.org/2004/02/skos/core#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Collection`: 
    Collection, "Collection",
    /// `Concept`: 
    Concept, "Concept",
    /// `Concept Scheme`: 
    ConceptScheme, "ConceptScheme",
    /// `Ordered Collection`: 
    OrderedCollection, "OrderedCollection",
    /// `alternative label`: The range of skos:altLabel is the class of RDF plain literals.
    altLabel, "altLabel",
    /// `has broader match`: 
    broadMatch, "broadMatch",
    /// `has broader`: Broader concepts are typically rendered as parents in a concept hierarchy (tree).
    broader, "broader",
    /// `has broader transitive`: 
    broaderTransitive, "broaderTransitive",
    /// `change note`: 
    changeNote, "changeNote",
    /// `has close match`: 
    closeMatch, "closeMatch",
    /// `definition`: 
    definition, "definition",
    /// `editorial note`: 
    editorialNote, "editorialNote",
    /// `has exact match`: skos:exactMatch is disjoint with each of the properties skos:broadMatch and skos:relatedMatch.
    exactMatch, "exactMatch",
    /// `example`: 
    example, "example",
    /// `has top concept`: 
    hasTopConcept, "hasTopConcept",
    /// `hidden label`: The range of skos:hiddenLabel is the class of RDF plain literals.
    hiddenLabel, "hiddenLabel",
    /// `history note`: 
    historyNote, "historyNote",
    /// `is in scheme`: 
    inScheme, "inScheme",
    /// `is in mapping relation with`: These concept mapping relations mirror semantic relations, and the data model defined below is similar (with the exception of skos:exactMatch) to the data model defined for semantic relations. A distinct vocabulary is provided for concept mapping relations, to provide a convenient way to differentiate links within a concept scheme from links between concept schemes. However, this pattern of usage is not a formal requirement of the SKOS data model, and relies on informal definitions of best practice.
    mappingRelation, "mappingRelation",
    /// `has member`: 
    member, "member",
    /// `has member list`: For any resource, every item in the list given as the value of the       skos:memberList property is also a value of the skos:member property.
    memberList, "memberList",
    /// `has narrower match`: 
    narrowMatch, "narrowMatch",
    /// `has narrower`: Narrower concepts are typically rendered as children in a concept hierarchy (tree).
    narrower, "narrower",
    /// `has narrower transitive`: 
    narrowerTransitive, "narrowerTransitive",
    /// `notation`: 
    notation, "notation",
    /// `note`: 
    note, "note",
    /// `preferred label`: A resource has no more than one value of skos:prefLabel per language tag, and no more than one value of skos:prefLabel without language tag.
    prefLabel, "prefLabel",
    /// `has related`: skos:related is disjoint with skos:broaderTransitive
    related, "related",
    /// `has related match`: 
    relatedMatch, "relatedMatch",
    /// `scope note`: 
    scopeNote, "scopeNote",
    /// `is in semantic relation with`: 
    semanticRelation, "semanticRelation",
    /// `is top concept in scheme`: 
    topConceptOf, "topConceptOf"
);
