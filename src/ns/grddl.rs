// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `grddl` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|grddl|
//! |**Namespace base IRI**|[http://www.w3.org/2003/g/data-view#](http://www.w3.org/2003/g/data-view#)|
//! |**Description**||
//! |**Is defined by**|[http://www.w3.org/2003/g/data-view#](http://www.w3.org/2003/g/data-view#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2003/g/data-view#",;
    /// `InformationResource`: A resource which has the property that all of its essential characteristics can be conveyed in a message
    InformationResource, "InformationResource",
    /// `RDFGraph`: a<br>    set of RDF triples
    RDFGraph, "RDFGraph",
    /// `XML document root nodes`: the root of the tree in the XPath data<br>    model
    RootNode, "RootNode",
    /// `Transformation`: an InformationResource that specifies<br>    a transformation from a set of XML documents to RDF graphs
    Transformation, "Transformation",
    /// `TransformationProperty`: a FunctionalProperty that relates<br>    XML document root nodes to<br>    RDF graphs
    TransformationProperty, "TransformationProperty",
    /// ``: 
    danc, "danc",
    /// ``: 
    grddl_wg, "grddl-wg",
    /// ``: 
    grddlProject, "grddlProject",
    /// `namespaceTransformation`: relates a namespace to a transformation for<br>    all documents in that namespace
    namespaceTransformation, "namespaceTransformation",
    /// `profileTransformation`: relates a profile document to a<br>    transformation for all documents bearing that profile
    profileTransformation, "profileTransformation",
    /// `result`: an<br>    RDF graph obtained from an information resource by directly<br>    parsing a representation in the standard RDF/XML syntax or<br>    indirectly by parsing some other dialect using a transformation<br>    nominated by the document
    result, "result",
    /// `transformation`: relates a source document to a<br>    transformation, usually represented in XSLT, that relates the source document syntax<br>    to the RDF graph syntax
    transformation, "transformation",
    /// `transformationProperty`: relates a transformation to the algorithm<br>    specified by the property that computes an RDF graph from an XML<br>    document node
    transformationProperty, "transformationProperty"
);
