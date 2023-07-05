// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `The Hydra Core Vocabulary` vocabulary
//!
//! This module requires `ns-hydra` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|The Hydra Core Vocabulary|
//! |**Prefix**|hydra|
//! |**Namespace base IRI**|<http://www.w3.org/ns/hydra/core#>|
//! |**Description**|The Hydra Core Vocabulary is a lightweight vocabulary to create hypermedia-driven Web APIs. By specifying a number of concepts commonly used in Web APIs it enables the creation of generic API clients.|
//! |**Is defined by**|<http://www.w3.org/ns/hydra/core#>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/ns/hydra/core#",

    terms: [
        /// `ApiDocumentation`: The Hydra API documentation class
        (ApiDocumentation, "ApiDocumentation"),
        /// `Base Uri source`: Provides a base abstract for base Uri source for Iri template resolution.
        (BaseUriSource, "BaseUriSource"),
        /// `BasicRepresentation`: A representation that serializes just the lexical form of a variable value, but omits language and type information.
        (BasicRepresentation, "BasicRepresentation"),
        /// `Hydra Class`: The class of Hydra classes.
        (Class, "Class"),
        /// `Collection`: A collection holding references to a number of related resources.
        (Collection, "Collection"),
        /// `Error`: A runtime error, used to report information beyond the returned status code.
        (Error, "Error"),
        /// `ExplicitRepresentation`: A representation that serializes a variable value including its language and type information and thus differentiating between IRIs and literals.
        (ExplicitRepresentation, "ExplicitRepresentation"),
        /// `Header specification`: Specifies a possible either expected or returned header values
        (HeaderSpecification, "HeaderSpecification"),
        /// `IRI Template`: The class of IRI templates.
        (IriTemplate, "IriTemplate"),
        /// `IriTemplateMapping`: A mapping from an IRI template variable to a property.
        (IriTemplateMapping, "IriTemplateMapping"),
        /// `Link`: The class of properties representing links.
        (Link, "Link"),
        /// `Link context`: States that the link's context IRI, as defined in RFC 5988, should be used as the base Uri
        (LinkContext, "LinkContext"),
        /// `Operation`: An operation.
        (Operation, "Operation"),
        /// `PartialCollectionView`: A PartialCollectionView describes a partial view of a Collection. Multiple PartialCollectionViews can be connected with the the next/previous properties to allow a client to retrieve all members of the collection.
        (PartialCollectionView, "PartialCollectionView"),
        /// `Hydra Resource`: The class of dereferenceable resources by means a client can attempt to dereference; however, the received responses should still be verified.
        (Resource, "Resource"),
        /// `RFC 3986 based`: States that the base Uri should be established using RFC 3986 reference resolution algorithm specified in section 5.
        (Rfc3986, "Rfc3986"),
        /// `RFC6570 IRI template`: An IRI template as defined by RFC6570.
        (Rfc6570Template, "Rfc6570Template"),
        /// `Status code description`: Additional information about a status code that might be returned.
        (Status, "Status"),
        /// `Supported Property`: A property known to be supported by a Hydra class.
        (SupportedProperty, "SupportedProperty"),
        /// `Templated Link`: A templated link.
        (TemplatedLink, "TemplatedLink"),
        /// `VariableRepresentation`: A representation specifies how to serialize variable values into strings.
        (VariableRepresentation, "VariableRepresentation"),
        /// `apiDocumentation`: A link to the API documentation
        (apiDocumentation, "apiDocumentation"),
        /// `closed set`: Determines whether the provided set of header values is closed or not.
        (closedSet, "closedSet"),
        /// `collection`: Collections somehow related to this resource.
        (collection, "collection"),
        /// `description`: A description.
        (description, "description"),
        /// `entrypoint`: A link to main entry point of the Web API
        (entrypoint, "entrypoint"),
        /// `expects`: The information expected by the Web API.
        (expects, "expects"),
        /// `expects header`: Specification of the header expected by the operation.
        (expectsHeader, "expectsHeader"),
        /// `extension`: Hint on what kind of extensions are in use.
        (extension, "extension"),
        /// `first`: The first resource of an interlinked set of resources.
        (first, "first"),
        /// `freetext query`: A property representing a freetext query.
        (freetextQuery, "freetextQuery"),
        /// `header name`: Name of the header.
        (headerName, "headerName"),
        /// `last`: The last resource of an interlinked set of resources.
        (last, "last"),
        /// `take`: Instructs to limit set only to N elements.
        (limit, "limit"),
        /// `manages`: This predicate is left for compatibility purposes and hydra:memberAssertion should be used instead.
        (manages, "manages"),
        /// `mapping`: A variable-to-property mapping of the IRI template.
        (mapping, "mapping"),
        /// `member`: A member of the collection
        (member, "member"),
        /// `member assertion`: Semantics of each member provided by the collection.
        (memberAssertion, "memberAssertion"),
        /// `method`: The HTTP method.
        (method, "method"),
        /// `next`: The resource following the current instance in an interlinked set of resources.
        (next, "next"),
        /// `object`: The object.
        (object, "object"),
        /// `skip`: Instructs to skip N elements of the set.
        (offset, "offset"),
        /// `operation`: An operation supported by the Hydra resource
        (operation, "operation"),
        /// `page index`: Instructs to provide a specific page of the collection at a given index.
        (pageIndex, "pageIndex"),
        /// `page reference`: Instructs to provide a specific page reference of the collection.
        (pageReference, "pageReference"),
        /// `possible status`: A status that might be returned by the Web API (other statuses should be expected and properly handled as well)
        (possibleStatus, "possibleStatus"),
        /// `possible header value`: Possible value of the header.
        (possibleValue, "possibleValue"),
        /// `previous`: The resource preceding the current instance in an interlinked set of resources.
        (previous, "previous"),
        /// `property`: A property
        (property, "property"),
        /// `readable`: True if the client can retrieve the property's value, false otherwise.
        (readable, "readable"),
        /// `required`: True if the property is required, false otherwise.
        (required, "required"),
        /// `relative Uri resolution`:
        (resolveRelativeUsing, "resolveRelativeUsing"),
        /// `returns`: The information returned by the Web API on success
        (returns, "returns"),
        /// `returns header`: Name of the header returned by the operation.
        (returnsHeader, "returnsHeader"),
        /// `search`: A IRI template that can be used to query a collection.
        (search, "search"),
        /// `status code`: The HTTP status code. Please note it may happen this value will be different to actual status code received.
        (statusCode, "statusCode"),
        /// `subject`: The subject.
        (subject, "subject"),
        /// `supported classes`: A class known to be supported by the Web API
        (supportedClass, "supportedClass"),
        /// `supported operation`: An operation supported by instances of the specific Hydra class, or the target of the Hydra link, or IRI template.
        (supportedOperation, "supportedOperation"),
        /// `supported properties`: The properties known to be supported by a Hydra class
        (supportedProperty, "supportedProperty"),
        /// `template`: A templated string with placeholders. The literal's datatype indicates the template syntax; if not specified, hydra:Rfc6570Template is assumed.
        (template, "template"),
        /// `title`: A title, often used along with a description.
        (title, "title"),
        /// `total items`: The total number of items referenced by a collection.
        (totalItems, "totalItems"),
        /// `variable`: An IRI template variable
        (variable, "variable"),
        /// `variable representation`: The representation format to use when expanding the IRI template.
        (variableRepresentation, "variableRepresentation"),
        /// `view`: A specific view of a resource.
        (view, "view"),
        /// `writable`: True if the client can change the property's value, false otherwise.
        (writable, "writable"),
        /// `writable`: This property is left for compatibility purposes and hydra:writable should be used instead.
        (writeable, "writeable")    ]
);
