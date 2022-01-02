// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The OWL 2 Schema vocabulary (OWL 2)` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The OWL 2 Schema vocabulary (OWL 2)|
//! |**Prefix**|owl|
//! |**Namespace base IRI**|[http://www.w3.org/2002/07/owl#](http://www.w3.org/2002/07/owl#)|
//! |**Description**|This ontology partially describes the built-in classes and   properties that together form the basis of the RDF/XML syntax of OWL 2.   The content of this ontology is based on Tables 6.1 and 6.2   in Section 6.4 of the OWL 2 RDF-Based Semantics specification,   available at http://www.w3.org/TR/owl2-rdf-based-semantics/.   Please note that those tables do not include the different annotations   (labels, comments and rdfs:isDefinedBy links) used in this file.   Also note that the descriptions provided in this ontology do not   provide a complete and correct formal description of either the syntax   or the semantics of the introduced terms (please see the OWL 2   recommendations for the complete and normative specifications).   Furthermore, the information provided by this ontology may be   misleading if not used with care. This ontology SHOULD NOT be imported   into OWL ontologies. Importing this file into an OWL 2 DL ontology   will cause it to become an OWL 2 Full ontology and may have other,   unexpected, consequences.|
//! |**Is defined by**|[http://www.w3.org/2002/07/owl#](http://www.w3.org/2002/07/owl#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2002/07/owl#",;
    AllDifferent, "AllDifferent",
    AllDisjointClasses, "AllDisjointClasses",
    AllDisjointProperties, "AllDisjointProperties",
    Annotation, "Annotation",
    AnnotationProperty, "AnnotationProperty",
    AsymmetricProperty, "AsymmetricProperty",
    Axiom, "Axiom",
    Class, "Class",
    DataRange, "DataRange",
    DatatypeProperty, "DatatypeProperty",
    DeprecatedClass, "DeprecatedClass",
    DeprecatedProperty, "DeprecatedProperty",
    FunctionalProperty, "FunctionalProperty",
    InverseFunctionalProperty, "InverseFunctionalProperty",
    IrreflexiveProperty, "IrreflexiveProperty",
    NamedIndividual, "NamedIndividual",
    NegativePropertyAssertion, "NegativePropertyAssertion",
    Nothing, "Nothing",
    ObjectProperty, "ObjectProperty",
    Ontology, "Ontology",
    OntologyProperty, "OntologyProperty",
    ReflexiveProperty, "ReflexiveProperty",
    Restriction, "Restriction",
    SymmetricProperty, "SymmetricProperty",
    Thing, "Thing",
    TransitiveProperty, "TransitiveProperty",
    allValuesFrom, "allValuesFrom",
    annotatedProperty, "annotatedProperty",
    annotatedSource, "annotatedSource",
    annotatedTarget, "annotatedTarget",
    assertionProperty, "assertionProperty",
    backwardCompatibleWith, "backwardCompatibleWith",
    bottomDataProperty, "bottomDataProperty",
    bottomObjectProperty, "bottomObjectProperty",
    cardinality, "cardinality",
    complementOf, "complementOf",
    datatypeComplementOf, "datatypeComplementOf",
    deprecated, "deprecated",
    differentFrom, "differentFrom",
    disjointUnionOf, "disjointUnionOf",
    disjointWith, "disjointWith",
    distinctMembers, "distinctMembers",
    equivalentClass, "equivalentClass",
    equivalentProperty, "equivalentProperty",
    hasKey, "hasKey",
    hasSelf, "hasSelf",
    hasValue, "hasValue",
    imports, "imports",
    incompatibleWith, "incompatibleWith",
    intersectionOf, "intersectionOf",
    inverseOf, "inverseOf",
    maxCardinality, "maxCardinality",
    maxQualifiedCardinality, "maxQualifiedCardinality",
    members, "members",
    minCardinality, "minCardinality",
    minQualifiedCardinality, "minQualifiedCardinality",
    onClass, "onClass",
    onDataRange, "onDataRange",
    onDatatype, "onDatatype",
    onProperties, "onProperties",
    onProperty, "onProperty",
    oneOf, "oneOf",
    priorVersion, "priorVersion",
    propertyChainAxiom, "propertyChainAxiom",
    propertyDisjointWith, "propertyDisjointWith",
    qualifiedCardinality, "qualifiedCardinality",
    sameAs, "sameAs",
    someValuesFrom, "someValuesFrom",
    sourceIndividual, "sourceIndividual",
    targetIndividual, "targetIndividual",
    targetValue, "targetValue",
    topDataProperty, "topDataProperty",
    topObjectProperty, "topObjectProperty",
    unionOf, "unionOf",
    versionIRI, "versionIRI",
    versionInfo, "versionInfo",
    withRestrictions, "withRestrictions"
);
