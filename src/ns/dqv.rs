// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Data Quality Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Data Quality Vocabulary|
//! |**Prefix**|dqv|
//! |**Namespace base IRI**|[http://www.w3.org/ns/dqv#](http://www.w3.org/ns/dqv#)|
//! |**Description**|The Data Quality Vocabulary (DQV) is seen as an extension to DCAT to cover the quality of the data, how frequently is it updated, whether it accepts user corrections, persistence commitments etc. When used by publishers, this vocabulary will foster trust in the data amongst developers.|
//! |**Is defined by**|[http://www.w3.org/ns/dqv#](http://www.w3.org/ns/dqv#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/dqv#",;
    /// `Category`: Represents a group of quality dimensions in which a common type of information is used as quality indicator.
    Category, "Category",
    /// `Dimension`: Represents criteria relevant for assessing quality. Each quality dimension must have one or more metric to measure it. A dimension is linked with a category using the dqv:inCategory property.
    Dimension, "Dimension",
    /// `Metric`: Represents a standard to measure a quality dimension. An observation (instance of dqv:QualityMeasurement) assigns a value in a given unit to a Metric.
    Metric, "Metric",
    /// `Quality Annotation`: Represents quality annotations, including ratings, quality certificates or feedback that can be associated to datasets or distributions. Quality annotations must have one oa:motivatedBy statement with an instance of oa:Motivation (and skos:Concept) that reflects a quality assessment purpose. We define this instance as dqv:qualityAssessment.
    QualityAnnotation, "QualityAnnotation",
    /// `Quality Certificate`: An annotation that associates a resource (especially, a dataset or a distribution) to another resource (for example, a document) that certifies the resource's quality according to a set of quality assessment rules.
    QualityCertificate, "QualityCertificate",
    /// `Quality Measurement`: Represents the evaluation of a given dataset (or dataset distribution) against a specific quality metric.
    QualityMeasurement, "QualityMeasurement",
    /// `Quality Measurement Dataset`: Represents a dataset of quality measurements, evaluations of one or more datasets (or dataset distributions) against specific quality metrics.
    QualityMeasurementDataset, "QualityMeasurementDataset",
    /// `Quality Metadata`: Represents quality metadata, it is defined to group quality certificates, policies, measurements and annotations under a named graph.
    QualityMetadata, "QualityMetadata",
    /// `Quality Policy`: Represents a policy or agreement that is chiefly governed by data quality concerns.
    QualityPolicy, "QualityPolicy",
    /// `User Quality feedback`: Represents feedback that users have on the quality of datasets or distributions. Besides dqv:qualityAssessment, which is the motivation required by all quality annotations, one of the predefined instances of oa:Motivation should be indicated as motivation to distinguish among the different kinds of feedback, e.g., classifications, questions.
    UserQualityFeedback, "UserQualityFeedback",
    /// `computed on`: Refers to the resource (e.g., a dataset, a linkset, a graph, a set of triples) on which the quality measurement is performed. In the DQV context, this property is generally expected to be used in statements in which objects are instances of dcat:Dataset or dcat:Distribution.
    computedOn, "computedOn",
    /// `expected data type`: Represents the expected data type for metric's observed value (e.g. xsd:boolean, xsd:double etc...)
    expectedDataType, "expectedDataType",
    /// `has quality annotation`: Refers to a quality annotation. Quality annotation can be applied to any kind of resource, e.g., a dataset, a linkset, a graph, a set of triples. However, in the DQV context, this property is generally expected to be used in statements in which subjects are instances of dcat:Dataset or dcat:Distribution.
    hasQualityAnnotation, "hasQualityAnnotation",
    /// `has quality measurement`: Refers to the performed quality measurements. Quality measurements can be performed to any kind of resource (e.g., a dataset, a linkset, a graph, a set of triples). However, in the DQV context, this property is generally expected to be used in statements in which subjects are instances of dcat:Dataset or dcat:Distribution.
    hasQualityMeasurement, "hasQualityMeasurement",
    /// `has quality metadata`: Refers to a grouping of quality information such as certificates, policies, measurements and annotations as a named graph. Quality information represented in such a grouping can pertain to any kind of resource (e.g., a dataset, a linkset, a graph, a set of triples). However, in the DQV context, this property is generally expected to be used in statements in which subjects are instances of dcat:Dataset or dcat:Distribution.
    hasQualityMetadata, "hasQualityMetadata",
    /// `in category`: Represents the category a dimension is grouped in.
    inCategory, "inCategory",
    /// `in dimension`: Represents the dimensions a quality metric, certificate and annotation allow a measurement of.
    inDimension, "inDimension",
    /// `is measurement of`: Indicates the metric being observed.
    isMeasurementOf, "isMeasurementOf",
    /// `Precision`: Precision is a quality dimension, which refers to the recorded level of details. It represents the exactness of a measurement or description. It is equivalent the notion of Precision from ISO 25012.
    precision, "precision",
    /// `Quality assessment`: Motivation that must be specified for quality annotations.
    qualityAssessment, "qualityAssessment",
    /// `value`: Refers to values computed by metric.
    value, "value"
);
