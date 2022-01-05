// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Vocabulary for publishing SDMX statistical data in RDF` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Vocabulary for publishing SDMX statistical data in RDF|
//! |**Prefix**|sdmx|
//! |**Namespace base IRI**|[http://purl.org/linked-data/sdmx#](http://purl.org/linked-data/sdmx#)|
//! |**Description**|This vocabulary extends the data cube vocabulary to support publication of statistical data in RDF, using an information model based on SDMX|
//! |**Is defined by**|[http://purl.org/linked-data/sdmx#](http://purl.org/linked-data/sdmx#)|
//!

use crate::namespace;

namespace!(
    "http://purl.org/linked-data/sdmx#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Concept scheme`: 
    CodeList, "CodeList",
    /// `concept`: Denotes an SDMX concept, used in the particular SDMX terminological sense, which is in principle narrower than skos:Concept
    Concept, "Concept",
    /// `Concept role (abstract)`: Abstract superclass of classes denoting statistical roles which be played by concepts
    ConceptRole, "ConceptRole",
    /// `Count property`: concept for a dimension or attribute that plays the role of an identifier where the identifier is taken from a known system of counts (e.g. the Nth current dataset update for a given day)
    CountRole, "CountRole",
    /// `Data Structure Definition`: A specialization of qb:DataStructureDefinition which additional constraints: if there are multiple measures then a MeasureDimensionProperty must be included in the DSD
    DataStructureDefinition, "DataStructureDefinition",
    /// `Entity property`: concept for a dimension or attribute that plays the role of the DataAttribute subject to whom the data refers (e.g. the reporting agent for primary reporting, the country for secondary reporting)
    EntityRole, "EntityRole",
    /// `Frequency property`: concept for a dimension or attribute that plays the role of frequency
    FrequencyRole, "FrequencyRole",
    /// `Identity property`: concept for a dimension or attribute that plays the role of an identifier which is taken from a known scheme of identifiers.
    IdentityRole, "IdentityRole",
    /// `Measure type property`: concept for a dimension that plays the role of identifying a type of measure
    MeasureTypeRole, "MeasureTypeRole",
    /// `Non-observation time property`: concept for a dimension or attribute that plays the role of a date/time identifier in the KeyFamily which is not related to the time of the observation
    NonObsTimeRole, "NonObsTimeRole",
    /// `Primary measure property`: concept for a measure that plays the role of the observation in a time series
    PrimaryMeasureRole, "PrimaryMeasureRole",
    /// `Time property`: concept for a dimension that specifies the time of the observation of the primaryMeasure
    TimeRole, "TimeRole",
    /// `measureType`: The concept corresponding to the generic measure type dimension which indicates which measure is being denoted by the primary measure on an observation
    measureTypeConcept, "measureTypeConcept",
    /// `primary measure`: Indicates an additional component used as the primary measure within the SDMX data. In the case of multi-measure data sets the RDF representation uses the specific measure rather than a subsuming overall measure. This property records the subsuming primary measure (typically sdmx-measure:obsValue) to enable round tripping of this information.
    primaryMeasure, "primaryMeasure"
);
