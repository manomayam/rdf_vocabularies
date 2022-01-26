// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-qb")))]
//! This module provides terms for `Vocabulary for multi-dimensional (e.g. statistical) data publishing` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Vocabulary for multi-dimensional (e.g. statistical) data publishing|
//! |**Prefix**|qb|
//! |**Namespace base IRI**|<http://purl.org/linked-data/cube#>|
//! |**Description**|This vocabulary allows multi-dimensional data, such as statistics, to be published in RDF. It is based on the core information model from SDMX (and thus also DDI).|
//! |**Is defined by**|<http://purl.org/linked-data/cube#>|
//!

use crate::namespace;

namespace!(
    "http://purl.org/linked-data/cube#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Attachable (abstract)`: Abstract superclass for everything that can have attributes and dimensions
    Attachable, "Attachable",
    /// `Attribute property`: The class of components which represent attributes of observations in the cube, e.g. unit of measurement
    AttributeProperty, "AttributeProperty",
    /// `Coded property`: Superclass of all coded ComponentProperties
    CodedProperty, "CodedProperty",
    /// `Component property (abstract)`: Abstract super-property of all properties representing dimensions, attributes or measures
    ComponentProperty, "ComponentProperty",
    /// `Component set`: Abstract class of things which reference one or more ComponentProperties
    ComponentSet, "ComponentSet",
    /// `Component specification`: Used to define properties of a component (attribute, dimension etc) which are specific to its usage in a DSD.
    ComponentSpecification, "ComponentSpecification",
    /// `Data set`: Represents a collection of observations, possibly organized into various slices, conforming to some common dimensional structure.
    DataSet, "DataSet",
    /// `Data structure definition`: Defines the structure of a DataSet or slice
    DataStructureDefinition, "DataStructureDefinition",
    /// `Dimension property`: The class of components which represent the dimensions of the cube
    DimensionProperty, "DimensionProperty",
    /// `Hierarchical Code List`: Represents a generalized hierarchy of concepts which can be used for coding. The hierarchy is defined by one or more roots together with a property which relates concepts in the hierarchy to thier child concept .  The same concepts may be members of multiple hierarchies provided that different qb:parentChildProperty values are used for each hierarchy.
    HierarchicalCodeList, "HierarchicalCodeList",
    /// `Measure property`: The class of components which represent the measured value of the phenomenon being observed
    MeasureProperty, "MeasureProperty",
    /// `Observation`: A single observation in the cube, may have one or more associated measured values
    Observation, "Observation",
    /// `Observation Group`: A, possibly arbitrary, group of observations.
    ObservationGroup, "ObservationGroup",
    /// `Slice`: Denotes a subset of a DataSet defined by fixing a subset of the dimensional values, component properties on the Slice
    Slice, "Slice",
    /// `Slice key`: Denotes a subset of the component properties of a DataSet which are fixed in the corresponding slices
    SliceKey, "SliceKey",
    /// `attribute`: An alternative to qb:componentProperty which makes explicit that the component is a attribute
    attribute, "attribute",
    /// `code list`: gives the code list associated with a CodedProperty
    codeList, "codeList",
    /// `component specification`: indicates a component specification which is included in the structure of the dataset
    component, "component",
    /// `component attachment`: Indicates the level at which the component property should be attached, this might an qb:DataSet, qb:Slice or qb:Observation, or a qb:MeasureProperty.
    componentAttachment, "componentAttachment",
    /// `component`: indicates a ComponentProperty (i.e. attribute/dimension) expected on a DataSet, or a dimension fixed in a SliceKey
    componentProperty, "componentProperty",
    /// `component required`: Indicates whether a component property is required (true) or optional (false) in the context of a DSD. Only applicable     to components correspond to an attribute. Defaults to false (optional).
    componentRequired, "componentRequired",
    /// `concept`: gives the concept which is being measured or indicated by a ComponentProperty
    concept, "concept",
    /// `data set`: indicates the data set of which this observation is a part
    dataSet, "dataSet",
    /// `dimension`: An alternative to qb:componentProperty which makes explicit that the component is a dimension
    dimension, "dimension",
    /// `hierarchyRoot`: Specifies a root of the hierarchy. A hierarchy may have multiple roots but must have at least one.
    hierarchyRoot, "hierarchyRoot",
    /// `measure`: An alternative to qb:componentProperty which makes explicit that the component is a measure
    measure, "measure",
    /// `measure dimension`: An alternative to qb:componentProperty which makes explicit that the component is a measure dimension
    measureDimension, "measureDimension",
    /// `measure type`: Generic measure dimension, the value of this dimension indicates which measure (from the set of measures in the DSD) is being given by the obsValue (or other primary measure)
    measureType, "measureType",
    /// `observation`: indicates a observation contained within this slice of the data set
    observation, "observation",
    /// `observation group`: Indicates a group of observations. The domain of this property is left open so that a group may be attached to different resources and need not be restricted to a single DataSet
    observationGroup, "observationGroup",
    /// `order`: indicates a priority order for the components of sets with this structure, used to guide presentations - lower order numbers come before higher numbers, un-numbered components come last
    order, "order",
    /// `parent-child property`: Specifies a property which relates a parent concept in the hierarchy to a child concept.
    parentChildProperty, "parentChildProperty",
    /// `slice`: Indicates a subset of a DataSet defined by fixing a subset of the dimensional values
    slice, "slice",
    /// `slice key`: indicates a slice key which is used for slices in this dataset
    sliceKey, "sliceKey",
    /// `slice structure`: indicates the sub-key corresponding to this slice
    sliceStructure, "sliceStructure",
    /// `structure`: indicates the structure to which this data set conforms
    structure, "structure"
);
