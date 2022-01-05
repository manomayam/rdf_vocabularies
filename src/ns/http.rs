// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `HTTP in RDF` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|HTTP in RDF|
//! |**Prefix**|http|
//! |**Namespace base IRI**|[http://www.w3.org/2011/http#](http://www.w3.org/2011/http#)|
//! |**Description**|A namespace for describing HTTP messages (http://www.w3.org/Protocols/rfc2616/rfc2616.html)|
//! |**Is defined by**|[http://www.w3.org/2011/http#](http://www.w3.org/2011/http#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/2011/http#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Connection`: A connection used for HTTP transfer.
    Connection, "Connection",
    /// `Entity Header`: An entity header in an HTTP message.
    EntityHeader, "EntityHeader",
    /// `General Header`: A general header in an HTTP message.
    GeneralHeader, "GeneralHeader",
    /// `Header Element`: A part of a deconstructed header value.
    HeaderElement, "HeaderElement",
    /// `Header Name`: A header name.
    HeaderName, "HeaderName",
    /// `Message`: An HTTP message.
    Message, "Message",
    /// `Message Header`: A header in an HTTP message.
    MessageHeader, "MessageHeader",
    /// `Method`: The HTTP method used for the request.
    Method, "Method",
    /// `Parameter`: A parameter for a part of a header value.
    Parameter, "Parameter",
    /// `Request`: An HTTP request.
    Request, "Request",
    /// `Request Header`: A header in an HTTP request message.
    RequestHeader, "RequestHeader",
    /// `Response`: An HTTP response.
    Response, "Response",
    /// `Response Header`: A header in an HTTP response message.
    ResponseHeader, "ResponseHeader",
    /// `Status code`: The status code of an HTTP response.
    StatusCode, "StatusCode",
    /// `Absolute path`: The absolute path sort of request URI.
    absolutePath, "absolutePath",
    /// `Absolute URI`: The absolute request URI.
    absoluteURI, "absoluteURI",
    /// `Authority`: The authority sort of request URI.
    authority, "authority",
    /// `Entity Body`: The entity body of an HTTP message.
    body, "body",
    /// `Connection authority`: The authority of a connection used for the HTTP transfer.
    connectionAuthority, "connectionAuthority",
    /// `Header element name`: The name of a header element.
    elementName, "elementName",
    /// `Header element value`: The value of a header element.
    elementValue, "elementValue",
    /// `Field name`: The name of an HTTP header field.
    fieldName, "fieldName",
    /// `Field value`: The value of an HTTP header field.
    fieldValue, "fieldValue",
    /// `Header name`: The name of an HTTP header.
    hdrName, "hdrName",
    /// `Header elements`: The deconstructed parts of an HTTP header value.
    headerElements, "headerElements",
    /// `Headers`: The headers in an HTTP message.
    headers, "headers",
    /// `HTTP version`: The HTTP version of an HTTP message.
    httpVersion, "httpVersion",
    /// `Method name`: The HTTP method name used for the HTTP request.
    methodName, "methodName",
    /// `Method`: The HTTP method used for the HTTP request.
    mthd, "mthd",
    /// `Parameter name`: The name of a parameter in a part of a deconstructed HTTP header value.
    paramName, "paramName",
    /// `Parameter value`: The value of a parameter in a part of a deconstructed HTTP header value.
    paramValue, "paramValue",
    /// `Header parameters`: The parameters in a part of a deconstructed HTTP header value.
    params, "params",
    /// `Reason phrase`: The reason phrase (status text) of an HTTP response.
    reasonPhrase, "reasonPhrase",
    /// `Request URI`: The request URI of an HTTP request.
    requestURI, "requestURI",
    /// `Requests`: The HTTP requests made via a connection.
    requests, "requests",
    /// `Response`: The HTTP response sent in answer to an HTTP request.
    resp, "resp",
    /// `Status code`: The status code of an HTTP response.
    sc, "sc",
    /// `Status code`: The status code number.
    statusCodeNumber, "statusCodeNumber",
    /// `Status code`: The status code value of an HTTP response.
    statusCodeValue, "statusCodeValue"
);
