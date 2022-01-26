// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-dcmitype")))]
//! This module provides terms for `DCMI Type Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|DCMI Type Vocabulary|
//! |**Prefix**|dcmitype|
//! |**Namespace base IRI**|<http://purl.org/dc/dcmitype/>|
//! |**Description**||
//! |**Is defined by**|<http://purl.org/dc/dcmitype/>|
//!

use crate::namespace;

namespace!(
    "http://purl.org/dc/dcmitype/",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Collection`: An aggregation of resources.
    Collection, "Collection",
    /// `Dataset`: Data encoded in a defined structure.
    Dataset, "Dataset",
    /// `Event`: A non-persistent, time-based occurrence.
    Event, "Event",
    /// `Image`: A visual representation other than text.
    Image, "Image",
    /// `Interactive Resource`: A resource requiring interaction from the user to be understood, executed, or experienced.
    InteractiveResource, "InteractiveResource",
    /// `Moving Image`: A series of visual representations imparting an impression of motion when shown in succession.
    MovingImage, "MovingImage",
    /// `Physical Object`: An inanimate, three-dimensional object or substance.
    PhysicalObject, "PhysicalObject",
    /// `Service`: A system that provides one or more functions.
    Service, "Service",
    /// `Software`: A computer program in source or compiled form.
    Software, "Software",
    /// `Sound`: A resource primarily intended to be heard.
    Sound, "Sound",
    /// `Still Image`: A static visual representation.
    StillImage, "StillImage",
    /// `Text`: A resource consisting primarily of words for reading.
    Text, "Text"
);
