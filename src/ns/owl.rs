// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The OWL 2 Schema vocabulary (OWL 2)` vocabulary
//!
//! This module requires `ns-owl` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The OWL 2 Schema vocabulary (OWL 2)|
//! |**Prefix**|owl|
//! |**Namespace base IRI**|<http://www.w3.org/2002/07/owl#>|
//! |**Description**|This ontology partially describes the built-in classes and   properties that together form the basis of the RDF/XML syntax of OWL 2.   The content of this ontology is based on Tables 6.1 and 6.2   in Section 6.4 of the OWL 2 RDF-Based Semantics specification,   available at http://www.w3.org/TR/owl2-rdf-based-semantics/.   Please note that those tables do not include the different annotations   (labels, comments and rdfs:isDefinedBy links) used in this file.   Also note that the descriptions provided in this ontology do not   provide a complete and correct formal description of either the syntax   or the semantics of the introduced terms (please see the OWL 2   recommendations for the complete and normative specifications).   Furthermore, the information provided by this ontology may be   misleading if not used with care. This ontology SHOULD NOT be imported   into OWL ontologies. Importing this file into an OWL 2 DL ontology   will cause it to become an OWL 2 Full ontology and may have other,   unexpected, consequences.|
//! |**Is defined by**|<http://www.w3.org/2002/07/owl#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/2002/07/owl#",

    terms: [
        /// `AllDifferent`: The class of collections of pairwise different individuals.
        (AllDifferent, "AllDifferent"),
        /// `AllDisjointClasses`: The class of collections of pairwise disjoint classes.
        (AllDisjointClasses, "AllDisjointClasses"),
        /// `AllDisjointProperties`: The class of collections of pairwise disjoint properties.
        (AllDisjointProperties, "AllDisjointProperties"),
        /// `Annotation`: The class of annotated annotations for which the RDF serialization consists of an annotated subject, predicate and object.
        (Annotation, "Annotation"),
        /// `AnnotationProperty`: The class of annotation properties.
        (AnnotationProperty, "AnnotationProperty"),
        /// `AsymmetricProperty`: The class of asymmetric properties.
        (AsymmetricProperty, "AsymmetricProperty"),
        /// `Axiom`: The class of annotated axioms for which the RDF serialization consists of an annotated subject, predicate and object.
        (Axiom, "Axiom"),
        /// `Class`: The class of OWL classes.
        (Class, "Class"),
        /// `DataRange`: The class of OWL data ranges, which are special kinds of datatypes. Note: The use of the IRI owl:DataRange has been deprecated as of OWL 2. The IRI rdfs:Datatype SHOULD be used instead.
        (DataRange, "DataRange"),
        /// `DatatypeProperty`: The class of data properties.
        (DatatypeProperty, "DatatypeProperty"),
        /// `DeprecatedClass`: The class of deprecated classes.
        (DeprecatedClass, "DeprecatedClass"),
        /// `DeprecatedProperty`: The class of deprecated properties.
        (DeprecatedProperty, "DeprecatedProperty"),
        /// `FunctionalProperty`: The class of functional properties.
        (FunctionalProperty, "FunctionalProperty"),
        /// `InverseFunctionalProperty`: The class of inverse-functional properties.
        (InverseFunctionalProperty, "InverseFunctionalProperty"),
        /// `IrreflexiveProperty`: The class of irreflexive properties.
        (IrreflexiveProperty, "IrreflexiveProperty"),
        /// `NamedIndividual`: The class of named individuals.
        (NamedIndividual, "NamedIndividual"),
        /// `NegativePropertyAssertion`: The class of negative property assertions.
        (NegativePropertyAssertion, "NegativePropertyAssertion"),
        /// `Nothing`: This is the empty class.
        (Nothing, "Nothing"),
        /// `ObjectProperty`: The class of object properties.
        (ObjectProperty, "ObjectProperty"),
        /// `Ontology`: The class of ontologies.
        (Ontology, "Ontology"),
        /// `OntologyProperty`: The class of ontology properties.
        (OntologyProperty, "OntologyProperty"),
        /// `ReflexiveProperty`: The class of reflexive properties.
        (ReflexiveProperty, "ReflexiveProperty"),
        /// `Restriction`: The class of property restrictions.
        (Restriction, "Restriction"),
        /// `SymmetricProperty`: The class of symmetric properties.
        (SymmetricProperty, "SymmetricProperty"),
        /// `Thing`: The class of OWL individuals.
        (Thing, "Thing"),
        /// `TransitiveProperty`: The class of transitive properties.
        (TransitiveProperty, "TransitiveProperty"),
        /// `allValuesFrom`: The property that determines the class that a universal property restriction refers to.
        (allValuesFrom, "allValuesFrom"),
        /// `annotatedProperty`: The property that determines the predicate of an annotated axiom or annotated annotation.
        (annotatedProperty, "annotatedProperty"),
        /// `annotatedSource`: The property that determines the subject of an annotated axiom or annotated annotation.
        (annotatedSource, "annotatedSource"),
        /// `annotatedTarget`: The property that determines the object of an annotated axiom or annotated annotation.
        (annotatedTarget, "annotatedTarget"),
        /// `assertionProperty`: The property that determines the predicate of a negative property assertion.
        (assertionProperty, "assertionProperty"),
        /// `backwardCompatibleWith`: The annotation property that indicates that a given ontology is backward compatible with another ontology.
        (backwardCompatibleWith, "backwardCompatibleWith"),
        /// `bottomDataProperty`: The data property that does not relate any individual to any data value.
        (bottomDataProperty, "bottomDataProperty"),
        /// `bottomObjectProperty`: The object property that does not relate any two individuals.
        (bottomObjectProperty, "bottomObjectProperty"),
        /// `cardinality`: The property that determines the cardinality of an exact cardinality restriction.
        (cardinality, "cardinality"),
        /// `complementOf`: The property that determines that a given class is the complement of another class.
        (complementOf, "complementOf"),
        /// `datatypeComplementOf`: The property that determines that a given data range is the complement of another data range with respect to the data domain.
        (datatypeComplementOf, "datatypeComplementOf"),
        /// `deprecated`: The annotation property that indicates that a given entity has been deprecated.
        (deprecated, "deprecated"),
        /// `differentFrom`: The property that determines that two given individuals are different.
        (differentFrom, "differentFrom"),
        /// `disjointUnionOf`: The property that determines that a given class is equivalent to the disjoint union of a collection of other classes.
        (disjointUnionOf, "disjointUnionOf"),
        /// `disjointWith`: The property that determines that two given classes are disjoint.
        (disjointWith, "disjointWith"),
        /// `distinctMembers`: The property that determines the collection of pairwise different individuals in a owl:AllDifferent axiom.
        (distinctMembers, "distinctMembers"),
        /// `equivalentClass`: The property that determines that two given classes are equivalent, and that is used to specify datatype definitions.
        (equivalentClass, "equivalentClass"),
        /// `equivalentProperty`: The property that determines that two given properties are equivalent.
        (equivalentProperty, "equivalentProperty"),
        /// `hasKey`: The property that determines the collection of properties that jointly build a key.
        (hasKey, "hasKey"),
        /// `hasSelf`: The property that determines the property that a self restriction refers to.
        (hasSelf, "hasSelf"),
        /// `hasValue`: The property that determines the individual that a has-value restriction refers to.
        (hasValue, "hasValue"),
        /// `imports`: The property that is used for importing other ontologies into a given ontology.
        (imports, "imports"),
        /// `incompatibleWith`: The annotation property that indicates that a given ontology is incompatible with another ontology.
        (incompatibleWith, "incompatibleWith"),
        /// `intersectionOf`: The property that determines the collection of classes or data ranges that build an intersection.
        (intersectionOf, "intersectionOf"),
        /// `inverseOf`: The property that determines that two given properties are inverse.
        (inverseOf, "inverseOf"),
        /// `maxCardinality`: The property that determines the cardinality of a maximum cardinality restriction.
        (maxCardinality, "maxCardinality"),
        /// `maxQualifiedCardinality`: The property that determines the cardinality of a maximum qualified cardinality restriction.
        (maxQualifiedCardinality, "maxQualifiedCardinality"),
        /// `members`: The property that determines the collection of members in either a owl:AllDifferent, owl:AllDisjointClasses or owl:AllDisjointProperties axiom.
        (members, "members"),
        /// `minCardinality`: The property that determines the cardinality of a minimum cardinality restriction.
        (minCardinality, "minCardinality"),
        /// `minQualifiedCardinality`: The property that determines the cardinality of a minimum qualified cardinality restriction.
        (minQualifiedCardinality, "minQualifiedCardinality"),
        /// `onClass`: The property that determines the class that a qualified object cardinality restriction refers to.
        (onClass, "onClass"),
        /// `onDataRange`: The property that determines the data range that a qualified data cardinality restriction refers to.
        (onDataRange, "onDataRange"),
        /// `onDatatype`: The property that determines the datatype that a datatype restriction refers to.
        (onDatatype, "onDatatype"),
        /// `onProperties`: The property that determines the n-tuple of properties that a property restriction on an n-ary data range refers to.
        (onProperties, "onProperties"),
        /// `onProperty`: The property that determines the property that a property restriction refers to.
        (onProperty, "onProperty"),
        /// `oneOf`: The property that determines the collection of individuals or data values that build an enumeration.
        (oneOf, "oneOf"),
        /// `priorVersion`: The annotation property that indicates the predecessor ontology of a given ontology.
        (priorVersion, "priorVersion"),
        /// `propertyChainAxiom`: The property that determines the n-tuple of properties that build a sub property chain of a given property.
        (propertyChainAxiom, "propertyChainAxiom"),
        /// `propertyDisjointWith`: The property that determines that two given properties are disjoint.
        (propertyDisjointWith, "propertyDisjointWith"),
        /// `qualifiedCardinality`: The property that determines the cardinality of an exact qualified cardinality restriction.
        (qualifiedCardinality, "qualifiedCardinality"),
        /// `sameAs`: The property that determines that two given individuals are equal.
        (sameAs, "sameAs"),
        /// `someValuesFrom`: The property that determines the class that an existential property restriction refers to.
        (someValuesFrom, "someValuesFrom"),
        /// `sourceIndividual`: The property that determines the subject of a negative property assertion.
        (sourceIndividual, "sourceIndividual"),
        /// `targetIndividual`: The property that determines the object of a negative object property assertion.
        (targetIndividual, "targetIndividual"),
        /// `targetValue`: The property that determines the value of a negative data property assertion.
        (targetValue, "targetValue"),
        /// `topDataProperty`: The data property that relates every individual to every data value.
        (topDataProperty, "topDataProperty"),
        /// `topObjectProperty`: The object property that relates every two individuals.
        (topObjectProperty, "topObjectProperty"),
        /// `unionOf`: The property that determines the collection of classes or data ranges that build a union.
        (unionOf, "unionOf"),
        /// `versionIRI`: The property that identifies the version IRI of an ontology.
        (versionIRI, "versionIRI"),
        /// `versionInfo`: The annotation property that provides version information for an ontology or another OWL construct.
        (versionInfo, "versionInfo"),
        /// `withRestrictions`: The property that determines the collection of facet-value pairs that define a datatype restriction.
        (withRestrictions, "withRestrictions")    ]
);
