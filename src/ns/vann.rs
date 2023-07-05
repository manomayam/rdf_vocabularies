// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `VANN: A vocabulary for annotating vocabulary descriptions` vocabulary
//!
//! This module requires `ns-vann` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|VANN: A vocabulary for annotating vocabulary descriptions|
//! |**Prefix**|vann|
//! |**Namespace base IRI**|<http://purl.org/vocab/vann/>|
//! |**Description**|This document describes a vocabulary for annotating descriptions of vocabularies with examples and usage notes.|
//! |**Is defined by**|<http://vocab.org/vann/vann-vocab-20100607.rdf>|
//!

use crate::namespace;

namespace!(
    base: "http://purl.org/vocab/vann/",

    terms: [
        /// `Changes`: A reference to a resource that describes changes between this version of a vocabulary and the previous.
        (changes, "changes"),
        /// `Example`: A reference to a resource that provides an example of how this resource can be used.
        (example, "example"),
        /// `Preferred Namespace Prefix`: The preferred namespace prefix to use when using terms from this vocabulary in an XML document.
        (preferredNamespacePrefix, "preferredNamespacePrefix"),
        /// `Preferred Namespace Uri`: The preferred namespace URI to use when using terms from this vocabulary in an XML document.
        (preferredNamespaceUri, "preferredNamespaceUri"),
        /// `Term Group`: A group of related terms in a vocabulary.
        (termGroup, "termGroup"),
        /// `Usage Note`: A reference to a resource that provides information on how this resource is to be used.
        (usageNote, "usageNote"),
        /// `vann-vocab-20040305`:
        (vann_vocab_20040305, "vann-vocab-20040305")    ]
);
