// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-prov")))]
//! This module provides terms for `W3C PROVenance Interchange` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|W3C PROVenance Interchange|
//! |**Prefix**|prov|
//! |**Namespace base IRI**|<http://www.w3.org/ns/prov#>|
//! |**Description**|This document is published by the Provenance Working Group (http://www.w3.org/2011/prov/wiki/Main_Page).  If you wish to make comments regarding this document, please send them to public-prov-comments@w3.org (subscribe public-prov-comments-request@w3.org, archives http://lists.w3.org/ Archives/Public/public-prov-comments/). All feedback is welcome.|
//! |**Is defined by**|<http://www.w3.org/ns/prov#>|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/prov#",;
    /// `W3C PROVenance Interchange`: This document is published by the Provenance Working Group (http://www.w3.org/2011/prov/wiki/Main_Page).  If you wish to make comments regarding this document, please send them to public-prov-comments@w3.org (subscribe public-prov-comments-request@w3.org, archives http://lists.w3.org/ Archives/Public/public-prov-comments/). All feedback is welcome.
    NAMESPACE_BASE, "",
    /// `Accept`: 
    Accept, "Accept",
    /// `Activity`: 
    Activity, "Activity",
    /// `ActivityInfluence`: ActivityInfluence provides additional descriptions of an Activity's binary influence upon any other kind of resource. Instances of ActivityInfluence use the prov:activity property to cite the influencing Activity.
    ActivityInfluence, "ActivityInfluence",
    /// `Agent`: 
    Agent, "Agent",
    /// `AgentInfluence`: AgentInfluence provides additional descriptions of an Agent's binary influence upon any other kind of resource. Instances of AgentInfluence use the prov:agent property to cite the influencing Agent.
    AgentInfluence, "AgentInfluence",
    /// `Association`: An instance of prov:Association provides additional descriptions about the binary prov:wasAssociatedWith relation from an prov:Activity to some prov:Agent that had some responsiblity for it. For example, :baking prov:wasAssociatedWith :baker; prov:qualifiedAssociation [ a prov:Association; prov:agent :baker; :foo :bar ].
    Association, "Association",
    /// `Attribution`: An instance of prov:Attribution provides additional descriptions about the binary prov:wasAttributedTo relation from an prov:Entity to some prov:Agent that had some responsible for it. For example, :cake prov:wasAttributedTo :baker; prov:qualifiedAttribution [ a prov:Attribution; prov:entity :baker; :foo :bar ].
    Attribution, "Attribution",
    /// `Bundle`: Note that there are kinds of bundles (e.g. handwritten letters, audio recordings, etc.) that are not expressed in PROV-O, but can be still be described by PROV-O.
    Bundle, "Bundle",
    /// `Collection`: 
    Collection, "Collection",
    /// `Communication`: An instance of prov:Communication provides additional descriptions about the binary prov:wasInformedBy relation from an informed prov:Activity to the prov:Activity that informed it. For example, :you_jumping_off_bridge prov:wasInformedBy :everyone_else_jumping_off_bridge; prov:qualifiedCommunication [ a prov:Communication; prov:activity :everyone_else_jumping_off_bridge; :foo :bar ].
    Communication, "Communication",
    /// `Contribute `: 
    Contribute, "Contribute",
    /// `Contributor`: 
    Contributor, "Contributor",
    /// `Copyright`: 
    Copyright, "Copyright",
    /// `Create`: 
    Create, "Create",
    /// `Creator`: 
    Creator, "Creator",
    /// `Delegation`: An instance of prov:Delegation provides additional descriptions about the binary prov:actedOnBehalfOf relation from a performing prov:Agent to some prov:Agent for whom it was performed. For example, :mixing prov:wasAssociatedWith :toddler . :toddler prov:actedOnBehalfOf :mother; prov:qualifiedDelegation [ a prov:Delegation; prov:entity :mother; :foo :bar ].
    Delegation, "Delegation",
    /// `Derivation`: An instance of prov:Derivation provides additional descriptions about the binary prov:wasDerivedFrom relation from some derived prov:Entity to another prov:Entity from which it was derived. For example, :chewed_bubble_gum prov:wasDerivedFrom :unwrapped_bubble_gum; prov:qualifiedDerivation [ a prov:Derivation; prov:entity :unwrapped_bubble_gum; :foo :bar ].
    Derivation, "Derivation",
    /// `Dictionary`: A given dictionary forms a given structure for its members. A different structure (obtained either by insertion or removal of members) constitutes a different dictionary.
    Dictionary, "Dictionary",
    /// `ProvenanceService`: Type for a generic provenance query service. Mainly for use in RDF provenance query service descriptions, to facilitate discovery in linked data environments.
    DirectQueryService, "DirectQueryService",
    /// `EmptyCollection`: 
    EmptyCollection, "EmptyCollection",
    /// `Empty Dictionary`: 
    EmptyDictionary, "EmptyDictionary",
    /// `End`: An instance of prov:End provides additional descriptions about the binary prov:wasEndedBy relation from some ended prov:Activity to an prov:Entity that ended it. For example, :ball_game prov:wasEndedBy :buzzer; prov:qualifiedEnd [ a prov:End; prov:entity :buzzer; :foo :bar; prov:atTime '2012-03-09T08:05:08-05:00'^^xsd:dateTime ].
    End, "End",
    /// `Entity`: 
    Entity, "Entity",
    /// `EntityInfluence`: EntityInfluence provides additional descriptions of an Entity's binary influence upon any other kind of resource. Instances of EntityInfluence use the prov:entity property to cite the influencing Entity.
    EntityInfluence, "EntityInfluence",
    /// `Generation`: An instance of prov:Generation provides additional descriptions about the binary prov:wasGeneratedBy relation from a generated prov:Entity to the prov:Activity that generated it. For example, :cake prov:wasGeneratedBy :baking; prov:qualifiedGeneration [ a prov:Generation; prov:activity :baking; :foo :bar ].
    Generation, "Generation",
    /// `Influence`: An instance of prov:Influence provides additional descriptions about the binary prov:wasInfluencedBy relation from some influenced Activity, Entity, or Agent to the influencing Activity, Entity, or Agent. For example, :stomach_ache prov:wasInfluencedBy :spoon; prov:qualifiedInfluence [ a prov:Influence; prov:entity :spoon; :foo :bar ] . Because prov:Influence is a broad relation, the more specific relations (Communication, Delegation, End, etc.) should be used when applicable.
    Influence, "Influence",
    /// `Insertion`: 
    Insertion, "Insertion",
    /// `InstantaneousEvent`: An instantaneous event, or event for short, happens in the world and marks a change in the world, in its activities and in its entities. The term 'event' is commonly used in process algebra with a similar meaning. Events represent communications or interactions; they are assumed to be atomic and instantaneous.
    InstantaneousEvent, "InstantaneousEvent",
    /// `Invalidation`: An instance of prov:Invalidation provides additional descriptions about the binary prov:wasInvalidatedBy relation from an invalidated prov:Entity to the prov:Activity that invalidated it. For example, :uncracked_egg prov:wasInvalidatedBy :baking; prov:qualifiedInvalidation [ a prov:Invalidation; prov:activity :baking; :foo :bar ].
    Invalidation, "Invalidation",
    /// `Key-Entity Pair`: 
    KeyEntityPair, "KeyEntityPair",
    /// `Location`: 
    Location, "Location",
    /// `Modify`: 
    Modify, "Modify",
    /// `Organization`: 
    Organization, "Organization",
    /// `Person`: 
    Person, "Person",
    /// `Plan`: There exist no prescriptive requirement on the nature of plans, their representation, the actions or steps they consist of, or their intended goals. Since plans may evolve over time, it may become necessary to track their provenance, so plans themselves are entities. Representing the plan explicitly in the provenance can be useful for various tasks: for example, to validate the execution as represented in the provenance record, to manage expectation failures, or to provide explanations.
    Plan, "Plan",
    /// `PrimarySource`: An instance of prov:PrimarySource provides additional descriptions about the binary prov:hadPrimarySource relation from some secondary prov:Entity to an earlier, primary prov:Entity. For example, :blog prov:hadPrimarySource :newsArticle; prov:qualifiedPrimarySource [ a prov:PrimarySource; prov:entity :newsArticle; :foo :bar ] .
    PrimarySource, "PrimarySource",
    /// `Publish`: 
    Publish, "Publish",
    /// `Publisher`: 
    Publisher, "Publisher",
    /// `Quotation`: An instance of prov:Quotation provides additional descriptions about the binary prov:wasQuotedFrom relation from some taken prov:Entity from an earlier, larger prov:Entity. For example, :here_is_looking_at_you_kid prov:wasQuotedFrom :casablanca_script; prov:qualifiedQuotation [ a prov:Quotation; prov:entity :casablanca_script; :foo :bar ].
    Quotation, "Quotation",
    /// `Removal`: 
    Removal, "Removal",
    /// `Replace`: 
    Replace, "Replace",
    /// `Revision`: An instance of prov:Revision provides additional descriptions about the binary prov:wasRevisionOf relation from some newer prov:Entity to an earlier prov:Entity. For example, :draft_2 prov:wasRevisionOf :draft_1; prov:qualifiedRevision [ a prov:Revision; prov:entity :draft_1; :foo :bar ].
    Revision, "Revision",
    /// `RightsAssignment`: 
    RightsAssignment, "RightsAssignment",
    /// `RightsHolder`: 
    RightsHolder, "RightsHolder",
    /// `Role`: 
    Role, "Role",
    /// `ServiceDescription`: Type for a generic provenance query service. Mainly for use in RDF provenance query service descriptions, to facilitate discovery in linked data environments.
    ServiceDescription, "ServiceDescription",
    /// `SoftwareAgent`: 
    SoftwareAgent, "SoftwareAgent",
    /// `Start`: An instance of prov:Start provides additional descriptions about the binary prov:wasStartedBy relation from some started prov:Activity to an prov:Entity that started it. For example, :foot_race prov:wasStartedBy :bang; prov:qualifiedStart [ a prov:Start; prov:entity :bang; :foo :bar; prov:atTime '2012-03-09T08:05:08-05:00'^^xsd:dateTime ] .
    Start, "Start",
    /// `Submit`: 
    Submit, "Submit",
    /// `Usage`: An instance of prov:Usage provides additional descriptions about the binary prov:used relation from some prov:Activity to an prov:Entity that it used. For example, :keynote prov:used :podium; prov:qualifiedUsage [ a prov:Usage; prov:entity :podium; :foo :bar ].
    Usage, "Usage",
    /// `actedOnBehalfOf`: An object property to express the accountability of an agent towards another agent. The subordinate agent acted on behalf of the responsible agent in an actual activity. 
    actedOnBehalfOf, "actedOnBehalfOf",
    /// `activity`: 
    activity, "activity",
    /// `activityOfInfluence`: 
    activityOfInfluence, "activityOfInfluence",
    /// `agent`: 
    agent, "agent",
    /// `agentOfInfluence`: 
    agentOfInfluence, "agentOfInfluence",
    /// `alternateOf`: 
    alternateOf, "alternateOf",
    /// `aq`: 
    aq, "aq",
    /// `asInBundle`: prov:asInBundle is used to specify which bundle the general entity of a prov:mentionOf property is described.  When :x prov:mentionOf :y and :y is described in Bundle :b, the triple :x prov:asInBundle :b is also asserted to cite the Bundle in which :y was described.
    asInBundle, "asInBundle",
    /// `atLocation`: The Location of any resource.
    atLocation, "atLocation",
    /// `atTime`: The time at which an InstantaneousEvent occurred, in the form of xsd:dateTime.
    atTime, "atTime",
    /// `category`: Classify prov-o terms into three categories, including 'starting-point', 'qualifed', and 'extended'. This classification is used by the prov-o html document to gently introduce prov-o terms to its users. 
    category, "category",
    /// `component`: Classify prov-o terms into six components according to prov-dm, including 'agents-responsibility', 'alternate', 'annotations', 'collections', 'derivations', and 'entities-activities'. This classification is used so that readers of prov-o specification can find its correspondence with the prov-dm specification.
    component, "component",
    /// `constraints`: A reference to the principal section of the PROV-CONSTRAINTS document that describes this concept.
    constraints, "constraints",
    /// `contributed`: 
    contributed, "contributed",
    /// `definition`: A definition quoted from PROV-DM or PROV-CONSTRAINTS that describes the concept expressed with this OWL term.
    definition, "definition",
    /// `derivedByInsertionFrom`: 
    derivedByInsertionFrom, "derivedByInsertionFrom",
    /// `derivedByRemovalFrom`: 
    derivedByRemovalFrom, "derivedByRemovalFrom",
    /// `describesService`: relates a generic provenance query service resource (type prov:ServiceDescription) to a specific query service description (e.g. a prov:DirectQueryService or a sd:Service).
    describesService, "describesService",
    /// `dictionary`: 
    dictionary, "dictionary",
    /// `dm`: A reference to the principal section of the PROV-DM document that describes this concept.
    dm, "dm",
    /// `editorialNote`: A note by the OWL development team about how this term expresses the PROV-DM concept, or how it should be used in context of semantic web or linked data.
    editorialNote, "editorialNote",
    /// `editorsDefinition`: When the prov-o term does not have a definition drawn from prov-dm, and the prov-o editor provides one.
    editorsDefinition, "editorsDefinition",
    /// `ended`: 
    ended, "ended",
    /// `endedAtTime`: The time at which an activity ended. See also prov:startedAtTime.
    endedAtTime, "endedAtTime",
    /// `entity`: 
    entity, "entity",
    /// `entityOfInfluence`: 
    entityOfInfluence, "entityOfInfluence",
    /// `generalizationOf`: 
    generalizationOf, "generalizationOf",
    /// `generated`: 
    generated, "generated",
    /// `generatedAsDerivation`: 
    generatedAsDerivation, "generatedAsDerivation",
    /// `generatedAtTime`: The time at which an entity was completely created and is available for use.
    generatedAtTime, "generatedAtTime",
    /// `hadActivity`: The _optional_ Activity of an Influence, which used, generated, invalidated, or was the responsibility of some Entity. This property is _not_ used by ActivityInfluence (use prov:activity instead).
    hadActivity, "hadActivity",
    /// `hadDelegate`: 
    hadDelegate, "hadDelegate",
    /// `hadDerivation`: 
    hadDerivation, "hadDerivation",
    /// `hadDictionaryMember`: 
    hadDictionaryMember, "hadDictionaryMember",
    /// `hadGeneration`: The _optional_ Generation involved in an Entity's Derivation.
    hadGeneration, "hadGeneration",
    /// `hadInfluence`: 
    hadInfluence, "hadInfluence",
    /// `hadMember`: 
    hadMember, "hadMember",
    /// `hadPlan`: The _optional_ Plan adopted by an Agent in Association with some Activity. Plan specifications are out of the scope of this specification.
    hadPlan, "hadPlan",
    /// `hadPrimarySource`: 
    hadPrimarySource, "hadPrimarySource",
    /// `hadRevision`: 
    hadRevision, "hadRevision",
    /// `hadRole`: The _optional_ Role that an Entity assumed in the context of an Activity. For example, :baking prov:used :spoon; prov:qualified [ a prov:Usage; prov:entity :spoon; prov:hadRole roles:mixing_implement ].
    hadRole, "hadRole",
    /// `hadUsage`: The _optional_ Usage involved in an Entity's Derivation.
    hadUsage, "hadUsage",
    /// `has_anchor`: Indicates anchor URI for a potentially dynamic resource instance.
    has_anchor, "has_anchor",
    /// `has_provenance`: Indicates a provenance-URI for a resource; the resource identified by this property presents a provenance record about its subject or anchor resource.
    has_provenance, "has_provenance",
    /// `hasProvenanceService`: Indicates a provenance query service that can access provenance related to its subject or anchor resource.
    has_query_service, "has_query_service",
    /// `influenced`: 
    influenced, "influenced",
    /// `influencer`: Subproperties of prov:influencer are used to cite the object of an unqualified PROV-O triple whose predicate is a subproperty of prov:wasInfluencedBy (e.g. prov:used, prov:wasGeneratedBy). prov:influencer is used much like rdf:object is used.
    influencer, "influencer",
    /// `informed`: 
    informed, "informed",
    /// `insertedKeyEntityPair`: 
    insertedKeyEntityPair, "insertedKeyEntityPair",
    /// `invalidated`: 
    invalidated, "invalidated",
    /// `invalidatedAtTime`: The time at which an entity was invalidated (i.e., no longer usable).
    invalidatedAtTime, "invalidatedAtTime",
    /// `inverse`: PROV-O does not define all property inverses. The directionalities defined in PROV-O should be given preference over those not defined. However, if users wish to name the inverse of a PROV-O property, the local name given by prov:inverse should be used.
    inverse, "inverse",
    /// `locationOf`: 
    locationOf, "locationOf",
    /// `mentionOf`: prov:mentionOf is used to specialize an entity as described in another bundle. It is to be used in conjuction with prov:asInBundle.  prov:asInBundle is used to cite the Bundle in which the generalization was mentioned.
    mentionOf, "mentionOf",
    /// `n`: A reference to the principal section of the PROV-DM document that describes this concept.
    n, "n",
    /// `order`: The position that this OWL term should be listed within documentation. The scope of the documentation (e.g., among all terms, among terms within a prov:category, among properties applying to a particular class, etc.) is unspecified.
    order, "order",
    /// `pairKey`: 
    pairEntity, "pairEntity",
    /// `pairKey`: 
    pairKey, "pairKey",
    /// `provenance pingback`: Relates a resource to a provenance pingback service that may receive additional provenance links about the resource.
    pingback, "pingback",
    /// `provenanceUriTemplate`: Relates a provenance service to a URI template string for constructing provenance-URIs.
    provenanceUriTemplate, "provenanceUriTemplate",
    /// `qualifiedAssociation`: If this Activity prov:wasAssociatedWith Agent :ag, then it can qualify the Association using prov:qualifiedAssociation [ a prov:Association;  prov:agent :ag; :foo :bar ].
    qualifiedAssociation, "qualifiedAssociation",
    /// `qualifiedAssociationOf`: 
    qualifiedAssociationOf, "qualifiedAssociationOf",
    /// `qualifiedAttribution`: If this Entity prov:wasAttributedTo Agent :ag, then it can qualify how it was influenced using prov:qualifiedAttribution [ a prov:Attribution;  prov:agent :ag; :foo :bar ].
    qualifiedAttribution, "qualifiedAttribution",
    /// `qualifiedAttributionOf`: 
    qualifiedAttributionOf, "qualifiedAttributionOf",
    /// `qualifiedCommunication`: If this Activity prov:wasInformedBy Activity :a, then it can qualify how it was influenced using prov:qualifiedCommunication [ a prov:Communication;  prov:activity :a; :foo :bar ].
    qualifiedCommunication, "qualifiedCommunication",
    /// `qualifiedCommunicationOf`: 
    qualifiedCommunicationOf, "qualifiedCommunicationOf",
    /// `qualifiedDelegation`: If this Agent prov:actedOnBehalfOf Agent :ag, then it can qualify how with prov:qualifiedResponsibility [ a prov:Responsibility;  prov:agent :ag; :foo :bar ].
    qualifiedDelegation, "qualifiedDelegation",
    /// `qualifiedDelegationOf`: 
    qualifiedDelegationOf, "qualifiedDelegationOf",
    /// `qualifiedDerivation`: If this Entity prov:wasDerivedFrom Entity :e, then it can qualify how it was derived using prov:qualifiedDerivation [ a prov:Derivation;  prov:entity :e; :foo :bar ].
    qualifiedDerivation, "qualifiedDerivation",
    /// `qualifiedDerivationOf`: 
    qualifiedDerivationOf, "qualifiedDerivationOf",
    /// `qualifiedEnd`: If this Activity prov:wasEndedBy Entity :e1, then it can qualify how it was ended using prov:qualifiedEnd [ a prov:End;  prov:entity :e1; :foo :bar ].
    qualifiedEnd, "qualifiedEnd",
    /// `qualifiedEndOf`: 
    qualifiedEndOf, "qualifiedEndOf",
    /// `qualifiedForm`: This annotation property links a subproperty of prov:wasInfluencedBy with the subclass of prov:Influence and the qualifying property that are used to qualify it.   Example annotation:      prov:wasGeneratedBy prov:qualifiedForm prov:qualifiedGeneration, prov:Generation .  Then this unqualified assertion:      :entity1 prov:wasGeneratedBy :activity1 .  can be qualified by adding:     :entity1 prov:qualifiedGeneration :entity1Gen .    :entity1Gen         a prov:Generation, prov:Influence;        prov:activity :activity1;        :customValue 1337 .  Note how the value of the unqualified influence (prov:wasGeneratedBy :activity1) is mirrored as the value of the prov:activity (or prov:entity, or prov:agent) property on the influence class.
    qualifiedForm, "qualifiedForm",
    /// `qualifiedGeneration`: If this Activity prov:generated Entity :e, then it can qualify how it performed the Generation using prov:qualifiedGeneration [ a prov:Generation;  prov:entity :e; :foo :bar ].
    qualifiedGeneration, "qualifiedGeneration",
    /// `qualifiedGenerationOf`: 
    qualifiedGenerationOf, "qualifiedGenerationOf",
    /// `qualifiedInfluence`: Because prov:qualifiedInfluence is a broad relation, the more specific relations (qualifiedCommunication, qualifiedDelegation, qualifiedEnd, etc.) should be used when applicable.
    qualifiedInfluence, "qualifiedInfluence",
    /// `qualifiedInfluenceOf`: 
    qualifiedInfluenceOf, "qualifiedInfluenceOf",
    /// `qualifiedInsertion`: 
    qualifiedInsertion, "qualifiedInsertion",
    /// `qualifiedInvalidation`: If this Entity prov:wasInvalidatedBy Activity :a, then it can qualify how it was invalidated using prov:qualifiedInvalidation [ a prov:Invalidation;  prov:activity :a; :foo :bar ].
    qualifiedInvalidation, "qualifiedInvalidation",
    /// `qualifiedInvalidationOf`: 
    qualifiedInvalidationOf, "qualifiedInvalidationOf",
    /// `qualifiedPrimarySource`: If this Entity prov:hadPrimarySource Entity :e, then it can qualify how using prov:qualifiedPrimarySource [ a prov:PrimarySource; prov:entity :e; :foo :bar ].
    qualifiedPrimarySource, "qualifiedPrimarySource",
    /// `qualifiedQuotation`: If this Entity prov:wasQuotedFrom Entity :e, then it can qualify how using prov:qualifiedQuotation [ a prov:Quotation;  prov:entity :e; :foo :bar ].
    qualifiedQuotation, "qualifiedQuotation",
    /// `qualifiedQuotationOf`: 
    qualifiedQuotationOf, "qualifiedQuotationOf",
    /// `qualifiedRemoval`: 
    qualifiedRemoval, "qualifiedRemoval",
    /// `qualifiedRevision`: If this Entity prov:wasRevisionOf Entity :e, then it can qualify how it was revised using prov:qualifiedRevision [ a prov:Revision;  prov:entity :e; :foo :bar ].
    qualifiedRevision, "qualifiedRevision",
    /// `qualifiedSourceOf`: 
    qualifiedSourceOf, "qualifiedSourceOf",
    /// `qualifiedStart`: If this Activity prov:wasStartedBy Entity :e1, then it can qualify how it was started using prov:qualifiedStart [ a prov:Start;  prov:entity :e1; :foo :bar ].
    qualifiedStart, "qualifiedStart",
    /// `qualifiedStartOf`: 
    qualifiedStartOf, "qualifiedStartOf",
    /// `qualifiedUsage`: If this Activity prov:used Entity :e, then it can qualify how it used it using prov:qualifiedUsage [ a prov:Usage; prov:entity :e; :foo :bar ].
    qualifiedUsage, "qualifiedUsage",
    /// `qualifiedUsingActivity`: 
    qualifiedUsingActivity, "qualifiedUsingActivity",
    /// `quotedAs`: 
    quotedAs, "quotedAs",
    /// `removedKey`: 
    removedKey, "removedKey",
    /// `revisedEntity`: 
    revisedEntity, "revisedEntity",
    /// `sharesDefinitionWith`: 
    sharesDefinitionWith, "sharesDefinitionWith",
    /// `specializationOf`: 
    specializationOf, "specializationOf",
    /// `started`: 
    started, "started",
    /// `startedAtTime`: The time at which an activity started. See also prov:endedAtTime.
    startedAtTime, "startedAtTime",
    /// `todo`: 
    todo, "todo",
    /// `unqualifiedForm`: Classes and properties used to qualify relationships are annotated with prov:unqualifiedForm to indicate the property used to assert an unqualified provenance relation.
    unqualifiedForm, "unqualifiedForm",
    /// `used`: A prov:Entity that was used by this prov:Activity. For example, :baking prov:used :spoon, :egg, :oven .
    used, "used",
    /// `value`: 
    value, "value",
    /// `wasActivityOfInfluence`: 
    wasActivityOfInfluence, "wasActivityOfInfluence",
    /// `wasAssociateFor`: 
    wasAssociateFor, "wasAssociateFor",
    /// `wasAssociatedWith`: An prov:Agent that had some (unspecified) responsibility for the occurrence of this prov:Activity.
    wasAssociatedWith, "wasAssociatedWith",
    /// `wasAttributedTo`: Attribution is the ascribing of an entity to an agent.
    wasAttributedTo, "wasAttributedTo",
    /// `wasDerivedFrom`: The more specific subproperties of prov:wasDerivedFrom (i.e., prov:wasQuotedFrom, prov:wasRevisionOf, prov:hadPrimarySource) should be used when applicable.
    wasDerivedFrom, "wasDerivedFrom",
    /// `wasEndedBy`: End is when an activity is deemed to have ended. An end may refer to an entity, known as trigger, that terminated the activity.
    wasEndedBy, "wasEndedBy",
    /// `wasGeneratedBy`: 
    wasGeneratedBy, "wasGeneratedBy",
    /// `wasInfluencedBy`: Because prov:wasInfluencedBy is a broad relation, its more specific subproperties (e.g. prov:wasInformedBy, prov:actedOnBehalfOf, prov:wasEndedBy, etc.) should be used when applicable.
    wasInfluencedBy, "wasInfluencedBy",
    /// `wasInformedBy`: An activity a2 is dependent on or informed by another activity a1, by way of some unspecified entity that is generated by a1 and used by a2.
    wasInformedBy, "wasInformedBy",
    /// `wasInvalidatedBy`: 
    wasInvalidatedBy, "wasInvalidatedBy",
    /// `wasMemberOf`: 
    wasMemberOf, "wasMemberOf",
    /// `wasPlanOf`: 
    wasPlanOf, "wasPlanOf",
    /// `wasPrimarySourceOf`: 
    wasPrimarySourceOf, "wasPrimarySourceOf",
    /// `wasQuotedFrom`: An entity is derived from an original entity by copying, or 'quoting', some or all of it.
    wasQuotedFrom, "wasQuotedFrom",
    /// `wasRevisionOf`: A revision is a derivation that revises an entity into a revised version.
    wasRevisionOf, "wasRevisionOf",
    /// `wasRoleIn`: 
    wasRoleIn, "wasRoleIn",
    /// `wasStartedBy`: Start is when an activity is deemed to have started. A start may refer to an entity, known as trigger, that initiated the activity.
    wasStartedBy, "wasStartedBy",
    /// `wasUsedBy`: 
    wasUsedBy, "wasUsedBy",
    /// `wasUsedInDerivation`: 
    wasUsedInDerivation, "wasUsedInDerivation"
);
