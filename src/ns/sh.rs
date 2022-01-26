// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `W3C Shapes Constraint Language (SHACL) Vocabulary` vocabulary
//!
//! This module requires `ns-sh` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|W3C Shapes Constraint Language (SHACL) Vocabulary|
//! |**Prefix**|sh|
//! |**Namespace base IRI**|<http://www.w3.org/ns/shacl#>|
//! |**Description**|This vocabulary defines terms used in SHACL, the W3C Shapes Constraint Language.|
//! |**Is defined by**|<http://www.w3.org/ns/shacl#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/shacl#",;
    /// `W3C Shapes Constraint Language (SHACL) Vocabulary`: This vocabulary defines terms used in SHACL, the W3C Shapes Constraint Language.
    NAMESPACE_BASE, "",
    /// `Abstract result`: The base class of validation results, typically not instantiated directly.
    AbstractResult, "AbstractResult",
    /// `And constraint component`: A constraint component that can be used to test whether a value node conforms to all members of a provided list of shapes.
    AndConstraintComponent, "AndConstraintComponent",
    /// `AndConstraintComponent-and`: 
    AndConstraintComponent_and, "AndConstraintComponent-and",
    /// `Blank node`: The node kind of all blank nodes.
    BlankNode, "BlankNode",
    /// `Blank node or IRI`: The node kind of all blank nodes or IRIs.
    BlankNodeOrIRI, "BlankNodeOrIRI",
    /// `Blank node or literal`: The node kind of all blank nodes or literals.
    BlankNodeOrLiteral, "BlankNodeOrLiteral",
    /// `Class constraint component`: A constraint component that can be used to verify that each value node is an instance of a given type.
    ClassConstraintComponent, "ClassConstraintComponent",
    /// `ClassConstraintComponent-class`: 
    ClassConstraintComponent_class, "ClassConstraintComponent-class",
    /// `Closed constraint component`: A constraint component that can be used to indicate that focus nodes must only have values for those properties that have been explicitly enumerated via sh:property/sh:path.
    ClosedConstraintComponent, "ClosedConstraintComponent",
    /// `ClosedConstraintComponent-closed`: 
    ClosedConstraintComponent_closed, "ClosedConstraintComponent-closed",
    /// `ClosedConstraintComponent-ignoredProperties`: 
    ClosedConstraintComponent_ignoredProperties, "ClosedConstraintComponent-ignoredProperties",
    /// `Constraint component`: The class of constraint components.
    ConstraintComponent, "ConstraintComponent",
    /// `Datatype constraint component`: A constraint component that can be used to restrict the datatype of all value nodes.
    DatatypeConstraintComponent, "DatatypeConstraintComponent",
    /// `DatatypeConstraintComponent-datatype`: 
    DatatypeConstraintComponent_datatype, "DatatypeConstraintComponent-datatype",
    /// `Disjoint constraint component`: A constraint component that can be used to verify that the set of value nodes is disjoint with the the set of nodes that have the focus node as subject and the value of a given property as predicate.
    DisjointConstraintComponent, "DisjointConstraintComponent",
    /// `DisjointConstraintComponent-disjoint`: 
    DisjointConstraintComponent_disjoint, "DisjointConstraintComponent-disjoint",
    /// `Equals constraint component`: A constraint component that can be used to verify that the set of value nodes is equal to the set of nodes that have the focus node as subject and the value of a given property as predicate.
    EqualsConstraintComponent, "EqualsConstraintComponent",
    /// `EqualsConstraintComponent-equals`: 
    EqualsConstraintComponent_equals, "EqualsConstraintComponent-equals",
    /// `Expression constraint component`: A constraint component that can be used to verify that a given node expression produces true for all value nodes.
    ExpressionConstraintComponent, "ExpressionConstraintComponent",
    /// `ExpressionConstraintComponent-expression`: 
    ExpressionConstraintComponent_expression, "ExpressionConstraintComponent-expression",
    /// `Function`: The class of SHACL functions.
    Function, "Function",
    /// `Has-value constraint component`: A constraint component that can be used to verify that one of the value nodes is a given RDF node.
    HasValueConstraintComponent, "HasValueConstraintComponent",
    /// `HasValueConstraintComponent-hasValue`: 
    HasValueConstraintComponent_hasValue, "HasValueConstraintComponent-hasValue",
    /// `IRI`: The node kind of all IRIs.
    IRI, "IRI",
    /// `IRI or literal`: The node kind of all IRIs or literals.
    IRIOrLiteral, "IRIOrLiteral",
    /// `In constraint component`: A constraint component that can be used to exclusively enumerate the permitted value nodes.
    InConstraintComponent, "InConstraintComponent",
    /// `InConstraintComponent-in`: 
    InConstraintComponent_in, "InConstraintComponent-in",
    /// `Info`: The severity for an informational validation result.
    Info, "Info",
    /// `JavaScript-based constraint`: The class of constraints backed by a JavaScript function.
    JSConstraint, "JSConstraint",
    /// `JSConstraint-js`: 
    JSConstraint_js, "JSConstraint-js",
    /// `JavaScript constraint component`: A constraint component with the parameter sh:js linking to a sh:JSConstraint containing a sh:script.
    JSConstraintComponent, "JSConstraintComponent",
    /// `JavaScript executable`: Abstract base class of resources that declare an executable JavaScript.
    JSExecutable, "JSExecutable",
    /// `JavaScript function`: The class of SHACL functions that execute a JavaScript function when called.
    JSFunction, "JSFunction",
    /// `JavaScript library`: Represents a JavaScript library, typically identified by one or more URLs of files to include.
    JSLibrary, "JSLibrary",
    /// `JavaScript rule`: The class of SHACL rules expressed using JavaScript.
    JSRule, "JSRule",
    /// `JavaScript target`: The class of targets that are based on JavaScript functions.
    JSTarget, "JSTarget",
    /// `JavaScript target type`: The (meta) class for parameterizable targets that are based on JavaScript functions.
    JSTargetType, "JSTargetType",
    /// `JavaScript validator`: A SHACL validator based on JavaScript. This can be used to declare SHACL constraint components that perform JavaScript-based validation when used.
    JSValidator, "JSValidator",
    /// `Language-in constraint component`: A constraint component that can be used to enumerate language tags that all value nodes must have.
    LanguageInConstraintComponent, "LanguageInConstraintComponent",
    /// `LanguageInConstraintComponent-languageIn`: 
    LanguageInConstraintComponent_languageIn, "LanguageInConstraintComponent-languageIn",
    /// `Less-than constraint component`: A constraint component that can be used to verify that each value node is smaller than all the nodes that have the focus node as subject and the value of a given property as predicate.
    LessThanConstraintComponent, "LessThanConstraintComponent",
    /// `LessThanConstraintComponent-lessThan`: 
    LessThanConstraintComponent_lessThan, "LessThanConstraintComponent-lessThan",
    /// `less-than-or-equals constraint component`: A constraint component that can be used to verify that every value node is smaller than all the nodes that have the focus node as subject and the value of a given property as predicate.
    LessThanOrEqualsConstraintComponent, "LessThanOrEqualsConstraintComponent",
    /// `LessThanOrEqualsConstraintComponent-lessThanOrEquals`: 
    LessThanOrEqualsConstraintComponent_lessThanOrEquals, "LessThanOrEqualsConstraintComponent-lessThanOrEquals",
    /// `Literal`: The node kind of all literals.
    Literal, "Literal",
    /// `Max-count constraint component`: A constraint component that can be used to restrict the maximum number of value nodes.
    MaxCountConstraintComponent, "MaxCountConstraintComponent",
    /// `MaxCountConstraintComponent-maxCount`: 
    MaxCountConstraintComponent_maxCount, "MaxCountConstraintComponent-maxCount",
    /// `Max-exclusive constraint component`: A constraint component that can be used to restrict the range of value nodes with a maximum exclusive value.
    MaxExclusiveConstraintComponent, "MaxExclusiveConstraintComponent",
    /// `MaxExclusiveConstraintComponent-maxExclusive`: 
    MaxExclusiveConstraintComponent_maxExclusive, "MaxExclusiveConstraintComponent-maxExclusive",
    /// `Max-inclusive constraint component`: A constraint component that can be used to restrict the range of value nodes with a maximum inclusive value.
    MaxInclusiveConstraintComponent, "MaxInclusiveConstraintComponent",
    /// `MaxInclusiveConstraintComponent-maxInclusive`: 
    MaxInclusiveConstraintComponent_maxInclusive, "MaxInclusiveConstraintComponent-maxInclusive",
    /// `Max-length constraint component`: A constraint component that can be used to restrict the maximum string length of value nodes.
    MaxLengthConstraintComponent, "MaxLengthConstraintComponent",
    /// `MaxLengthConstraintComponent-maxLength`: 
    MaxLengthConstraintComponent_maxLength, "MaxLengthConstraintComponent-maxLength",
    /// `Min-count constraint component`: A constraint component that can be used to restrict the minimum number of value nodes.
    MinCountConstraintComponent, "MinCountConstraintComponent",
    /// `MinCountConstraintComponent-minCount`: 
    MinCountConstraintComponent_minCount, "MinCountConstraintComponent-minCount",
    /// `Min-exclusive constraint component`: A constraint component that can be used to restrict the range of value nodes with a minimum exclusive value.
    MinExclusiveConstraintComponent, "MinExclusiveConstraintComponent",
    /// `MinExclusiveConstraintComponent-minExclusive`: 
    MinExclusiveConstraintComponent_minExclusive, "MinExclusiveConstraintComponent-minExclusive",
    /// `Min-inclusive constraint component`: A constraint component that can be used to restrict the range of value nodes with a minimum inclusive value.
    MinInclusiveConstraintComponent, "MinInclusiveConstraintComponent",
    /// `MinInclusiveConstraintComponent-minInclusive`: 
    MinInclusiveConstraintComponent_minInclusive, "MinInclusiveConstraintComponent-minInclusive",
    /// `Min-length constraint component`: A constraint component that can be used to restrict the minimum string length of value nodes.
    MinLengthConstraintComponent, "MinLengthConstraintComponent",
    /// `MinLengthConstraintComponent-minLength`: 
    MinLengthConstraintComponent_minLength, "MinLengthConstraintComponent-minLength",
    /// `Node constraint component`: A constraint component that can be used to verify that all value nodes conform to the given node shape.
    NodeConstraintComponent, "NodeConstraintComponent",
    /// `NodeConstraintComponent-node`: 
    NodeConstraintComponent_node, "NodeConstraintComponent-node",
    /// `Node kind`: The class of all node kinds, including sh:BlankNode, sh:IRI, sh:Literal or the combinations of these: sh:BlankNodeOrIRI, sh:BlankNodeOrLiteral, sh:IRIOrLiteral.
    NodeKind, "NodeKind",
    /// `Node-kind constraint component`: A constraint component that can be used to restrict the RDF node kind of each value node.
    NodeKindConstraintComponent, "NodeKindConstraintComponent",
    /// `NodeKindConstraintComponent-nodeKind`: 
    NodeKindConstraintComponent_nodeKind, "NodeKindConstraintComponent-nodeKind",
    /// `Node shape`: A node shape is a shape that specifies constraint that need to be met with respect to focus nodes.
    NodeShape, "NodeShape",
    /// `Not constraint component`: A constraint component that can be used to verify that value nodes do not conform to a given shape.
    NotConstraintComponent, "NotConstraintComponent",
    /// `NotConstraintComponent-not`: 
    NotConstraintComponent_not, "NotConstraintComponent-not",
    /// `Or constraint component`: A constraint component that can be used to restrict the value nodes so that they conform to at least one out of several provided shapes.
    OrConstraintComponent, "OrConstraintComponent",
    /// `OrConstraintComponent-or`: 
    OrConstraintComponent_or, "OrConstraintComponent-or",
    /// `Parameter`: The class of parameter declarations, consisting of a path predicate and (possibly) information about allowed value type, cardinality and other characteristics.
    Parameter, "Parameter",
    /// `Parameterizable`: Superclass of components that can take parameters, especially functions and constraint components.
    Parameterizable, "Parameterizable",
    /// `Pattern constraint component`: A constraint component that can be used to verify that every value node matches a given regular expression.
    PatternConstraintComponent, "PatternConstraintComponent",
    /// `PatternConstraintComponent-flags`: 
    PatternConstraintComponent_flags, "PatternConstraintComponent-flags",
    /// `PatternConstraintComponent-pattern`: 
    PatternConstraintComponent_pattern, "PatternConstraintComponent-pattern",
    /// `Prefix declaration`: The class of prefix declarations, consisting of pairs of a prefix with a namespace.
    PrefixDeclaration, "PrefixDeclaration",
    /// `Property constraint component`: A constraint component that can be used to verify that all value nodes conform to the given property shape.
    PropertyConstraintComponent, "PropertyConstraintComponent",
    /// `PropertyConstraintComponent-property`: 
    PropertyConstraintComponent_property, "PropertyConstraintComponent-property",
    /// `Property group`: Instances of this class represent groups of property shapes that belong together.
    PropertyGroup, "PropertyGroup",
    /// `Property shape`: A property shape is a shape that specifies constraints on the values of a focus node for a given property or path.
    PropertyShape, "PropertyShape",
    /// `Qualified-max-count constraint component`: A constraint component that can be used to verify that a specified maximum number of value nodes conforms to a given shape.
    QualifiedMaxCountConstraintComponent, "QualifiedMaxCountConstraintComponent",
    /// `QualifiedMaxCountConstraintComponent-qualifiedMaxCount`: 
    QualifiedMaxCountConstraintComponent_qualifiedMaxCount, "QualifiedMaxCountConstraintComponent-qualifiedMaxCount",
    /// `QualifiedMaxCountConstraintComponent-qualifiedValueShape`: 
    QualifiedMaxCountConstraintComponent_qualifiedValueShape, "QualifiedMaxCountConstraintComponent-qualifiedValueShape",
    /// `QualifiedMaxCountConstraintComponent-qualifiedValueShapesDisjoint`: 
    QualifiedMaxCountConstraintComponent_qualifiedValueShapesDisjoint, "QualifiedMaxCountConstraintComponent-qualifiedValueShapesDisjoint",
    /// `Qualified-min-count constraint component`: A constraint component that can be used to verify that a specified minimum number of value nodes conforms to a given shape.
    QualifiedMinCountConstraintComponent, "QualifiedMinCountConstraintComponent",
    /// `QualifiedMinCountConstraintComponent-qualifiedMinCount`: 
    QualifiedMinCountConstraintComponent_qualifiedMinCount, "QualifiedMinCountConstraintComponent-qualifiedMinCount",
    /// `QualifiedMinCountConstraintComponent-qualifiedValueShape`: 
    QualifiedMinCountConstraintComponent_qualifiedValueShape, "QualifiedMinCountConstraintComponent-qualifiedValueShape",
    /// `QualifiedMinCountConstraintComponent-qualifiedValueShapesDisjoint`: 
    QualifiedMinCountConstraintComponent_qualifiedValueShapesDisjoint, "QualifiedMinCountConstraintComponent-qualifiedValueShapesDisjoint",
    /// `Result annotation`: A class of result annotations, which define the rules to derive the values of a given annotation property as extra values for a validation result.
    ResultAnnotation, "ResultAnnotation",
    /// `Rule`: The class of SHACL rules. Never instantiated directly.
    Rule, "Rule",
    /// `SPARQL ASK executable`: The class of SPARQL executables that are based on an ASK query.
    SPARQLAskExecutable, "SPARQLAskExecutable",
    /// `SPARQL ASK validator`: The class of validators based on SPARQL ASK queries. The queries are evaluated for each value node and are supposed to return true if the given node conforms.
    SPARQLAskValidator, "SPARQLAskValidator",
    /// `SPARQL constraint`: The class of constraints based on SPARQL SELECT queries.
    SPARQLConstraint, "SPARQLConstraint",
    /// `SPARQL constraint component`: A constraint component that can be used to define constraints based on SPARQL queries.
    SPARQLConstraintComponent, "SPARQLConstraintComponent",
    /// `SPARQLConstraintComponent-sparql`: 
    SPARQLConstraintComponent_sparql, "SPARQLConstraintComponent-sparql",
    /// `SPARQL CONSTRUCT executable`: The class of SPARQL executables that are based on a CONSTRUCT query.
    SPARQLConstructExecutable, "SPARQLConstructExecutable",
    /// `SPARQL executable`: The class of resources that encapsulate a SPARQL query.
    SPARQLExecutable, "SPARQLExecutable",
    /// `SPARQL function`: A function backed by a SPARQL query - either ASK or SELECT.
    SPARQLFunction, "SPARQLFunction",
    /// `SPARQL CONSTRUCT rule`: The class of SHACL rules based on SPARQL CONSTRUCT queries.
    SPARQLRule, "SPARQLRule",
    /// `SPARQL SELECT executable`: The class of SPARQL executables based on a SELECT query.
    SPARQLSelectExecutable, "SPARQLSelectExecutable",
    /// `SPARQL SELECT validator`: The class of validators based on SPARQL SELECT queries. The queries are evaluated for each focus node and are supposed to produce bindings for all focus nodes that do not conform.
    SPARQLSelectValidator, "SPARQLSelectValidator",
    /// `SPARQL target`: The class of targets that are based on SPARQL queries.
    SPARQLTarget, "SPARQLTarget",
    /// `SPARQL target type`: The (meta) class for parameterizable targets that are based on SPARQL queries.
    SPARQLTargetType, "SPARQLTargetType",
    /// `SPARQL UPDATE executable`: The class of SPARQL executables based on a SPARQL UPDATE.
    SPARQLUpdateExecutable, "SPARQLUpdateExecutable",
    /// `Severity`: The class of validation result severity levels, including violation and warning levels.
    Severity, "Severity",
    /// `Shape`: A shape is a collection of constraints that may be targeted for certain nodes.
    Shape, "Shape",
    /// `Target`: The base class of targets such as those based on SPARQL queries.
    Target, "Target",
    /// `Target type`: The (meta) class for parameterizable targets.	Instances of this are instantiated as values of the sh:target property.
    TargetType, "TargetType",
    /// `A rule based on triple (subject, predicate, object) pattern.`: 
    TripleRule, "TripleRule",
    /// `Unique-languages constraint component`: A constraint component that can be used to specify that no pair of value nodes may use the same language tag.
    UniqueLangConstraintComponent, "UniqueLangConstraintComponent",
    /// `UniqueLangConstraintComponent-uniqueLang`: 
    UniqueLangConstraintComponent_uniqueLang, "UniqueLangConstraintComponent-uniqueLang",
    /// `Validation report`: The class of SHACL validation reports.
    ValidationReport, "ValidationReport",
    /// `Validation result`: The class of validation results.
    ValidationResult, "ValidationResult",
    /// `Validator`: The class of validators, which provide instructions on how to process a constraint definition. This class serves as base class for the SPARQL-based validators and other possible implementations.
    Validator, "Validator",
    /// `Violation`: The severity for a violation validation result.
    Violation, "Violation",
    /// `Warning`: The severity for a warning validation result.
    Warning, "Warning",
    /// `Exactly one constraint component`: A constraint component that can be used to restrict the value nodes so that they conform to exactly one out of several provided shapes.
    XoneConstraintComponent, "XoneConstraintComponent",
    /// `XoneConstraintComponent-xone`: 
    XoneConstraintComponent_xone, "XoneConstraintComponent-xone",
    /// `alternative path`: The (single) value of this property must be a list of path elements, representing the elements of alternative paths.
    alternativePath, "alternativePath",
    /// `and`: RDF list of shapes to validate the value nodes against.
    and, "and",
    /// `annotation property`: The annotation property that shall be set.
    annotationProperty, "annotationProperty",
    /// `annotation value`: The (default) values of the annotation property.
    annotationValue, "annotationValue",
    /// `annotation variable name`: The name of the SPARQL variable from the SELECT clause that shall be used for the values.
    annotationVarName, "annotationVarName",
    /// `ask`: The SPARQL ASK query to execute.
    ask, "ask",
    /// `class`: The type that all value nodes must have.
    class, "class",
    /// `closed`: If set to true then the shape is closed.
    closed, "closed",
    /// `condition`: The shapes that the focus nodes need to conform to before a rule is executed on them.
    condition, "condition",
    /// `conforms`: True if the validation did not produce any validation results, and false otherwise.
    conforms, "conforms",
    /// `construct`: The SPARQL CONSTRUCT query to execute.
    construct, "construct",
    /// `datatype`: Specifies an RDF datatype that all value nodes must have.
    datatype, "datatype",
    /// `deactivated`: If set to true then all nodes conform to this.
    deactivated, "deactivated",
    /// `declare`: Links a resource with its namespace prefix declarations.
    declare, "declare",
    /// `default value`: A default value for a property, for example for user interface tools to pre-populate input fields.
    defaultValue, "defaultValue",
    /// `description`: Human-readable descriptions for the property in the context of the surrounding shape.
    description, "description",
    /// `detail`: Links a result with other results that provide more details, for example to describe violations against nested shapes.
    detail, "detail",
    /// `disjoint`: Specifies a property where the set of values must be disjoint with the value nodes.
    disjoint, "disjoint",
    /// `entailment`: An entailment regime that indicates what kind of inferencing is required by a shapes graph.
    entailment, "entailment",
    /// `equals`: Specifies a property that must have the same values as the value nodes.
    equals, "equals",
    /// `expression`: The node expression that must return true for the value nodes.
    expression, "expression",
    /// `filter shape`: The shape that all input nodes of the expression need to conform to.
    filterShape, "filterShape",
    /// `flags`: An optional flag to be used with regular expression pattern matching.
    flags, "flags",
    /// `focus node`: The focus node that was validated when the result was produced.
    focusNode, "focusNode",
    /// `group`: Can be used to link to a property group to indicate that a property shape belongs to a group of related property shapes.
    group, "group",
    /// `has value`: Specifies a value that must be among the value nodes.
    hasValue, "hasValue",
    /// `ignored properties`: An optional RDF list of properties that are also permitted in addition to those explicitly enumerated via sh:property/sh:path.
    ignoredProperties, "ignoredProperties",
    /// `in`: Specifies a list of allowed values so that each value node must be among the members of the given list.
    in_, "in",
    /// `intersection`: A list of node expressions that shall be intersected.
    intersection, "intersection",
    /// `inverse path`: The (single) value of this property represents an inverse path (object to subject).
    inversePath, "inversePath",
    /// `JavaScript constraint`: Constraints expressed in JavaScript.
    js, "js",
    /// `JavaScript function name`: The name of the JavaScript function to execute.
    jsFunctionName, "jsFunctionName",
    /// `JavaScript library`: Declares which JavaScript libraries are needed to execute this.
    jsLibrary, "jsLibrary",
    /// `JavaScript library URL`: Declares the URLs of a JavaScript library. This should be the absolute URL of a JavaScript file. Implementations may redirect those to local files.
    jsLibraryURL, "jsLibraryURL",
    /// `label template`: Outlines how human-readable labels of instances of the associated Parameterizable shall be produced. The values can contain {?paramName} as placeholders for the actual values of the given parameter.
    labelTemplate, "labelTemplate",
    /// `language in`: Specifies a list of language tags that all value nodes must have.
    languageIn, "languageIn",
    /// `less than`: Specifies a property that must have smaller values than the value nodes.
    lessThan, "lessThan",
    /// `less than or equals`: Specifies a property that must have smaller or equal values than the value nodes.
    lessThanOrEquals, "lessThanOrEquals",
    /// `max count`: Specifies the maximum number of values in the set of value nodes.
    maxCount, "maxCount",
    /// `max exclusive`: Specifies the maximum exclusive value of each value node.
    maxExclusive, "maxExclusive",
    /// `max inclusive`: Specifies the maximum inclusive value of each value node.
    maxInclusive, "maxInclusive",
    /// `max length`: Specifies the maximum string length of each value node.
    maxLength, "maxLength",
    /// `message`: A human-readable message (possibly with placeholders for variables) explaining the cause of the result.
    message, "message",
    /// `min count`: Specifies the minimum number of values in the set of value nodes.
    minCount, "minCount",
    /// `min exclusive`: Specifies the minimum exclusive value of each value node.
    minExclusive, "minExclusive",
    /// `min inclusive`: Specifies the minimum inclusive value of each value node.
    minInclusive, "minInclusive",
    /// `min length`: Specifies the minimum string length of each value node.
    minLength, "minLength",
    /// `name`: Human-readable labels for the property in the context of the surrounding shape.
    name, "name",
    /// `namespace`: The namespace associated with a prefix in a prefix declaration.
    namespace, "namespace",
    /// `node`: Specifies the node shape that all value nodes must conform to.
    node, "node",
    /// `node kind`: Specifies the node kind (e.g. IRI or literal) each value node.
    nodeKind, "nodeKind",
    /// `shape validator`: The validator(s) used to evaluate a constraint in the context of a node shape.
    nodeValidator, "nodeValidator",
    /// `nodes`: The node expression producing the input nodes of a filter shape expression.
    nodes, "nodes",
    /// `not`: Specifies a shape that the value nodes must not conform to.
    not, "not",
    /// `object`: An expression producing the nodes that shall be inferred as objects.
    object, "object",
    /// `one or more path`: The (single) value of this property represents a path that is matched one or more times.
    oneOrMorePath, "oneOrMorePath",
    /// `optional`: Indicates whether a parameter is optional.
    optional, "optional",
    /// `or`: Specifies a list of shapes so that the value nodes must conform to at least one of the shapes.
    or, "or",
    /// `order`: Specifies the relative order of this compared to its siblings. For example use 0 for the first, 1 for the second.
    order, "order",
    /// `parameter`: The parameters of a function or constraint component.
    parameter, "parameter",
    /// `path`: Specifies the property path of a property shape.
    path, "path",
    /// `pattern`: Specifies a regular expression pattern that the string representations of the value nodes must match.
    pattern, "pattern",
    /// `predicate`: An expression producing the properties that shall be inferred as predicates.
    predicate, "predicate",
    /// `prefix`: The prefix of a prefix declaration.
    prefix, "prefix",
    /// `prefixes`: The prefixes that shall be applied before parsing the associated SPARQL query.
    prefixes, "prefixes",
    /// `property`: Links a shape to its property shapes.
    property, "property",
    /// `property validator`: The validator(s) used to evaluate a constraint in the context of a property shape.
    propertyValidator, "propertyValidator",
    /// `qualified max count`: The maximum number of value nodes that can conform to the shape.
    qualifiedMaxCount, "qualifiedMaxCount",
    /// `qualified min count`: The minimum number of value nodes that must conform to the shape.
    qualifiedMinCount, "qualifiedMinCount",
    /// `qualified value shape`: The shape that a specified number of values must conform to.
    qualifiedValueShape, "qualifiedValueShape",
    /// `qualified value shapes disjoint`: Can be used to mark the qualified value shape to be disjoint with its sibling shapes.
    qualifiedValueShapesDisjoint, "qualifiedValueShapesDisjoint",
    /// `result`: The validation results contained in a validation report.
    result, "result",
    /// `result annotation`: Links a SPARQL validator with zero or more sh:ResultAnnotation instances, defining how to derive additional result properties based on the variables of the SELECT query.
    resultAnnotation, "resultAnnotation",
    /// `result message`: Human-readable messages explaining the cause of the result.
    resultMessage, "resultMessage",
    /// `result path`: The path of a validation result, based on the path of the validated property shape.
    resultPath, "resultPath",
    /// `result severity`: The severity of the result, e.g. warning.
    resultSeverity, "resultSeverity",
    /// `return type`: The expected type of values returned by the associated function.
    returnType, "returnType",
    /// `rule`: The rules linked to a shape.
    rule, "rule",
    /// `select`: The SPARQL SELECT query to execute.
    select, "select",
    /// `severity`: Defines the severity that validation results produced by a shape must have. Defaults to sh:Violation.
    severity, "severity",
    /// `shapes graph`: Shapes graphs that should be used when validating this data graph.
    shapesGraph, "shapesGraph",
    /// `shapes graph well-formed`: If true then the validation engine was certain that the shapes graph has passed all SHACL syntax requirements during the validation process.
    shapesGraphWellFormed, "shapesGraphWellFormed",
    /// `source constraint`: The constraint that was validated when the result was produced.
    sourceConstraint, "sourceConstraint",
    /// `source constraint component`: The constraint component that is the source of the result.
    sourceConstraintComponent, "sourceConstraintComponent",
    /// `source shape`: The shape that is was validated when the result was produced.
    sourceShape, "sourceShape",
    /// `constraint (in SPARQL)`: Links a shape with SPARQL constraints.
    sparql, "sparql",
    /// `subject`: An expression producing the resources that shall be inferred as subjects.
    subject, "subject",
    /// `suggested shapes graph`: Suggested shapes graphs for this ontology. The values of this property may be used in the absence of specific sh:shapesGraph statements.
    suggestedShapesGraph, "suggestedShapesGraph",
    /// `target`: Links a shape to a target specified by an extension language, for example instances of sh:SPARQLTarget.
    target, "target",
    /// `target class`: Links a shape to a class, indicating that all instances of the class must conform to the shape.
    targetClass, "targetClass",
    /// `target node`: Links a shape to individual nodes, indicating that these nodes must conform to the shape.
    targetNode, "targetNode",
    /// `target objects of`: Links a shape to a property, indicating that all all objects of triples that have the given property as their predicate must conform to the shape.
    targetObjectsOf, "targetObjectsOf",
    /// `target subjects of`: Links a shape to a property, indicating that all subjects of triples that have the given property as their predicate must conform to the shape.
    targetSubjectsOf, "targetSubjectsOf",
    /// `this`: A node expression that represents the current focus node.
    this, "this",
    /// `union`: A list of node expressions that shall be used together.
    union, "union",
    /// `unique languages`: Specifies whether all node values must have a unique (or no) language tag.
    uniqueLang, "uniqueLang",
    /// `update`: The SPARQL UPDATE to execute.
    update, "update",
    /// `validator`: The validator(s) used to evaluate constraints of either node or property shapes.
    validator, "validator",
    /// `value`: An RDF node that has caused the result.
    value, "value",
    /// `exactly one`: Specifies a list of shapes so that the value nodes must conform to exactly one of the shapes.
    xone, "xone",
    /// `zero or more path`: The (single) value of this property represents a path that is matched zero or more times.
    zeroOrMorePath, "zeroOrMorePath",
    /// `zero or one path`: The (single) value of this property represents a path that is matched zero or one times.
    zeroOrOnePath, "zeroOrOnePath"
);
