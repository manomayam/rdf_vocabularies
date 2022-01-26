// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `grddl` vocabulary
//!
//! This module requires `ns-grddl` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|grddl|
//! |**Namespace base IRI**|<http://www.w3.org/2003/g/data-view#>|
//! |**Description**||
//! |**Is defined by**|<http://www.w3.org/2003/g/data-view#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2003/g/data-view#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `InformationResource`: A resource which has the property that all of its essential characteristics can be conveyed in a message
    InformationResource, "InformationResource",
    /// `RDF graphs`: a     set of RDF triples
    RDFGraph, "RDFGraph",
    /// `RootNode`: the root of the tree in the XPath data     model
    RootNode, "RootNode",
    /// `Transformation`: an InformationResource that specifies     a transformation from a set of XML documents to RDF graphs
    Transformation, "Transformation",
    /// `TransformationProperty`: a FunctionalProperty that relates     XML document root nodes to     RDF graphs
    TransformationProperty, "TransformationProperty",
    /// `danc`: 
    danc, "danc",
    /// `grddl-wg`: 
    grddl_wg, "grddl-wg",
    /// `grddlProject`: 
    grddlProject, "grddlProject",
    /// `namespaceTransformation`: relates a namespace to a transformation for     all documents in that namespace
    namespaceTransformation, "namespaceTransformation",
    /// `profileTransformation`: relates a profile document to a     transformation for all documents bearing that profile
    profileTransformation, "profileTransformation",
    /// `result`: an     RDF graph obtained from an information resource by directly     parsing a representation in the standard RDF/XML syntax or     indirectly by parsing some other dialect using a transformation     nominated by the document
    result, "result",
    /// `transformation`: relates a source document to a     transformation, usually represented in XSLT, that relates the source document syntax     to the RDF graph syntax
    transformation, "transformation",
    /// `transformationProperty`: relates a transformation to the algorithm     specified by the property that computes an RDF graph from an XML     document node
    transformationProperty, "transformationProperty"
);
