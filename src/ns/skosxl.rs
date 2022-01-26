// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-skosxl")))]
//! This module provides terms for `SKOS XL Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|SKOS XL Vocabulary|
//! |**Prefix**|skosxl|
//! |**Namespace base IRI**|<http://www.w3.org/2008/05/skos-xl#>|
//! |**Description**|An RDF vocabulary extending SKOS and allowing the description and linking of lexical entities.|
//! |**Is defined by**|<http://www.w3.org/2008/05/skos-xl#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2008/05/skos-xl#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Label`: 
    Label, "Label",
    /// `alternative label`: If C skosxl:altLabel L and L skosxl:literalForm V, then X skos:altLabel V.
    altLabel, "altLabel",
    /// `hidden label`: If C skosxl:hiddenLabel L and L skosxl:literalForm V, then C skos:hiddenLabel V.
    hiddenLabel, "hiddenLabel",
    /// `label relation`: 
    labelRelation, "labelRelation",
    /// `literal form`: If two instances of the class skosxl:Label have the same literal form, they are not necessarily the same resource.
    literalForm, "literalForm",
    /// `preferred label`: If C skosxl:prefLabel L and L skosxl:literalForm V, then X skos:prefLabel V.
    prefLabel, "prefLabel"
);
