// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `W3C Shapes Constraint Language (SHACL) Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|W3C Shapes Constraint Language (SHACL) Vocabulary|
//! |**Prefix**|sh|
//! |**Namespace base IRI**|[http://www.w3.org/ns/shacl#](http://www.w3.org/ns/shacl#)|
//! |**Description**|This vocabulary defines terms used in SHACL, the W3C Shapes Constraint Language.|
//! |**Is defined by**|[http://www.w3.org/ns/shacl#](http://www.w3.org/ns/shacl#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/shacl#",;
    AbstractResult, "AbstractResult",
    AndConstraintComponent, "AndConstraintComponent",
    AndConstraintComponent_and, "AndConstraintComponent-and",
    BlankNode, "BlankNode",
    BlankNodeOrIRI, "BlankNodeOrIRI",
    BlankNodeOrLiteral, "BlankNodeOrLiteral",
    ClassConstraintComponent, "ClassConstraintComponent",
    ClassConstraintComponent_class, "ClassConstraintComponent-class",
    ClosedConstraintComponent, "ClosedConstraintComponent",
    ClosedConstraintComponent_closed, "ClosedConstraintComponent-closed",
    ClosedConstraintComponent_ignoredProperties, "ClosedConstraintComponent-ignoredProperties",
    ConstraintComponent, "ConstraintComponent",
    DatatypeConstraintComponent, "DatatypeConstraintComponent",
    DatatypeConstraintComponent_datatype, "DatatypeConstraintComponent-datatype",
    DisjointConstraintComponent, "DisjointConstraintComponent",
    DisjointConstraintComponent_disjoint, "DisjointConstraintComponent-disjoint",
    EqualsConstraintComponent, "EqualsConstraintComponent",
    EqualsConstraintComponent_equals, "EqualsConstraintComponent-equals",
    ExpressionConstraintComponent, "ExpressionConstraintComponent",
    ExpressionConstraintComponent_expression, "ExpressionConstraintComponent-expression",
    Function, "Function",
    HasValueConstraintComponent, "HasValueConstraintComponent",
    HasValueConstraintComponent_hasValue, "HasValueConstraintComponent-hasValue",
    IRI, "IRI",
    IRIOrLiteral, "IRIOrLiteral",
    InConstraintComponent, "InConstraintComponent",
    InConstraintComponent_in, "InConstraintComponent-in",
    Info, "Info",
    JSConstraint, "JSConstraint",
    JSConstraint_js, "JSConstraint-js",
    JSConstraintComponent, "JSConstraintComponent",
    JSExecutable, "JSExecutable",
    JSFunction, "JSFunction",
    JSLibrary, "JSLibrary",
    JSRule, "JSRule",
    JSTarget, "JSTarget",
    JSTargetType, "JSTargetType",
    JSValidator, "JSValidator",
    LanguageInConstraintComponent, "LanguageInConstraintComponent",
    LanguageInConstraintComponent_languageIn, "LanguageInConstraintComponent-languageIn",
    LessThanConstraintComponent, "LessThanConstraintComponent",
    LessThanConstraintComponent_lessThan, "LessThanConstraintComponent-lessThan",
    LessThanOrEqualsConstraintComponent, "LessThanOrEqualsConstraintComponent",
    LessThanOrEqualsConstraintComponent_lessThanOrEquals, "LessThanOrEqualsConstraintComponent-lessThanOrEquals",
    Literal, "Literal",
    MaxCountConstraintComponent, "MaxCountConstraintComponent",
    MaxCountConstraintComponent_maxCount, "MaxCountConstraintComponent-maxCount",
    MaxExclusiveConstraintComponent, "MaxExclusiveConstraintComponent",
    MaxExclusiveConstraintComponent_maxExclusive, "MaxExclusiveConstraintComponent-maxExclusive",
    MaxInclusiveConstraintComponent, "MaxInclusiveConstraintComponent",
    MaxInclusiveConstraintComponent_maxInclusive, "MaxInclusiveConstraintComponent-maxInclusive",
    MaxLengthConstraintComponent, "MaxLengthConstraintComponent",
    MaxLengthConstraintComponent_maxLength, "MaxLengthConstraintComponent-maxLength",
    MinCountConstraintComponent, "MinCountConstraintComponent",
    MinCountConstraintComponent_minCount, "MinCountConstraintComponent-minCount",
    MinExclusiveConstraintComponent, "MinExclusiveConstraintComponent",
    MinExclusiveConstraintComponent_minExclusive, "MinExclusiveConstraintComponent-minExclusive",
    MinInclusiveConstraintComponent, "MinInclusiveConstraintComponent",
    MinInclusiveConstraintComponent_minInclusive, "MinInclusiveConstraintComponent-minInclusive",
    MinLengthConstraintComponent, "MinLengthConstraintComponent",
    MinLengthConstraintComponent_minLength, "MinLengthConstraintComponent-minLength",
    NodeConstraintComponent, "NodeConstraintComponent",
    NodeConstraintComponent_node, "NodeConstraintComponent-node",
    NodeKind, "NodeKind",
    NodeKindConstraintComponent, "NodeKindConstraintComponent",
    NodeKindConstraintComponent_nodeKind, "NodeKindConstraintComponent-nodeKind",
    NodeShape, "NodeShape",
    NotConstraintComponent, "NotConstraintComponent",
    NotConstraintComponent_not, "NotConstraintComponent-not",
    OrConstraintComponent, "OrConstraintComponent",
    OrConstraintComponent_or, "OrConstraintComponent-or",
    Parameter, "Parameter",
    Parameterizable, "Parameterizable",
    PatternConstraintComponent, "PatternConstraintComponent",
    PatternConstraintComponent_flags, "PatternConstraintComponent-flags",
    PatternConstraintComponent_pattern, "PatternConstraintComponent-pattern",
    PrefixDeclaration, "PrefixDeclaration",
    PropertyConstraintComponent, "PropertyConstraintComponent",
    PropertyConstraintComponent_property, "PropertyConstraintComponent-property",
    PropertyGroup, "PropertyGroup",
    PropertyShape, "PropertyShape",
    QualifiedMaxCountConstraintComponent, "QualifiedMaxCountConstraintComponent",
    QualifiedMaxCountConstraintComponent_qualifiedMaxCount, "QualifiedMaxCountConstraintComponent-qualifiedMaxCount",
    QualifiedMaxCountConstraintComponent_qualifiedValueShape, "QualifiedMaxCountConstraintComponent-qualifiedValueShape",
    QualifiedMaxCountConstraintComponent_qualifiedValueShapesDisjoint, "QualifiedMaxCountConstraintComponent-qualifiedValueShapesDisjoint",
    QualifiedMinCountConstraintComponent, "QualifiedMinCountConstraintComponent",
    QualifiedMinCountConstraintComponent_qualifiedMinCount, "QualifiedMinCountConstraintComponent-qualifiedMinCount",
    QualifiedMinCountConstraintComponent_qualifiedValueShape, "QualifiedMinCountConstraintComponent-qualifiedValueShape",
    QualifiedMinCountConstraintComponent_qualifiedValueShapesDisjoint, "QualifiedMinCountConstraintComponent-qualifiedValueShapesDisjoint",
    ResultAnnotation, "ResultAnnotation",
    Rule, "Rule",
    SPARQLAskExecutable, "SPARQLAskExecutable",
    SPARQLAskValidator, "SPARQLAskValidator",
    SPARQLConstraint, "SPARQLConstraint",
    SPARQLConstraintComponent, "SPARQLConstraintComponent",
    SPARQLConstraintComponent_sparql, "SPARQLConstraintComponent-sparql",
    SPARQLConstructExecutable, "SPARQLConstructExecutable",
    SPARQLExecutable, "SPARQLExecutable",
    SPARQLFunction, "SPARQLFunction",
    SPARQLRule, "SPARQLRule",
    SPARQLSelectExecutable, "SPARQLSelectExecutable",
    SPARQLSelectValidator, "SPARQLSelectValidator",
    SPARQLTarget, "SPARQLTarget",
    SPARQLTargetType, "SPARQLTargetType",
    SPARQLUpdateExecutable, "SPARQLUpdateExecutable",
    Severity, "Severity",
    Shape, "Shape",
    Target, "Target",
    TargetType, "TargetType",
    TripleRule, "TripleRule",
    UniqueLangConstraintComponent, "UniqueLangConstraintComponent",
    UniqueLangConstraintComponent_uniqueLang, "UniqueLangConstraintComponent-uniqueLang",
    ValidationReport, "ValidationReport",
    ValidationResult, "ValidationResult",
    Validator, "Validator",
    Violation, "Violation",
    Warning, "Warning",
    XoneConstraintComponent, "XoneConstraintComponent",
    XoneConstraintComponent_xone, "XoneConstraintComponent-xone",
    alternativePath, "alternativePath",
    and, "and",
    annotationProperty, "annotationProperty",
    annotationValue, "annotationValue",
    annotationVarName, "annotationVarName",
    ask, "ask",
    class, "class",
    closed, "closed",
    condition, "condition",
    conforms, "conforms",
    construct, "construct",
    datatype, "datatype",
    deactivated, "deactivated",
    declare, "declare",
    defaultValue, "defaultValue",
    description, "description",
    detail, "detail",
    disjoint, "disjoint",
    entailment, "entailment",
    equals, "equals",
    expression, "expression",
    filterShape, "filterShape",
    flags, "flags",
    focusNode, "focusNode",
    group, "group",
    hasValue, "hasValue",
    ignoredProperties, "ignoredProperties",
    in_, "in",
    intersection, "intersection",
    inversePath, "inversePath",
    js, "js",
    jsFunctionName, "jsFunctionName",
    jsLibrary, "jsLibrary",
    jsLibraryURL, "jsLibraryURL",
    labelTemplate, "labelTemplate",
    languageIn, "languageIn",
    lessThan, "lessThan",
    lessThanOrEquals, "lessThanOrEquals",
    maxCount, "maxCount",
    maxExclusive, "maxExclusive",
    maxInclusive, "maxInclusive",
    maxLength, "maxLength",
    message, "message",
    minCount, "minCount",
    minExclusive, "minExclusive",
    minInclusive, "minInclusive",
    minLength, "minLength",
    name, "name",
    namespace, "namespace",
    node, "node",
    nodeKind, "nodeKind",
    nodeValidator, "nodeValidator",
    nodes, "nodes",
    not, "not",
    object, "object",
    oneOrMorePath, "oneOrMorePath",
    optional, "optional",
    or, "or",
    order, "order",
    parameter, "parameter",
    path, "path",
    pattern, "pattern",
    predicate, "predicate",
    prefix, "prefix",
    prefixes, "prefixes",
    property, "property",
    propertyValidator, "propertyValidator",
    qualifiedMaxCount, "qualifiedMaxCount",
    qualifiedMinCount, "qualifiedMinCount",
    qualifiedValueShape, "qualifiedValueShape",
    qualifiedValueShapesDisjoint, "qualifiedValueShapesDisjoint",
    result, "result",
    resultAnnotation, "resultAnnotation",
    resultMessage, "resultMessage",
    resultPath, "resultPath",
    resultSeverity, "resultSeverity",
    returnType, "returnType",
    rule, "rule",
    select, "select",
    severity, "severity",
    shapesGraph, "shapesGraph",
    shapesGraphWellFormed, "shapesGraphWellFormed",
    sourceConstraint, "sourceConstraint",
    sourceConstraintComponent, "sourceConstraintComponent",
    sourceShape, "sourceShape",
    sparql, "sparql",
    subject, "subject",
    suggestedShapesGraph, "suggestedShapesGraph",
    target, "target",
    targetClass, "targetClass",
    targetNode, "targetNode",
    targetObjectsOf, "targetObjectsOf",
    targetSubjectsOf, "targetSubjectsOf",
    this, "this",
    union, "union",
    uniqueLang, "uniqueLang",
    update, "update",
    validator, "validator",
    value, "value",
    xone, "xone",
    zeroOrMorePath, "zeroOrMorePath",
    zeroOrOnePath, "zeroOrOnePath"
);