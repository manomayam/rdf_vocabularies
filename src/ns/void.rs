// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `void` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|void|
//! |**Namespace base IRI**|[http://rdfs.org/ns/void#](http://rdfs.org/ns/void#)|
//! |**Description**||
//! |**Is defined by**|[http://vocab.deri.ie/void.ttl](http://vocab.deri.ie/void.ttl)|
//!

use crate::namespace;

namespace!(
    "http://rdfs.org/ns/void#",;
    /// `dataset`: A set of RDF triples that are published, maintained or aggregated by a single provider.
    Dataset, "Dataset",
    /// `dataset description`: A web resource whose foaf:primaryTopic or foaf:topics include void:Datasets.
    DatasetDescription, "DatasetDescription",
    /// `linkset`: A collection of RDF links between two void:Datasets.
    Linkset, "Linkset",
    /// `technical feature`: A technical feature of a void:Dataset, such as a supported RDF serialization format.
    TechnicalFeature, "TechnicalFeature",
    /// `class`: The rdfs:Class that is the rdf:type of all entities in a class-based partition.
    class, "class",
    /// `class partition`: A subset of a void:Dataset that contains only the entities of a certain rdfs:Class.
    classPartition, "classPartition",
    /// `classes`: The total number of distinct classes in a void:Dataset. In other words, the number of distinct resources occuring as objects of rdf:type triples in the dataset.
    classes, "classes",
    /// `Data Dump`: An RDF dump, partial or complete, of a void:Dataset.
    dataDump, "dataDump",
    /// `distinct objects`: The total number of distinct objects in a void:Dataset. In other words, the number of distinct resources that occur in the object position of triples in the dataset. Literals are included in this count.
    distinctObjects, "distinctObjects",
    /// `distinct subjects`: The total number of distinct subjects in a void:Dataset. In other words, the number of distinct resources that occur in the subject position of triples in the dataset.
    distinctSubjects, "distinctSubjects",
    /// `number of documents`: The total number of documents, for datasets that are published as a set of individual documents, such as RDF/XML documents or RDFa-annotated web pages. Non-RDF documents, such as web pages in HTML or images, are usually not included in this count. This property is intended for datasets where the total number of triples or entities is hard to determine. void:triples or void:entities should be preferred where practical.
    documents, "documents",
    /// `number of entities`: The total number of entities that are described in a void:Dataset.
    entities, "entities",
    /// `example resource of dataset`: 
    exampleResource, "exampleResource",
    /// `feature`: 
    feature, "feature",
    /// `in dataset`: Points to the void:Dataset that a document is a part of.
    inDataset, "inDataset",
    /// `a link predicate`: 
    linkPredicate, "linkPredicate",
    /// `Objects Target`: The dataset describing the objects of the triples contained in the Linkset.
    objectsTarget, "objectsTarget",
    /// `open search description`: An OpenSearch description document for a free-text search service over a void:Dataset.
    openSearchDescription, "openSearchDescription",
    /// `number of properties`: The total number of distinct properties in a void:Dataset. In other words, the number of distinct resources that occur in the predicate position of triples in the dataset.
    properties, "properties",
    /// `property`: The rdf:Property that is the predicate of all triples in a property-based partition.
    property, "property",
    /// `property partition`: A subset of a void:Dataset that contains only the triples of a certain rdf:Property.
    propertyPartition, "propertyPartition",
    /// `root resource`: A top concept or entry point for a void:Dataset that is structured in a tree-like fashion. All resources in a dataset can be reached by following links from its root resources in a small number of steps.
    rootResource, "rootResource",
    /// `has a SPARQL endpoint at`: 
    sparqlEndpoint, "sparqlEndpoint",
    /// `Subjects Target`: The dataset describing the subjects of triples contained in the Linkset.
    subjectsTarget, "subjectsTarget",
    /// `has subset`: 
    subset, "subset",
    /// `Target`: One of the two datasets linked by the Linkset.
    target, "target",
    /// `number of triples`: The total number of triples contained in a void:Dataset.
    triples, "triples",
    /// `has an URI look-up endpoint at`: Defines a simple URI look-up protocol for accessing a dataset.
    uriLookupEndpoint, "uriLookupEndpoint",
    /// `has URI regular expression pattern`: Defines a regular expression pattern matching URIs in the dataset.
    uriRegexPattern, "uriRegexPattern",
    /// `URI space`: A URI that is a common string prefix of all the entity URIs in a void:Dataset.
    uriSpace, "uriSpace",
    /// `vocabulary`: A vocabulary that is used in the dataset.
    vocabulary, "vocabulary"
);
