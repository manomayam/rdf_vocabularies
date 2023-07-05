// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The RDF Concepts Vocabulary (RDF)` vocabulary
//!
//! This module requires `ns-rdf` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The RDF Concepts Vocabulary (RDF)|
//! |**Prefix**|rdf|
//! |**Namespace base IRI**|<http://www.w3.org/1999/02/22-rdf-syntax-ns#>|
//! |**Description**|This is the RDF Schema for the RDF vocabulary terms in the RDF Namespace, defined in RDF 1.1 Concepts.|
//! |**Is defined by**|<http://www.w3.org/1999/02/22-rdf-syntax-ns#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/1999/02/22-rdf-syntax-ns#",

    terms: [
        /// `Alt`: The class of containers of alternatives.
        (Alt, "Alt"),
        /// `Bag`: The class of unordered containers.
        (Bag, "Bag"),
        /// `CompoundLiteral`: A class representing a compound literal.
        (CompoundLiteral, "CompoundLiteral"),
        /// `HTML`: The datatype of RDF literals storing fragments of HTML content
        (HTML, "HTML"),
        /// `JSON`: The datatype of RDF literals storing JSON content.
        (JSON, "JSON"),
        /// `List`: The class of RDF Lists.
        (List, "List"),
        /// `PlainLiteral`: The class of plain (i.e. untyped) literal values, as used in RIF and OWL 2
        (PlainLiteral, "PlainLiteral"),
        /// `Property`: The class of RDF properties.
        (Property, "Property"),
        /// `Seq`: The class of ordered containers.
        (Seq, "Seq"),
        /// `Statement`: The class of RDF statements.
        (Statement, "Statement"),
        /// `XMLLiteral`: The datatype of XML literal values.
        (XMLLiteral, "XMLLiteral"),
        /// `direction`: The base direction component of a CompoundLiteral.
        (direction, "direction"),
        /// `first`: The first item in the subject RDF list.
        (first, "first"),
        /// `langString`: The datatype of language-tagged string values
        (langString, "langString"),
        /// `language`: The language component of a CompoundLiteral.
        (language, "language"),
        /// `nil`: The empty list, with no items in it. If the rest of a list is nil then the list has no more items in it.
        (nil, "nil"),
        /// `object`: The object of the subject RDF statement.
        (object, "object"),
        /// `predicate`: The predicate of the subject RDF statement.
        (predicate, "predicate"),
        /// `rest`: The rest of the subject RDF list after the first item.
        (rest, "rest"),
        /// `subject`: The subject of the subject RDF statement.
        (subject, "subject"),
        /// `type`: The subject is an instance of a class.
        (type_, "type"),
        /// `value`: Idiomatic property used for structured values.
        (value, "value")    ]
);
