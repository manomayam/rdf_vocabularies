// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `R2RML vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|R2RML vocabulary|
//! |**Prefix**|rr|
//! |**Namespace base IRI**|[http://www.w3.org/ns/r2rml#](http://www.w3.org/ns/r2rml#)|
//! |**Description**|RDB to RDF Mapping Language - Vocabulary|
//! |**Is defined by**|[http://www.w3.org/ns/r2rml#](http://www.w3.org/ns/r2rml#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/r2rml#",;
    /// ``: 
    BaseTableOrView, "BaseTableOrView",
    /// ``: Denotes a blank node, used with termType
    BlankNode, "BlankNode",
    /// ``: Represents a graph map.
    GraphMap, "GraphMap",
    /// ``: Denotes an IRI, used with termpType.
    IRI, "IRI",
    /// ``: Represents a join condition.
    Join, "Join",
    /// ``: Denotes a Literal, used with termType.
    Literal, "Literal",
    /// ``: Represents a logical table.
    LogicalTable, "LogicalTable",
    /// ``: Represents an object map.
    ObjectMap, "ObjectMap",
    /// ``: Represents a predicate map.
    PredicateMap, "PredicateMap",
    /// ``: Represents a predicate-object map.
    PredicateObjectMap, "PredicateObjectMap",
    /// ``: 
    R2RMLView, "R2RMLView",
    /// ``: Denotes a reference to an object map.
    RefObjectMap, "RefObjectMap",
    /// ``: Core SQL 2008
    SQL2008, "SQL2008",
    /// ``: Represents a subject map.
    SubjectMap, "SubjectMap",
    /// `Term Map`: A function that generates an RDF term from a logical table row.
    TermMap, "TermMap",
    /// ``: Represents a triples map.
    TriplesMap, "TriplesMap",
    /// ``: Names a column in the child table of a join.
    child, "child",
    /// ``: The subject value generated for a logical table row will be asserted as an instance of this RDFS class.
    class, "class",
    /// ``: Name of a column in the logical table. When generating RDF triples from a logical table row, value from the specified column is used as the subject, predicate, or object (based upon the specific domain).
    column, "column",
    /// ``: 
    constant, "constant",
    /// ``: Specifies the datatype of the object component for the generated triple from a logical table row.
    datatype, "datatype",
    /// ``: Denotes a default graph
    defaultGraph, "defaultGraph",
    /// ``: An IRI reference for use as the graph name of all triples generated with the GraphMap.
    graph, "graph",
    /// ``: Specifies a GraphMap. When used with a SubjectMap element, all the RDF triples generated from a logical row will be stored in the specified named graph. Otherwise, the RDF triple generated using the (predicate, object) pair will be stored in the specified named graph.
    graphMap, "graphMap",
    /// ``: An expression that allows, at query processing time, use of index-based access to the the (underlying) relational tables, instead of simply retrieving the table rows first and then applying a filter. This property is useful for retrieval based on conditions involving subject, predicate, or object generated from logical table column(s) and involves some transformation.
    inverseExpression, "inverseExpression",
    /// ``: Specifies the join condition for joining the child logical table with the parent logical table of the foreign key constraint.
    joinCondition, "joinCondition",
    /// ``: Specified the language for the object component for the generated triple from a logical table row.
    language, "language",
    /// ``: Definition of logical table to be mapped.
    logicalTable, "logicalTable",
    /// ``: Specifies the object for the generated triple from the logical table row.
    object, "object",
    /// ``: An ObjectMap element to generate the object component of the (predicate, object) pair from a logical table row.
    objectMap, "objectMap",
    /// ``: Names a column in the parent table of a join.
    parent, "parent",
    /// ``: Specifies the TriplesMap element corresponding to the parent logical table of the foreign key constraint.
    parentTriplesMap, "parentTriplesMap",
    /// ``: Specifies the predicate for the generated triple from the logical table row.
    predicate, "predicate",
    /// ``: A PredicateMap element to generate the predicate component of the (predicate, object) pair from a logical table row.
    predicateMap, "predicateMap",
    /// ``: A PredicateObjectMap element to generate (predicate, object) pair from a logical table row.
    predicateObjectMap, "predicateObjectMap",
    /// ``: A valid SQL query.
    sqlQuery, "sqlQuery",
    /// ``: An identifier for a SQL version.
    sqlVersion, "sqlVersion",
    /// ``: An IRI reference for use as subject for all the RDF triples generated from a logical table row.
    subject, "subject",
    /// ``: A SubjectMap element to generate a subject from a logical table row.
    subjectMap, "subjectMap",
    /// ``: Schema-qualified name of a table or view.
    tableName, "tableName",
    /// ``: A template (format string) to specify how to generate a value for a subject, predicate, or object, using one or more columns from a logical table row.
    template, "template",
    /// ``: A string indicating whether subject or object generated using the value from column name specified for rr:column should be an IRI reference, blank node, or a literal.
    termType, "termType"
);
