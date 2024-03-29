// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `DCMI Type Vocabulary` vocabulary
//!
//! This module requires `ns-dcmitype` feature to be enabled.
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
    base: "http://purl.org/dc/dcmitype/",

    terms: [
        /// `Collection`: An aggregation of resources.
        (Collection, "Collection"),
        /// `Dataset`: Data encoded in a defined structure.
        (Dataset, "Dataset"),
        /// `Event`: A non-persistent, time-based occurrence.
        (Event, "Event"),
        /// `Image`: A visual representation other than text.
        (Image, "Image"),
        /// `Interactive Resource`: A resource requiring interaction from the user to be understood, executed, or experienced.
        (InteractiveResource, "InteractiveResource"),
        /// `Moving Image`: A series of visual representations imparting an impression of motion when shown in succession.
        (MovingImage, "MovingImage"),
        /// `Physical Object`: An inanimate, three-dimensional object or substance.
        (PhysicalObject, "PhysicalObject"),
        /// `Service`: A system that provides one or more functions.
        (Service, "Service"),
        /// `Software`: A computer program in source or compiled form.
        (Software, "Software"),
        /// `Sound`: A resource primarily intended to be heard.
        (Sound, "Sound"),
        /// `Still Image`: A static visual representation.
        (StillImage, "StillImage"),
        /// `Text`: A resource consisting primarily of words for reading.
        (Text, "Text")    ]
);
