// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `SemWeb Vocab Status ontology` vocabulary
//!
//! This module requires `ns-vs` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|SemWeb Vocab Status ontology|
//! |**Prefix**|vs|
//! |**Namespace base IRI**|<http://www.w3.org/2003/06/sw-vocab-status/ns#>|
//! |**Description**|An RDF vocabulary for relating SW vocabulary terms to their status.|
//! |**Is defined by**|<http://www.w3.org/2003/06/sw-vocab-status/ns#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/2003/06/sw-vocab-status/ns#",

    terms: [
        /// `more info`: more information about the status etc of a term, typically human oriented
        (moreinfo, "moreinfo"),
        /// `term status`: the status of a vocabulary term, expressed as a short symbolic string; known values include 'unstable','testing', 'stable' and 'archaic'
        (term_status, "term_status"),
        /// `user docs`: human-oriented documentation, examples etc for use of this term
        (userdocs, "userdocs")    ]
);
