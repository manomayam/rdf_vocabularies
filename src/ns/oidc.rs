// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Solid OIDC` vocabulary
//!
//! This module requires `ns-oidc` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Solid OIDC|
//! |**Prefix**|oidc|
//! |**Namespace base IRI**|<http://www.w3.org/ns/solid/oidc#>|
//! |**Description**|The OpenID Connect vocabulary used by the Solid-OIDC authentication specification.|
//! |**Is defined by**|<http://www.w3.org/ns/solid/oidc>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/solid/oidc#",;
    /// `Solid OIDC`: The OpenID Connect vocabulary used by the Solid-OIDC authentication specification.
    NAMESPACE_BASE, "",
    /// `application type`: TODO
    application_type, "application_type",
    /// `client name`: A name for this client that can be presented to an End-User
    client_name, "client_name",
    /// `client URI`: A URI for this client's home page
    client_uri, "client_uri",
    /// `contacts`: A collection of URIs where individuals or entities responsible for this client can be contacted.
    contacts, "contacts",
    /// `default max age`: TODO
    default_max_age, "default_max_age",
    /// `grant types`: A collection of OAuth 2.0 Grant Types that the client will restrict itself to using.
    grant_types, "grant_types",
    /// `logo URI`: A URI for this client's logo
    logo_uri, "logo_uri",
    /// `policy URI`: A URI for this client's policy document
    policy_uri, "policy_uri",
    /// `redirect URIs`: A collection of registered URIs used by the client for redirection
    redirect_uris, "redirect_uris",
    /// `require auth time`: TODO
    require_auth_time, "require_auth_time",
    /// `response types`: A collection of OAuth 2.0 Response Types that the client will restrict itself to using.
    response_types, "response_types",
    /// `scope`: An OAuth2 scope for the requested access token.
    scope, "scope",
    /// `token endpoint auth method`: The requested client authentication method for the Token Endpoint
    token_endpoint_auth_method, "token_endpoint_auth_method",
    /// `ToS URI`: A URI for this client's terms of service document
    tos_uri, "tos_uri"
);
