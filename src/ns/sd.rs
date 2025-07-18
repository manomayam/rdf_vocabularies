// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `sd` vocabulary
//!
//! This module requires `ns-sd` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|sd|
//! |**Namespace base IRI**|<http://www.w3.org/ns/sparql-service-description#>|
//! |**Description**||
//! |**Is defined by**|<http://www.w3.org/ns/sparql-service-description#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/ns/sparql-service-description#",

    terms: [
        /// `Aggregate`: An instance of sd:Aggregate represents an aggregate that may be used in a SPARQL aggregate query (for instance in a HAVING clause or SELECT expression) besides the standard list of supported aggregates COUNT, SUM, MIN, MAX, AVG, GROUP_CONCAT, and SAMPLE.
        (Aggregate, "Aggregate"),
        /// `Basic Federated Query`: sd:BasicFederatedQuery, when used as the object of the sd:feature property, indicates that the SPARQL service supports basic federated query using the SERVICE keyword as defined by SPARQL 1.1 Federation Extensions.
        (BasicFederatedQuery, "BasicFederatedQuery"),
        /// `Dataset`: An instance of sd:Dataset represents a RDF Dataset comprised of a default graph and zero or more named graphs.
        (Dataset, "Dataset"),
        /// `Dereferences URIs`: sd:DereferencesURIs, when used as the object of the sd:feature property, indicates that a SPARQL service will dereference URIs used in FROM/FROM NAMED and USING/USING NAMED clauses and use the resulting RDF in the dataset during query evaluation.
        (DereferencesURIs, "DereferencesURIs"),
        /// `Empty Graphs`: sd:EmptyGraphs, when used as the object of the sd:feature property, indicates that the underlying graph store supports empty graphs. A graph store that supports empty graphs MUST NOT remove graphs that are left empty after triples are removed from them.
        (EmptyGraphs, "EmptyGraphs"),
        /// `Entailment Profile`: An instance of sd:EntailmentProfile represents a profile of an entailment regime. An entailment profile MAY impose restrictions on what constitutes valid RDF with respect to entailment.
        (EntailmentProfile, "EntailmentProfile"),
        /// `Entailment Regime`: An instance of sd:EntailmentRegime represents an entailment regime used in basic graph pattern matching (as described by SPARQL 1.1 Query Language).
        (EntailmentRegime, "EntailmentRegime"),
        /// `Feature`: An instance of sd:Feature represents a feature of a SPARQL service. Specific types of features include functions, aggregates, languages, and entailment regimes and profiles. This document defines five instances of sd:Feature: sd:DereferencesURIs, sd:UnionDefaultGraph, sd:RequiresDataset, sd:EmptyGraphs, and sd:BasicFederatedQuery.
        (Feature, "Feature"),
        /// `Function`: An instance of sd:Function represents a function that may be used in a SPARQL SELECT expression or a FILTER, HAVING, GROUP BY, ORDER BY, or BIND clause.
        (Function, "Function"),
        /// `Graph`: An instance of sd:Graph represents the description of an RDF graph.
        (Graph, "Graph"),
        /// `Graph Collection`: An instance of sd:GraphCollection represents a collection of zero or more named graph descriptions. Each named graph description belonging to an sd:GraphCollection MUST be linked with the sd:namedGraph predicate.
        (GraphCollection, "GraphCollection"),
        /// `Language`: An instance of sd:Language represents one of the SPARQL languages, including specific configurations providing particular features or extensions. This document defines three instances of sd:Language: sd:SPARQL10Query, sd:SPARQL11Query, and sd:SPARQL11Update.
        (Language, "Language"),
        /// `Named Graph`: An instance of sd:NamedGraph represents a named graph having a name (via sd:name) and an optional graph description (via sd:graph).
        (NamedGraph, "NamedGraph"),
        /// `Requires Dataset`: sd:RequiresDataset, when used as the object of the sd:feature property, indicates that the SPARQL service requires an explicit dataset declaration (based on either FROM/FROM NAMED clauses in a query, USING/USING NAMED clauses in an update, or the appropriate SPARQL Protocol parameters).
        (RequiresDataset, "RequiresDataset"),
        /// `SPARQL 1.0 Query`: sd:SPARQL10Query is an sd:Language representing the SPARQL 1.0 Query language.
        (SPARQL10Query, "SPARQL10Query"),
        /// `SPARQL 1.1 Query`: sd:SPARQL11Query is an sd:Language representing the SPARQL 1.1 Query language.
        (SPARQL11Query, "SPARQL11Query"),
        /// `SPARQL 1.1 Update`: sd:SPARQLUpdate is an sd:Language representing the SPARQL 1.1 Update language.
        (SPARQL11Update, "SPARQL11Update"),
        /// `Service`: An instance of sd:Service represents a SPARQL service made available via the SPARQL Protocol.
        (Service, "Service"),
        /// `Union Default Graph`: sd:UnionDefaultGraph, when used as the object of the sd:feature property, indicates that the default graph of the dataset used during query and update evaluation (when an explicit dataset is not specified) is comprised of the union of all the named graphs in that dataset.
        (UnionDefaultGraph, "UnionDefaultGraph"),
        /// `available graph descriptions`: Relates an instance of sd:Service to a description of the graphs which are allowed in the construction of a dataset either via the SPARQL Protocol, with FROM/FROM NAMED clauses in a query, or with USING/USING NAMED in an update request, if the service limits the scope of dataset construction.
        (availableGraphs, "availableGraphs"),
        /// `default dataset description`: Relates an instance of sd:Service to a description of the default dataset available when no explicit dataset is specified in the query, update request or via protocol parameters.
        (defaultDataset, "defaultDataset"),
        /// `default entailment regime`: Relates an instance of sd:Service with a resource representing an entailment regime used for basic graph pattern matching. This property is intended for use when a single entailment regime by default applies to all graphs in the default dataset of the service. In situations where a different entailment regime applies to a specific graph in the dataset, the sd:entailmentRegime property should be used to indicate this fact in the description of that graph.
        (defaultEntailmentRegime, "defaultEntailmentRegime"),
        /// `default graph`: Relates an instance of sd:Dataset to the description of its default graph.
        (defaultGraph, "defaultGraph"),
        /// `default supported entailment profile`: Relates an instance of sd:Service with a resource representing a supported profile of the default entailment regime (as declared by sd:defaultEntailmentRegime).
        (defaultSupportedEntailmentProfile, "defaultSupportedEntailmentProfile"),
        /// `endpoint`: The SPARQL endpoint of an sd:Service that implements the SPARQL Protocol service. The object of the sd:endpoint property is an IRI.
        (endpoint, "endpoint"),
        /// `entailment regime`: Relates a named graph description with a resource representing an entailment regime used for basic graph pattern matching over that graph.
        (entailmentRegime, "entailmentRegime"),
        /// `extension aggregate`: Relates an instance of sd:Service to an aggregate that may be used in a SPARQL aggregate query (for instance in a HAVING clause or SELECT expression) besides the standard list of supported aggregates COUNT, SUM, MIN, MAX, AVG, GROUP_CONCAT, and SAMPLE
        (extensionAggregate, "extensionAggregate"),
        /// `extension function`: Relates an instance of sd:Service to a function that may be used in a SPARQL SELECT expression or a FILTER, HAVING, GROUP BY, ORDER BY, or BIND clause.
        (extensionFunction, "extensionFunction"),
        /// `feature`: Relates an instance of sd:Service with a resource representing a supported feature.
        (feature, "feature"),
        /// `graph`: Relates a named graph to its graph description.
        (graph, "graph"),
        /// `input format`: Relates an instance of sd:Service to a format that is supported for parsing RDF input; for example, via a SPARQL 1.1 Update LOAD statement, or when URIs are dereferenced in FROM/FROM NAMED/USING/USING NAMED clauses.
        (inputFormat, "inputFormat"),
        /// `language extension`: Relates an instance of sd:Service to a resource representing an implemented extension to the SPARQL Query or Update language.
        (languageExtension, "languageExtension"),
        /// `name`: Relates a named graph to the name by which it may be referenced in a FROM/FROM NAMED clause. The object of the sd:name property is an IRI.
        (name, "name"),
        /// `named graph`: Relates an instance of sd:GraphCollection (or its subclass sd:Dataset) to the description of one of its named graphs. The description of such a named graph MUST include the sd:name property and MAY include the sd:graph property.
        (namedGraph, "namedGraph"),
        /// `property feature`: Relates an instance of sd:Service to a resource representing an implemented feature that extends the SPARQL Query or Update language and that is accessed by using the named property.
        (propertyFeature, "propertyFeature"),
        /// `result format`: Relates an instance of sd:Service to a format that is supported for serializing query results.
        (resultFormat, "resultFormat"),
        /// `supported entailment profile`: Relates a named graph description with a resource representing a supported profile of the entailment regime (as declared by sd:entailmentRegime) used for basic graph pattern matching over that graph.
        (supportedEntailmentProfile, "supportedEntailmentProfile"),
        /// `supported language`: Relates an instance of sd:Service to a SPARQL language (e.g. Query and Update) that it implements.
        (supportedLanguage, "supportedLanguage")    ]
);
