// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-shex")))]
//! This module provides terms for `shex` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|shex|
//! |**Namespace base IRI**|<http://www.w3.org/ns/shex#>|
//! |**Description**||
//! |**Is defined by**|<http://www.w3.org/ns/shex#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/shex#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Annotation`: Annotations provide a format-independent way to provide additional information about elements in a schema. 
    Annotation, "Annotation",
    /// `Each Of`: A TripleExpression composed of one or more sub-expressions, all of which must match.
    EachOf, "EachOf",
    /// `IRI Stem`: An IRI prefix used for matching IRIs.
    IriStem, "IriStem",
    /// `IRI StemRange`: An IRI prefix (or wildcard) along with a set of excluded values, used for node matching.
    IriStemRange, "IriStemRange",
    /// `Language`: An Language tag used for matching Literal Languages.
    Language, "Language",
    /// `Language Stem`: An Language prefix used for matching Literal Languages.
    LanguageStem, "LanguageStem",
    /// `Language StemRange`: An Language prefix (or wildcard) along with a set of excluded values, used for node matching.
    LanguageStemRange, "LanguageStemRange",
    /// `Literal Stem`: An Literal prefix used for matching Literals.
    LiteralStem, "LiteralStem",
    /// `Literal StemRange`: An Literal prefix (or wildcard) along with a set of excluded values, used for node matching.
    LiteralStemRange, "LiteralStemRange",
    /// `Node Constraint`: A constraint on the type or value of an RDF Node.
    NodeConstraint, "NodeConstraint",
    /// `Node Kind`: The set of kinds of RDF Nodes.
    NodeKind, "NodeKind",
    /// `One Of`: A TripleExpression composed of one or more sub-expressions, one of which must match.
    OneOf, "OneOf",
    /// `Schema`: A Schema contains the set of shapes, used for matching a focus node.
    Schema, "Schema",
    /// `Semantic Actions`: A list of Semantic Actions that serve as an extension point for Shape Expressions. They appear in lists in Schema's startActs and Shape, OneOf, EachOf and TripleConstraint's semActs.
    SemAct, "SemAct",
    /// `Shape Or`: A shapes schema is captured in a Schema object where shapes is a mapping from shape label to shape expression.
    Shape, "Shape",
    /// `Shape And`: A ShapeExpression composed of one or more sub-expressions, all of which must match.
    ShapeAnd, "ShapeAnd",
    /// `Shape Expression`: The abstract class of Shape Expressions.
    ShapeExpression, "ShapeExpression",
    /// `Shape External`: A reference to a shape defined in some external Schema.
    ShapeExternal, "ShapeExternal",
    /// `Shape Not`: A ShapeNot is satisfied when it’s included ShapeExpression is not satisfied.
    ShapeNot, "ShapeNot",
    /// `Shape Or`: A ShapeExpression composed of one or more sub-expressions, one of which must match.
    ShapeOr, "ShapeOr",
    /// `Stem`: Abstract class for Stems
    Stem, "Stem",
    /// `StemRange`: Abstract Class for Stem Ranges
    StemRange, "StemRange",
    /// `Triple Constraint`: A constraint on a triple having a specific predicate and optionally a shape expression used for matching values.
    TripleConstraint, "TripleConstraint",
    /// `Triple Expression`: The abstract class of Triple Expressions.
    TripleExpression, "TripleExpression",
    /// `Wildcard`: Indicates that a stem is a Wildcard, rather than a URI prefix.
    Wildcard, "Wildcard",
    /// `annotation`: Annotations on a TripleExpression.
    annotation, "annotation",
    /// `bnode`: Requires node to be a Blank Node
    bnode, "bnode",
    /// `closed`: Indicates that a Shape is closed, meaning that it may contain no property values other than those used within TripleConstraints.
    closed, "closed",
    /// `code`: Code executed by Semantic Action.
    code, "code",
    /// `datatype`: A datatype constraint.
    datatype, "datatype",
    /// `exclusion`: Values that are excluded from value matching.
    exclusion, "exclusion",
    /// `expression`: Expression associated with the TripleExpression.
    expression, "expression",
    /// `expressions`: List of 2 or more expressions associated with the TripleExpression.
    expressions, "expressions",
    /// `extra`: Properties which may have extra values beyond those matched through a constraint.
    extra, "extra",
    /// `flags`: Regular expression flags
    flags, "flags",
    /// `fraction digits`: for "fractiondigits" constraints, v is less than or equals the number of digits to the right of the decimal place in the XML Schema canonical form[xmlschema-2] of the value of n, ignoring trailing zeros.
    fractiondigits, "fractiondigits",
    /// `inverse`: Constrains the subject of a triple, rather than the object.
    inverse, "inverse",
    /// `iri`: Requires node to be an IRI
    iri, "iri",
    /// `language tag`: The value used to match the language tag of a language-tagged string.
    languageTag, "languageTag",
    /// `length`: The exact length of the value of the cell.
    length, "length",
    /// `literal`: Requires node to be an rdf:Literal
    literal, "literal",
    /// `maximum cardinality`: Maximum number of times this TripleExpression may match; -1 for “*”
    max, "max",
    /// `max exclusive`: An atomic property that contains a single number that is the maximum valid value (exclusive).
    maxexclusive, "maxexclusive",
    /// `max inclusive`: An atomic property that contains a single number that is the maximum valid value (inclusive).
    maxinclusive, "maxinclusive",
    /// `max length`: A numeric atomic property that contains a single integer that is the maximum length of the value.
    maxlength, "maxlength",
    /// `minimum cardinatliy`: Minimum number of times this TripleExpression may match.
    min, "min",
    /// `min exclusive`: An atomic property that contains a single number that is the minimum valid value (exclusive).
    minexclusive, "minexclusive",
    /// `min inclusive`: An atomic property that contains a single number that is the minimum valid value (inclusive).
    mininclusive, "mininclusive",
    /// `min length`: An atomic property that contains a single integer that is the minimum length of the value.
    minlength, "minlength",
    /// `name`: Identifier of SemAct extension.
    name, "name",
    /// `node kind`: Restiction on the kind of node matched; restricted to the defined instances of NodeKind. One of shex:iri, shex:bnode, shex:literal, or shex:nonliteral.
    nodeKind, "nodeKind",
    /// `nonliteral`: Requires node to be a Blank Node or IRI
    nonliteral, "nonliteral",
    /// `numericFacet`: Abstract property of numeric facets on a NodeConstraint.
    numericFacet, "numericFacet",
    /// `object`: The object of an Annotation.
    object, "object",
    /// `pattern`: A regular expression used for matching a value.
    pattern, "pattern",
    /// `predicate`: The predicate of a TripleConstraint or Annotation.
    predicate, "predicate",
    /// `semantic action`: Semantic Actions on this TripleExpression.
    semActs, "semActs",
    /// `shape expression`: Shape Expression referenced by this shape.
    shapeExpr, "shapeExpr",
    /// `shape expressions`: A list of 2 or more Shape Expressions referenced by this shape.
    shapeExprs, "shapeExprs",
    /// `shapes`: Shapes in this Schema.
    shapes, "shapes",
    /// `start`: A ShapeExpression matched against the focus node prior to any other mapped expressions.
    start, "start",
    /// `start actions`: Semantic Actions run on the Schema.
    startActs, "startActs",
    /// `stem`: A stem value used for matching or excluding values.
    stem, "stem",
    /// `stringFacet`: An abstract property of string facets on a NodeConstraint.
    stringFacet, "stringFacet",
    /// `total digits`: for "totaldigits" constraints, v equals the number of digits in the XML Schema canonical form[xmlschema-2] of the value of n
    totaldigits, "totaldigits",
    /// `value expression`: A ShapeExpression used for matching the object (or subject if inverted) of a TripleConstraint.
    valueExpr, "valueExpr",
    /// `values`: A value restriction on a NodeConstraint.
    values, "values",
    /// `xsFacet`: An abstract property of string and numeric facets on a NodeConstraint.
    xsFacet, "xsFacet"
);
