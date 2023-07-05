// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Access Control Policy (ACP)` vocabulary
//!
//! This module requires `ns-acp` feature to be enabled.
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Access Control Policy (ACP)|
//! |**Prefix**|acp|
//! |**Namespace base IRI**|<http://www.w3.org/ns/solid/acp#>|
//! |**Description**|ACP is a language for describing, controlling, and granting access to resources.|
//! |**Is defined by**|<https://solid.github.io/authorization-panel/acp-specification/acp.ttl>|
//!

use crate::namespace;

namespace!(
    base: "http://www.w3.org/ns/solid/acp#",

    terms: [
        /// `Access Control`: All Access Controls controlling member resources access via the acp:memberAccessControl property MUST be included in the set of Access Controls linked as acp:accessControl in the effective authorization graph of a resource.
        (AccessControl, "AccessControl"),
        /// `Access Control Resource`: Both the acp:resource property and its inverse acp:accessControlResource MUST be taken into account in determining the Access Control Resources controlling access to resources.
        (AccessControlResource, "AccessControlResource"),
        /// `Access Grant`: Instances of the Access Grant class define sets of Access Modes granted in particular Contexts.
        (AccessGrant, "AccessGrant"),
        /// `Access Mode`: The ACP specification does not define specific Access Modes. Instead, any Access Mode granted is an instance of the Access Mode class. Access Modes and their granularity can be tailored to the needs of an application and Access Modes defined in other vocabularies can also be used (for example, instances of ACL Access).
        (AccessMode, "AccessMode"),
        /// `Always Satisfied Restriction`: Defined instances of the Always Satisfied Restriction class are used in Matcher restrictions to indicate that the restriction is always satisfied. The default behaviour of a Matcher is to not be satisfied, so this is the only way to make a Matcher always satisfied.
        (AlwaysSatisfiedRestriction, "AlwaysSatisfiedRestriction"),
        /// `Authenticated Agent`: In a Matcher, agent attributes using the Authenticated Agent named individual MUST match Contexts that contain an agent.
        (AuthenticatedAgent, "AuthenticatedAgent"),
        /// `Authenticated Client`: In a Matcher, client attributes using the Authenticated Client named individual MUST match Contexts that contain a client.
        (AuthenticatedClient, "AuthenticatedClient"),
        /// `Authenticated Issuer`: In a Matcher, issuer attributes using the Authenticated Issuer named individual MUST match Contexts that contain an issuer.
        (AuthenticatedIssuer, "AuthenticatedIssuer"),
        /// `Context`: Instances of the Context class describe instances of resource access.
        (Context, "Context"),
        /// `Creator Agent`: In a Matcher, agent attributes using the Creator Agent named individual MUST match Contexts where a defined creator matches the defined agent.
        (CreatorAgent, "CreatorAgent"),
        /// `Matcher`: A Matcher MUST be satisfied if and only if: it defines at least one attribute; and, at least one value of each defined attribute matches the Context. ACP engines MUST match the context attributes defined by this specification according to IRI equality and literal term equality.
        (Matcher, "Matcher"),
        /// `Owner Agent`: In a Matcher, agent attributes using the Owner Agent named individual MUST match Contexts where a defined owner matches the defined agent.
        (OwnerAgent, "OwnerAgent"),
        /// `Access Policy`: An Access Mode MUST be granted if and only if in the set of Effective Policies controlling access to it: a satisfied policy allows the Access Mode; and, no satisfied policy denies it.
        (Policy, "Policy"),
        /// `Public Agent`: In a Matcher, agent attributes using the Public Agent named individual MUST match all Contexts.
        (PublicAgent, "PublicAgent"),
        /// `Public Client`: In a Matcher, client attributes using the Public Client named individual MUST match all Contexts.
        (PublicClient, "PublicClient"),
        /// `Public Issuer`: In a Matcher, issuer attributes using the Public Issuer named individual MUST match all Contexts.
        (PublicIssuer, "PublicIssuer"),
        /// `access control`: The access control property connects ACRs to Access Controls.
        (accessControl, "accessControl"),
        /// `access control resource`: The access control resource property connects resources to ACRs controlling access to them. It is the inverse of acp:resource.
        (accessControlResource, "accessControlResource"),
        /// `agent`: The agent attribute describes agents initiating requests.
        (agent, "agent"),
        /// `all of`: The all of property connects Policies to a set of Matchers, all of which MUST be satisfied for the Policy to be satisfied.
        (allOf, "allOf"),
        /// `allow`: The allow property connects Policies to the Access Modes they allow if satisfied.
        (allow, "allow"),
        /// `any of`: The any of property connects Policies to a set of Matchers, at least one of which MUST be satisfied for the Policy to be satisfied.
        (anyOf, "anyOf"),
        /// `apply`: The apply property connects Access Controls to the Policies they apply to resources.
        (apply, "apply"),
        /// `attribute`: Sub-properties of ACP attribute are used to describe instances of resource access.
        (attribute, "attribute"),
        /// `client`: In a Matcher, client attributes define a set of clients, at least one of which MUST match the Context for the Matcher to be satisfied.
        (client, "client"),
        /// `context`: The context property connects Access Grants to the Contexts in which they're given.
        (context, "context"),
        /// `creator`: The creator attribute describes creators of requested resources.
        (creator, "creator"),
        /// `deny`: The deny property connects Policies to the Access Modes they deny if satisfied.
        (deny, "deny"),
        /// `grant`: The grant property connects Access Grants to the Access Modes they grant.
        (grant, "grant"),
        /// `issuer`: The issuer attribute describes identity providers used to assert the identity of agents requesting resources.
        (issuer, "issuer"),
        /// `member access control`: The member access control property transitively connects ACRs of member resources to Access Controls.
        (memberAccessControl, "memberAccessControl"),
        /// `none of`: The none of property connects Policies to a set of Matchers, all of which MUST NOT be satisfied for the Policy to be satisfied.
        (noneOf, "noneOf"),
        /// `owner`: The owner attribute describes owners of requested resources.
        (owner, "owner"),
        /// `resource`: The resource property connects ACRs to resources they control. It is the inverse of acp:accessControlResource.
        (resource, "resource"),
        /// `target`: The target attribute describes requested resources.
        (target, "target"),
        /// `vc`: The vc attribute describes types of Verifiable Credentials (VC) presented as part of resource access requests.
        (vc, "vc")    ]
);
