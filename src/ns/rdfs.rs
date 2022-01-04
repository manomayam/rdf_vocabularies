// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The RDF Schema vocabulary (RDFS)` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The RDF Schema vocabulary (RDFS)|
//! |**Prefix**|rdfs|
//! |**Namespace base IRI**|[<http://www.w3.org/2000/01/rdf-schema#>](<http://www.w3.org/2000/01/rdf-schema#>)|
//! |**Description**||
//! |**Is defined by**|[<http://www.w3.org/2000/01/rdf-schema#>](<http://www.w3.org/2000/01/rdf-schema#>)|
//!

use crate::namespace;

namespace!(
    "<http://www.w3.org/2000/01/rdf-schema#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Class`: The class of classes.
    Class, "Class",
    /// `Container`: The class of RDF containers.
    Container, "Container",
    /// `ContainerMembershipProperty`: The class of container membership properties, rdf:_1, rdf:_2, ...,                     all of which are sub-properties of 'member'.
    ContainerMembershipProperty, "ContainerMembershipProperty",
    /// `Datatype`: The class of RDF datatypes.
    Datatype, "Datatype",
    /// `Literal`: The class of literal values, eg. textual strings and integers.
    Literal, "Literal",
    /// `Resource`: The class resource, everything.
    Resource, "Resource",
    /// `comment`: A description of the subject resource.
    comment, "comment",
    /// `domain`: A domain of the subject property.
    domain, "domain",
    /// `isDefinedBy`: The defininition of the subject resource.
    isDefinedBy, "isDefinedBy",
    /// `label`: A human-readable name for the subject.
    label, "label",
    /// `member`: A member of the subject resource.
    member, "member",
    /// `range`: A range of the subject property.
    range, "range",
    /// `seeAlso`: Further information about the subject resource.
    seeAlso, "seeAlso",
    /// `subClassOf`: The subject is a subclass of a class.
    subClassOf, "subClassOf",
    /// `subPropertyOf`: The subject is a subproperty of a property.
    subPropertyOf, "subPropertyOf"
);
