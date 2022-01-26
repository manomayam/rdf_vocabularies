// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-dash")))]
//! This module provides terms for `DASH Data Shapes Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|DASH Data Shapes Vocabulary|
//! |**Prefix**|dash|
//! |**Namespace base IRI**|<http://datashapes.org/dash#>|
//! |**Description**|DASH is a SHACL library for frequently needed features and design patterns. Almost all features in this library are 100% standards compliant and will work on any engine that fully supports SHACL.|
//! |**Is defined by**|<http://datashapes.org/dash#>|
//!

use crate::namespace;

namespace!(
    "http://datashapes.org/dash#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Action`: An executable command triggered by an agent, backed by a Script implementation. Actions may get deactivated using sh:deactivated.
    Action, "Action",
    /// `Action group`: A group of ResourceActions, used to arrange items in menus etc. Similar to sh:PropertyGroups, they may have a sh:order and should have labels (in multiple languages if applicable).
    ActionGroup, "ActionGroup",
    /// `Action test case`: A test case that evaluates a dash:Action using provided input parameters. Requires exactly one value for dash:action and will operate on the test case's graph (with imports) as both data and shapes graph.  Currently only supports read-only actions, allowing the comparison of actual results with the expected results.
    ActionTestCase, "ActionTestCase",
    /// `All objects`: A reusable instance of dash:AllObjectsTarget.
    AllObjects, "AllObjects",
    /// `All objects target`: A target containing all objects in the data graph as focus nodes.
    AllObjectsTarget, "AllObjectsTarget",
    /// `All subjects`: A reusable instance of dash:AllSubjectsTarget.
    AllSubjects, "AllSubjects",
    /// `All subjects target`: A target containing all subjects in the data graph as focus nodes.
    AllSubjectsTarget, "AllSubjectsTarget",
    /// `Auto-complete editor`: An auto-complete field to enter the label of instances of a class. This is the fallback editor for any URI resource if no other editors are more suitable.
    AutoCompleteEditor, "AutoCompleteEditor",
    /// `Blank node viewer`: A Viewer for blank nodes, rendering as the label of the blank node.
    BlankNodeViewer, "BlankNodeViewer",
    /// `Boolean select editor`: An editor for boolean literals, rendering as a select box with values true and false.  Also displays the current value (such as "1"^^xsd:boolean), but only allows to switch to true or false.
    BooleanSelectEditor, "BooleanSelectEditor",
    /// `Closed by types constraint component`: A constraint component that can be used to declare that focus nodes are "closed" based on their rdf:types, meaning that focus nodes may only have values for the properties that are explicitly enumerated via sh:property/sh:path in property constraints at their rdf:types and the superclasses of those. This assumes that the type classes are also shapes.
    ClosedByTypesConstraintComponent, "ClosedByTypesConstraintComponent",
    /// `ClosedByTypesConstraintComponent-closedByTypes`: 
    ClosedByTypesConstraintComponent_closedByTypes, "ClosedByTypesConstraintComponent-closedByTypes",
    /// `Co-exists-with constraint component`: A constraint component that can be used to express a constraint on property shapes so that if the property path has any value then the given property must also have a value, and vice versa.
    CoExistsWithConstraintComponent, "CoExistsWithConstraintComponent",
    /// `CoExistsWithConstraintComponent-coExistsWith`: 
    CoExistsWithConstraintComponent_coExistsWith, "CoExistsWithConstraintComponent-coExistsWith",
    /// `Constraint reification shape`: Can be used to attach sh:severity and sh:messages to individual constraints using reification.
    ConstraintReificationShape, "ConstraintReificationShape",
    /// `ConstraintReificationShape-message`: 
    ConstraintReificationShape_message, "ConstraintReificationShape-message",
    /// `ConstraintReificationShape-severity`: 
    ConstraintReificationShape_severity, "ConstraintReificationShape-severity",
    /// `DASH JavaScript library`: 
    DASHJSLibrary, "DASHJSLibrary",
    /// `Date or date time`: An rdf:List that can be used in property constraints as value for sh:or to indicate that all values of a property must be either xsd:date or xsd:dateTime.
    DateOrDateTime, "DateOrDateTime",
    /// `Date picker editor`: An editor for xsd:date literals, offering a calendar-like date picker.
    DatePickerEditor, "DatePickerEditor",
    /// `Date time picker editor`: An editor for xsd:dateTime literals, offering a calendar-like date picker and a time selector.
    DateTimePickerEditor, "DateTimePickerEditor",
    /// `Depiction`: Depiction properties provide images representing the focus nodes. Typical examples may be a photo of an animal or the map of a country.
    DepictionRole, "DepictionRole",
    /// `Description`: Description properties should produce text literals that may be used as an introduction/summary of what a focus node does.
    DescriptionRole, "DescriptionRole",
    /// `Details editor`: An editor for non-literal values, typically displaying a nested form where the values of the linked resource can be edited directly on the "parent" form. Implementations that do not support this (yet) could fall back to an auto-complete widget.
    DetailsEditor, "DetailsEditor",
    /// `Details viewer`: A Viewer for resources that shows the details of the value using its default view shape as a nested form-like display.
    DetailsViewer, "DetailsViewer",
    /// `Editor`: The class of widgets for editing value nodes.
    Editor, "Editor",
    /// `Enum select editor`: A drop-down editor for enumerated values (typically based on sh:in lists).
    EnumSelectEditor, "EnumSelectEditor",
    /// `Execution platform`: An platform (such as TopBraid) that may have features needed to execute, for example, SPARQL queries.
    ExecutionPlatform, "ExecutionPlatform",
    /// `Explore action`: An action typically showing up in an Explore section of a selected resource. Cannot make changes to the data.
    ExploreAction, "ExploreAction",
    /// `Failure result`: A result representing a validation failure such as an unsupported recursion.
    FailureResult, "FailureResult",
    /// `Failure test case result`: Represents a failure of a test case.
    FailureTestCaseResult, "FailureTestCaseResult",
    /// `Function test case`: A test case that verifies that a given SPARQL expression produces a given, expected result.
    FunctionTestCase, "FunctionTestCase",
    /// `Graph store test case`: A test case that can be used to verify that an RDF file could be loaded (from a file) and that the resulting RDF graph is equivalent to a given TTL file.
    GraphStoreTestCase, "GraphStoreTestCase",
    /// `Graph update`: A suggestion consisting of added and/or deleted triples, represented as rdf:Statements via dash:addedTriple and dash:deletedTriple.
    GraphUpdate, "GraphUpdate",
    /// `Graph validation test case`: A test case that performs SHACL constraint validation on the whole graph and compares the results with the expected validation results stored with the test case. By default this excludes meta-validation (i.e. the validation of the shape definitions themselves). If that's desired, set dash:validateShapes to true.
    GraphValidationTestCase, "GraphValidationTestCase",
    /// `HTML or string or langString`: An rdf:List that can be used in property constraints as value for sh:or to indicate that all values of a property must be either rdf:HTML, xsd:string or rdf:langString (in that order of preference).
    HTMLOrStringOrLangString, "HTMLOrStringOrLangString",
    /// `HTML viewer`: A Viewer for HTML encoded text from rdf:HTML literals, rendering as parsed HTML DOM elements. Also displays the language if the HTML has a lang attribute on its root DOM element.
    HTMLViewer, "HTMLViewer",
    /// `Has value in constraint component`: A constraint component that can be used to express a constraint on property shapes so that one of the values of the property path must be a member of a given list of nodes.
    HasValueInConstraintComponent, "HasValueInConstraintComponent",
    /// `HasValueInConstraintComponent-hasValueIn`: 
    HasValueInConstraintComponent_hasValueIn, "HasValueInConstraintComponent-hasValueIn",
    /// `Has Value target`: A target type for all subjects where a given predicate has a certain object value.
    HasValueTarget, "HasValueTarget",
    /// `Has value with class constraint component`: A constraint component that can be used to express a constraint on property shapes so that one of the values of the property path must be an instance of a given class.
    HasValueWithClassConstraintComponent, "HasValueWithClassConstraintComponent",
    /// `HasValueWithClassConstraintComponent-hasValueWithClass`: 
    HasValueWithClassConstraintComponent_hasValueWithClass, "HasValueWithClassConstraintComponent-hasValueWithClass",
    /// `Hyperlink viewer`: A Viewer for literals, rendering as a hyperlink to a URL.  For literals it assumes the lexical form is the URL.  This is often used as default viewer for xsd:anyURI literals. Unsupported for blank nodes.
    HyperlinkViewer, "HyperlinkViewer",
    /// `ID`: ID properties are short strings or other literals that identify the focus node among siblings. Examples may include social security numbers.
    IDRole, "IDRole",
    /// `Icon`: Icon properties produce images that are typically small and almost square-shaped, and that may be displayed in the upper left corner of a focus node's display. Values should be xsd:string or xsd:anyURI literals or IRI nodes pointing at URLs. Those URLs should ideally be vector graphics such as .svg files.  Instances of the same class often have the same icon, and this icon may be computed using a sh:values rule or as sh:defaultValue.  If the value is a relative URL then those should be resolved against the server that delivered the surrounding page.
    IconRole, "IconRole",
    /// `Image viewer`: A Viewer for URI values that are recognized as images by a browser, rendering as an image.
    ImageViewer, "ImageViewer",
    /// `Indexed constraint component`: A constraint component that can be used to mark property shapes to be indexed, meaning that each of its value nodes must carry a dash:index from 0 to N.
    IndexedConstraintComponent, "IndexedConstraintComponent",
    /// `IndexedConstraintComponent-indexed`: 
    IndexedConstraintComponent_indexed, "IndexedConstraintComponent-indexed",
    /// `Inferencing test case`: A test case to verify whether an inferencing engine is producing identical results to those stored as expected results.
    InferencingTestCase, "InferencingTestCase",
    /// `Instances select editor`: A drop-down editor for all instances of the target class (based on sh:class of the property).
    InstancesSelectEditor, "InstancesSelectEditor",
    /// `SHACL-JS test case`: A test case that calls a given SHACL-JS JavaScript function like a sh:JSFunction and compares its result with the dash:expectedResult.
    JSTestCase, "JSTestCase",
    /// `Key info`: The Key info role may be assigned to properties that are likely of special interest to a reader, so that they should appear whenever a summary of a focus node is shown.
    KeyInfoRole, "KeyInfoRole",
    /// `Label`: Properties with this role produce strings that may serve as display label for the focus nodes. Labels should be either plain string literals or strings with a language tag. The values should also be single-line.
    LabelRole, "LabelRole",
    /// `Label viewer`: A Viewer for URI resources, rendering as a hyperlink to that URI based on the display label of the resource. Also includes other ways of interacting with the URI such as opening a nested summary display.
    LabelViewer, "LabelViewer",
    /// `LangString viewer`: A Viewer for literals with a language tag, rendering as the text plus a language indicator.
    LangStringViewer, "LangStringViewer",
    /// `List node shape`: Defines constraints on what it means for a node to be a node within a well-formed RDF list. Note that this does not check whether the rdf:rest items are also well-formed lists as this would lead to unsupported recursion.
    ListNodeShape, "ListNodeShape",
    /// `List shape`: Defines constraints on what it means for a node to be a well-formed RDF list.  The focus node must either be rdf:nil or not recursive. Furthermore, this shape uses dash:ListNodeShape as a "helper" to walk through all members of the whole list (including itself).
    ListShape, "ListShape",
    /// `Literal viewer`: A simple viewer for literals, rendering the lexical form of the value.
    LiteralViewer, "LiteralViewer",
    /// `Modify action`: An action typically showing up in a Modify section of a selected resource. May make changes to the data.
    ModifyAction, "ModifyAction",
    /// `Multi editor`: An editor for multiple/all value nodes at once.
    MultiEditor, "MultiEditor",
    /// `Multi viewer`: A viewer for multiple/all values at once.
    MultiViewer, "MultiViewer",
    /// `Node expression viewer`: A viewer for SHACL Node Expressions.
    NodeExpressionViewer, "NodeExpressionViewer",
    /// `Non-recursive constraint component`: Used to state that a property or path must not point back to itself.
    NonRecursiveConstraintComponent, "NonRecursiveConstraintComponent",
    /// `NonRecursiveConstraintComponent-nonRecursive`: 
    NonRecursiveConstraintComponent_nonRecursive, "NonRecursiveConstraintComponent-nonRecursive",
    /// `None`: A Shape that is no node can conform to.
    None, "None",
    /// `Parameter constraint component`: A constraint component that can be used to verify that all value nodes conform to the given Parameter.
    ParameterConstraintComponent, "ParameterConstraintComponent",
    /// `ParameterConstraintComponent-parameter`: 
    ParameterConstraintComponent_parameter, "ParameterConstraintComponent-parameter",
    /// `Primary key constraint component`: Enforces a constraint that the given property (sh:path) serves as primary key for all resources in the target of the shape. If a property has been declared to be the primary key then each resource must have exactly one value for that property. Furthermore, the URIs of those resources must start with a given string (dash:uriStart), followed by the URL-encoded primary key value. For example if dash:uriStart is "http://example.org/country-" and the primary key for an instance is "de" then the URI must be "http://example.org/country-de". Finally, as a result of the URI policy, there can not be any other resource with the same value under the same primary key policy.
    PrimaryKeyConstraintComponent, "PrimaryKeyConstraintComponent",
    /// `PrimaryKeyConstraintComponent-uriStart`: 
    PrimaryKeyConstraintComponent_uriStart, "PrimaryKeyConstraintComponent-uriStart",
    /// `Property auto-complete editor`: An editor for properties that are either defined as instances of rdf:Property or used as IRI values of sh:path. The component uses auto-complete to find these properties by their rdfs:labels or sh:names.
    PropertyAutoCompleteEditor, "PropertyAutoCompleteEditor",
    /// `Property label viewer`: A viewer for properties that renders a hyperlink using the display label or sh:name, allowing users to either navigate to the rdf:Property resource or the property shape definition. Should be used in conjunction with PropertyAutoCompleteEditor.
    PropertyLabelViewer, "PropertyLabelViewer",
    /// `Property role`: The class of roles that a property (shape) may take for its focus nodes.
    PropertyRole, "PropertyRole",
    /// `Query test case`: A test case running a given SPARQL SELECT query and comparing its results with those stored as JSON Result Set in the expected result property.
    QueryTestCase, "QueryTestCase",
    /// `rdfQuery JavaScript Library`: 
    RDFQueryJSLibrary, "RDFQueryJSLibrary",
    /// `Reifiable-by constraint component`: 
    ReifiableByConstraintComponent, "ReifiableByConstraintComponent",
    /// `ReifiableByConstraintComponent-reifiableBy`: 
    ReifiableByConstraintComponent_reifiableBy, "ReifiableByConstraintComponent-reifiableBy",
    /// `Resource action`: An Action that can be executed for a selected resource. Such Actions show up in context menus once they have been assigned a sh:group.
    ResourceAction, "ResourceAction",
    /// `Rich text editor`: A rich text editor to enter the lexical value of a literal and a drop down to select language. The selected language is stored in the HTML lang attribute of the root node in the HTML DOM tree.
    RichTextEditor, "RichTextEditor",
    /// `Root class constraint component`: A constraint component defining the parameter dash:rootClass, which restricts the values to be either the root class itself or one of its subclasses. This is typically used in conjunction with properties that have rdfs:Class as their type.
    RootClassConstraintComponent, "RootClassConstraintComponent",
    /// `RootClassConstraintComponent-rootClass`: 
    RootClassConstraintComponent_rootClass, "RootClassConstraintComponent-rootClass",
    /// `SPARQL CONSTRUCT template`: Encapsulates one or more SPARQL CONSTRUCT queries that can be parameterized. Parameters will become pre-bound variables in the queries.
    SPARQLConstructTemplate, "SPARQLConstructTemplate",
    /// `SPARQL SELECT template`: Encapsulates a SPARQL SELECT query that can be parameterized. Parameters will become pre-bound variables in the query.
    SPARQLSelectTemplate, "SPARQLSelectTemplate",
    /// `SPARQL UPDATE suggestion generator`: A SuggestionGenerator based on a SPARQL UPDATE query (sh:update), producing an instance of dash:GraphUpdate. The INSERTs become dash:addedTriple and the DELETEs become dash:deletedTriple. The WHERE clause operates on the data graph with the pre-bound variables $focusNode, $predicate and $value, as well as the other pre-bound variables for the parameters of the constraint.  In many cases, there may be multiple possible suggestions to fix a problem. For example, with sh:maxLength there are many ways to slice a string. In those cases, the system will first iterate through the result variables from a SELECT query (sh:select) and apply these results as pre-bound variables into the UPDATE query.
    SPARQLUpdateSuggestionGenerator, "SPARQLUpdateSuggestionGenerator",
    /// `Script`: An executable unit implemented in one or more languages such as JavaScript.
    Script, "Script",
    /// `Script constraint`: The class of constraints that are based on Scripts. Depending on whether dash:onAllValues is set to true, these scripts can access the following pre-assigned variables:  - focusNode: the focus node of the constraint (a NamedNode) - if dash:onAllValues is not true: value: the current value node (e.g. a JavaScript string for xsd:string literals, a number for numeric literals or true or false for xsd:boolean literals. All other literals become LiteralNodes, and non-literals become instances of NamedNode) - if dash:onAllValues is true: values: an array of current value nodes, as above.  If the expression returns an array then each array member will be mapped to one validation result, following the mapping rules below.  For string results, a validation result will use the string as sh:message. For boolean results, a validation result will be produced if the result is false (true means no violation).  For object results, a validation result will be produced using the value of the field "message" of the object as result message. If the field "value" has a value then this will become the sh:value in the violation.  Unless another sh:message has been directly returned, the sh:message of the dash:ScriptConstraint will be used, similar to sh:message at SPARQL Constraints. These sh:messages can access the values {$focusNode}, ${value} etc as template variables.
    ScriptConstraint, "ScriptConstraint",
    /// `Script constraint component`: 
    ScriptConstraintComponent, "ScriptConstraintComponent",
    /// `ScriptConstraintComponent-scriptConstraint`: 
    ScriptConstraintComponent_scriptConstraint, "ScriptConstraintComponent-scriptConstraint",
    /// `Script function`: Script functions can be used from SPARQL queries and will be injected into the generated prefix object (in JavaScript, for ADS scripts). The dash:js will be inserted into a generated JavaScript function and therefore needs to use the return keyword to produce results. These JS snippets can access the parameter values based on the local name of the sh:Parameter's path. For example ex:value can be accessed using value.  SPARQL use note: Since these functions may be used from any data graph and any shapes graph, they must not rely on any API apart from what's available in the shapes graph that holds the rdf:type triple of the function itself. In other words, at execution time from SPARQL, the ADS shapes graph will be the home graph of the function's declaration.
    ScriptFunction, "ScriptFunction",
    /// `Script suggestion generator`: A Suggestion Generator that is backed by an Active Data Shapes script. The script needs to return a JSON object or an array of JSON objects if it shall generate multiple suggestions. It may also return null to indicate that nothing was suggested. Note that the whole script is evaluated as a (JavaScript) expression, and those will use the last value as result. So simply putting an object at the end of your script should do. Alternatively, define the bulk of the operation as a function and simply call that function in the script.  Each response object can have the following fields:  { 	message: "The human readable message",  // Defaults to the rdfs:label(s) of the suggestion generator 	add: [ // An array of triples to add, each triple as an array with three nodes 		[ subject, predicate, object ], 		[ ... ] 	], 	delete: [ 		... like add, for the triples to delete 	] }  Suggestions with neither added nor deleted triples will be discarded.  At execution time, the script operates on the data graph as the active graph, with the following pre-bound variables: - focusNode: the NamedNode that is the sh:focusNode of the validation result - predicate: the NamedNode representing the predicate of the validation result, assuming sh:resultPath is a URI - value: the value node from the validation result's sh:value, cast into the most suitable JS object - the other pre-bound variables for the parameters of the constraint, e.g. in a sh:maxCount constraint it would be maxCount  The script will be executed in read-only mode, i.e. it cannot modify the graph.  Example with dash:js:  ({ 	message: `Copy labels into ${graph.localName(predicate)}`, 	add: focusNode.values(rdfs.label).map(label =>  		[ focusNode, predicate, label ]     ) })
    ScriptSuggestionGenerator, "ScriptSuggestionGenerator",
    /// `Script test case`: A test case that evaluates a script. Requires exactly one value for dash:js and will operate on the test case's graph (with imports) as both data and shapes graph.  Supports read-only scripts only at this stage.
    ScriptTestCase, "ScriptTestCase",
    /// `Script validator`: A SHACL validator based on an Active Data Shapes script.  See the comment at dash:ScriptConstraint for the basic evaluation approach. Note that in addition to focusNode and value/values, the script can access pre-bound variables for each declared argument of the constraint component.
    ScriptValidator, "ScriptValidator",
    /// `Shape class`: A class that is also a node shape. This class can be used as rdf:type instead of the combination of rdfs:Class and sh:NodeShape.
    ShapeClass, "ShapeClass",
    /// `Shape script`: A shape script contains extra code that gets injected into the API for the associated node shape. In particular you can use this to define additional functions that operate on the current focus node (the this variable in JavaScript).
    ShapeScript, "ShapeScript",
    /// `Single editor`: An editor for individual value nodes.
    SingleEditor, "SingleEditor",
    /// `Single line constraint component`: A constraint component that can be used to declare that all values that are literals must have a lexical form that contains no line breaks ('\n' or '\r').  User interfaces may use the dash:singleLine flag to prefer a text field over a (multi-line) text area.
    SingleLineConstraintComponent, "SingleLineConstraintComponent",
    /// `SingleLineConstraintComponent-singleLine`: 
    SingleLineConstraintComponent_singleLine, "SingleLineConstraintComponent-singleLine",
    /// `Single viewer`: A viewer for a single value.
    SingleViewer, "SingleViewer",
    /// `Stem constraint component`: A constraint component that can be used to verify that every value node is an IRI and the IRI starts with a given string value.
    StemConstraintComponent, "StemConstraintComponent",
    /// `StemConstraintComponent-stem`: 
    StemConstraintComponent_stem, "StemConstraintComponent-stem",
    /// `String or langString`: An rdf:List that can be used in property constraints as value for sh:or to indicate that all values of a property must be either xsd:string or rdf:langString.
    StringOrLangString, "StringOrLangString",
    /// `string or langString or HTML`: An rdf:List that can be used in property constraints as value for sh:or to indicate that all values of a property must be either xsd:string, rdf:langString or rdf:HTML (in that order of preference).
    StringOrLangStringOrHTML, "StringOrLangStringOrHTML",
    /// `Sub-Class editor`: An editor for properties that declare a dash:rootClass. The editor allows selecting either the class itself or one of its subclasses.
    SubClassEditor, "SubClassEditor",
    /// `Sub set of constraint component`: A constraint component that can be used to state that the set of value nodes must be a subset of the value of a given property.
    SubSetOfConstraintComponent, "SubSetOfConstraintComponent",
    /// `SubSetOfConstraintComponent-subSetOf`: 
    SubSetOfConstraintComponent_subSetOf, "SubSetOfConstraintComponent-subSetOf",
    /// `Success result`: A result representing a successfully validated constraint.
    SuccessResult, "SuccessResult",
    /// `Success test case result`: Represents a successful run of a test case.
    SuccessTestCaseResult, "SuccessTestCaseResult",
    /// `Suggestion`: Base class of suggestions that modify a graph to "fix" the source of a validation result.
    Suggestion, "Suggestion",
    /// `Suggestion generator`: Base class of objects that can generate suggestions (added or deleted triples) for a validation result of a given constraint component.
    SuggestionGenerator, "SuggestionGenerator",
    /// `Suggestion result`: Class of results that have been produced as suggestions, not through SHACL validation. How the actual results are produced is up to implementers. Each instance of this class should have values for sh:focusNode, sh:resultMessage, sh:resultSeverity (suggested default: sh:Info), and dash:suggestion to point at one or more suggestions.
    SuggestionResult, "SuggestionResult",
    /// `Symmetric constraint component`: A contraint component for property shapes to validate that a property is symmetric. For symmetric properties, if A relates to B then B must relate to A.
    SymmetricConstraintComponent, "SymmetricConstraintComponent",
    /// `SymmetricConstraintComponent-symmetric`: 
    SymmetricConstraintComponent_symmetric, "SymmetricConstraintComponent-symmetric",
    /// `Test case`: A test case to verify that a (SHACL-based) feature works as expected.
    TestCase, "TestCase",
    /// `Test case result`: Base class for results produced by running test cases.
    TestCaseResult, "TestCaseResult",
    /// `Test environment`: Abstract base class for test environments, holding information on how to set up a test case.
    TestEnvironment, "TestEnvironment",
    /// `Text area editor`: A multi-line text area to enter the value of a literal.
    TextAreaEditor, "TextAreaEditor",
    /// `Text area with lang editor`: A multi-line text area to enter the value of a literal and a drop down to select a language.
    TextAreaWithLangEditor, "TextAreaWithLangEditor",
    /// `Text field editor`: A simple input field to enter the value of a literal, without the ability to change language or datatype.  This is the fallback editor for any literal if no other editors are more suitable.
    TextFieldEditor, "TextFieldEditor",
    /// `Text field with lang editor`: A single-line input field to enter the value of a literal and a drop down to select language, which is mandatory unless xsd:string is among the permissible datatypes.
    TextFieldWithLangEditor, "TextFieldWithLangEditor",
    /// `URI editor`: An input field to enter the URI of a resource, e.g. rdfs:seeAlso links or images.
    URIEditor, "URIEditor",
    /// `URI viewer`: A Viewer for URI resources, rendering as a hyperlink to that URI. Also includes other ways of interacting with the URI such as opening a nested summary display.
    URIViewer, "URIViewer",
    /// `Unique value for class constraint component`: A constraint component that can be used to state that the values of a property must be unique for all instances of a given class (and its subclasses).
    UniqueValueForClassConstraintComponent, "UniqueValueForClassConstraintComponent",
    /// `UniqueValueForClassConstraintComponent-uniqueValueForClass`: 
    UniqueValueForClassConstraintComponent_uniqueValueForClass, "UniqueValueForClassConstraintComponent-uniqueValueForClass",
    /// `Untrusted HTML viewer`: A Viewer for HTML content from untrusted sources. This viewer will sanitize the HTML before rendering. Any a, button, checkbox, form, hidden, input, img, script, select, style and textarea tags and class and style attributes will be removed.
    UntrustedHTMLViewer, "UntrustedHTMLViewer",
    /// `Validation test case`: Abstract superclass for test cases concerning SHACL constraint validation. Future versions may add new kinds of validatin test cases, e.g. to validate a single resource only.
    ValidationTestCase, "ValidationTestCase",
    /// `Value table viewer`: A viewer that renders all values of a given property as a table, with one value per row, and the columns defined by the shape that is the sh:node or sh:class of the property.
    ValueTableViewer, "ValueTableViewer",
    /// `Viewer`: The class of widgets for viewing value nodes.
    Viewer, "Viewer",
    /// `Widget`: Base class of user interface components that can be used to display or edit value nodes.
    Widget, "Widget",
    /// `abstract`: Indicates that a class is "abstract" and cannot be used in asserted rdf:type triples. Only non-abstract subclasses of abstract classes should be instantiated directly.
    abstract_, "abstract",
    /// `action group`: Links an Action with the ActionGroup that it should be arranged in.
    actionGroup, "actionGroup",
    /// `action icon class`: The (CSS) class of an Action for display purposes alongside the label.
    actionIconClass, "actionIconClass",
    /// `added triple`: May link a dash:GraphUpdate with one or more triples (represented as instances of rdf:Statement) that should be added to fix the source of the result.
    addedTriple, "addedTriple",
    /// `all`: Represents all users/roles, for example as a possible value of the default view for role property.
    all, "all",
    /// `applicable to class`: Can be used to state that a shape is applicable to instances of a given class. This is a softer statement than "target class": a target means that all instances of the class must conform to the shape. Being applicable to simply means that the shape may apply to (some) instances of the class. This information can be used by algorithms or humans.
    applicableToClass, "applicableToClass",
    /// `arg`: 
    arg, "arg",
    /// `cachable`: If set to true then the results of the SHACL function can be cached in between invocations with the same arguments. In other words, they are stateless and do not depend on triples in any graph, or the current time stamp etc.
    cachable, "cachable",
    /// `closed by types`: 
    closedByTypes, "closedByTypes",
    /// `co-exists with`: Specifies a property that must have a value whenever the property path has a value, and must have no value whenever the property path has no value.
    coExistsWith, "coExistsWith",
    /// `composite`: Can be used to indicate that a property/path represented by a property constraint represents a composite relationship. In a composite relationship, the life cycle of a "child" object (value of the property/path) depends on the "parent" object (focus node). If the parent gets deleted, then the child objects should be deleted, too. Tools may use dash:composite (if set to true) to implement cascading delete operations.
    composite, "composite",
    /// `constraintOrShape`: 
    constraintOrShape, "constraintOrShape",
    /// `default language`: Can be used to annotate a graph (usually the owl:Ontology) with the default language that tools should suggest for new literal values. For example, predominantly English graphs should have "en" as default language.
    defaultLang, "defaultLang",
    /// `default view for role`: Links a node shape with the roles for which it shall be used as default view. User interfaces can use these values to select how to present a given RDF resource. The values of this property are URIs representing a group of users or agents. There is a dedicated URI dash:all representing all users.
    defaultViewForRole, "defaultViewForRole",
    /// `deleted triple`: May link a dash:GraphUpdate result with one or more triples (represented as instances of rdf:Statement) that should be deleted to fix the source of the result.
    deletedTriple, "deletedTriple",
    /// `dependency predicate`: Can be used in dash:js node expressions to enumerate the predicates that the computation of the values may depend on. This can be used by clients to determine whether an edit requires re-computation of values on a form or elsewhere. For example, if the dash:js is something like "focusNode.firstName + focusNode.lastName" then the dependency predicates should be ex:firstName and ex:lastName.
    dependencyPredicate, "dependencyPredicate",
    /// `details endpoint`: Can be used to link a SHACL property shape with the URL of a SPARQL endpoint that may contain further RDF triples for the value nodes delivered by the property. This can be used to inform a processor that it should switch to values from an external graph when the user wants to retrieve more information about a value.  This property should be regarded as an "annotation", i.e. it does not have any impact on validation or other built-in SHACL features. However, selected tools may want to use this information. One implementation strategy would be to periodically fetch the values specified by the sh:node or sh:class shape associated with the property, using the property shapes in that shape, and add the resulting triples into the main query graph.  An example value is "https://query.wikidata.org/sparql".
    detailsEndpoint, "detailsEndpoint",
    /// `details graph`: Can be used to link a SHACL property shape with a SHACL node expression that produces the URIs of one or more graphs that contain further RDF triples for the value nodes delivered by the property. This can be used to inform a processor that it should switch to another data graph when the user wants to retrieve more information about a value.  The node expressions are evaluated with the focus node as input. (It is unclear whether there are also cases where the result may be different for each specific value, in which case the node expression would need a second input argument).  This property should be regarded as an "annotation", i.e. it does not have any impact on validation or other built-in SHACL features. However, selected tools may want to use this information.
    detailsGraph, "detailsGraph",
    /// `editor`: Can be used to link a property shape with an editor, to state a preferred editing widget in user interfaces.
    editor, "editor",
    /// `excluded prefix`: Specifies a prefix that shall be excluded from the Script code generation.
    excludedPrefix, "excludedPrefix",
    /// `expected result`: The expected result(s) of a test case. The value range of this property is different for each kind of test cases.
    expectedResult, "expectedResult",
    /// `expected result is JSON`: A flag to indicate that the expected result represents a JSON string. If set to true, then tests would compare JSON structures (regardless of whitespaces) instead of actual syntax.
    expectedResultIsJSON, "expectedResultIsJSON",
    /// `expected result is Turtle`: A flag to indicate that the expected result represents an RDF graph encoded as a Turtle file. If set to true, then tests would compare graphs instead of actual syntax.
    expectedResultIsTTL, "expectedResultIsTTL",
    /// `fixed`: Can be used to mark that certain validation results have already been fixed.
    fixed, "fixed",
    /// `has class`: 
    hasClass, "hasClass",
    /// `has max exclusive`: Checks whether a given node (?value) has a value less than (<) the provided ?maxExclusive. Returns false if this cannot be determined, e.g. because values do not have comparable types.
    hasMaxExclusive, "hasMaxExclusive",
    /// `has max inclusive`: Checks whether a given node (?value) has a value less than or equal to (<=) the provided ?maxInclusive. Returns false if this cannot be determined, e.g. because values do not have comparable types.
    hasMaxInclusive, "hasMaxInclusive",
    /// `has max length`: Checks whether a given string (?value) has a length within a given maximum string length.
    hasMaxLength, "hasMaxLength",
    /// `has min exclusive`: Checks whether a given node (?value) has value greater than (>) the provided ?minExclusive. Returns false if this cannot be determined, e.g. because values do not have comparable types.
    hasMinExclusive, "hasMinExclusive",
    /// `has min inclusive`: Checks whether a given node (?value) has value greater than or equal to (>=) the provided ?minInclusive. Returns false if this cannot be determined, e.g. because values do not have comparable types.
    hasMinInclusive, "hasMinInclusive",
    /// `has min length`: Checks whether a given string (?value) has a length within a given minimum string length.
    hasMinLength, "hasMinLength",
    /// `has node kind`: Checks whether a given node (?value) has a given sh:NodeKind (?nodeKind). For example, sh:hasNodeKind(42, sh:Literal) = true.
    hasNodeKind, "hasNodeKind",
    /// `has pattern`: Checks whether the string representation of a given node (?value) matches a given regular expression (?pattern). Returns false if the value is a blank node.
    hasPattern, "hasPattern",
    /// `has root class`: 
    hasRootClass, "hasRootClass",
    /// `has stem`: Checks whether a given node is an IRI starting with a given stem.
    hasStem, "hasStem",
    /// `has value in`: Specifies a constraint that at least one of the value nodes must be a member of the given list.
    hasValueIn, "hasValueIn",
    /// `has value with class`: Specifies a constraint that at least one of the value nodes must be an instance of a given class.
    hasValueWithClass, "hasValueWithClass",
    /// `height`: The height.
    height, "height",
    /// `hidden`: Properties marked as hidden do not appear in user interfaces, yet remain part of the shape for other purposes such as validation and scripting or GraphQL schema generation.
    hidden, "hidden",
    /// `included execution platform`: Can be used to state that one (subject) execution platform includes all features of another platform (object).
    includedExecutionPlatform, "includedExecutionPlatform",
    /// `index`: 
    index, "index",
    /// `indexed`: 
    indexed, "indexed",
    /// `is deactivated`: Checks whether a given shape or constraint has been marked as "deactivated" using sh:deactivated.
    isDeactivated, "isDeactivated",
    /// `is in`: 
    isIn, "isIn",
    /// `is language in`: 
    isLanguageIn, "isLanguageIn",
    /// `is NodeKind BlankNode`: Checks if a given sh:NodeKind is one that includes BlankNodes.
    isNodeKindBlankNode, "isNodeKindBlankNode",
    /// `is NodeKind IRI`: Checks if a given sh:NodeKind is one that includes IRIs.
    isNodeKindIRI, "isNodeKindIRI",
    /// `is NodeKind Literal`: Checks if a given sh:NodeKind is one that includes Literals.
    isNodeKindLiteral, "isNodeKindLiteral",
    /// `is subclass of`: Returns true if a given class (first argument) is a subclass of a given other class (second argument), or identical to that class. This is equivalent to an rdfs:subClassOf* check.
    isSubClassOf, "isSubClassOf",
    /// `isSubClassOf-subclass`: 
    isSubClassOf_subclass, "isSubClassOf-subclass",
    /// `isSubClassOf-superclass`: 
    isSubClassOf_superclass, "isSubClassOf-superclass",
    /// `JavaScript source code`: The JavaScript source code of a Script.
    js, "js",
    /// `local constraint`: Can be set to true for those constraint components where the validation does not require to visit any other triples than the shape definitions and the direct property values of the focus node mentioned in the property constraints. Examples of this include sh:minCount and sh:hasValue.  Constraint components that are marked as such can be optimized by engines, e.g. they can be evaluated client-side at form submission time, without having to make a round-trip to a server, assuming the client has downloaded a complete snapshot of the resource.  Any component marked with dash:staticConstraint is also a dash:localConstraint.
    localConstraint, "localConstraint",
    /// `local values`: If set to true at a property shape then any sh:values rules of this property will be ignored when 'all inferences' are computed. This is useful for property values that shall only be computed for individual focus nodes (e.g. when a user visits a resource) but not for large inference runs.
    localValues, "localValues",
    /// `mime types`: For file-typed properties, this can be used to specify the expected/allowed mime types of its values. This can be used, for example, to limit file input boxes or file selectors. If multiple values are allowed then they need to be separated by commas.  Example values are listed at https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/MIME_types/Common_types
    mimeTypes, "mimeTypes",
    /// `nodeKind`: 
    nodeKind, "nodeKind",
    /// `nonRecursive`: 
    nonRecursive, "nonRecursive",
    /// `object`: 
    object, "object",
    /// `on all values`: If set to true for a ScriptConstraint or ScriptValidator, then the associated script will receive all value nodes at once, as a value of the variable values. By default (or false), the script is called for each value node individually.
    onAllValues, "onAllValues",
    /// `predicate`: 
    predicate, "predicate",
    /// `property suggestion generator`: Links the constraint component with instances of dash:SuggestionGenerator that may be used to produce suggestions for a given validation result that was produced by a property constraint.
    propertySuggestionGenerator, "propertySuggestionGenerator",
    /// `read only`: Used as a hint for user interfaces that values of the associated property should not be editable. The values of this may be the boolean literals true or false or, more generally, a SHACL node expression that must evaluate to true or false.
    readOnly, "readOnly",
    /// `reifiable by`: Can be used to specify the node shape that may be applied to reified statements produced by a property shape. The property shape must have a URI resource as its sh:path. The values of this property must be node shapes. User interfaces can use this information to determine which properties to present to users when reified statements are explored or edited. Use dash:None to indicate that no reification should be permitted.
    reifiableBy, "reifiableBy",
    /// `required execution platform`: Links a SPARQL executable with the platforms that it can be executed on. This can be used by a SHACL implementation to determine whether a constraint validator or rule shall be ignored based on the current platform. For example, if a SPARQL query uses a function or magic property that is only available in TopBraid then a non-TopBraid platform can ignore the constraint (or simply always return no validation results). If this property has no value then the assumption is that the execution will succeed. As soon as one value exists, the assumption is that the engine supports at least one of the given platforms.
    requiredExecutionPlatform, "requiredExecutionPlatform",
    /// `resource action`: Links a class with the Resource Actions that can be applied to instances of that class.
    resourceAction, "resourceAction",
    /// `root class`: 
    rootClass, "rootClass",
    /// `scriptConstraint`: 
    scriptConstraint, "scriptConstraint",
    /// `shape`: States that a subject resource has a given shape. This property can, for example, be used to capture results of SHACL validation on static data.
    shape, "shape",
    /// `shape script`: 
    shapeScript, "shapeScript",
    /// `single line`: 
    singleLine, "singleLine",
    /// `static constraint`: Can be set to true for those constraint components where the validation does not require to visit any other triples than the parameters. Examples of this include sh:datatype or sh:nodeKind, where no further triples need to be queried to determine the result.  Constraint components that are marked as such can be optimized by engines, e.g. they can be evaluated client-side at form submission time, without having to make a round-trip to a server.
    staticConstraint, "staticConstraint",
    /// `stem`: Specifies a string value that the IRI of the value nodes must start with.
    stem, "stem",
    /// `sub set of`: 
    subSetOf, "subSetOf",
    /// `subclass`: 
    subclass, "subclass",
    /// `subject`: 
    subject, "subject",
    /// `suggestion`: Can be used to link a result with one or more suggestions on how to address or improve the underlying issue.
    suggestion, "suggestion",
    /// `suggestion confidence`: An optional confidence between 0% and 100%. Suggestions with 100% confidence are strongly recommended. Can be used to sort recommended updates.
    suggestionConfidence, "suggestionConfidence",
    /// `suggestion generator`: Links a sh:SPARQLConstraint or sh:JSConstraint with instances of dash:SuggestionGenerator that may be used to produce suggestions for a given validation result that was produced by the constraint.
    suggestionGenerator, "suggestionGenerator",
    /// `suggestion`: Can be used to link a suggestion with the group identifier to which it belongs. By default this is a link to the dash:SuggestionGenerator, but in principle this could be any value.
    suggestionGroup, "suggestionGroup",
    /// `superclass`: 
    superclass, "superclass",
    /// `symmetric`: True to declare that the associated property path is symmetric.
    symmetric, "symmetric",
    /// `template`: 
    template, "template",
    /// `to string`: Returns a literal with datatype xsd:string that has the input value as its string. If the input value is an (URI) resource then its URI will be used.
    toString, "toString",
    /// `unique value for class`: 
    uniqueValueForClass, "uniqueValueForClass",
    /// `uriStart`: 
    uriStart, "uriStart",
    /// `URI template`: Inserts a given value into a given URI template, producing a new xsd:anyURI literal.  In the future this should support RFC 6570 but for now it is limited to simple {...} patterns.
    uriTemplate, "uriTemplate",
    /// `validate shapes`: True to also validate the shapes itself (i.e. parameter declarations).
    validateShapes, "validateShapes",
    /// `value`: 
    value, "value",
    /// `value count`: Computes the number of objects for a given subject/predicate combination.
    valueCount, "valueCount",
    /// `viewer`: Can be used to link a property shape with a viewer, to state a preferred viewing widget in user interfaces.
    viewer, "viewer",
    /// `width`: The width.
    width, "width",
    /// `x`: The x position.
    x, "x",
    /// `y`: The y position.
    y, "y"
);
