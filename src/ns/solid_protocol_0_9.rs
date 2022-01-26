// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Solid Protocol` vocabulary
//!
//! This module requires `ns-solid_protocol_0_9` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Solid Protocol|
//! |**Prefix**|solid_protocol_0_9|
//! |**Namespace base IRI**|<https://solidproject.org/TR/protocol#>|
//! |**Description**|This document connects a set of specifications that, together, provide applications with secure and permissioned access to externally stored data in an interoperable way.|
//! |**Is defined by**|<http://rdf.greggkellogg.net/distiller?command=serialize&output_format=ntriples&url=https:%2F%2Fsolidproject.org%2FTR%2Fprotocol&raw>|
//!

use crate::namespace;

namespace!(
    "https://solidproject.org/TR/protocol#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `a4448776-625a-11ec-b335-d7489507d779`: 
    a4448776_625a_11ec_b335_d7489507d779, "a4448776-625a-11ec-b335-d7489507d779",
    /// `accept-put`: 
    accept_put, "accept-put",
    /// `accessibility-considerations`: 
    accessibility_considerations, "accessibility-considerations",
    /// `agent`: 
    agent, "agent",
    /// `append-operation`: 
    append_operation, "append-operation",
    /// `authentication`: 
    authentication, "authentication",
    /// `authorization`: 
    authorization, "authorization",
    /// `auxiliary-resources`: 
    auxiliary_resources, "auxiliary-resources",
    /// `auxiliary-resources-description-resource`: 
    auxiliary_resources_description_resource, "auxiliary-resources-description-resource",
    /// `auxiliary-resources-web-access-control`: 
    auxiliary_resources_web_access_control, "auxiliary-resources-web-access-control",
    /// `back-to-top`: 
    back_to_top, "back-to-top",
    /// `client-authentication`: Clients MUST conform to HTTP/1.1 Authentication [RFC7235] if it needs to access resources requiring authentication (see WebID).
    client_authentication, "client-authentication",
    /// `client-authentication-different-credentials`: When a client receives a response with a 403 or 404 status code, the client MAY repeat the request with different credentials.
    client_authentication_different_credentials, "client-authentication-different-credentials",
    /// `client-caching`: Clients MAY conform to HTTP/1.1 Caching [RFC7234].
    client_caching, "client-caching",
    /// `client-conditional-requests`: Clients MAY conform to HTTP/1.1 Conditional Requests [RFC7232].
    client_conditional_requests, "client-conditional-requests",
    /// `client-content-type`: Clients MUST use the Content-Type HTTP header in PUT, POST and PATCH requests [RFC7231].
    client_content_type, "client-content-type",
    /// `client-http-11`: Clients MUST conform to HTTP/1.1 Message Syntax and Routing [RFC7230] and HTTP/1.1 Semantics and Content [RFC7231].
    client_http_11, "client-http-11",
    /// `client-http-2`: Clients MAY conform to HTTP/2 [RFC7540].
    client_http_2, "client-http-2",
    /// `client-ldn`: A Solid client MUST conform to the LDN specification by implementing the Sender or Consumer parts to discover the location of a resource’s Inbox, and to send notifications to an Inbox or to retrieve the contents of an Inbox [LDN].
    client_ldn, "client-ldn",
    /// `client-link-auxiliary-type`: 
    client_link_auxiliary_type, "client-link-auxiliary-type",
    /// `client-link-describes`: 
    client_link_describes, "client-link-describes",
    /// `client-link-storage`: 
    client_link_storage, "client-link-storage",
    /// `client-range-requests`: Clients MAY conform to HTTP/1.1 Range Requests [RFC7233].
    client_range_requests, "client-range-requests",
    /// `client-rdf-storage`: 
    client_rdf_storage, "client-rdf-storage",
    /// `client-storage-discovery`: 
    client_storage_discovery, "client-storage-discovery",
    /// `client-wac`: Clients MUST conform to the Web Access Control specification [WAC].
    client_wac, "client-wac",
    /// `client-websockets-api`: Clients SHOULD implement the Solid WebSockets API [SOLID-WEBSOCKETS-API].
    client_websockets_api, "client-websockets-api",
    /// `conditional-update`: 
    conditional_update, "conditional-update",
    /// `conformance`: 
    conformance, "conformance",
    /// `considerations`: 
    considerations, "considerations",
    /// `contained-resource-metadata`: 
    contained_resource_metadata, "contained-resource-metadata",
    /// `contained-resource-metadata-considerations`: 
    contained_resource_metadata_considerations, "contained-resource-metadata-considerations",
    /// `contained-resource-metadata-dcterms-modified`: 
    contained_resource_metadata_dcterms_modified, "contained-resource-metadata-dcterms-modified",
    /// `contained-resource-metadata-rdf-type`: 
    contained_resource_metadata_rdf_type, "contained-resource-metadata-rdf-type",
    /// `contained-resource-metadata-stat-mtime`: 
    contained_resource_metadata_stat_mtime, "contained-resource-metadata-stat-mtime",
    /// `contained-resource-metadata-stat-size`: 
    contained_resource_metadata_stat_size, "contained-resource-metadata-stat-size",
    /// `contained-resource-metadata-statements`: 
    contained_resource_metadata_statements, "contained-resource-metadata-statements",
    /// `container-last-modified-comparison`: 
    container_last_modified_comparison, "container-last-modified-comparison",
    /// `container-resource`: 
    container_resource, "container-resource",
    /// `cors`: 
    cors, "cors",
    /// `cors-protocol-blocking`: 
    cors_protocol_blocking, "cors-protocol-blocking",
    /// `cors-server`: 
    cors_server, "cors-server",
    /// `data-pod`: 
    data_pod, "data-pod",
    /// `deleting-resources`: 
    deleting_resources, "deleting-resources",
    /// `document-permission`: 
    document_permission, "document-permission",
    /// `document-policy-offer`: 
    document_policy_offer, "document-policy-offer",
    /// `http`: 
    http, "http",
    /// `http-client`: 
    http_client, "http-client",
    /// `http-definitions`: 
    http_definitions, "http-definitions",
    /// `http-headers`: 
    http_headers, "http-headers",
    /// `http-server`: 
    http_server, "http-server",
    /// `identifiable-information`: 
    identifiable_information, "identifiable-information",
    /// `identity`: 
    identity, "identity",
    /// `informative-references`: 
    informative_references, "informative-references",
    /// `internationalization-considerations`: 
    internationalization_considerations, "internationalization-considerations",
    /// `introduction`: 
    introduction, "introduction",
    /// `live-update`: 
    live_update, "live-update",
    /// `n3-patch`: 
    n3_patch, "n3-patch",
    /// `n3-patch-example`: 
    n3_patch_example, "n3-patch-example",
    /// `namespaces`: 
    namespaces, "namespaces",
    /// `normative-references`: 
    normative_references, "normative-references",
    /// `notifications`: 
    notifications, "notifications",
    /// `origin`: 
    origin, "origin",
    /// `owner`: 
    owner, "owner",
    /// `privacy-considerations`: 
    privacy_considerations, "privacy-considerations",
    /// `read-operation`: 
    read_operation, "read-operation",
    /// `reading-resources`: 
    reading_resources, "reading-resources",
    /// `reading-writing-resources`: 
    reading_writing_resources, "reading-writing-resources",
    /// `references`: 
    references, "references",
    /// `resource`: 
    resource, "resource",
    /// `resource-containment`: 
    resource_containment, "resource-containment",
    /// `resource-metadata`: 
    resource_metadata, "resource-metadata",
    /// `resource-representations`: 
    resource_representations, "resource-representations",
    /// `resource-type-heuristics`: 
    resource_type_heuristics, "resource-type-heuristics",
    /// `resources`: 
    resources, "resources",
    /// `root-container`: 
    root_container, "root-container",
    /// `security-considerations`: 
    security_considerations, "security-considerations",
    /// `security-privacy-review`: 
    security_privacy_review, "security-privacy-review",
    /// `security-privacy-review-considerations`: 
    security_privacy_review_considerations, "security-privacy-review-considerations",
    /// `security-privacy-review-first-third-party`: 
    security_privacy_review_first_third_party, "security-privacy-review-first-third-party",
    /// `security-privacy-review-minimum-data`: 
    security_privacy_review_minimum_data, "security-privacy-review-minimum-data",
    /// `security-privacy-review-native-ui`: 
    security_privacy_review_native_ui, "security-privacy-review-native-ui",
    /// `security-privacy-review-non-fully-active`: 
    security_privacy_review_non_fully_active, "security-privacy-review-non-fully-active",
    /// `security-privacy-review-other-data`: 
    security_privacy_review_other_data, "security-privacy-review-other-data",
    /// `security-privacy-review-persistent-origin-specific-state`: 
    security_privacy_review_persistent_origin_specific_state, "security-privacy-review-persistent-origin-specific-state",
    /// `security-privacy-review-personal-data`: 
    security_privacy_review_personal_data, "security-privacy-review-personal-data",
    /// `security-privacy-review-private-browsing`: 
    security_privacy_review_private_browsing, "security-privacy-review-private-browsing",
    /// `security-privacy-review-purpose`: 
    security_privacy_review_purpose, "security-privacy-review-purpose",
    /// `security-privacy-review-relaxed-sop`: 
    security_privacy_review_relaxed_sop, "security-privacy-review-relaxed-sop",
    /// `security-privacy-review-remote-device`: 
    security_privacy_review_remote_device, "security-privacy-review-remote-device",
    /// `security-privacy-review-send-to-platform`: 
    security_privacy_review_send_to_platform, "security-privacy-review-send-to-platform",
    /// `security-privacy-review-sensitive-data`: 
    security_privacy_review_sensitive_data, "security-privacy-review-sensitive-data",
    /// `security-privacy-review-sensor-data`: 
    security_privacy_review_sensor_data, "security-privacy-review-sensor-data",
    /// `security-privacy-review-string-to-script`: 
    security_privacy_review_string_to_script, "security-privacy-review-string-to-script",
    /// `security-privacy-review-temporary-id`: 
    security_privacy_review_temporary_id, "security-privacy-review-temporary-id",
    /// `security-privacy-review-underlying-platform-data`: 
    security_privacy_review_underlying_platform_data, "security-privacy-review-underlying-platform-data",
    /// `self-describing-resources`: 
    self_describing_resources, "self-describing-resources",
    /// `server-accept-headers`: Servers MUST indicate supported media types in the HTTP Accept-Patch [RFC5789], Accept-Post [LDP] and Accept-Put [The Accept-Put Response Header] response headers that correspond to acceptable HTTP methods listed in Allow header value in response to HTTP GET and HEAD requests.
    server_accept_headers, "server-accept-headers",
    /// `server-allow-methods`: Servers MUST indicate their support for HTTP Methods by responding to HTTP GET and HEAD requests for the target resource with the HTTP Method tokens in the HTTP response header Allow.
    server_allow_methods, "server-allow-methods",
    /// `server-authentication`: Servers MUST conform to HTTP/1.1 Authentication [RFC7235].
    server_authentication, "server-authentication",
    /// `server-authorization-redirect`: Servers MUST authorize prior to this optional redirect.
    server_authorization_redirect, "server-authorization-redirect",
    /// `server-basic-container`: The representation and behaviour of containers in Solid corresponds to LDP Basic Container and MUST be supported by server.
    server_basic_container, "server-basic-container",
    /// `server-caching`: Servers SHOULD conform to HTTP/1.1 Caching [RFC7234].
    server_caching, "server-caching",
    /// `server-conditional-requests`: Servers MUST conform to HTTP/1.1 Conditional Requests [RFC7232].
    server_conditional_requests, "server-conditional-requests",
    /// `server-contained-resource-metadata`: Servers SHOULD include resource metadata about contained resources as part of the container description, unless that information is inapplicable to the server.
    server_contained_resource_metadata, "server-contained-resource-metadata",
    /// `server-content-type`: Server MUST reject PUT, POST and PATCH requests without the Content-Type header with a status code of 400.
    server_content_type, "server-content-type",
    /// `server-cors`: A server MUST implement the CORS protocol [FETCH] such that, to the extent possible, the browser allows Solid apps to send any request and combination of request headers to the server, and the Solid app can read any response and response headers received from the server. If the server wishes to block access to a resource, this MUST NOT happen via CORS but MUST instead be communicated to the Solid app in the browser through HTTP status codes such as 401, 403, or 404 [RFC7231].
    server_cors, "server-cors",
    /// `server-cors-acao-vary`: the server MUST set the Access-Control-Allow-Origin header to the valid Origin value from the request and list Origin in the Vary header value.
    server_cors_acao_vary, "server-cors-acao-vary",
    /// `server-cors-accept-acah`: Servers SHOULD also explicitly list Accept under Access-Control-Allow-Headers
    server_cors_accept_acah, "server-cors-accept-acah",
    /// `server-cors-access-control-headers`: whenever a server receives an HTTP request containing a valid Origin header [RFC6454], the server MUST respond with the appropriate Access-Control-* headers as specified in the CORS protocol [FETCH].
    server_cors_access_control_headers, "server-cors-access-control-headers",
    /// `server-cors-aceh`: The server MUST make all used response headers readable for the Solid app through Access-Control-Expose-Headers (with the possible exception of the Access-Control-* headers themselves).
    server_cors_aceh, "server-cors-aceh",
    /// `server-cors-enumerate`: server SHOULD explicitly enumerate all used response headers under Access-Control-Expose-Headers rather than resorting to *, which does not cover all cases (such as credentials mode set to include).
    server_cors_enumerate, "server-cors-enumerate",
    /// `server-cors-options`: A server MUST also support the HTTP OPTIONS method [RFC7231] such that it can respond appropriately to CORS preflight requests.
    server_cors_options, "server-cors-options",
    /// `server-delete-protect-nonempty-container`: If the container contains resources, the server MUST respond with the 409 status code and response body describing the error.
    server_delete_protect_nonempty_container, "server-delete-protect-nonempty-container",
    /// `server-delete-protect-root-container`: When a DELETE request targets storage’s root container or its associated ACL resource, the server MUST respond with the 405 status code.
    server_delete_protect_root_container, "server-delete-protect-root-container",
    /// `server-delete-remove-auxiliary-resource`: When a contained resource is deleted, the server MUST also delete the associated auxiliary resources (see the Auxiliary Resources section).
    server_delete_remove_auxiliary_resource, "server-delete-remove-auxiliary-resource",
    /// `server-delete-remove-containment`: When a contained resource is deleted, the server MUST also remove the corresponding containment triple.
    server_delete_remove_containment, "server-delete-remove-containment",
    /// `server-delete-remove-empty-container`: When a DELETE request targets a container, the server MUST delete the container if it contains no resources.
    server_delete_remove_empty_container, "server-delete-remove-empty-container",
    /// `server-description-resource-authorization`: When an HTTP request targets a description resource, the server MUST apply the authorization rule that is used for the subject resource with which the description resource is associated.
    server_description_resource_authorization, "server-description-resource-authorization",
    /// `server-description-resource-max`: Servers MUST NOT directly associate more than one description resource to a subject resource.
    server_description_resource_max, "server-description-resource-max",
    /// `server-disallow-delete`: Server MUST exclude the DELETE method in the HTTP response header Allow in response to requests to these resources [RFC7231].
    server_disallow_delete, "server-disallow-delete",
    /// `server-etag`: Servers MAY use the HTTP ETag header with a strong validator for RDF bearing representations in order to encourage clients to opt-in to using the If-Match header in their requests.
    server_etag, "server-etag",
    /// `server-http-11`: Servers MUST conform to HTTP/1.1 Message Syntax and Routing [RFC7230] and HTTP/1.1 Semantics and Content [RFC7231].
    server_http_11, "server-http-11",
    /// `server-http-2`: Servers SHOULD conform to HTTP/2 [RFC7540].
    server_http_2, "server-http-2",
    /// `server-ldn`: A Solid server MUST conform to the LDN specification by implementing the Receiver parts to receive notifications and make Inbox contents available [LDN].
    server_ldn, "server-ldn",
    /// `server-link-storage`: Servers exposing the storage resource MUST advertise by including the HTTP Link header with rel="type" targeting http://www.w3.org/ns/pim/space#Storage when responding to storage’s request URI.
    server_link_storage, "server-link-storage",
    /// `server-method-not-allowed`: Servers MUST respond with the 405 status code to requests using HTTP methods that are not supported by the target resource.
    server_method_not_allowed, "server-method-not-allowed",
    /// `server-n3-patch-delete`: When ?deletions is non-empty, servers MUST treat the request as a Read and Write operation.
    server_n3_patch_delete, "server-n3-patch-delete",
    /// `server-n3-patch-insert`: When ?insertions is non-empty, servers MUST (also) treat the request as an Append operation.
    server_n3_patch_insert, "server-n3-patch-insert",
    /// `server-n3-patch-where`: When ?conditions is non-empty, servers MUST treat the request as a Read operation.
    server_n3_patch_where, "server-n3-patch-where",
    /// `server-options-asterisk-accept-headers`: Servers MAY include the HTTP Accept-Patch, Accept-Post and Accept-Put headers in the response of a OPTIONS * request.
    server_options_asterisk_accept_headers, "server-options-asterisk-accept-headers",
    /// `server-patch-n3-accept`: Servers MUST accept a PATCH request with an N3 Patch body when the target of the request is an RDF document [RDF11-CONCEPTS].
    server_patch_n3_accept, "server-patch-n3-accept",
    /// `server-patch-n3-advertise`: Servers MUST indicate support of N3 Patch by listing text/n3 as a value of the Accept-Patch header [RFC5789] of relevant responses.
    server_patch_n3_advertise, "server-patch-n3-advertise",
    /// `server-patch-n3-blank-nodes`: The ?insertions and ?deletions formulae MUST NOT contain blank nodes.
    server_patch_n3_blank_nodes, "server-patch-n3-blank-nodes",
    /// `server-patch-n3-default`: 
    server_patch_n3_default, "server-patch-n3-default",
    /// `server-patch-n3-deletes`: A patch resource MUST contain at most one triple of the form ?patch solid:deletes ?deletions.
    server_patch_n3_deletes, "server-patch-n3-deletes",
    /// `server-patch-n3-formulae`: When present, ?deletions, ?insertions, and ?conditions MUST be non-nested cited formulae [N3] consisting only of triples and/or triple patterns [SPARQL11-QUERY]. When not present, they are presumed to be the empty formula {}.
    server_patch_n3_formulae, "server-patch-n3-formulae",
    /// `server-patch-n3-inserts`: 
    server_patch_n3_inserts, "server-patch-n3-inserts",
    /// `server-patch-n3-invalid`: Servers MUST respond with a 422 status code [RFC4918] if a patch document does not satisfy all of the above constraints.
    server_patch_n3_invalid, "server-patch-n3-invalid",
    /// `server-patch-n3-patch-identifier`: A patch resource MUST be identified by a URI or blank node, which we refer to as ?patch in the remainder of this section.
    server_patch_n3_patch_identifier, "server-patch-n3-patch-identifier",
    /// `server-patch-n3-patches`: A patch document MUST contain one or more patch resources.
    server_patch_n3_patches, "server-patch-n3-patches",
    /// `server-patch-n3-semantics`: Servers MUST process a patch resource against the target document as follows:
    server_patch_n3_semantics, "server-patch-n3-semantics",
    /// `server-patch-n3-semantics-deletions-non-empty-all-triples`: server MUST respond with a 409 status code.
    server_patch_n3_semantics_deletions_non_empty_all_triples, "server-patch-n3-semantics-deletions-non-empty-all-triples",
    /// `server-patch-n3-semantics-no-mapping`: server MUST respond with a 409 status code.
    server_patch_n3_semantics_no_mapping, "server-patch-n3-semantics-no-mapping",
    /// `server-patch-n3-single`: The patch document MUST contain exactly one patch resource, identified by one or more of the triple patterns described above, which all share the same ?patch subject.
    server_patch_n3_single, "server-patch-n3-single",
    /// `server-patch-n3-type`: ?patch rdf:type solid:InsertDeletePatch
    server_patch_n3_type, "server-patch-n3-type",
    /// `server-patch-n3-variables`: The ?insertions and ?deletions formulae MUST NOT contain variables that do not occur in the ?conditions formula.
    server_patch_n3_variables, "server-patch-n3-variables",
    /// `server-patch-n3-where`: A patch resource MUST contain at most one triple of the form ?patch solid:where ?conditions.
    server_patch_n3_where, "server-patch-n3-where",
    /// `server-post-container`: Servers MUST allow creating new resources with a POST request to URI path ending /.
    server_post_container, "server-post-container",
    /// `server-post-container-create-container`: Servers MUST create a container with URI path ending /{id}/ in container / for requests including the HTTP Link header with rel="type" targeting a valid LDP container type.
    server_post_container_create_container, "server-post-container-create-container",
    /// `server-post-container-create-resource`: Servers MUST create a resource with URI path ending /{id} in container /.
    server_post_container_create_resource, "server-post-container-create-resource",
    /// `server-post-slug-auxiliary-resource`: When a POST method request with the Slug header targets an auxiliary resource, the server MUST respond with the 403 status code and response body describing the error.
    server_post_slug_auxiliary_resource, "server-post-slug-auxiliary-resource",
    /// `server-post-target-not-found`: When a POST method request targets a resource without an existing representation, the server MUST respond with the 404 status code.
    server_post_target_not_found, "server-post-target-not-found",
    /// `server-post-uri-assignment`: When a successful POST request creates a resource, the server MUST assign a URI to that resource.
    server_post_uri_assignment, "server-post-uri-assignment",
    /// `server-protect-contained-resource-metadata`: Servers MUST NOT allow HTTP POST, PUT and PATCH to update a container’s resource metadata statements; if the server receives such a request, it MUST respond with a 409 status code.
    server_protect_contained_resource_metadata, "server-protect-contained-resource-metadata",
    /// `server-protect-containment`: Servers MUST NOT allow HTTP PUT or PATCH on a container to update its containment triples; if the server receives such a request, it MUST respond with a 409 status code.
    server_protect_containment, "server-protect-containment",
    /// `server-put-patch-auxiliary-resource`: When a PUT or PATCH method request targets an auxiliary resource, the server MUST create or update it.
    server_put_patch_auxiliary_resource, "server-put-patch-auxiliary-resource",
    /// `server-put-patch-intermediate-containers`: Servers MUST create intermediate containers and include corresponding containment triples in container representations derived from the URI path component of PUT and PATCH requests.
    server_put_patch_intermediate_containers, "server-put-patch-intermediate-containers",
    /// `server-put-patch-uri-assignment`: When a successful PUT or PATCH request creates a resource, the server MUST use the effective request URI to assign the URI to that resource.
    server_put_patch_uri_assignment, "server-put-patch-uri-assignment",
    /// `server-range-requests`: Servers MAY conform to HTTP/1.1 Range Requests [RFC7233].
    server_range_requests, "server-range-requests",
    /// `server-representation-turtle-jsonld`: When a server creates a resource on HTTP PUT, POST or PATCH requests such that the request’s representation data encodes an RDF document [RDF11-CONCEPTS] (as determined by the Content-Type header), the server MUST accept GET requests on this resource when the value of the Accept header requests a representation in text/turtle or application/ld+json [Turtle] [JSON-LD11].
    server_representation_turtle_jsonld, "server-representation-turtle-jsonld",
    /// `server-representation-write-redirect`: When a PUT, POST, PATCH or DELETE method request targets a representation URL that is different than the resource URL, the server MUST respond with a 307 or 308 status code and Location header specifying the preferred URI reference.
    server_representation_write_redirect, "server-representation-write-redirect",
    /// `server-safe-methods`: Servers MUST support the HTTP GET, HEAD and OPTIONS methods [RFC7231] for clients to read resources or to determine communication options.
    server_safe_methods, "server-safe-methods",
    /// `server-slug-uri-assignment`: Servers MAY allow clients to suggest the URI of a resource created through POST, using the HTTP Slug header as defined in [RFC5023].
    server_slug_uri_assignment, "server-slug-uri-assignment",
    /// `server-storage`: Servers MUST provide one or more storages (pim:Storage) – a space of URIs in which data can be accessed. A storage is the root container for all of its contained resources (see Resource Containment).
    server_storage, "server-storage",
    /// `server-storage-link-owner`: When a server wants to advertise the owner of a storage, the server MUST include the Link header with rel="http://www.w3.org/ns/solid/terms#owner" targeting the URI of the owner in the response of HTTP HEAD or GET requests targeting the root container.
    server_storage_link_owner, "server-storage-link-owner",
    /// `server-storage-nonoverlapping`: When a server supports multiple storages, the URIs MUST be allocated to non-overlapping space.
    server_storage_nonoverlapping, "server-storage-nonoverlapping",
    /// `server-storage-track-owner`: Servers MUST keep track of at least one owner of a storage in an implementation defined way.
    server_storage_track_owner, "server-storage-track-owner",
    /// `server-tls-https`: Servers SHOULD use TLS connections through the https URI scheme in order to secure the communication with clients.
    server_tls_https, "server-tls-https",
    /// `server-tls-https-redirect`: When both http and https URI schemes are supported, the server MUST redirect all http URIs to their https counterparts using a response with a 301 status code and a Location header.
    server_tls_https_redirect, "server-tls-https-redirect",
    /// `server-unauthenticated`: When a client does not provide valid credentials when requesting a resource that requires it (see WebID), servers MUST send a response with a 401 status code (unless 404 is preferred for security reasons).
    server_unauthenticated, "server-unauthenticated",
    /// `server-uri-redirect-differing`: Instead, the server MAY respond to requests for the latter URI with a 301 redirect to the former.
    server_uri_redirect_differing, "server-uri-redirect-differing",
    /// `server-uri-trailing-slash-distinct`: If two URIs differ only in the trailing slash, and the server has associated a resource with one of them, then the other URI MUST NOT correspond to another resource.
    server_uri_trailing_slash_distinct, "server-uri-trailing-slash-distinct",
    /// `server-wac`: Servers MUST conform to the Web Access Control specification [WAC].
    server_wac, "server-wac",
    /// `server-websockets-api`: Servers SHOULD implement the Solid WebSockets API [SOLID-WEBSOCKETS-API].
    server_websockets_api, "server-websockets-api",
    /// `societal-impact-review`: 
    societal_impact_review, "societal-impact-review",
    /// `societal-impact-review-can-users-of-the-web-platform-choose-not-to-use-features-of-your-specification`: 
    societal_impact_review_can_users_of_the_web_platform_choose_not_to_use_features_of_your_specification, "societal-impact-review-can-users-of-the-web-platform-choose-not-to-use-features-of-your-specification",
    /// `societal-impact-review-have-you-completed-the-security-privacy-self-review-questionnaire`: 
    societal_impact_review_have_you_completed_the_security_privacy_self_review_questionnaire, "societal-impact-review-have-you-completed-the-security-privacy-self-review-questionnaire",
    /// `societal-impact-review-to-what-extent-do-the-features-in-your-specification-result-in-increased-power-consumption-or-emissions`: 
    societal_impact_review_to_what_extent_do_the_features_in_your_specification_result_in_increased_power_consumption_or_emissions, "societal-impact-review-to-what-extent-do-the-features-in-your-specification-result-in-increased-power-consumption-or-emissions",
    /// `societal-impact-review-what-are-the-power-dynamics-at-play-in-implementations-of-your-specification`: 
    societal_impact_review_what_are_the_power_dynamics_at_play_in_implementations_of_your_specification, "societal-impact-review-what-are-the-power-dynamics-at-play-in-implementations-of-your-specification",
    /// `societal-impact-review-what-effect-may-features-of-your-specification-have-on-minority-groups`: 
    societal_impact_review_what_effect_may_features_of_your_specification_have_on_minority_groups, "societal-impact-review-what-effect-may-features-of-your-specification-have-on-minority-groups",
    /// `societal-impact-review-what-groups-of-people-are-excluded-from-using-features-of-your-specification`: 
    societal_impact_review_what_groups_of_people_are_excluded_from_using_features_of_your_specification, "societal-impact-review-what-groups-of-people-are-excluded-from-using-features-of-your-specification",
    /// `societal-impact-review-what-is-the-expected-lifetime-of-your-specification-feature-s`: 
    societal_impact_review_what_is_the_expected_lifetime_of_your_specification_feature_s, "societal-impact-review-what-is-the-expected-lifetime-of-your-specification-feature-s",
    /// `societal-impact-review-what-kinds-of-activities-do-you-anticipate-your-specification-becoming-a-critical-part-of`: 
    societal_impact_review_what_kinds_of_activities_do_you_anticipate_your_specification_becoming_a_critical_part_of, "societal-impact-review-what-kinds-of-activities-do-you-anticipate-your-specification-becoming-a-critical-part-of",
    /// `societal-impact-review-what-points-of-centralization-does-your-feature-bring-to-the-web-platform`: 
    societal_impact_review_what_points_of_centralization_does_your_feature_bring_to_the_web_platform, "societal-impact-review-what-points-of-centralization-does-your-feature-bring-to-the-web-platform",
    /// `societal-impact-review-what-risks-do-you-see-in-features-of-your-specification-being-misused-or-used-differently-from-how-you-intended`: 
    societal_impact_review_what_risks_do_you_see_in_features_of_your_specification_being_misused_or_used_differently_from_how_you_intended, "societal-impact-review-what-risks-do-you-see-in-features-of-your-specification-being-misused-or-used-differently-from-how-you-intended",
    /// `solid-app`: 
    solid_app, "solid-app",
    /// `solid-oidc`: 
    solid_oidc, "solid-oidc",
    /// `sotd`: 
    sotd, "sotd",
    /// `storage`: 
    storage, "storage",
    /// `storage-owner-uri-ownership`: 
    storage_owner_uri_ownership, "storage-owner-uri-ownership",
    /// `terminology`: 
    terminology, "terminology",
    /// `trust-between-owners`: 
    trust_between_owners, "trust-between-owners",
    /// `uniform-resource-identifier`: 
    uniform_resource_identifier, "uniform-resource-identifier",
    /// `uri`: 
    uri, "uri",
    /// `uri-allocation`: 
    uri_allocation, "uri-allocation",
    /// `uri-persistence`: 
    uri_persistence, "uri-persistence",
    /// `uri-reuse`: 
    uri_reuse, "uri-reuse",
    /// `uri-slash-semantics`: 
    uri_slash_semantics, "uri-slash-semantics",
    /// `web-access-control`: 
    web_access_control, "web-access-control",
    /// `webid`: 
    webid, "webid",
    /// `webid-tls`: 
    webid_tls, "webid-tls",
    /// `websockets`: 
    websockets, "websockets",
    /// `write-operation`: 
    write_operation, "write-operation",
    /// `writing-resources`: 
    writing_resources, "writing-resources"
);
