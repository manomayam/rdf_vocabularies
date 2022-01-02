// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `SemWeb Vocab Status ontology` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|SemWeb Vocab Status ontology|
//! |**Prefix**|vs|
//! |**Namespace base IRI**|[http://www.w3.org/2003/06/sw-vocab-status/ns#](http://www.w3.org/2003/06/sw-vocab-status/ns#)|
//! |**Description**|An RDF vocabulary for relating SW vocabulary terms to their status.|
//! |**Is defined by**|[http://www.w3.org/2003/06/sw-vocab-status/ns#](http://www.w3.org/2003/06/sw-vocab-status/ns#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2003/06/sw-vocab-status/ns#",;
    /// `more info`: more information about the status etc of a term, typically human oriented
    moreinfo, "moreinfo",
    /// `term status`: the status of a vocabulary term, expressed as a short symbolic string; known values include 'unstable','testing', 'stable' and 'archaic'
    term_status, "term_status",
    /// `user docs`: human-oriented documentation, examples etc for use of this term
    userdocs, "userdocs"
);
