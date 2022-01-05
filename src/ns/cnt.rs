// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Representing Content in RDF` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Representing Content in RDF|
//! |**Prefix**|cnt|
//! |**Namespace base IRI**|[http://www.w3.org/2011/content#](http://www.w3.org/2011/content#)|
//! |**Description**|Representing Content in RDF as defined by http://www.w3.org/TR/Content-in-RDF/|
//! |**Is defined by**|[http://www.w3.org/2011/content#](http://www.w3.org/2011/content#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2011/content#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Content`: The content.
    Content, "Content",
    /// `Base64 content`: The base64 encoded content (can be used for binary content).
    ContentAsBase64, "ContentAsBase64",
    /// `Text content`: The text content (can be used for text content).
    ContentAsText, "ContentAsText",
    /// `XML content`: The XML content (can only be used for XML-wellformed content).
    ContentAsXML, "ContentAsXML",
    /// `Document type declaration`: The document type declaration.
    DoctypeDecl, "DoctypeDecl",
    /// `Base64 encoded byte sequence`: The Base64 encoded byte sequence of the content.
    bytes, "bytes",
    /// `Character encoding`: The character encoding used to create a character sequence from a byte sequence or vice versa.
    characterEncoding, "characterEncoding",
    /// `Character sequence`: The character sequence of the text content.
    chars, "chars",
    /// `XML character encoding`: The character encoding declared in the XML declaration.
    declaredEncoding, "declaredEncoding",
    /// `Document type name`: The document type name.
    doctypeName, "doctypeName",
    /// `Document type declaration`: The document type declaration.
    dtDecl, "dtDecl",
    /// `Internal DTD subset`: The internal document type definition subset within the document type declarations.
    internalSubset, "internalSubset",
    /// `XML leading misc`: The XML content preceding the document type declaration.
    leadingMisc, "leadingMisc",
    /// `Public ID`: The document type declarations's public identifier.
    publicId, "publicId",
    /// `XML rest`: The XML content following the document type declaration.
    rest, "rest",
    /// `XML standalone document declaration`: The standalone declaration in the XML declaration.
    standalone, "standalone",
    /// `System ID`: The document type declarations's system identifier (typed: xsd:anyURI)
    systemId, "systemId",
    /// `XML version`: The XML version declared in the XML declaration.
    version, "version"
);
