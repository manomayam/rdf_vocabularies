// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `International Council on Archives Records in Contexts Ontology             (ICA RiC-O) version 0.2` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|International Council on Archives Records in Contexts Ontology             (ICA RiC-O) version 0.2|
//! |**Prefix**|rico|
//! |**Namespace base IRI**|[https://www.ica.org/standards/RiC/ontology#](https://www.ica.org/standards/RiC/ontology#)|
//! |**Description**|<html:div xml:lang="en" id="introduction">                 <html:h3>Introduction</html:h3>                 <html:p>RiC-O (Records in Contexts-Ontology) is an OWL ontology for describing                     archival record resources. As the second part of Records in Contexts standard,                     it is a formal representation of Records in Contexts Conceptual Model (RiC-CM).                     This version, which is v0.2, is the current official release. It is compliant                     with RiC-CM v0.2, that will be published soon after the release of RiC-O                     v0.2.</html:p>                 <html:p>The following diagram shows the main RiC-CM v0.2 entities and a few                     relations between them: <html:img src="https://raw.githubusercontent.com/ICA-EGAD/RiC-O/master/diagrams/diagrams_v0-2/RiC-CM-overview/diagram_RiC-CM-overview-RiC-v0-2.jpg" alt="A partial overview of RiC-CM v0.2 main entities" class="diagram"></html:img>                 </html:p>                 <html:div id="design-principles">                     <html:h4>RiC-O design principles</html:h4>                     <html:p>The following design principles were followed when developing                         RiC-O.</html:p>                     <html:p>                         <html:strong>RiC-O is a domain or reference ontology</html:strong>.</html:p>                     <html:p>It provides a generic vocabulary and formal rules for creating RDF                         datasets (or generating them from existing archival metadata) that describe                         in a consistent way any kind of archival record resource. It can support                         publishing RDF datasets as Linked Data, querying them using SPARQL, and                         making inferences using the logic of the ontology.</html:p>                     <html:p>While some projects have built some specific ontologies for describing                         archives, at this time no generic domain ontology exists for the specific                         needs of the archival community. This is why EGAD decided to develop RiC-O                         as the second part of RiC standard.</html:p>                     <html:p>Apart this first, main target, RiC-O also can help archival institutions                         and engineers to design and develop other technical implementations of                         RiC-CM that represent record resources and their layers of contexts as                         oriented, interconnected graphs. Of course, other technical implementations                         may be developed later on, including XML models, or (hopefully) new versions                         of EAD and EAC-CPF XML models.</html:p>                     <html:p>As RiC-O is a generic, domain ontology, it does not address by itself                         every specific need or expectation that may occur in every archival                         institution or project. It is rather a high level framework and a project                         can either limit itself to the use of a selection of components, or can add                         more subcomponents where needed.</html:p>                     <html:p>As a domain ontology, RiC-O, at this stage at least, does not borrow any                         component from other existing ontologies (such as the cultural heritage                         models – IFLA-LRM and CIDOC-CRM, PREMIS, or PROV-O). It should therefore be                         easier, for an archival institution or archival project, to understand,                         implement and maintain RiC-O within its system.</html:p>                     <html:p>Later on, RiC-O will be aligned with these existing models. This is of                         course essential for interconnecting RDF datasets conforming to RiC-O with                         other datasets, or for using parts of RiC-O in other contexts than the                         archival or records management realm.</html:p>                     <html:p>                         <html:strong>RiC-O must be immediately usable.</html:strong>                     </html:p>                     <html:p>This is a key feature for a new model. In particular, it is very                         important that existing archival metadata, that are created or generated in                         current information systems, can be converted to RDF conforming to RiC-O,                         without losing any data, structural or partially implicit information. What                         is at stake here is that metadata conforming to the previous existing ICA                         standards can be processed successfully.</html:p>                     <html:p>During the ongoing development process, a lot of successful testing has                         been made, using XML/EAD finding aids and XML/EAC-CPF authority records,                         that have been converted to RDF datasets, either by hand or using scripts. A                         conversion software is being developed and will soon be available.</html:p>                     <html:p>While some existing metadata sets may have a very fine level of                         granularity and accuracy, already using, for example, controlled                         vocabularies, or describing curation events separately, often these metadata                         don’t have the very precise structure that RiC-CM recommends. Even then,                         such a conversion process should remain possible. In order to allow this,                         RiC-O sometimes provides several methods for representing information (as                         described below). From this point of view, the current official version of                         RiC-O may be considered a transitional ontology, in which some components                         may be deprecated later on.</html:p>                     <html:p> The usability of a model also depends on its documentation. That’s why                         the current official release has been fully documented (this documentation                         will be continously improved).</html:p>                     <html:p>RiC-O will also soon be acompanied with examples (RDF datasets). Some                         tutorials should also be written, and EGAD will organize practical                         workshops.</html:p>                     <html:p>                         <html:strong>RiC-O has to provide a flexible                         framework</html:strong>.</html:p>                     <html:p>This is a very important principle too. It is related with the usability                         principle quoted above. Moreover, archival description is flexible by                         essence. It is quite commonly noted that the level of granularity of                         information varies from one finding aid to another (or from one authority                         record to another), or even within the same finding aid. Some series or                         agents are described summarily because little is known about them and there                         is little time for extensive research, while other series, even records, or                         agents are described in detail; some relations (e.g. that relating to                         provenance) may be described without any detail while others may be                         thoroughly documented, as ISAAR(CPF) and EAC-CPF allow it.</html:p>                     <html:p>Being generally flexible, for an OWL ontology, depends first on the                         polyhierarchical systems of classes and properties it provides. A                         superproperty or superclass, more general or generic than its subproperties                         or subclasses, must exist and be available for handling information, while                         at the same time more accurate subcomponents must be there for handling more                         accurate description. Also, RiC-O should provide several methods for                         expressing whether relations are well attested and certain, or are more vague,                         as well as direct and short paths between entities alongside more complex                         ones.</html:p>                     <html:p>                         <html:strong>RiC-O opens new potential for archival                             description.</html:strong>                     </html:p>                     <html:p>This means that Linked Data tools and interfaces should enable end users                         to go through RDF/RiC-O graphs, to query them using SPARQL in an efficient                         way and to consult archival metadata and their contexts in new ways. As an example,                         an end user should be able to ask « What are (according to your dataset) the                         corporate bodies that succeeded to this given entity from its end of                         existence, by 1840, to nowadays (as concerns this given activity) ?» or «                         tell me what instantiations of this photograph exist? » « what are the                         existing copies of this original charter?», and get a list of these                         entities. In other words, institutions or projects that make the effort to                         implement RiC-O will get new insight into the content and context of their                         archives that wasn't visible with the existing ICA-standards. It should be                         even more interesting if you can infer new assertions from the RDF datasets                         you built, and of course link your datasets to other ressources outside of                         your institution.</html:p>                     <html:p>                         <html:strong>RiC-O should be extensible</html:strong>.</html:p>                     <html:p>Institutions are free to extend the ontology by adding new subclasses or                         subproperties if needed. RiC-O has also the potential to be useable in other                         contexts than purely archival ones. This implies that hierarchies of classes                         and properties are defined and that mappings are developed with other                         ontologies as mentioned above. It may also imply that RiC-O should provide                         “hooks” enabling connections with, for example, existing SKOS                         vocabularies.</html:p>                 </html:div>                 <html:div id="understanding-RiCO">                     <html:h4>Understanding RiC-O: a quick overview of some features</html:h4>                     <html:div id="fromRiCCM-to-RiCO">                         <html:h5>From RiC-CM to RiC-O</html:h5>                         <html:p>In the <html:strong>system of classes of RiC-O,</html:strong> for                             each RiC-CM entity, you can find a corresponding class. These classes                             are organized according to the same hierarchy as in RiC-CM. In some                             projects, you may need very few of them (e.g. Agent, Record Resource and                             Activity only), while in other ones, you may need more (e.g. Corporate                             Body and Person; Record; Place; Provenance Relation).</html:p>                         <html:p>Certain classes only exist in RiC-O and not in RiC-CM. These                             additional classes address special needs:</html:p>                         <html:ul>                             <html:li>some correspond to RiC-CM attributes, when it may be considered                                 necessary to handle them as full entities. This is the case for                                     <html:a href="#rico:Type">Type</html:a> and its subclasses, that                                 correspond to RiC-CM attributes that contain controlled values, and                                 that can help to articulate RiC-O with external RDF resources like                                 SKOS vocabularies; and also for <html:a href="#rico:Language">Language</html:a>, <html:a href="#rico:Name">Name</html:a> and                                     <html:a href="#rico:Identifier">Identifier</html:a>, that can be                                 considered as full entities and as key linking nodes in a RDF graph.                                 All these classes have been grouped under a <html:a href="#rico:Concept">Concept</html:a> class. </html:li>                             <html:li>some classes have been added in order to provide a more                                 accurate definition and model for some entities. <html:a href="#rico:Place">Place</html:a> thus comes along with a                                     <html:a href="#rico:PhysicalLocation">Physical Location                                     class</html:a>, and with a <html:a href="#rico:Coordinates">Coordinates class</html:a>. A Place is considered both a                                  geographical and historical entity. As a historical entity, among                                 other features, it has a history, and may be preceded or succeeded                                 by other Places. A Place also may have zero to many Physical                                 Location through time (for instance, its boundaries, if it is an                                 administrative area or a country, may change). Each Physical                                 Location may be connected to zero to many Coordinates. This model is                                 quite close to the Linked Places Format (<html:a href="https://github.com/LinkedPasts/linked-places">https://github.com/LinkedPasts/linked-places</html:a>). Another                                 example of such an addition is the <html:a href="#rico:Proxy">Proxy                                     class</html:a>, that represents (stands for) a Record Resource                                 as it exists in a specific Record Set.</html:li>                             <html:li>finally, a system of classes helps to implement the Relations                                 section of RiC-CM.<html:br></html:br> While these relations also are                                 represented as simple, binary object properties (e.g. <html:a href="#rico:hasProvenance">‘hasProvenance’</html:a> that                                 corresponds to RiC-R026 relation), you may need to assign different                                 attributes to a relation, e.g. a date, certainty or description, as                                 it is already possible, and quite often done, in a XML/EAC-CPF file.                                 One of the standard available methods for representing such a                                 documented relation in RDF for now is to use a class. RDF* and                                 SPARQL* specification, which is being developed by the W3C RDF-DEV                                 Community Group, provides a far simpler method (allowing to consider                                 a triple as the subject or object of another triple; see <html:a href="https://w3c.github.io/rdf-star/">https://w3c.github.io/rdf-star/</html:a>) and is already being                                 used by some tools; however it is not yet a W3C standard. Thus, for                                 example, in RiC-O an <html:a href="#rico:AgentOriginationRelation">AgentOriginationRelation class</html:a> exists. This class may                                 connect one to many Agents to one to many created or accumulated                                 Record Resources or Instantiations, and has some specific object                                 properties (certainty, date, description, source). Back to the                                 ‘hasProvenance’ object property, let us add that it is formally                                 defined in RiC-O, using OWL 2 property chain axiom (see <html:a href="https://www.w3.org/TR/owl2-new-features/">https://www.w3.org/TR/owl2-new-features/</html:a>, as a                                 ‘shortcut’ for the longer path                                 ‘recordResourceOrInstantiationIsSourceOfAgentOriginationRelation/agentOriginationRelationHasTarget’,                                 where the intermediate node is an instance of Agent Origination Relation:<html:br></html:br>                                 <html:code> <owl:propertyChainAxiom                                     rdf:parseType="Collection"> <html:br></html:br> <rdf:Description                                     rdf:about="https://www.ica.org/standards/RiC/ontology#recordResourceOrInstantiationIsSourceOfAgentOriginationRelation"/>                                     <html:br></html:br> <rdf:Description                                     rdf:about="https://www.ica.org/standards/RiC/ontology#agentOriginationRelationHasTarget"/>                                     <html:br></html:br> </owl:propertyChainAxiom> </html:code>                                 <html:br></html:br>A triplestore, with the appropriate configuration, may                                 thus infer the direct ‘hasProvenance’ assertion from this long                                 path.</html:li>                         </html:ul>                         <html:p>Most of the <html:strong>datatype properties in RiC-O                             </html:strong>correspond to RiC-CM attributes that contain free, plain                             text. See for example <html:a href="#rico:descriptiveNote">rico:descriptiveNote</html:a>, <html:a href="#rico:history">rico:history</html:a> and <html:a href="#scopeAndContent">rico:scopeAndContent</html:a>.</html:p>                         <html:p>In addition to these datatype properties, the Name and Identifier                             RiC-CM attributes also have corresponding classes (subclasses of <html:a href="#rico:Appellation">rico:Appellation</html:a>). A resource may                             have several Identifiers and each comes with different attributes (e.g.                             archival reference code, system number, digital object identifier), in                             this case the identifiers will be modelled in a class. In many simpler                             usecases it's sufficent to just use the <html:a href="#rico:identifier">identifier datatype property</html:a>, typically for the archival                             reference code.</html:p>                         <html:p>The Location RiC-CM attribute also has a <html:a href="#rico:PhysicalLocation">rico:Physical Location corresponding                                 class</html:a> (for users who want to use Place, Physical Location                             and Coordinates for handling places).</html:p>                         <html:p>As already said too, every RiC-CM attribute that has ‘controlled                             value’ or ‘rule-based’ as a schema value, has a class as corresponding                             component in RiC-O. For these CM attributes that correspond to a RiC-O                             class, as it is necessary to provide an immediately usable ontology, two                             supplementary datatype properties exist that allow not to use the                             classes, at least for a while, if you want to implement RiC-O and create                             RiC-O/RDF datasets from existing archival metadata without being able to                             handle URIs for the information you have.</html:p>                         <html:p>For example, you may not be able to handle and maintain URIs for                             some controlled values you use in EAD finding aids for carrier types:                             maybe your information system does not use a vocabulary for this, and                             you cannot for a while consider these carrier types as full entities.                             Nevertheless you want to produce RiC-O datasets where every piece of                             information is kept, and you want to avoid blank nodes. If RiC-O would                             only provide the Carrier Type class, it would be an issue for you. So                             RiC-O provides a <html:a href="#rico:type">rico:type datatype                                 property</html:a>, with range rdfs:literal, which allows you to move                             forward.</html:p>                         <html:p>Therefore, for the RiC-CM *Type attributes, you have a corresponding                                 <html:a href="#rico:type">rico:type datatype property</html:a>. For                             RiC-CM Coordinates attribute, you also have <html:a href="#rico:geographicalCoordinates">rico:geographicalCoordinates                                 datatype property</html:a>.</html:p>                         <html:p>These datatype properties have a skos:scopeNote which says (for                             example) "Provided for usability reasons. May be made deprecated or                             removed later on. Use only if you don't use Physical Location and                             Coordinates classes with Place."</html:p>                         <html:p>The same key design principle (RiC-O must be immediately usable) led                             us to define some datatype properties that would enable users to use                             RiC-O in simple usecases where they do not want to consider dates and                             rules as full entities. Thus, there of course is Date and Rule classes                             in RiC-O (since there are Date and Rule entities in RiC-CM). And you                             also have 'date' datatype properties; plus a <html:a href="#rico:ruleFollowed">rico:ruleFollowed datatype                                 property</html:a>. The same analysis led us to keep the <html:a href="#rico:history">rico:history</html:a> datatype property in                             RiC-O (same as RiC-CM history attribute), while RiC-CM and RiC-O also                             provide the <html:a href="#rico:Event">Event class</html:a>, and using a                             series of Events may of course be a better method, easier to query, link                             and display, than simply using a history prose discourse. The two                             methods may be used in parallel within the same dataset by an                             institution that, for example, would decide to emphasize only the                             accession, appraisal and description events among the whole history of                             Record resources.</html:p>                         <html:p>These datatype properties have the same kind of skos:scopeNote as                             above.</html:p>                         <html:p>Finally, we have introduced a few datatype properties that do not                             correspond to any RiC-CM attribute.</html:p>                         <html:p>Some are superproperties, and thus group datatype properties                                 (<html:a href="#rico:physicalOrLogicalExtent">rico:physicalOrLogicalExtent</html:a>, with rico:carrierExtent,                             rico:instantiationExtent and rico:recordResourceExtent as subproperties                             ; <html:a href="#rico:textualValue">rico:textualValue</html:a>, with                             rico:expressedDate and rico:normalizedValue as subproperties; <html:a href="#rico:measure">rico:measure</html:a> (and its subproperties);                                 <html:a href="#rico:referenceSystem">rico:referenceSystem</html:a>,                             superproperty of rico:dateStandard (and of other datatype properties                             that do not exist in RiC-CM.)</html:p>                         <html:p>Some are simply more specific properties : <html:a href="#rico:accrualStatus">rico:accrualStatus</html:a> ; <html:a href="#rico:recordResourceStructure">rico:recordResourceStructure</html:a> and <html:a href="#rico:instantiationStructure">rico:instantiationStructure</html:a>, subproperties of                             rico:structure ; <html:a href="#rico:title">rico:title</html:a>                             (subproperty of rico:name) ; rico:altitude, rico:latitude,                             rico:longitude (subproperties of rico:measure), rico:geodesicSystem and                             rico:altimetricSystem (subproperties of rico:referenceSystem).</html:p>                         <html:p>In order to connect all the classes created, <html:strong>a                                 significant number of object properties have been                                 defined</html:strong>. While their 'flat' list is a long one, they                             are grouped hierarchically, so that one can use the upper to                             intermediate level ones for simplicity sake, or choose the most accurate                             and expressive ones, or extend the system adding a subproperty easily.                             It is expected that, in most use cases, a subset of these properties                             only will be needed. As already said above, some of the object                             properties are also formally defined as shortcuts.</html:p>                         <html:p>Finally, let us mention that we added to RiC-O six individuals,                             considering that they would address current and frequent needs:</html:p>                         <html:ul>                             <html:li>Two (<html:a href="#FindingAid">FindingAid</html:a> and <html:a href="#AuthorityRecord">AuthorityRecord</html:a>) are                                  instances of both RiC-O Documentary Form Type and SKOS Concept.                                 They can be used for categorizing Records, finding aids and                                 authority records being considered as Records. A Record that would                                 have ‘Finding Aid’ as a Documentary Form Type, can be connected to                                 one to many Record Resources using 'rico:describes’ object                                 property.</html:li>                             <html:li>Four (<html:a href="#Fonds">Fonds</html:a>, <html:a href="#Series">Series</html:a>, <html:a href="#File">File</html:a>, and <html:a href="#Collection">Collection</html:a>) are both instances of the Record Set Type                                 class, and of skos:Concept. Their definition is taken from the                                 ISAD(G) glossary. They can be used for categorizing Record                                 Sets.</html:li>                         </html:ul>                         <html:p>In the future, we can imagine that many other categories of the kind                             will be defined by the archival community, forming at least rich SKOS                             (hopefully multilingual) vocabularies. Considering the concepts thus                             defined as being also instances of some RiC-O classes may be of great                             interest for producing a richer description (for example, an instance of                             the <html:a href="#rico:DocumentaryFormType">Documentary Form Type                                 class</html:a> may have a history and some temporal relations to                             other documentary form types).</html:p>                     </html:div>                     <html:div id="RiCO-documentation">                         <html:h5>RiC-O documentation and annotation properties</html:h5>                         <html:p>Each class or property has at least an English label (rdfs:label)                             and description (rdfs:comment). Some have a skos:scopeNote or a                             skos:example.</html:p>                         <html:p>When a RiC-O class or property corresponds in a way to a RiC-CM                             component, its description and scope note are, either the same as, or                             derived from, their definition and scope note in RiC-CM.</html:p>                         <html:p>We have created two annotation properties, subproperties of                             rdfs:comment, for handling:</html:p>                         <html:ul>                             <html:li>Information about the corresponding RiC-CM component when                                 appliable (<html:a href="#rico:RiCCMCorrespondingComponent">rico:RiCCMCorrespondingComponent</html:a>). Various phrasings                                 are used in this property depending on the rule applied for defining                                 the RiC-CM component.</html:li>                             <html:li>Information, most often in prose text for now, about possible                                 mappings with other models or ontologies (<html:a href="#rico:closeTo">rico:closeTo</html:a>, rarely used in this                                 0.1 version)).</html:li>                         </html:ul>                         <html:p>Finally, in this official 0.2 release, any change in the definition                             of a class or property, since December 2019, is documented using a                             skos:changeNote.</html:p>                     </html:div>                 </html:div>                 <html:div id="next-steps">                     <html:h4>Next steps</html:h4>                     <html:p>The following is a non exhaustive list of known issues, topics or tasks                         on which EGAD has begun to work and will continue to work in the next                         months:</html:p>                     <html:ul>                         <html:li>providing more examples of known implementations of RiC-O in                             different institutions and contexts. The goal is to show different                             practices on how RiC-O is being implemented. We have begun to release                             such examples in the <html:a href="https://github.com/ICA-EGAD/RiC-O">public RiC-O repository on GitHub</html:a>. One can also have a                             look at the <html:a href="https://ica-egad.github.io/RiC-O/projects-and-tools.html">Projects and tools page on RiC-O website</html:a>.</html:li>                         <html:li>finishing the system of relations (where some subclasses are still                             missing)</html:li>                         <html:li>assessing, and changing in some cases, the tense of the verbs in                             some object properties (e.g. for the properties that correspond to some                             RiC-CM relations). This has been done, following RiC-CM v0.2 updates,                             where many relations have changed name so that they can be used for                             recording both past and present situations.</html:li>                         <html:li>articulating the Event and Activity classes, and the Relation                             system of classes</html:li>                         <html:li>improving the names of object properties. This has been done,                             following RiC-CM v0.2 updates and applying a few naming rules, so that,                             for example, the same verb is used for naming a relation and the inverse                             relation when it exists.</html:li>                         <html:li>adding suggestions of mappings (in rico:closeTo) and OWL                             equivalences between some classes or properties and components in other                             models (among which - this is not an exhaustive list- CIDOC-CRM,                             IFLA-LRM, PREMIS, PROV-O, Wikidata and Schema.org)</html:li>                         <html:li>documenting RiC-O in French and Spanish</html:li>                         <html:li>providing users with some SPARQL constructs for                             inferring.</html:li>                     </html:ul>                 </html:div>             </html:div>|
//! |**Is defined by**|[https://github.com/ICA-EGAD/RiC-O/raw/master/ontology/current-version/RiC-O_v0-2.rdf](https://github.com/ICA-EGAD/RiC-O/raw/master/ontology/current-version/RiC-O_v0-2.rdf)|
//!

use crate::namespace;

namespace!(
    "https://www.ica.org/standards/RiC/ontology#",;
    /// `Accumulation Relation`: Connects at least one Record Resource or Instantiation to at<br>            least one Agent, when the Agent accumulates it, be it intentionally (collecting it) or<br>            not (receiving it in the course of its activities).
    AccumulationRelation, "AccumulationRelation",
    /// `Activity`: The doing of something for some human purpose.
    Activity, "Activity",
    /// `Activity Documentation Relation`: Connects at least one Record Resource or Instantiation to at<br>            least one Activity, when the Record Resource or Instantiation results from the<br>            activity.
    ActivityDocumentationRelation, "ActivityDocumentationRelation",
    /// `Activity Type`: Categorization of an Activity.
    ActivityType, "ActivityType",
    /// `Agent`: A Person, or Group, or an entity created by a Person or Group<br>            (Mechanism), or a Position, that acts in the world.
    Agent, "Agent",
    /// `Agent Control Relation`: Connects at least one Agent, to at least another Agent, when the<br>            first one(s) control(s) in a way the activities of the second one(s).
    AgentControlRelation, "AgentControlRelation",
    /// `Agent Hierarchical Relation`: Connects at least one Agent to at least another Agent, when the<br>            first one is hierarchically superior to the second one.
    AgentHierarchicalRelation, "AgentHierarchicalRelation",
    /// `Agent Name`: A label, title or term designating an Agent in order to make it<br>            distinguishable from other similar entities.
    AgentName, "AgentName",
    /// `Agent Origination Relation`: Connects at least one Record Resource or an Instantiation to at<br>            least one Agent that creates or accumulates the Record Resource, receives it, or sends<br>            it.
    AgentOriginationRelation, "AgentOriginationRelation",
    /// `Agent Temporal Relation`: Connects at least one Agent, to at least another Agent, that<br>            succeeds it chronologically for, for instance, fullfilling some functions or performing<br>            some activities.
    AgentTemporalRelation, "AgentTemporalRelation",
    /// `Agent Relation`: Connects at least two Agents.
    AgentToAgentRelation, "AgentToAgentRelation",
    /// `Appellation`: A concept of any kind that is used for designating an Entity and<br>            referring to it.
    Appellation, "Appellation",
    /// `Appellation Relation`: The relation between an Appellation and at least one Thing that<br>            the Appellation designates.
    AppellationRelation, "AppellationRelation",
    /// `Authority Relation`: Connects at least one Agent, and at least one Thing over which<br>            the Agent has some authority.
    AuthorityRelation, "AuthorityRelation",
    /// `Authorship Relation`: Connects at least one Record to at least one Person, Group or<br>            Position that is responsible for conceiving and formulating the information contained in<br>            the Record.
    AuthorshipRelation, "AuthorshipRelation",
    /// `Carrier Extent`: The extent of a Record Resource carrier
    CarrierExtent, "CarrierExtent",
    /// `Carrier Type`: Categorization of physical material in or on which information<br>            is represented.
    CarrierType, "CarrierType",
    /// `Child Relation`: Connects at lest one Person, to at least another Person, when<br>            the first has child the second one.
    ChildRelation, "ChildRelation",
    /// `Concept`: An idea, unit of thought, abstract cultural object or<br>            category
    Concept, "Concept",
    /// `Content Type`: The fundamental form of communication in which a Record is<br>            expressed and the human sense through which it is intended to be<br>            perceived.
    ContentType, "ContentType",
    /// `Coordinates`: Longitudinal and latitudinal information of a<br>            Place.
    Coordinates, "Coordinates",
    /// `Corporate Body`: An organized group of persons that act together as an Agent, and<br>            that has a recognized legal or social status.
    CorporateBody, "CorporateBody",
    /// `Corporate Body Type`: Categorization of a Corporate Body.
    CorporateBodyType, "CorporateBodyType",
    /// `Correspondence Relation`: Connects at least two Persons, when they correspond to each<br>            other.
    CorrespondenceRelation, "CorrespondenceRelation",
    /// `Creation Relation`: Connects at least one Record Resource or Instantiation to at<br>            least one Agent, when the Agent is either responsible for all or some of the content of<br>            the Record Resource, or is a contributor to the genesis or production of the<br>            Instantiation.
    CreationRelation, "CreationRelation",
    /// `Date`: Chronological information associated with an entity that<br>            contributes to its identification and contextualization.
    Date, "Date",
    /// `Date Range`: Chronological information associated with an entity that<br>            contributes to its identification and contextualization, that implies or explicitly<br>            states a start date and end date.
    DateRange, "DateRange",
    /// `Date Set`: Non-contiguous single dates or date ranges.
    DateSet, "DateSet",
    /// `Demographic Group`: Categorization of a person according to characteristics such as<br>            age, gender, education, place of origin, ethnic/cultural identification, religion,<br>            etc.
    DemographicGroup, "DemographicGroup",
    /// `Derivation Relation`: Connects an Instantiation to at least one Instantiation that is<br>            derived from it.
    DerivationRelation, "DerivationRelation",
    /// `Descendance Relation`: Connects at least one Person to at least another Person, when<br>            the first has/have descendant the second one(s).
    DescendanceRelation, "DescendanceRelation",
    /// `Documentary Form Type`: Categorization of the document with respect to its extrinsic and<br>            intrinsic elements that together communicate its content, administrative and documentary<br>            context, and authority
    DocumentaryFormType, "DocumentaryFormType",
    /// `Event`: Something that happens in time and space.
    Event, "Event",
    /// `Event Relation`: Connects at least one Event to at least one Thing, when the<br>            first is associated with the existence and lifecycle of the second one.
    EventRelation, "EventRelation",
    /// `Event Type`: Categorization of an Event.
    EventType, "EventType",
    /// `Extent`: Countable characteristics of the content of an entity expressed<br>            as a quantity.
    Extent, "Extent",
    /// `Extent Type`: Categorization of the extent that is being measured
    ExtentType, "ExtentType",
    /// `Family`: Two or more persons related by birth, or through marriage,<br>            adoption, civil union, or other social conventions that bind them together as a socially<br>            recognized familial group.
    Family, "Family",
    /// `Family Relation`: Connects at least two Persons, when they have some family link,<br>            i.e. belong to the same family.
    FamilyRelation, "FamilyRelation",
    /// `Family Type`: Categorization of a Family.
    FamilyType, "FamilyType",
    /// `Functional Equivalence Relation`: Connects at least two Instantiations which may be considered as<br>            equivalent.
    FunctionalEquivalenceRelation, "FunctionalEquivalenceRelation",
    /// `Group`: Two or more Agents that act together as an Agent.
    Group, "Group",
    /// `Group Subdivision Relation`: Connects a Group and at least another Group, when the first one<br>            as the second one(s) among its subdivisions.
    GroupSubdivisionRelation, "GroupSubdivisionRelation",
    /// `Identifier`: A word, number, letter, symbol, or any combination of these used<br>            to uniquely identify or reference an individual instance of an entity within a specific<br>            information domain.
    Identifier, "Identifier",
    /// `Identifier Type`: Categorization of an Identifier.
    IdentifierType, "IdentifierType",
    /// `Instantiation`: The inscription of information made by an Agent on a physical<br>            carrier in any persistent, recoverable form as a means of communicating information<br>            through time and space.
    Instantiation, "Instantiation",
    /// `Instantiation Extent`: The extent of an Instantiation.
    InstantiationExtent, "InstantiationExtent",
    /// `Instantiation to Instantiation Relation`: Connects at least two instantiations
    InstantiationToInstantiationRelation, "InstantiationToInstantiationRelation",
    /// `Intellectual Property Rights Relation`: Connects at least one Agent and one Record Resource or<br>            Instantiation on which the Agent has some intellectual property rights.
    IntellectualPropertyRightsRelation, "IntellectualPropertyRightsRelation",
    /// `Knowing Of Relation`: Connects at least one Person to at least another one, when the<br>            first one has some knowledge of the second one through time or space.
    KnowingOfRelation, "KnowingOfRelation",
    /// `Knowing Relation`: Connects at least two Persons that directly know each other<br>            during their existence. This relation is symmetric.
    KnowingRelation, "KnowingRelation",
    /// `Language`: A spoken or written human language represented in the Record or<br>            Record Part, or used by the Agent.
    Language, "Language",
    /// `Leadership Relation`: Connects at least one Person and at least one Group, when the<br>            first one leads the second one.
    LeadershipRelation, "LeadershipRelation",
    /// `Legal Status`: A status defined by law.
    LegalStatus, "LegalStatus",
    /// `Management Relation`: Connects at least one Agent, and at least one Record Resource or<br>            Instantiation that the Agent manages.
    ManagementRelation, "ManagementRelation",
    /// `Mandate`: Delegation of authority by an Agent to another Agent to perform<br>            an Activity.
    Mandate, "Mandate",
    /// `Mandate Relation`: Connects at least one Mandate, and at least one Agent, when the<br>            first gives the second one the authority or competencies to act. May also involve one to<br>            many Activities that the Mandate(s) assign(s) to the Agent(s).
    MandateRelation, "MandateRelation",
    /// `Mechanism`: A process or system created by a Person or Group that performs<br>            an Activity.
    Mechanism, "Mechanism",
    /// `Membership Relation`: Connects a Group and at least one Person, when the first one has<br>            the second one(s) among its members.
    MembershipRelation, "MembershipRelation",
    /// `Migration Relation`: Connects an Instantiation and at least another Instantiation,<br>            when the first is migrated into the second one(s).
    MigrationRelation, "MigrationRelation",
    /// `Name`: A label, title or term designating the entity in order to make<br>            it distinguishable from other similar entities.
    Name, "Name",
    /// `Occupation Type`: Categorization of a profession, trade, or craft pursued by a<br>            person in fulfilment of an Activity.
    OccupationType, "OccupationType",
    /// `Ownership Relation`: Connects at least one Group, Person or Position, and at least a<br>            Thing that these Agent(s) own(s).
    OwnershipRelation, "OwnershipRelation",
    /// `Performance Relation`: Connects at least one Activity to at least one Agent, when the<br>            first is performed by the second one(s).
    PerformanceRelation, "PerformanceRelation",
    /// `Person`: A human being with a social identity or persona.
    Person, "Person",
    /// `Physical Location`: A delimitation of the physical territory of a<br>            Place.
    PhysicalLocation, "PhysicalLocation",
    /// `Place`: Bounded, named geographic area or region.
    Place, "Place",
    /// `Place Name`: A label, title or term designating a Place in order to make it<br>            distinguishable from other similar entities.
    PlaceName, "PlaceName",
    /// `Place Relation`: Connects a Place and at least one Thing, when the first is<br>            associated with the existence and lifecycle of the second one.
    PlaceRelation, "PlaceRelation",
    /// `Place Type`: Categorization of a Place.
    PlaceType, "PlaceType",
    /// `Position`: The functional role of a Person within a Group.
    Position, "Position",
    /// `Position Holding Relation`: Connects at least one Person, and at least one Position that the<br>            Person occupies.
    PositionHoldingRelation, "PositionHoldingRelation",
    /// `Position to Group Relation`: Connects at least one Position, and a Group, when the first<br>            one(s) exist(s) in/is defined within the second one.
    PositionToGroupRelation, "PositionToGroupRelation",
    /// `Production Technique Type`: Categorization of the method used in the representation of<br>            information on the Instantiation.
    ProductionTechniqueType, "ProductionTechniqueType",
    /// `Provenance Relation`: Specifies the provenance or origin of at least one Record<br>            Resource or Instantiation, for example the relation between a Record Resource and the<br>            Agent which created it or the Activity from which it resulted.
    ProvenanceRelation, "ProvenanceRelation",
    /// `Proxy`: A Proxy represents (stands for) a Record Resource as it exists<br>            in a specific Record Set.
    Proxy, "Proxy",
    /// `Record`: Information inscribed at least once by any method on any<br>            physical carrier in any persistent, recoverable form by an Agent in the course of life<br>            or work Activity.
    Record, "Record",
    /// `Record Part`: Part of a Record with discrete information content that<br>            contributes to the Record's physical or intellectual completeness.
    RecordPart, "RecordPart",
    /// `Record Resource`: A Record, Record Set, or Record Part produced or acquired and<br>            retained by an Agent in the course of Activity.
    RecordResource, "RecordResource",
    /// `Record Resource Extent`: The extent of the content of a Record Resource.
    RecordResourceExtent, "RecordResourceExtent",
    /// `Record Resource Genetic Relation`: Connects two to more Record Resources when there is a genetic<br>            relation between them. Genetic in this sense is as defined by diplomatics, i.e. the<br>            process by which a Record Resource is developed.
    RecordResourceGeneticRelation, "RecordResourceGeneticRelation",
    /// `Record Resource Holding Relation`: Connects at least one Agent, and one or more Record Resource or<br>            Instantiation that the Agent holds.
    RecordResourceHoldingRelation, "RecordResourceHoldingRelation",
    /// `Record Resource to Instantiation Relation`: Connects a Record Resource to one or more Instantiations that<br>            instantiate it.
    RecordResourceToInstantiationRelation, "RecordResourceToInstantiationRelation",
    /// `Record Resource to Record Resource Relation`: Connects at least two Record Resources.
    RecordResourceToRecordResourceRelation, "RecordResourceToRecordResourceRelation",
    /// `Record Set`: One or more records that are associated by categorization and/or<br>            physical aggregation by the creator or other Agent.
    RecordSet, "RecordSet",
    /// `Record Set Type`: A broad categorization of the type of Record Set.
    RecordSetType, "RecordSetType",
    /// `Record State`: Categorization of the production or reproduction status of a<br>            Record or Record Part.
    RecordState, "RecordState",
    /// `Relation`: The top level relation class. It connects at least two Things.<br>            An instance of a Relation may have some datatype and object properties : a descriptive<br>            note (datatype property) like any Thing ; certainty (for 'certain', 'quite probable',<br>            'uncertain','unknown'); a date (use either the date datatype property or the Date class<br>            and isAssociatedWithDate object property ; a state (relationState) ; a location (use<br>            Place class and isAssociatedWithPlace object property) ; a source of information that<br>            can be used as an evidence for it (use either source datatype property or hasSource<br>            object property).
    Relation, "Relation",
    /// `Representation Type`: Categorization of the method of recording the content type of a<br>            Record Resource.
    RepresentationType, "RepresentationType",
    /// `RiC-CM corresponding component`: When it exists, specifies the identifier and name of RiC-CM<br>            component that corresponds to the annotated class or property.
    RiCCMCorrespondingComponent, "RiCCMCorrespondingComponent",
    /// `Role Type`: The role an agent plays in some context (usually in some<br>            creation relation). Not to be confused with a position (position of an agent in some<br>            group). For example, a person who is the head of some corporate body may play the role<br>            of annotator (of a record) in a creation relation.
    RoleType, "RoleType",
    /// `Rule`: Conditions that govern the existence or authority of an Agent or the performance of an Activity, or that contribute to the distinct characteristics of things created or managed by an Agent.
    Rule, "Rule",
    /// `Rule Relation`: Connects at least one Rule to at least one Thing, when it is<br>            associated with existence and lifecycle of the Thing.
    RuleRelation, "RuleRelation",
    /// `Rule Type`: Categorization of a Rule.
    RuleType, "RuleType",
    /// `Sequential Relation`: Connects at least one Thing to at least one Thing that follows<br>            it in some sequence.
    SequentialRelation, "SequentialRelation",
    /// `Sibling Relation`: Connects at least two Persons, when they are<br>            siblings.
    SiblingRelation, "SiblingRelation",
    /// `Single Date`: Chronological information associated with an entity that<br>            contributes to its identification and contextualization, related to a single point in<br>            time.
    SingleDate, "SingleDate",
    /// `Spouse Relation`: Connects at least two Persons, when they are<br>            spouses.
    SpouseRelation, "SpouseRelation",
    /// `Teaching Relation`: Connects at least one Person to at least another Person, who is<br>            their student.
    TeachingRelation, "TeachingRelation",
    /// `Temporal Relation`: Connects at least one Thing to at least one Thing that follows<br>            it in chronological order.
    TemporalRelation, "TemporalRelation",
    /// `Thing`: Any idea, material thing, or event within the realm of human<br>            experience.
    Thing, "Thing",
    /// `Title`: A name that is used for a Record Resource or a<br>            Rule
    Title, "Title",
    /// `Type`: A superclass for any category of some thing. A type<br>            characterizes an entity.
    Type, "Type",
    /// `Type Relation`: Connects a category (a Type) and at least one Thing that belongs<br>            to this category.
    TypeRelation, "TypeRelation",
    /// `Unit Of Measurement`: A definite magnitude of a quantity, defined and adopted by convention or by<br>            law, that is used as a standard for measurement of the same kind of quantity. Can be<br>            spacial units (cm, m), weigt (g, kg), time (s, h), storage (MB, TB) or more informal<br>            units used in the archival context like number of boxes, pages or words.
    UnitOfMeasurement, "UnitOfMeasurement",
    /// `Whole Part Relation`: Connects a Thing to at least one constitutive or component part<br>            of that Thing.
    WholePartRelation, "WholePartRelation",
    /// `Work Relation`: Connects at least two Agents that have some type of work<br>            relation in the course of their activities.
    WorkRelation, "WorkRelation",
    /// `accrual`: Information on the anticipated accession(s) to the Record<br>            Set.
    accrual, "accrual",
    /// `accrual status`: Information on the status of an Accrual
    accrualStatus, "accrualStatus",
    /// `accumulation relation has source `: Connects an Accumulation Relation to one of the accumulated<br>            Record Resources or Instantiations
    accumulationRelationHasSource, "accumulationRelationHasSource",
    /// `accumulation relation has target `: Connects an Accumulation Relation to one of the accumulating<br>            Agents
    accumulationRelationHasTarget, "accumulationRelationHasTarget",
    /// `activity documentation relation has source `: Connects an Activity Documentation Relation to one of the<br>            resulting Record Resources or Instantiations
    activityDocumentationRelationHasSource, "activityDocumentationRelationHasSource",
    /// `activity documentation relation has target `: Connects an Activity Documentation Relation to one of the<br>            documented Activities
    activityDocumentationRelationHasTarget, "activityDocumentationRelationHasTarget",
    /// `activity is context of relation `: Connects an Activity to an Agent Temporal Relation (when the<br>            Activity is transferred from an Agent to another one) or a Mandate Relation (the Mandate<br>            assigns the Activity to the Agent or defines it).
    activityIsContextOfRelation, "activityIsContextOfRelation",
    /// `activity is source of performance relation `: Connects an Activity that is performed to a Performance<br>            Relation
    activityIsSourceOfPerformanceRelation, "activityIsSourceOfPerformanceRelation",
    /// `activity is target of activity documentation relation<br>        `: Connects an Activity to an Activity Documentation<br>            Relation
    activityIsTargetOfActivityDocumentationRelation, "activityIsTargetOfActivityDocumentationRelation",
    /// `affects or affected`: Connects an Event to a Thing on which the Event has or had some<br>            significant impact.
    affectsOrAffected, "affectsOrAffected",
    /// `agent control relation has source `: Connects an Agent Control Relation to one of the controlling<br>            Agents
    agentControlRelationHasSource, "agentControlRelationHasSource",
    /// `agent control relation has target `: Connects an Agent Control Relation to one of the controlled<br>            Agents
    agentControlRelationHasTarget, "agentControlRelationHasTarget",
    /// `agent has work relation `: Connects an Agent to a Work Relation
    agentHasWorkRelation, "agentHasWorkRelation",
    /// `agent hierarchical relation has source `: Connects an Agent Hierarchical Relation to one of the<br>            hierarchically superior Agents
    agentHierarchicalRelationHasSource, "agentHierarchicalRelationHasSource",
    /// `agent hierarchical relation has target `: Connects an Agent Hierarchical Relation to one of the<br>            hierarchically inferior Agents
    agentHierarchicalRelationHasTarget, "agentHierarchicalRelationHasTarget",
    /// `agent is connected to agent relation `: Connects an Agent to an Agent Relation
    agentIsConnectedToAgentRelation, "agentIsConnectedToAgentRelation",
    /// `agent is source of agent control relation `: Connects a controlling Agent to an Agent Control<br>            Relation
    agentIsSourceOfAgentControlRelation, "agentIsSourceOfAgentControlRelation",
    /// `agent is source of agent hierarchical relation `: Connects a hierarchically superior Agent to an Agent<br>            Hierarchical Relation
    agentIsSourceOfAgentHierarchicalRelation, "agentIsSourceOfAgentHierarchicalRelation",
    /// `agent is source of agent temporal relation `: Connects a predecessor Agent to an Agent Temporal<br>            Relation
    agentIsSourceOfAgentTemporalRelation, "agentIsSourceOfAgentTemporalRelation",
    /// `agent is source of authority relation `: Connects an Agent thas has the authority, to an Authority<br>            Relation
    agentIsSourceOfAuthorityRelation, "agentIsSourceOfAuthorityRelation",
    /// `agent is source of intellectual property rights relation<br>        `: Connects an Agent having the intellectual property rights, to an<br>            Intellectual Property Rights Relation
    agentIsSourceOfIntellectualPropertyRightsRelation, "agentIsSourceOfIntellectualPropertyRightsRelation",
    /// `agent is source of management relation `: Connects a manager Agent to a Management Relation
    agentIsSourceOfManagementRelation, "agentIsSourceOfManagementRelation",
    /// `agent is source of ownership relation `: Connects an owner Agent to an Ownership Relation
    agentIsSourceOfOwnershipRelation, "agentIsSourceOfOwnershipRelation",
    /// `agent is source of record resource holding relation `: Connects an Agent that holds a Record Resource or Instantiation,<br>            to a Record Resource Holding Relation
    agentIsSourceOfRecordResourceHoldingRelation, "agentIsSourceOfRecordResourceHoldingRelation",
    /// `agent is target of accumulation relation `: Connects one of the accumulating Agents to an Accumulation<br>            Relation
    agentIsTargetOfAccumulationRelation, "agentIsTargetOfAccumulationRelation",
    /// `agent is target of agent control relation `: Connects one of the controlled Agents to an Agent Control<br>            Relation
    agentIsTargetOfAgentControlRelation, "agentIsTargetOfAgentControlRelation",
    /// `agent is target of agent hierarchical relation `: Connects one of the hierarchically inferior Agents to an Agent<br>            Hierarchical Relation
    agentIsTargetOfAgentHierarchicalRelation, "agentIsTargetOfAgentHierarchicalRelation",
    /// `agent is target of agent origination relation `: Connects one of the Agents that created or accumulated the<br>            Record resource or Instantiation, to an Agent Origination Relation
    agentIsTargetOfAgentOriginationRelation, "agentIsTargetOfAgentOriginationRelation",
    /// `agent is target of agent temporal relation `: Connects a successor Agent to an Agent Temporal<br>            Relation
    agentIsTargetOfAgentTemporalRelation, "agentIsTargetOfAgentTemporalRelation",
    /// `agent is target of authorship relation`: Connects a Person, Group or Position to an Authorship<br>            Relation.
    agentIsTargetOfAuthorshipRelation, "agentIsTargetOfAuthorshipRelation",
    /// `agent is target of creation relation `: Connects a creator Agent to a Creation Relation
    agentIsTargetOfCreationRelation, "agentIsTargetOfCreationRelation",
    /// `agent is target of mandate relation `: Connects a mandated Agent to a Mandate Relation
    agentIsTargetOfMandateRelation, "agentIsTargetOfMandateRelation",
    /// `agent is target of performance relation `: Connects an Agent to a Performance Relation
    agentIsTargetOfPerformanceRelation, "agentIsTargetOfPerformanceRelation",
    /// `agent or activity is target of provenance relation `: Connects an Agent or Activity that is the provenance of a Record<br>            resource or Instantiation, to a Provenance Relation
    agentOrActivityIsTargetOfProvenanceRelation, "agentOrActivityIsTargetOfProvenanceRelation",
    /// `agent origination relation has source `: Connects an Agent Origination Relation to one of the resulting<br>            Record Resource or Instantiation
    agentOriginationRelationHasSource, "agentOriginationRelationHasSource",
    /// `agent origination relation has target `: Connects an Agent Origination Relation to one of the creating or<br>            accumulating Agents
    agentOriginationRelationHasTarget, "agentOriginationRelationHasTarget",
    /// `agent relation connects `: Connects an Agent Relation to one of the involved<br>            Agents
    agentRelationConnects, "agentRelationConnects",
    /// `agent temporal relation has source `: Connects an Agent Temporal Relation to one of the predecessor<br>            Agents
    agentTemporalRelationHasSource, "agentTemporalRelationHasSource",
    /// `agent temporal relation has target `: Connects an Agent Temporal Relation to one of the successor<br>            Agents
    agentTemporalRelationHasTarget, "agentTemporalRelationHasTarget",
    /// `altimetric system`: Reference system used for altitude
    altimetricSystem, "altimetricSystem",
    /// `altitude`: The height of a Place above a reference level, especially above<br>            sea level.
    altitude, "altitude",
    /// `appellation is source of appellation relation `: Connects an Appellation to an Appellation<br>            Relation
    appellationIsSourceOfAppellationRelation, "appellationIsSourceOfAppellationRelation",
    /// `appellation relation has source `: Connects an Appellation Relation to the concerned<br>            Appellation
    appellationRelationHasSource, "appellationRelationHasSource",
    /// `appellation relation has target `: Connects an Appellation Relation to one of the designated<br>            Things
    appellationRelationHasTarget, "appellationRelationHasTarget",
    /// `as concerns activity `: Connects an Agent Temporal Relation or Mandate Relation, to an<br>            Activity that is, either transferred from an Agent to another one, or assigned by a<br>            Mandate to an Agent.
    asConcernsActivity, "asConcernsActivity",
    /// `authenticity note`: Description of evidences that the Record Resource or<br>            Instantiation is what it purports to be, was created or sent by the said Agent, at the<br>            said time and has not been tampered or corrupted.
    authenticityNote, "authenticityNote",
    /// `authority relation has source `: Connects an Authority Relation to an Agent that has the<br>            authority
    authorityRelationHasSource, "authorityRelationHasSource",
    /// `authority relation has target `: Connects an Authority Relation to a Thing over which the<br>            Authority is performed
    authorityRelationHasTarget, "authorityRelationHasTarget",
    /// `authorized by `: Inverse of 'authorizes' object property
    authorizedBy, "authorizedBy",
    /// `authorizes`: Connects a Mandate to the Agent that the Mandate gives the<br>            authority or competencies to act.
    authorizes, "authorizes",
    /// `authorizing agent `: Connects a Mandate Relation to an Agent that assigns the<br>            Mandate.
    authorizingAgent, "authorizingAgent",
    /// `authorizing mandate`: Information on a Mandate that authorizes an Agent to perform an<br>            Activity.
    authorizingMandate, "authorizingMandate",
    /// `authorship relation has source`: Connects an Authorship Relation to one of the Records involved<br>            in the relation.
    authorshipRelationHasSource, "authorshipRelationHasSource",
    /// `authorship relation has target`: Connects an Authorship Relation to one of the author Person,<br>            Group or Position.
    authorshipRelationHasTarget, "authorshipRelationHasTarget",
    /// `beginning date`: Date at which something began.
    beginningDate, "beginningDate",
    /// `birth date`: Date at which a Person was born.
    birthDate, "birthDate",
    /// `calendar`: Used system of reckoning time in which the beginning, length,<br>            and divisions of a year are defined, sometimes along with multiyear<br>            cycles.
    calendar, "calendar",
    /// `carrier extent`: Number of physical units and/or physical dimensions of the<br>            carrier of a record resource instantiation. Various carriers, depending on specific<br>            needs, may have more than one relevant dimension. In some cases, indicating the number<br>            of physical units may be sufficient, while in other case, relevant dimensions should be<br>            used in order to characterize the carrier.
    carrierExtent, "carrierExtent",
    /// `certainty`: Qualifies the level of certitude of the accuracy of a Date, an<br>            Event or a Relation.
    certainty, "certainty",
    /// `child relation has source `: Connects a Child Relation to a parent Person
    childRelationHasSource, "childRelationHasSource",
    /// `child relation has target `: Connects a Child Relation to a child Person
    childRelationHasTarget, "childRelationHasTarget",
    /// `classification`: A term, number or alphanumeric string that is usually taken from<br>            an external classification vocabulary or scheme that qualifies the Record<br>            Resource.
    classification, "classification",
    /// `close to`: An annotation property for recording a possible mapping to a<br>            component in another model or ontology
    closeTo, "closeTo",
    /// `conditions of access`: Terms and circumstances affecting the availability of a Record<br>            Resource for consultation. Such conditions may originate in laws, regulations and<br>            policies, including those pertaining to privacy and security concerns or restrictions;<br>            they may concern a specific Instantiation of a Record Resource, for example, conditions<br>            that require preservation treatment; or they may specify the software or hardware<br>            necessary to access the Instantiation.
    conditionsOfAccess, "conditionsOfAccess",
    /// `conditions of use`: Terms and circumstances affecting the use of a Record Resource<br>            after access has been provided. Includes conditions governing reproduction of the Record<br>            Resource under applicable copyright (intellectual property) and/or property legislation,<br>            and of the Instantiation, due to conservation status.
    conditionsOfUse, "conditionsOfUse",
    /// `contains or contained`: Connects a Place to a region that is or was within<br>            it.
    containsOrContained, "containsOrContained",
    /// `correspondence relation connects `: Connects a Correspondence Relation to one of the Persons<br>            involved
    correspondenceRelationConnects, "correspondenceRelationConnects",
    /// `creation date`: Date at which an entity was created.
    creationDate, "creationDate",
    /// `creation relation has source `: Connects a Creation Relation to one of the created Record<br>            Resources or Instantiations
    creationRelationHasSource, "creationRelationHasSource",
    /// `creation relation has target `: Connects a Creation Relation to one of the creator<br>            Agents
    creationRelationHasTarget, "creationRelationHasTarget",
    /// `creation with role `: Connects a Creation Relation to the Role Type that the creator<br>            Agent(s) has in the creation process
    creationWithRole, "creationWithRole",
    /// `date`: Chronological information associated with an entity that<br>            contributes to its identification and contextualization.
    date, "date",
    /// `date qualifier`: Indicates the precision of a date. It specifies if, and to what<br>            extent, the value is an estimation.
    dateQualifier, "dateQualifier",
    /// `date standard`: Identifier of the standard of the Normalized<br>            date.
    dateStandard, "dateStandard",
    /// `death date`: Date at which a Person died.
    deathDate, "deathDate",
    /// `deletion date`: Date at which an entity was deleted.
    deletionDate, "deletionDate",
    /// `derivation relation has source `: Connects a Derivation Relation to the Instantiation from which<br>            one or more Instantiations is derived.
    derivationRelationHasSource, "derivationRelationHasSource",
    /// `derivation relation has target `: Connects a Derivation Relation to one of the derived<br>            Instantiations
    derivationRelationHasTarget, "derivationRelationHasTarget",
    /// `descendance relation has source `: Connects a Descendance Relation to one of the ancestor<br>            Persons
    descendanceRelationHasSource, "descendanceRelationHasSource",
    /// `descendance relation has target `: Connects a Descendance Relation to one of the descendant<br>            Persons
    descendanceRelationHasTarget, "descendanceRelationHasTarget",
    /// `describes or described`: Connects a Record Resource to a Thing that it<br>            describes.
    describesOrDescribed, "describesOrDescribed",
    /// `descriptive note`: Descriptive information about an entity that is not otherwise<br>            addressed.
    descriptiveNote, "descriptiveNote",
    /// `documented by`: Inverse of 'documents' object property.
    documentedBy, "documentedBy",
    /// `documents`: Connects a Record Resource or an Instantiation to the Activity<br>            that generates the Record Resource or Instantiation.
    documents, "documents",
    /// `end date`: Date at which something ended.
    endDate, "endDate",
    /// `event is source of event relation `: Connects an Event to an Event Relation
    eventIsSourceOfEventRelation, "eventIsSourceOfEventRelation",
    /// `event relation has source `: Connects an Event Relation to an Event
    eventRelationHasSource, "eventRelationHasSource",
    /// `event relation has target `: Connects an Event Relation to an associated Thing
    eventRelationHasTarget, "eventRelationHasTarget",
    /// `exists or existed in`: Connects a Position to a Group in which that Position exists or<br>            existed, or that is defined by that Group�s organizational structure.
    existsOrExistedIn, "existsOrExistedIn",
    /// `expressed date`: Natural language expression of a Date.
    expressedDate, "expressedDate",
    /// `expresses or expressed`: Inverse of 'is or was expressed by' object<br>            property.
    expressesOrExpressed, "expressesOrExpressed",
    /// `family relation connects `: Connects a Family Relation to a Person.
    familyRelationConnects, "familyRelationConnects",
    /// `follows in time`: Inverse of 'precedes in time' object property.
    followsInTime, "followsInTime",
    /// `follows or followed`: Inverse of 'precedesOrPreceded' object property.
    followsOrFollowed, "followsOrFollowed",
    /// `functional equivalence relation connects `: Connects a Functional Equivalence Relation to one of the<br>            functionally equivalent Instantiations.
    functionalEquivalenceRelationConnects, "functionalEquivalenceRelationConnects",
    /// `geodesic system`: Reference system used for geographical<br>            coordinates.
    geodesicSystem, "geodesicSystem",
    /// `geographical coordinates`: Longitudinal and latitudinal information of a<br>            Place.
    geographicalCoordinates, "geographicalCoordinates",
    /// `group is source of group subdivision relation `: Connects the Group that has at least a subdivision, to a Group<br>            Subdivision Relation
    groupIsSourceOfGroupSubdivisionRelation, "groupIsSourceOfGroupSubdivisionRelation",
    /// `group is source of membership relation `: Connects the Group (that has one to many members) to a<br>            Membership Relation
    groupIsSourceOfMembershipRelation, "groupIsSourceOfMembershipRelation",
    /// `group is target of group subdivision relation `: Connects a Group that is a subdivision, to a Group Subdivision<br>            Relation
    groupIsTargetOfGroupSubdivisionRelation, "groupIsTargetOfGroupSubdivisionRelation",
    /// `group is target of leadership relation `: Connects a Group (which has a leader) to a Leadership<br>            Relation
    groupIsTargetOfLeadershipRelation, "groupIsTargetOfLeadershipRelation",
    /// `group is target of position to group relation `: Connects the Group (in which a Position exists) to a Position To<br>            Group Relation
    groupIsTargetOfPositionToGroupRelation, "groupIsTargetOfPositionToGroupRelation",
    /// `group subdivision relation has source `: Connects a Group Subdivision Relation to the Group that has<br>            subdivisions
    groupSubdivisionRelationHasSource, "groupSubdivisionRelationHasSource",
    /// `group subdivision relation has target `: Connects a Group Subdivision Relation to one of the Groups that<br>            is a subdivision
    groupSubdivisionRelationHasTarget, "groupSubdivisionRelationHasTarget",
    /// `has accumulator`: Connects a Record Resource or an Instantiation to the Agent that<br>            accumulates it, be it intentionally (collecting) or not (receiving in the course of its<br>            activities).
    hasAccumulator, "hasAccumulator",
    /// `has activity type`: Connects an Activity to an Activity Type that categorizes<br>            it.
    hasActivityType, "hasActivityType",
    /// `has addressee`: Connects a Record Resource or an Instantiation to the Agent that<br>            it is addressed to.
    hasAddressee, "hasAddressee",
    /// `has ancestor`: Inverse of 'has descendant' object property.
    hasAncestor, "hasAncestor",
    /// `has author`: Connects a Record to the Group, Person or Position that is<br>            responsible for conceiving and formulating the information contained in the<br>            Record.
    hasAuthor, "hasAuthor",
    /// `has beginning date `: Inverse of 'is beginning date of' object property
    hasBeginningDate, "hasBeginningDate",
    /// `has birth date `: Inverse of 'is birth date of' object property
    hasBirthDate, "hasBirthDate",
    /// `has carrier type`: Connects an Instantiation to a Carrier Type which categorizes<br>            its carrier.
    hasCarrierType, "hasCarrierType",
    /// `has child`: Connects a Person to one of their children.
    hasChild, "hasChild",
    /// `has collector`: Connects a Record Resource or an Instantiation to the Agent that<br>            collects it intentionally (is a collector).
    hasCollector, "hasCollector",
    /// `has content of type`: Connects a Record or a Record Part to a Content Type which<br>            categorizes its content.
    hasContentOfType, "hasContentOfType",
    /// `has copy `: Connects a Record Resource to a copy of that Record<br>            Resource.
    hasCopy, "hasCopy",
    /// `has creator`: Connects a Record Resource or an Instantiation to an Agent that<br>            is either responsible for all or some of the content of the Record Resource or is a<br>            contributor to the genesis or production of an Instantiation.
    hasCreator, "hasCreator",
    /// `has death date `: Inverse of 'is death date of' object property
    hasDeathDate, "hasDeathDate",
    /// `has derived instantiation `: Connects an Instantiation to an Instantiation that is derived<br>            from it.
    hasDerivedInstantiation, "hasDerivedInstantiation",
    /// `has descendant`: Connects a Person to one of their descendants.
    hasDescendant, "hasDescendant",
    /// `has documentary form type`: Connects a Record or Record Part to its Documentary Form<br>            Type.
    hasDocumentaryFormType, "hasDocumentaryFormType",
    /// `has draft `: Inverse of 'is draft of' object property.
    hasDraft, "hasDraft",
    /// `has end date `: Inverse of 'is end date of' object property.
    hasEndDate, "hasEndDate",
    /// `has event type`: Connects an Event to an Event Type which categorizes<br>            it.
    hasEventType, "hasEventType",
    /// `has extent`: Connects a Record Resource or Instantiation to an Extent
    hasExtent, "hasExtent",
    /// `has extent type`: Connects an Extent to an Extent Type that categorizes what is being<br>            measured.
    hasExtentType, "hasExtentType",
    /// `has family association with`: Connects two Persons that have some type of family link, i.e.<br>            belong to the same family. This relation is symmetric.
    hasFamilyAssociationWith, "hasFamilyAssociationWith",
    /// `has family type`: Connects a Family to a Family Type that categorizes<br>            it.
    hasFamilyType, "hasFamilyType",
    /// `has genetic link to record resource `: Connects two Record Resources when there is a genetic link<br>            between them. Genetic in this sense is as defined by diplomatics, i.e. the process by<br>            which a Record Resource is developed. This relation is symmetric.
    hasGeneticLinkToRecordResource, "hasGeneticLinkToRecordResource",
    /// `has identifier type`: Connects an Identifier and an Identifier Type that categorizes<br>            it.
    hasIdentifierType, "hasIdentifierType",
    /// `has instantiation`: Connects a Record Resource to one of its<br>            Instantiations.
    hasInstantiation, "hasInstantiation",
    /// `has modification date `: Inverse of 'is modification date of' object<br>            property.
    hasModificationDate, "hasModificationDate",
    /// `has or had agent name`: Connects an Agent and (one of) its present or past Agent<br>            Name.
    hasOrHadAgentName, "hasOrHadAgentName",
    /// `has or had all members with category`: Connects a Record Set and a Category (Type) to which all the<br>            Records or Record Parts that are or were included in the Record Set<br>            belong.
    hasOrHadAllMembersWithCategory, "hasOrHadAllMembersWithCategory",
    /// `has of had all members with content type`: Connects a Record Set and a Content Type that categorizes all<br>            the Records or Record Parts that are or were included in the Record Set.
    hasOrHadAllMembersWithContentType, "hasOrHadAllMembersWithContentType",
    /// `has of had all members with documentary form type`: Connects a Record Set and a Documentary Form Type that<br>            categorizes all the Records or Record Parts that are or were included in the Record<br>            Set.
    hasOrHadAllMembersWithDocumentaryFormType, "hasOrHadAllMembersWithDocumentaryFormType",
    /// `has of had all members with language`: Connects a Record Set and a Language used by all the Records or<br>            Record Parts that are or were included in the Record Set.
    hasOrHadAllMembersWithLanguage, "hasOrHadAllMembersWithLanguage",
    /// `has of had all members with legal status`: Connects a Record Set and a Legal Status that categorizes all<br>            the Records or Record Parts that are or were included in the Record Set.
    hasOrHadAllMembersWithLegalStatus, "hasOrHadAllMembersWithLegalStatus",
    /// `has of had all members with record state`: Connects a Record Set and a Record State that categorizes all<br>            the Records or Record Parts that are or were included in the Record Set.
    hasOrHadAllMembersWithRecordState, "hasOrHadAllMembersWithRecordState",
    /// `has or had appellation`: Connects a Thing to an Appellation that is or was used for<br>            designating it.
    hasOrHadAppellation, "hasOrHadAppellation",
    /// `has or had authority over`: Connects an Agent to a Thing the Agent has or had authority<br>            over.
    hasOrHadAuthorityOver, "hasOrHadAuthorityOver",
    /// `has or had category`: Connects a Thing to a Type that categorizes or categorized<br>            it.
    hasOrHadCategory, "hasOrHadCategory",
    /// `has or had component`: Connects an Instantiation to one of its present or past<br>            component instantiations.
    hasOrHadComponent, "hasOrHadComponent",
    /// `has or had constituent`: Connects a Record to a Record Part that is or was a component of<br>            that Record.
    hasOrHadConstituent, "hasOrHadConstituent",
    /// `has or had controller`: Inverse of 'is or was controller of' object<br>            property.
    hasOrHadController, "hasOrHadController",
    /// `has coordinates`: Connects a Physical Location to its past or present coordinates<br>            in a reference system.
    hasOrHadCoordinates, "hasOrHadCoordinates",
    /// `has or had corporate body type`: Connects a Corporate Body to a Corporate Body Type which<br>            categorizes or categorized it.
    hasOrHadCorporateBodyType, "hasOrHadCorporateBodyType",
    /// `has or had correspondent`: Connects two Persons that correspond or have corresponded with<br>            each other. This relation is symmetric.
    hasOrHadCorrespondent, "hasOrHadCorrespondent",
    /// `has or had demographic group`: Connects a Person or a Group to a Demographic Group to which it<br>            belongs or belonged.
    hasOrHadDemographicGroup, "hasOrHadDemographicGroup",
    /// `has or had holder`: Inverse of 'is or was holder of' object property.
    hasOrHadHolder, "hasOrHadHolder",
    /// `has or had identifier`: Connects a Thing to one of its past or present<br>            Identifiers.
    hasOrHadIdentifier, "hasOrHadIdentifier",
    /// `has or had intellectual property rights holder`: Inverse of 'is or was holder of intellectual property rights of'<br>            object property.
    hasOrHadIntellectualPropertyRightsHolder, "hasOrHadIntellectualPropertyRightsHolder",
    /// `has or had jurisdiction`: Inverse of 'is or was jurisdiction of' object<br>            property.
    hasOrHadJurisdiction, "hasOrHadJurisdiction",
    /// `has or had language`: Connects an Agent or Record Resource to a Language that it uses<br>            or used.
    hasOrHadLanguage, "hasOrHadLanguage",
    /// `has or had leader`: Inverse of 'is or was leader of' object property.
    hasOrHadLeader, "hasOrHadLeader",
    /// `has or had legal status`: Connects an Agent or Record Resource to a Legal Status which<br>            categorized or categorizes it.
    hasOrHadLegalStatus, "hasOrHadLegalStatus",
    /// `has or had location`: Inverse of 'is or was location of' object<br>            property.
    hasOrHadLocation, "hasOrHadLocation",
    /// `has or had main subject`: Connects a Record Resource to a Thing that is or was its main<br>            subject.
    hasOrHadMainSubject, "hasOrHadMainSubject",
    /// `has or had manager`: Inverse of 'is or was manager of' object<br>            property.
    hasOrHadManager, "hasOrHadManager",
    /// `has or had member`: Connects a Group to a Person that is or was a member of that<br>            Group.
    hasOrHadMember, "hasOrHadMember",
    /// `has or had name`: Connects a Thing to one of its past or present<br>            Names.
    hasOrHadName, "hasOrHadName",
    /// `has or had occupation of type`: Connects a Person to an Occupation Type that categorized or<br>            categorizes his/her occupation (profession, trade or craft).
    hasOrHadOccupationOfType, "hasOrHadOccupationOfType",
    /// `has or had owner`: Inverse of 'is or was owner of' object property.
    hasOrHadOwner, "hasOrHadOwner",
    /// `has or had part`: Connects a Thing to a constitutive or component part of that<br>            Thing.
    hasOrHadPart, "hasOrHadPart",
    /// `has or had participant`: Connects an Event to a Thing that is or was actively or<br>            passively involved in it.
    hasOrHadParticipant, "hasOrHadParticipant",
    /// `has or had physical location`: Connects a Place to one of its past or present Physical<br>            Location.
    hasOrHadPhysicalLocation, "hasOrHadPhysicalLocation",
    /// `has or had place name`: Connects a Place to one of its past or present<br>            names.
    hasOrHadPlaceName, "hasOrHadPlaceName",
    /// `has or had place type`: Connects a Place to a Place Type that categorized or categorizes<br>            it.
    hasOrHadPlaceType, "hasOrHadPlaceType",
    /// `has or had position`: Inverse of 'exists or existed in' object<br>            property.
    hasOrHadPosition, "hasOrHadPosition",
    /// `has or had rule type`: Connects a Rule to a Rule Type that categorized or categorizes<br>            it.
    hasOrHadRuleType, "hasOrHadRuleType",
    /// `has or had some members with category`: Connects a Record Set and a Category (Type) to which some of the<br>            Records or Record Parts that are or were included in the Record Set<br>            belong.
    hasOrHadSomeMembersWithCategory, "hasOrHadSomeMembersWithCategory",
    /// `has or had some members with content type`: Connects a Record Set and a Content Type that categorizes some<br>            of the Records or Record Parts that are or were included in the Record<br>            Set.
    hasOrHadSomeMembersWithContentType, "hasOrHadSomeMembersWithContentType",
    /// `has or had some members with language`: Connects a Record Set and a Language used by some of the Records<br>            or Record Parts that are or were included in the Record Set.
    hasOrHadSomeMembersWithLanguage, "hasOrHadSomeMembersWithLanguage",
    /// `has or had some members with legal status`: Connects a Record Set and a Legal Status that categorizes some<br>            of the Records or Record Parts that are or were included in the Record<br>            Set.
    hasOrHadSomeMembersWithLegalStatus, "hasOrHadSomeMembersWithLegalStatus",
    /// `has or had some members with record state`: Connects a Record Set and a Record State that categorizes some<br>            of the Records or Record Parts that are or were included in the Record<br>            Set.
    hasOrHadSomeMembersWithRecordState, "hasOrHadSomeMembersWithRecordState",
    /// `has or had some members with documentary form type`: Connects a Record Set and a Documentary Form Type that<br>            categorizes some of the Records or Record Parts that are or were included in the Record<br>            Set.
    hasOrHadSomeMemberswithDocumentaryFormType, "hasOrHadSomeMemberswithDocumentaryFormType",
    /// `has or had spouse`: Connects two Persons that are or were married. This relation is<br>            symmetric.
    hasOrHadSpouse, "hasOrHadSpouse",
    /// `has or had student`: Inverse of 'has or had teacher' object property.
    hasOrHadStudent, "hasOrHadStudent",
    /// `has or had subdivision`: Connects a Group to one of its present or past<br>            subdivisions.
    hasOrHadSubdivision, "hasOrHadSubdivision",
    /// `has or had subevent`: Connects an Event to one of a series of Events that constitute<br>            the original, broader, past or ongoing Event.
    hasOrHadSubevent, "hasOrHadSubevent",
    /// `has or had subject`: Connects a Record Resource to a Thing that is or was its<br>            subject.
    hasOrHadSubject, "hasOrHadSubject",
    /// `has or had subordinate`: Connects an Agent to an Agent that is hierarchically<br>            inferior.
    hasOrHadSubordinate, "hasOrHadSubordinate",
    /// `has or had teacher`: Connects a Person to another Person who is or was their<br>            student.
    hasOrHadTeacher, "hasOrHadTeacher",
    /// `has or had title`: Connects a Record Resource, Instantiation or Rule to a title<br>            that is or was used for designating it.
    hasOrHadTitle, "hasOrHadTitle",
    /// `has or had work relation with`: Connects two Agents that have or had some type of work relation<br>            in the course of their activities. This relation is symmetric.
    hasOrHadWorkRelationWith, "hasOrHadWorkRelationWith",
    /// `has original `: Inverse of 'is original of' object property.
    hasOriginal, "hasOriginal",
    /// `has production technique type`: Connects an Instantiation to a Production Technique Type that<br>            categorizes its production technique.
    hasProductionTechniqueType, "hasProductionTechniqueType",
    /// `has provenance `: Connects a Record Resource or an Instantiation to an Agent that<br>            creates or accumulates the Record Resource, receives it, or sends it.
    hasProvenance, "hasProvenance",
    /// `hasPublisher`: Connects a Record resource to an Agent who published<br>            it.
    hasPublisher, "hasPublisher",
    /// `has receiver`: Connects a Record Resource or an Instantiation to the Agent that<br>            receives it in the course of its activities.
    hasReceiver, "hasReceiver",
    /// `has record set type`: Connects a Record Set to a Record Set Type that categorizes<br>            it.
    hasRecordSetType, "hasRecordSetType",
    /// `has record state`: Connects a Record or Record Part to a Record State that<br>            categorizes its state.
    hasRecordState, "hasRecordState",
    /// `has reply`: Connects a Record Resource to a reply, usually in the form of<br>            correspondence.
    hasReply, "hasReply",
    /// `has representation type`: Connects an Instantiation to a Representation Type that<br>            categorizes its representation type.
    hasRepresentationType, "hasRepresentationType",
    /// `has sender `: Connects a Record Resource or an Instantiation to the Agent that<br>            sends it
    hasSender, "hasSender",
    /// `has sibling`: Connects two Persons that are siblings. This relation is<br>            symmetric.
    hasSibling, "hasSibling",
    /// `has source `: Connects a Record Resource or Relation to a Record Resource or<br>            Agent that is used as a source of information for identifying or describing<br>            it.
    hasSource, "hasSource",
    /// `has successor`: Connects an Agent to another Agent that succeeds it<br>            chronologically.
    hasSuccessor, "hasSuccessor",
    /// `has unit of measurement`: Connects an Extent to a Unit Of Measurement
    hasUnitOfMeasurement, "hasUnitOfMeasurement",
    /// `height`: Vertical dimension of an entity.
    height, "height",
    /// `history`: Summary of the development of an entity, since its origin until<br>            present time.
    history, "history",
    /// `identifier`: A word, number, letter, symbol, or any combination of these used<br>            to uniquely identify or reference an individual instance of an entity within a specific<br>            information domain. Includes Global Persistent Identifiers (globally unique and<br>            persistently resolvable identifier for the entity) and/or Local<br>            Identifiers.
    identifier, "identifier",
    /// `includes or included`: Connects a Record Set to a Record or Record Set it aggregates,<br>            or aggregated in the past.
    includesOrIncluded, "includesOrIncluded",
    /// `Instantiation extent`: Countable characteristics of the Instantiation expressed as a<br>            quantity.
    instantiationExtent, "instantiationExtent",
    /// `instantiation is connected to functional equivalence relation<br>        `: Connects an Instantiation to a Functional Equivalence<br>            Relation
    instantiationIsConnectedToFunctionalEquivalenceRelation, "instantiationIsConnectedToFunctionalEquivalenceRelation",
    /// `instantiation is connected to instantiation relation `: Connects an Instantiation to an Instantiation to Instantiation<br>            Relation.
    instantiationIsConnectedToInstantiationRelation, "instantiationIsConnectedToInstantiationRelation",
    /// `instantiation is source of derivation relation `: Connects an Instantiation (from which at least one Instantiation<br>            is derived) to a Derivation Relation.
    instantiationIsSourceOfDerivationRelation, "instantiationIsSourceOfDerivationRelation",
    /// `instantiation is source of migration relation `: Connects an Instantiation (from which at least one Instantiation<br>            is migrated) to a Migration Relation.
    instantiationIsSourceOfMigrationRelation, "instantiationIsSourceOfMigrationRelation",
    /// `instantiation is target of derivation relation `: Connects a derived Instantiation to a Derivation<br>            Relation.
    instantiationIsTargetOfDerivationRelation, "instantiationIsTargetOfDerivationRelation",
    /// `instantiation is target of migration relation `: Connects an Instantiation which results from a migration, to a<br>            Migration Relation.
    instantiationIsTargetOfMigrationRelation, "instantiationIsTargetOfMigrationRelation",
    /// `instantiation is target of record resource to instantiation<br>            relation `: Connects an Instantiation of a Record Resource to the Record<br>            Resource to Instantiation Relation.
    instantiationIsTargetOfRecordResourceToInstantiationRelation, "instantiationIsTargetOfRecordResourceToInstantiationRelation",
    /// `Instantiation structure`: Information about the physical arrangement and composition of an<br>            Instantiation.
    instantiationStructure, "instantiationStructure",
    /// `instantiation to instantiation relation connects `: Connects an Instantiation to Instantiation Relation to one of<br>            the related Instantiations.
    instantiationToInstantiationRelationConnects, "instantiationToInstantiationRelationConnects",
    /// `integrity`: Information about the completeness of a Record Resource or<br>            Instantiation.
    integrity, "integrity",
    /// `intellectual property rights relation has source `: Connects an IntellectualPropertyRightsRelation to one of the<br>            Group, Person or Position that holds the rights.
    intellectualPropertyRightsRelationHasSource, "intellectualPropertyRightsRelationHasSource",
    /// `intellectual property rights relation has target `: Connects an IintellectualPropertyRightsRelation to one of the<br>            Record Resource or Instantiation on which the rights are held.
    intellectualPropertyRightsRelationHasTarget, "intellectualPropertyRightsRelationHasTarget",
    /// `is accumulator of`: Inverse of 'has accumulator' object property.
    isAccumulatorOf, "isAccumulatorOf",
    /// `is activity type of`: Connects an Activity Type to an Activity that it<br>            categorizes.
    isActivityTypeOf, "isActivityTypeOf",
    /// `is addressee of `: Inverse of 'has addressee' object property.
    isAddresseeOf, "isAddresseeOf",
    /// `is agent associated with agent `: Connects two Agents. This object property is<br>            symmetric.
    isAgentAssociatedWithAgent, "isAgentAssociatedWithAgent",
    /// `is associated with date `: Inverse of 'is date associated with' object<br>            property.
    isAssociatedWithDate, "isAssociatedWithDate",
    /// `is associated with event `: Inverse of 'is event associated with' object<br>            property.
    isAssociatedWithEvent, "isAssociatedWithEvent",
    /// `is associated with place `: Inverse of 'is place associated with' object<br>            property.
    isAssociatedWithPlace, "isAssociatedWithPlace",
    /// `is associated with rule `: Inverse of 'is rule associated with' object<br>            property.
    isAssociatedWithRule, "isAssociatedWithRule",
    /// `is author of`: Inverse of 'has author' object property.
    isAuthorOf, "isAuthorOf",
    /// `is authorizing agent in mandate relation `: Connects an Agent that assigns the Mandate, to a Mandate<br>            Relation.
    isAuthorizingAgentInMandateRelation, "isAuthorizingAgentInMandateRelation",
    /// `is beginning date of `: Connects a Date to a Thing that came into existence on that<br>            Date.
    isBeginningDateOf, "isBeginningDateOf",
    /// `is birth date of `: Connects a Date to a Person that was born on that<br>            Date.
    isBirthDateOf, "isBirthDateOf",
    /// `is carrier type of`: Connects a Carrier Type to an Instantiation whose carrier it<br>            categorizes.
    isCarrierTypeOf, "isCarrierTypeOf",
    /// `is child of`: Inverse of 'has child' object property.
    isChildOf, "isChildOf",
    /// `is collector of`: Inverse of 'has collector' object property.
    isCollectorOf, "isCollectorOf",
    /// `is content type of`: Connects a Content Type to a Record or Record Part whose content<br>            it categorizes.
    isContentTypeOf, "isContentTypeOf",
    /// `is copy of`: Inverse of 'has copy' object property.
    isCopyOf, "isCopyOf",
    /// `is creator of`: Inverse of 'has creator' object property.
    isCreatorOf, "isCreatorOf",
    /// `is date associated with `: Connects a Date to a Thing that the Date is associated with the<br>            existence and lifecycle of.
    isDateAssociatedWith, "isDateAssociatedWith",
    /// `is death date of `: Connects a Date to a Person who died on that<br>            Date.
    isDeathDateOf, "isDeathDateOf",
    /// `is derived from instantiation `: Inverse of 'has derived instantiation' object<br>            property.
    isDerivedFromInstantiation, "isDerivedFromInstantiation",
    /// `is documentary form type of`: Connects a Documentary Form Type to a Record or Record Part that<br>            it categorizes.
    isDocumentaryFormTypeOf, "isDocumentaryFormTypeOf",
    /// `is draft of `: Connects a draft to the final version of a<br>            Record.
    isDraftOf, "isDraftOf",
    /// `is end date of `: Connects a Date to a Thing whose existence ended on that<br>            Date.
    isEndDateOf, "isEndDateOf",
    /// `is equivalent to `: Connects two Things that are considered<br>            equivalent.
    isEquivalentTo, "isEquivalentTo",
    /// `is event associated with `: Connects an Event to a Thing that is associated with the<br>            existence and lifecycle of the Event.
    isEventAssociatedWith, "isEventAssociatedWith",
    /// `is event type of`: Connects an Event Type to an Event that is<br>            categorizes.
    isEventTypeOf, "isEventTypeOf",
    /// `is extent of`: Connects an Extent to a Record Resource or Instantiation
    isExtentOf, "isExtentOf",
    /// `is extent type of`: Connects an Extent Type to an Extent that it<br>            categorizes.
    isExtentTypeOf, "isExtentTypeOf",
    /// `is family type of`: Connects a Family Type to a Family that is<br>            categorizes.
    isFamilyTypeOf, "isFamilyTypeOf",
    /// `is from use date of `: Connects a Date to an Appellation, when it is the date at which<br>            the Appellation was first used.
    isFromUseDateOf, "isFromUseDateOf",
    /// `is functionally equivalent to`: Connects two Instantiations which may be considered as<br>            equivalent. This relation is symmetric.
    isFunctionallyEquivalentTo, "isFunctionallyEquivalentTo",
    /// `is identifier type of`: Connects an Identifier Type and an Identifier that it<br>            categorizes.
    isIdentifierTypeOf, "isIdentifierTypeOf",
    /// `is instantiation associated with instantiation `: Connects two Instantiations. This object property is<br>            symmetric.
    isInstantiationAssociatedWithInstantiation, "isInstantiationAssociatedWithInstantiation",
    /// `is instantiation of`: Inverse of 'has instantiation' object property.
    isInstantiationOf, "isInstantiationOf",
    /// `is last update date of `: Connects a Date and a Thing that was last modified at this<br>            Date.
    isLastUpdateDateOf, "isLastUpdateDateOf",
    /// `is modification date of `: Connects a Date to a Thing that was modified on that<br>            Date.
    isModificationDateOf, "isModificationDateOf",
    /// `is or was adjacent to`: Connects two Places that are or were geographically adjacent.<br>            This is a symmetric object property.
    isOrWasAdjacentTo, "isOrWasAdjacentTo",
    /// `is or was affected by`: Inverse of 'affects or affected' object property.
    isOrWasAffectedBy, "isOrWasAffectedBy",
    /// `is or was agent name of`: Connects an Agent Name to an Agent it designates or<br>            designated.
    isOrWasAgentNameOf, "isOrWasAgentNameOf",
    /// `is or was appellation of`: Connects an Appellation to a Thing that it designates or<br>            designated.
    isOrWasAppellationOf, "isOrWasAppellationOf",
    /// `is or was category of`: Connects a Type (a category) to a Thing that it categorizes or<br>            categorized.
    isOrWasCategoryOf, "isOrWasCategoryOf",
    /// `is or was category of all members of`: Connects a Category (Type) and a Record Set whose all present or<br>            past Record or Record Part members belong to that Category.
    isOrWasCategoryOfAllMembersOf, "isOrWasCategoryOfAllMembersOf",
    /// `is or was category of some members of`: Connects a Category (Type) and a Record Set whose some present<br>            or past Record or Record Part members belong to that Category.
    isOrWasCategoryOfSomeMembersOf, "isOrWasCategoryOfSomeMembersOf",
    /// `is or was component of`: Inverse of 'has or had component' object<br>            property.
    isOrWasComponentOf, "isOrWasComponentOf",
    /// `is or was constituent of`: Inverse of 'has or had constituent' object<br>            property.
    isOrWasConstituentOf, "isOrWasConstituentOf",
    /// `is or was contained by`: Inverse of 'contains or contained' object<br>            property.
    isOrWasContainedBy, "isOrWasContainedBy",
    /// `is or was content type of all members of`: Connects a Content Type and a Record Set whose all past or<br>            present Record or Record Part members have that Content Type.
    isOrWasContentTypeOfAllMembersOf, "isOrWasContentTypeOfAllMembersOf",
    /// `is or was content type of some members of`: Connects a Content Type and a Record Set whose some past or<br>            present Record or Record Part members have that Content Type.
    isOrWasContentTypeOfSomeMembersOf, "isOrWasContentTypeOfSomeMembersOf",
    /// `is or was controller of`: Connects an Agent to another Agent it controls or controlled via<br>            Activities, i.e. controls by function.
    isOrWasControllerOf, "isOrWasControllerOf",
    /// `is or was coordinates of`: Connects an instance of Coordinates to a Physical Location it<br>            locates or located on earth, according to some reference system.
    isOrWasCoordinatesOf, "isOrWasCoordinatesOf",
    /// `is or was corporate body type of`: Connects a Corporate Body Type to a Corporate Body that it<br>            categorizes or categorized.
    isOrWasCorporateBodyTypeOf, "isOrWasCorporateBodyTypeOf",
    /// `is or was demographic group of`: Connects a Demographic Group to a Person or Group which belongs<br>            or belonged to it.
    isOrWasDemographicGroupOf, "isOrWasDemographicGroupOf",
    /// `is or was described by`: Inverse of 'describes or described' object<br>            property.
    isOrWasDescribedBy, "isOrWasDescribedBy",
    /// `is or was documentary form type of all members of`: Connects a Documentary Form Type and a Record Set whose all past<br>            or present Record or Record Part members have that Documentary Form Type.
    isOrWasDocumentaryFormTypeOfAllMembersOf, "isOrWasDocumentaryFormTypeOfAllMembersOf",
    /// `is or was documentary form type of some members of`: Connects a Documentary Form Type and a Record Set whose some<br>            past or present Record or Record Part members have that Documentary Form<br>            Type.
    isOrWasDocumentaryFormTypeOfSomeMembersOf, "isOrWasDocumentaryFormTypeOfSomeMembersOf",
    /// `is or was enforced by`: Connects a Rule to an Agent that enforces or enforced the<br>            Rule.
    isOrWasEnforcedBy, "isOrWasEnforcedBy",
    /// `is or was expressed by`: Connects a Rule to a Record Resource that expresses or expressed<br>            the Rule.
    isOrWasExpressedBy, "isOrWasExpressedBy",
    /// `is or was holder of`: Connects an Agent to a Record Resource or Instantiation that the<br>            Agent holds or held.
    isOrWasHolderOf, "isOrWasHolderOf",
    /// `is or was holder of intellectual property rights of`: Connects an Agent to a Record Resource or Instantiation on which<br>            the Agent has or had some intellectual property rights.
    isOrWasHolderOfIntellectualPropertyRightsOf, "isOrWasHolderOfIntellectualPropertyRightsOf",
    /// `is or was identifier of`: Connects an Identifier to a Thing that it identified or<br>            identifies.
    isOrWasIdentifierOf, "isOrWasIdentifierOf",
    /// `is or was included in`: Inverse of 'includes or included' object<br>            property.
    isOrWasIncludedIn, "isOrWasIncludedIn",
    /// `is or was jurisdiction of`: Connects a Place to an Agent that has or had jurisdiction over<br>            the Place.
    isOrWasJurisdictionOf, "isOrWasJurisdictionOf",
    /// `is or was language of`: Connects a Language to an Agent, Record or Record Part that uses<br>            or used it.
    isOrWasLanguageOf, "isOrWasLanguageOf",
    /// `is or was language of all members of`: Connects a Language and a Record Set whose all present or past<br>            Record or Record Part members use that Language.
    isOrWasLanguageOfAllMembersOf, "isOrWasLanguageOfAllMembersOf",
    /// `is or was language of some members of`: Connects a Language and a Record Set whose some present or past<br>            Record or Record Part members use that Language.
    isOrWasLanguageOfSomeMembersOf, "isOrWasLanguageOfSomeMembersOf",
    /// `is or was leader of`: Connects a Person to the Group that Person leads or led in the<br>            past.
    isOrWasLeaderOf, "isOrWasLeaderOf",
    /// `is or was legal status of`: Connects a Legal Status to an Agent or Record Resource that it<br>            categorizes.
    isOrWasLegalStatusOf, "isOrWasLegalStatusOf",
    /// `is or was legal status of all members of`: Connects a Legal Status and a Record Set whose all past or<br>            present Record or Record Part members have that Legal Status.
    isOrWasLegalStatusOfAllMembersOf, "isOrWasLegalStatusOfAllMembersOf",
    /// `is or was legal status of some members of`: Connects a Legal Status and a Record Set whose some past or<br>            present Record or Record Part members have that Legal Status.
    isOrWasLegalStatusOfSomeMembersOf, "isOrWasLegalStatusOfSomeMembersOf",
    /// `is or was location of`: Connects a Place to a Thing that is or was located in the<br>            Place.
    isOrWasLocationOf, "isOrWasLocationOf",
    /// `is or was main subject of`: Inverse of 'has or had main subject' object<br>            property.
    isOrWasMainSubjectOf, "isOrWasMainSubjectOf",
    /// `is or was manager of`: Connects an Agent to a Record Resource or Instantiation that the<br>            Agent managed or manages.
    isOrWasManagerOf, "isOrWasManagerOf",
    /// `is or was member of`: Inverse of 'has or had member' object property.
    isOrWasMemberOf, "isOrWasMemberOf",
    /// `is or was name of`: Connects a Name to a Thing that it designated or<br>            designates.
    isOrWasNameOf, "isOrWasNameOf",
    /// `is or was occupation type of`: Connects an Occupation Type to a Person whose occupation is or<br>            was categorized by it.
    isOrWasOccupationTypeOf, "isOrWasOccupationTypeOf",
    /// `is or was occupied by`: Inverse of 'occupies or occupied' object<br>            property.
    isOrWasOccupiedBy, "isOrWasOccupiedBy",
    /// `is or was owner of`: Connects a Group, Person or Position to a Thing that this Agent<br>            owns or owned.
    isOrWasOwnerOf, "isOrWasOwnerOf",
    /// `is or was part of`: Inverse of 'has or had part' relation.
    isOrWasPartOf, "isOrWasPartOf",
    /// `is or was participant in`: Inverse of 'has or had participant' object<br>            property.
    isOrWasParticipantIn, "isOrWasParticipantIn",
    /// `is or was performed by`: Connects an Activity to an Agent that performed or performs the<br>            Activity.
    isOrWasPerformedBy, "isOrWasPerformedBy",
    /// `is or was physical location of`: Connects a Physical Location to a Place, when it is or was its<br>            location.
    isOrWasPhysicalLocationOf, "isOrWasPhysicalLocationOf",
    /// `is or was place name of`: Connects a Place Name to a Place that was or is designated by<br>            it.
    isOrWasPlaceNameOf, "isOrWasPlaceNameOf",
    /// `is or was place type of`: Connects a Place Type to a Place that is or was categorized by<br>            it.
    isOrWasPlaceTypeOf, "isOrWasPlaceTypeOf",
    /// `is or was record state of all members of`: Connects a Record State and a Record Set whose all past or<br>            present Record or Record Part members have that Record State.
    isOrWasRecordStateOfAllMembersOf, "isOrWasRecordStateOfAllMembersOf",
    /// `is or was record state of some members of`: Connects a Record State and a Record Set whose some past or<br>            present Record or Record Part members have that Record State.
    isOrWasRecordStateOfSomeMembersOf, "isOrWasRecordStateOfSomeMembersOf",
    /// `is or was regulated by`: Inverse of the 'regulates or regulated' object<br>            property.
    isOrWasRegulatedBy, "isOrWasRegulatedBy",
    /// `is or was responsible for enforcing`: Inverse of 'is or was enforced by' object<br>            property.
    isOrWasResponsibleForEnforcing, "isOrWasResponsibleForEnforcing",
    /// `is or was rule type of`: connects a Rule Type to a Rule that it categorized or<br>            categorizes.
    isOrWasRuleTypeOf, "isOrWasRuleTypeOf",
    /// `is or was subdivision of`: Inverse of 'has or had subdivision' object<br>            property.
    isOrWasSubdivisionOf, "isOrWasSubdivisionOf",
    /// `is or was subevent of`: Inverse of 'has or had subevent' object property.
    isOrWasSubeventOf, "isOrWasSubeventOf",
    /// `is or was subject of`: Inverse of 'has or had subject' object property.
    isOrWasSubjectOf, "isOrWasSubjectOf",
    /// `is or was subordinate to`: Inverse of 'has or had subordinate' object<br>            property.
    isOrWasSubordinateTo, "isOrWasSubordinateTo",
    /// `is or was title of`: Connects a Title to a Record Resource, Instantiation or Rule<br>            that it designated or designates.
    isOrWasTitleOf, "isOrWasTitleOf",
    /// `is or was under authority of`: Inverse of 'has or had authority over' object<br>            property.
    isOrWasUnderAuthorityOf, "isOrWasUnderAuthorityOf",
    /// `is original of `: Connects the original version of a Record to a copy or a later<br>            version.
    isOriginalOf, "isOriginalOf",
    /// `is place associated with `: Connects a Place to a Thing that Place is associated with the<br>            existence and lifecycle of.
    isPlaceAssociatedWith, "isPlaceAssociatedWith",
    /// `is production technique type of`: Connects a Production Technique Type to an Instantiation whose<br>            production technique is categorized by it.
    isProductionTechniqueTypeOf, "isProductionTechniqueTypeOf",
    /// `is provenance of `: inverse of 'has provenance' object property.
    isProvenanceOf, "isProvenanceOf",
    /// `isPublisherOf`: Connects an Agent to a Record Resource that it<br>            published.
    isPublisherOf, "isPublisherOf",
    /// `is receiver of`: Inverse of 'received by' object property.
    isReceiverOf, "isReceiverOf",
    /// `is record resource associated with record resource `: Connects two Record Resources. This object property is<br>            symmetric.
    isRecordResourceAssociatedWithRecordResource, "isRecordResourceAssociatedWithRecordResource",
    /// `is record set type of`: Connects a Record Set Type to a Record Set that it<br>            categorizes.
    isRecordSetTypeOf, "isRecordSetTypeOf",
    /// `is record state of`: Connects a Record State to a Record or Record Part whose state<br>            it categorizes.
    isRecordStateOf, "isRecordStateOf",
    /// `is related to `: The most generic object property. Connects an Thing to any other<br>            Thing This is a symmetric object property.
    isRelatedTo, "isRelatedTo",
    /// `is reply to`: Inverse of 'has reply' object property.
    isReplyTo, "isReplyTo",
    /// `is representation type of`: Connects a Representation Type to an Instantiation that it<br>            categorizes.
    isRepresentationTypeOf, "isRepresentationTypeOf",
    /// `is responsible for issuing `: Inverse of 'issued by' object property.
    isResponsibleForIssuing, "isResponsibleForIssuing",
    /// `is rule associated with `: Connects a Rule to a Thing that is associated with the existence<br>            and lifecycle of the Rule.
    isRuleAssociatedWith, "isRuleAssociatedWith",
    /// `is sender of `: Inverse of 'has sender' object property.
    isSenderOf, "isSenderOf",
    /// `is source of `: Connects a Record Resource or an Agent to a Record Resource or<br>            Relation, when the first is used as a source of information for identifying or<br>            describing the second one.
    isSourceOf, "isSourceOf",
    /// `is successor of`: Inverse of 'has successor' object property.
    isSuccessorOf, "isSuccessorOf",
    /// `is to use date of `: Connects a Date to an Appellation, when it is the date till<br>            which the Appellation was used.
    isToUseDateOf, "isToUseDateOf",
    /// `is unit of measurement of`: Inverse of 'has unit of measurement' object property
    isUnitOfMeasurementOf, "isUnitOfMeasurementOf",
    /// `issued by `: Connects a Rule to the Agent that issued or published the<br>            Rule.
    issuedBy, "issuedBy",
    /// `knowing of relation has source `: Connects a Knowing Of Relation to a 'knowing of' Person (a<br>            Person who has some knowledge of another one.)
    knowingOfRelationHasSource, "knowingOfRelationHasSource",
    /// `knowing of relation has target `: Connects a Knowing Of Relation to a 'known by' Person (a Person<br>            on which another one has some has some knowledge.)
    knowingOfRelationHasTarget, "knowingOfRelationHasTarget",
    /// `knowing relation connects `: Connects Knowing Relation to any known Person<br>            involved.
    knowingRelationConnects, "knowingRelationConnects",
    /// `known by `: Inverse of 'knows of' object property.
    knownBy, "knownBy",
    /// `knows `: Connects two Persons that directly know each other during their<br>            existence. This object property is symmetric.
    knows, "knows",
    /// `knows of `: Connects a Person to another Person they have some knowledge of<br>            through time or space.
    knowsOf, "knowsOf",
    /// `last modification date`: Date at which an entity was last updated.
    lastModificationDate, "lastModificationDate",
    /// `latitude`: Distance in degrees north or south of the<br>            equator.
    latitude, "latitude",
    /// `leadership relation has source `: Connects a Leadership Relation to a Person who is involved as a<br>            leader.
    leadershipRelationHasSource, "leadershipRelationHasSource",
    /// `leadership relation has target `: Connects a Leadership Relation to a lead Group.
    leadershipRelationHasTarget, "leadershipRelationHasTarget",
    /// `leadership with position `: Connects a Leadership Relation to the Position occupied by the<br>            leading Person.
    leadershipWithPosition, "leadershipWithPosition",
    /// `location`: A delimitation of the physical territory of a place. This<br>            datatype property is used to describe basic human-readable text such as an address, a<br>            cadastral reference, or less precise information found in a record.
    location, "location",
    /// `longitude`: Distance in degrees east or west of a prime<br>            meridian.
    longitude, "longitude",
    /// `management relation has source `: Connects a Management Relation to an Agent who is involved as a<br>            manager.
    managementRelationHasSource, "managementRelationHasSource",
    /// `management relation has target `: Connects a Management Relation to a Record Resource or<br>            Instantiation that is involved as a managed thing.
    managementRelationHasTarget, "managementRelationHasTarget",
    /// `mandate is source of mandate relation `: Connects a Mandate to a Mandate Relation.
    mandateIsSourceOfMandateRelation, "mandateIsSourceOfMandateRelation",
    /// `mandate relation has source `: Connects a Mandate Relation to a Mandate.
    mandateRelationHasSource, "mandateRelationHasSource",
    /// `mandate relation has target `: Connects a Mandate Relation to an Agent who is given the<br>            authority or competencies to act.
    mandateRelationHasTarget, "mandateRelationHasTarget",
    /// `measure`: The extent, quantity, amount, or degree of an entity, as<br>            determined by measurement or calculation.
    measure, "measure",
    /// `membership relation has source `: Connects a Membership Relation to the Group that has<br>            member(s).
    membershipRelationHasSource, "membershipRelationHasSource",
    /// `membership relation has target `: Connects a Membership Relation to a Person who is involved as a<br>            member.
    membershipRelationHasTarget, "membershipRelationHasTarget",
    /// `membership with position `: Connects a Membership Relation to the Position occupied by the<br>            member Person(s).
    membershipWithPosition, "membershipWithPosition",
    /// `migrated from`: Inverse of 'migrated into' object property.
    migratedFrom, "migratedFrom",
    /// `migrated into`: Connects an Instantiation to a version it has been migrated<br>            to.
    migratedInto, "migratedInto",
    /// `migration relation has source `: Connects a Migration Relation to the migrated<br>            Instantiation.
    migrationRelationHasSource, "migrationRelationHasSource",
    /// `migration relation has target `: Connects a Migration Relation to a resulting<br>            Instantiation.
    migrationRelationHasTarget, "migrationRelationHasTarget",
    /// `modification date`: Date of the modification of an entity.
    modificationDate, "modificationDate",
    /// `name`: A label, title or term designating the entity in order to make<br>            it distinguishable from other similar entities. For Record Resource or Instantiation,<br>            the Name is generally assigned by an Agent as most do not have a Name given when<br>            created.
    name, "name",
    /// `normalized date value`: Date representation based on a standard, preferably<br>            machine-readable.
    normalizedDateValue, "normalizedDateValue",
    /// `normalized value`: Value representation based on a standard, preferably<br>            machine-readable.
    normalizedValue, "normalizedValue",
    /// `occupies or occupied`: Connects a Person to a Position they occupy or<br>            occupied.
    occupiesOrOccupied, "occupiesOrOccupied",
    /// `overlaps or overlapped`: Connects two Places that geographically overlap or overlapped.<br>            This object property is symmetric.
    overlapsOrOverlapped, "overlapsOrOverlapped",
    /// `ownership relation has source `: Connects an Ownership Relation to a Person, Group or Position<br>            that is involved as an owner.
    ownershipRelationHasSource, "ownershipRelationHasSource",
    /// `owner ship relation has target `: Connects an Ownership Relation to a Thing that is<br>            owned.
    ownershipRelationHasTarget, "ownershipRelationHasTarget",
    /// `performance relation has source `: Connects a Performance Relation to a performed<br>            Activity.
    performanceRelationHasSource, "performanceRelationHasSource",
    /// `performance relation has target `: Connects a Performance Relation to a performing<br>            Agent.
    performanceRelationHasTarget, "performanceRelationHasTarget",
    /// `performs or performed`: Inverse of 'is or was performed by' object<br>            property.
    performsOrPerformed, "performsOrPerformed",
    /// `person has correspondence relation `: Connects a Person to a Correspondence Relation.
    personHasCorrespondenceRelation, "personHasCorrespondenceRelation",
    /// `person has family relation `: Connects a Person to a Family Relation.
    personHasFamilyRelation, "personHasFamilyRelation",
    /// `person has knowing relation `: Connects a Person to a Knowing Relation.
    personHasKnowingRelation, "personHasKnowingRelation",
    /// `person has sibling relation `: Connects a Person to a Sibling Relation.
    personHasSiblingRelation, "personHasSiblingRelation",
    /// `person has spouse relation `: Connects a Person to a Spouse Relation.
    personHasSpouseRelation, "personHasSpouseRelation",
    /// `person is source of child relation `: Connects a Person (as a parent) to a Child<br>            Relation.
    personIsSourceOfChildRelation, "personIsSourceOfChildRelation",
    /// `person is source of descendance relation `: Connects a Person (as an ancestor) to a Descendance<br>            Relation.
    personIsSourceOfDescendanceRelation, "personIsSourceOfDescendanceRelation",
    /// `person is source of knowing of relation `: Connects a Person (who has some knowledge of another one) to a<br>            Knowing Of Relation.
    personIsSourceOfKnowingOfRelation, "personIsSourceOfKnowingOfRelation",
    /// `person is source of leadership relation `: Connects a Person (as a leader) to a Leadership<br>            Relation.
    personIsSourceOfLeadershipRelation, "personIsSourceOfLeadershipRelation",
    /// `person is source of position holding relation `: Connects a Person (who occupies a Position) to a Position<br>            Holding Relation.
    personIsSourceOfPositionHoldingRelation, "personIsSourceOfPositionHoldingRelation",
    /// `person is source of teaching relation `: Connects a Person (as a teacher) to a Teaching<br>            Relation.
    personIsSourceOfTeachingRelation, "personIsSourceOfTeachingRelation",
    /// `person is target of child relation `: Connects a Person (as a child) to a Child<br>            Relation.
    personIsTargetOfChildRelation, "personIsTargetOfChildRelation",
    /// `person is target of descendance relation `: Connects a Person (as a descendant) to a Descendance<br>            Relation.
    personIsTargetOfDescendanceRelation, "personIsTargetOfDescendanceRelation",
    /// `person is target of knowing of relation `: Connects a Person (of which another Person has some knowledge)<br>            to a Knowing Of Relation.
    personIsTargetOfKnowingOfRelation, "personIsTargetOfKnowingOfRelation",
    /// `person is target of membership relation `: Connects a Person (as a member of a Group) to a Membership<br>            Relation.
    personIsTargetOfMembershipRelation, "personIsTargetOfMembershipRelation",
    /// `person is target of teaching relation `: Connects a Person (as a student) to a Teaching<br>            Relation.
    personIsTargetOfTeachingRelation, "personIsTargetOfTeachingRelation",
    /// `physical characteristics`: Information about the physical features of the Instantiation.<br>            Includes information about the physical nature and condition such as conservation<br>            status.
    physicalCharacteristics, "physicalCharacteristics",
    /// `physical or logical extent`: Countable characteristics of the content of an entity expressed<br>            as a quantity.
    physicalOrLogicalExtent, "physicalOrLogicalExtent",
    /// `place is source of place relation `: Connects a Place (as associated to a Thing) to a Place<br>            Relation.
    placeIsSourceOfPlaceRelation, "placeIsSourceOfPlaceRelation",
    /// `place relation has source `: Connects a Place Relation to the Place concerned.
    placeRelationHasSource, "placeRelationHasSource",
    /// `place relation has target `: Connects a Place Relation to a Thing that is associated to the<br>            Place.
    placeRelationHasTarget, "placeRelationHasTarget",
    /// `position holding relation has source `: Connects a Position Holding Relation to a Person (who occupies a<br>            Position).
    positionHoldingRelationHasSource, "positionHoldingRelationHasSource",
    /// `position holding relation has target `: Connects a Position Holding Relation to a Position (that is<br>            occupied).
    positionHoldingRelationHasTarget, "positionHoldingRelationHasTarget",
    /// `position is context of leadership relation `: Connects a Position to a Leadership Relation (the leading Person<br>            occupies that Position).
    positionIsContextOfLeadershipRelation, "positionIsContextOfLeadershipRelation",
    /// `position is context of membership relation `: Connects a Position to a Membership Relation (the member Person<br>            occupies that Position).
    positionIsContextOfMembershipRelation, "positionIsContextOfMembershipRelation",
    /// `position is source of position to group relation `: Connects a Position (that exists within a Group) to a Position<br>            to Group Relation.
    positionIsSourceOfPositionToGroupRelation, "positionIsSourceOfPositionToGroupRelation",
    /// `position is target of position holding relation `: Connects a Position (that is occupied by a Person) to a Position<br>            Holding Relation.
    positionIsTargetOfPositionHoldingRelation, "positionIsTargetOfPositionHoldingRelation",
    /// `position to group relation has source `: Connects a Position to Group Relation to a Position (that exists<br>            in a Group).
    positionToGroupRelationHasSource, "positionToGroupRelationHasSource",
    /// `position to group relation has target `: Connects a Position to Group Relation to a Group (in which a<br>            Position exists).
    positionToGroupRelationHasTarget, "positionToGroupRelationHasTarget",
    /// `precedes in time`: Connects a Thing to a Thing that follows it in chronological<br>            order.
    precedesInTime, "precedesInTime",
    /// `precedes or preceded`: Connects a Thing to a Thing that follows or followed it in some<br>            sequence.
    precedesOrPreceded, "precedesOrPreceded",
    /// `production technique`: Method used in the representation of information on the<br>            Instantiation.
    productionTechnique, "productionTechnique",
    /// `provenance relation has source `: Connects a Provenance Relation to a Record Resource or<br>            Instantiation.
    provenanceRelationHasSource, "provenanceRelationHasSource",
    /// `provenance relation has target `: Connects a Provenance Relation to an Agent or<br>            Activity.
    provenanceRelationHasTarget, "provenanceRelationHasTarget",
    /// `proxy for `: Connects a Proxy to the Record Resource it stands for in the<br>            specific context of a Record Set.
    proxyFor, "proxyFor",
    /// `proxy in `: Connects a Proxy to the Record Set in which it stands for<br>            (represents) another Record Resource.
    proxyIn, "proxyIn",
    /// `publication date`: Date of the publication of a Record Resource.
    publicationDate, "publicationDate",
    /// `quality of representation`: Conditions of an Instantiation that impact the legibility or<br>            completeness of Record Resource, and thus the viability of its use. Conditions may be<br>            associated with deficiencies in the processes of Record (re)creation or capture, or the<br>            deterioration of the Instantiation (e.g. its carrier) causing loss of information of the<br>            record over time
    qualityOfRepresentation, "qualityOfRepresentation",
    /// `quantity`: Machine-readable quantity.
    quantity, "quantity",
    /// `record is source of authorship relation`: Connects a Record and an Authorship Relation.
    recordIsSourceOfAuthorshipRelation, "recordIsSourceOfAuthorshipRelation",
    /// `Record Resource extent`: The quantity of information content as human experienced<br>            represented in the Record Resource. The method and precision of expressing the quantity<br>            of information represented in a Record Resource will vary by the kind of Record Resource<br>            being described as well as by processing economy constraints. For record sets, quantity<br>            may be expressed as number of records, or, for analogue records in particular, by the<br>            physical storage dimensions of the Record members. For individual records or record<br>            parts, quantity may be expressed in more precise terms. Use if you don't use<br>            RecordResourceExtent class and its properties for handling such<br>            information.
    recordResourceExtent, "recordResourceExtent",
    /// `record resource genetic relation connects `: Connects a Record Resource Genetic Relation to one of the<br>            associated Record Resources.
    recordResourceGeneticRelationConnects, "recordResourceGeneticRelationConnects",
    /// `record resource holding relation has source `: Connects a Record Resource Holding Relation to an Agent (as the<br>            holder of a Record Resource or Instantiation).
    recordResourceHoldingRelationHasSource, "recordResourceHoldingRelationHasSource",
    /// `record resource holding relation has target `: Connects a Record Resource Holding Relation to a Record Resource<br>            or Instantiation (that is held by an Agent).
    recordResourceHoldingRelationHasTarget, "recordResourceHoldingRelationHasTarget",
    /// `record resource is connected to record resource genetic relation<br>        `: Connects a Record Resource to a Record Resource Genetic<br>            Relation.
    recordResourceIsConnectedToRecordResourceGeneticRelation, "recordResourceIsConnectedToRecordResourceGeneticRelation",
    /// `record resource is connected to record resource relation<br>        `: Connects a Record Resource to a Record Resource<br>            Relation.
    recordResourceIsConnectedToRecordResourceRelation, "recordResourceIsConnectedToRecordResourceRelation",
    /// `record resource is source of record resource to instantiation<br>            relation `: Connects a Record Resource (that was instantiated) to a Record<br>            Resource To Instantiation Relation
    recordResourceIsSourceOfRecordResourceToInstantiationRelation, "recordResourceIsSourceOfRecordResourceToInstantiationRelation",
    /// `record resource or instantiation is source of accumulation<br>            relation `: Connects a Record Resource or Instantiation (that is<br>            accumulated) to an Accumulation Relation.
    recordResourceOrInstantiationIsSourceOfAccumulationRelation, "recordResourceOrInstantiationIsSourceOfAccumulationRelation",
    /// `record resource or instantiation is source of activity<br>            documentation relation `: Connects a Record Resource or Instantiation (that documents an<br>            Activity) to an Activity Documentation Relation.
    recordResourceOrInstantiationIsSourceOfActivityDocumentationRelation, "recordResourceOrInstantiationIsSourceOfActivityDocumentationRelation",
    /// `record resource or instantiation is source of agent origination<br>            relation `: Connects a Record Resource or Instantiation (that is created,<br>            sent or accumulated) to an Agent Origination Relation.
    recordResourceOrInstantiationIsSourceOfAgentOriginationRelation, "recordResourceOrInstantiationIsSourceOfAgentOriginationRelation",
    /// `record resource or instantiation is source of creation relation<br>        `: Connects a Record Resource or Instantiation (that is created) to<br>            a Creation Relation.
    recordResourceOrInstantiationIsSourceOfCreationRelation, "recordResourceOrInstantiationIsSourceOfCreationRelation",
    /// `record resource or instantiation is source of provenance relation<br>        `: Connects a Record Resource or Instantiation (that is created or<br>            accumulated by an Agent, or documents an Activity) to a Provenance<br>            Relation.
    recordResourceOrInstantiationIsSourceOfProvenanceRelation, "recordResourceOrInstantiationIsSourceOfProvenanceRelation",
    /// `record resource or instantiation is target of intellectual<br>            property rights relation `: Connects a Record Resource or Instantiation (on which some<br>            intellectual property rights are held) to an Intellectual Property Rights<br>            Relation.
    recordResourceOrInstantiationIsTargetOfIntellectualPropertyRightsRelation, "recordResourceOrInstantiationIsTargetOfIntellectualPropertyRightsRelation",
    /// `record resource or instantiation is target of management relation<br>        `: Connects a Record Resource or Instantiation (that is managed by<br>            an Agent) to a Management Relation.
    recordResourceOrInstantiationIsTargetOfManagementRelation, "recordResourceOrInstantiationIsTargetOfManagementRelation",
    /// `record resource or instantiation is target of record resource<br>            holding relation `: Connects a Record Resource or Instantiation (that is held by an<br>            Agent) to a Record Resource Holding Relation.
    recordResourceOrInstantiationIsTargetOfRecordResourceHoldingRelation, "recordResourceOrInstantiationIsTargetOfRecordResourceHoldingRelation",
    /// `record resource relation connects `: Connects a Record Resource relation to one of the related Record<br>            Resources.
    recordResourceRelationConnects, "recordResourceRelationConnects",
    /// `Record Resource structure`: Information about the intellectual arrangement and composition<br>            of a Record Resource. For Record and Record Part, it encompasses information about the<br>            intellectual composition of the record, the presence of record parts and their<br>            functions. For Record Set, it encompasses information about the methodology or criteria<br>            used for arranging the Record Set members or Record members within the containing Record<br>            Set
    recordResourceStructure, "recordResourceStructure",
    /// `record resource to instantiation relation has source `: Connects a Record Resource To Instantiation Relation to the<br>            Record Resource (that was instantiated). 
    recordResourceToInstantiationRelationHasSource, "recordResourceToInstantiationRelationHasSource",
    /// `record resource to instantiation relation has target `: Connects a Record Resource To Instantiation Relation to an<br>            Instantiation of the involved Record Resource. 
    recordResourceToInstantiationRelationHasTarget, "recordResourceToInstantiationRelationHasTarget",
    /// `reference system`: Framework or standard used to represent an<br>            information.
    referenceSystem, "referenceSystem",
    /// `regulates or regulated`: Connects a Rule to a Thing that it regulates or<br>            regulated.
    regulatesOrRegulated, "regulatesOrRegulated",
    /// `relation connects `: Connects an n-ary Relation to any of the Things<br>            involved.
    relationConnects, "relationConnects",
    /// `relation has context `: Connects an n-ary Relation to a Thing that is a secondary,<br>            contextual entity during the existence of the Relation.
    relationHasContext, "relationHasContext",
    /// `relation has source `: Connects an n-ary Relation to a Thing that is its<br>            source.
    relationHasSource, "relationHasSource",
    /// `relation has target `: Connects an n-ary Relation to a Thing that is its<br>            target.
    relationHasTarget, "relationHasTarget",
    /// `Relation state`: Used to qualify the state of a Relation (e. g. present, past,<br>            ongoing, unknown).
    relationState, "relationState",
    /// `results or resulted from`: Inverse of 'results or resulted in' object<br>            property.
    resultsOrResultedFrom, "resultsOrResultedFrom",
    /// `results or resulted in`: Connects an Event to a Thing that results or resulted from the<br>            Event.
    resultsOrResultedIn, "resultsOrResultedIn",
    /// `role is context of creation relation `: Connects a Role Type to a Creation Relation (this Role Type<br>            being the specific role played by the creating Person in the context of this<br>            Relation).
    roleIsContextOfCreationRelation, "roleIsContextOfCreationRelation",
    /// `rule followed`: The rule or conditions that govern the existence or lifecycle of<br>            a Thing.
    ruleFollowed, "ruleFollowed",
    /// `rule is source of rule relation `: Connects a Rule to a Rule Relation.
    ruleIsSourceOfRuleRelation, "ruleIsSourceOfRuleRelation",
    /// `rule relation has source `: Connects a Rule Relation to a Rule.
    ruleRelationHasSource, "ruleRelationHasSource",
    /// `rule relation has target `: Connects a Rule Relation to a Thing (that is associated to a<br>            Rule).
    ruleRelationHasTarget, "ruleRelationHasTarget",
    /// `scope and content`: Summary of the scope (such as time periods, geography) and<br>            content (such as subject matter, administrative processes) of the Record Resource. It<br>            should highlight the information conveyed in the Record Resource, why it was created,<br>            received, and/or maintained, and the Agents connected to it. Scope and Content provides<br>            a more complete summary of the informational content of the Record Resource. It may<br>            include description of relations with agents, activities, dates and places, or with<br>            other record resources. It is not to be confused with the History attribute which<br>            focuses on the origination and subsequence changes to a Record Resource.
    scopeAndContent, "scopeAndContent",
    /// `sequential relation has source `: Connects a Sequential Relation to a Thing that precedes other<br>            Thing(s) in the sequence.
    sequentialRelationHasSource, "sequentialRelationHasSource",
    /// `sequential relation has target `: Connects a Sequential Relation to a Thing that follows other<br>            Thing(s) in the sequence.
    sequentialRelationHasTarget, "sequentialRelationHasTarget",
    /// `sibling relation connects `: Connects a Sibling Relation to one of the siblings<br>            involved.
    siblingRelationConnects, "siblingRelationConnects",
    /// `source`: Information about a source used to identify or describe an<br>            entity.
    source, "source",
    /// `spouse relation connects `: Connects a Sibling Relation to one of the spouses<br>            involved.
    spouseRelationConnects, "spouseRelationConnects",
    /// `structure`: Information about the intellectual arrangement and composition<br>            of a Record Resource or the physical arrangement and composition of an Instantiation.<br>            For Record and Record Part, it encompasses information about the intellectual<br>            composition of the record, the presence of record parts and their functions. For Record<br>            Set, it encompasses information about the methodology or criteria used for arranging the<br>            Record Set members or Record members within the containing Record Set. For<br>            Instantiation, it may comprise information about the composition of the physical<br>            elements of the instantiation
    structure, "structure",
    /// `teaching relation has source `: Connects a Teaching Relation to a Person (who is a<br>            teacher).
    teachingRelationHasSource, "teachingRelationHasSource",
    /// `teaching relation has target `: Connects a Teaching Relation to a Person (who is a<br>            student).
    teachingRelationHasTarget, "teachingRelationHasTarget",
    /// `technical characteristics`: Describes any relevant physical or software feature of any<br>            device involved in the creation or management of a Record Resource.
    technicalCharacteristics, "technicalCharacteristics",
    /// `temporal relation has source `: Connects a Temporal Relation to a Thing that precedes other<br>            Thing(s) in time.
    temporalRelationHasSource, "temporalRelationHasSource",
    /// `temporal relation has target `: Connects a Temporal Relation to a Thing that follows other<br>            Thing(s) in time.
    temporalRelationHasTarget, "temporalRelationHasTarget",
    /// `textual value`: A textual expression of an Appellation or Date.
    textualValue, "textualValue",
    /// `thing is connected to relation `: Connects a Thing to a n-ary Relation.
    thingIsConnectedToRelation, "thingIsConnectedToRelation",
    /// `thing is context of relation `: Connects a Thing (that is a secondary, contextual entity during<br>            the existence of the Relation) to a n-ary Relation.
    thingIsContextOfRelation, "thingIsContextOfRelation",
    /// `thing is source of relation `: Connects a Thing (that is the source of a Relation) to a<br>            Relation.
    thingIsSourceOfRelation, "thingIsSourceOfRelation",
    /// `thing is source of sequential relation `: Connects a Thing to a Sequential Relation, when this Thing<br>            precedes other Thing(s) in the sequence.
    thingIsSourceOfSequentialRelation, "thingIsSourceOfSequentialRelation",
    /// `thing is source of temporal relation `: Connects a Thing to a Temporal Relation, when this Thing<br>            precedes other Thing(s) in time.
    thingIsSourceOfTemporalRelation, "thingIsSourceOfTemporalRelation",
    /// `thing is source of whole part relation `: Connects a Thing to a Whole Part Relation, when this Thing has<br>            Part other Thing(s).
    thingIsSourceOfWholePartRelation, "thingIsSourceOfWholePartRelation",
    /// `thing is target of appellation relation `: Connects a Thing (that is designated by an Appellation) to an<br>            Appellation Relation.
    thingIsTargetOfAppellationRelation, "thingIsTargetOfAppellationRelation",
    /// `thing is target of authority relation `: Connects a Thing (that is under authority of an Agent) to an<br>            Authority Relation.
    thingIsTargetOfAuthorityRelation, "thingIsTargetOfAuthorityRelation",
    /// `thing is target of event relation `: Connects a Thing (that is associated with an Event) to an Event<br>            Relation.
    thingIsTargetOfEventRelation, "thingIsTargetOfEventRelation",
    /// `thing is target of ownership relation `: Connects a Thing (that is owned by a Group, a Person or a<br>            Position) to an Ownership Relation.
    thingIsTargetOfOwnershipRelation, "thingIsTargetOfOwnershipRelation",
    /// `thing is target of place relation `: Connects a Thing (that is associated with a Place) to a Place<br>            Relation.
    thingIsTargetOfPlaceRelation, "thingIsTargetOfPlaceRelation",
    /// `thing is target of relation `: Connects a Thing to a n-ary Relation.
    thingIsTargetOfRelation, "thingIsTargetOfRelation",
    /// `thing is target of rule relation `: Connects a Thing (that is associated with a Rule) to a Rule<br>            Relation.
    thingIsTargetOfRuleRelation, "thingIsTargetOfRuleRelation",
    /// `thing is target of sequential relation `: Connects a Thing (that follows other Thing(s) in a sequence) to<br>            a Sequential Relation.
    thingIsTargetOfSequentialRelation, "thingIsTargetOfSequentialRelation",
    /// `thing is target of temporal relation `: Connects a Thing (that follows other Thing(s) in time) to a<br>            Temporal Relation.
    thingIsTargetOfTemporalRelation, "thingIsTargetOfTemporalRelation",
    /// `thing is target of type relation `: Connects a Thing (that is categorized by a Type) to a Type<br>            Relation.
    thingIsTargetOfTypeRelation, "thingIsTargetOfTypeRelation",
    /// `thing is target of whole part relation `: Connects a Thing to a Whole Part Relation, when this Thing is<br>            Part of another Thing.
    thingIsTargetOfWholePartRelation, "thingIsTargetOfWholePartRelation",
    /// `title`: An identifying name of a Record Resource, Instantiation or<br>            Rule.
    title, "title",
    /// `type`: A term used to characterize an entity.
    type_, "type",
    /// `type is source of type relation `: Connects a Type (a category) to a Type Relation.
    typeIsSourceOfTypeRelation, "typeIsSourceOfTypeRelation",
    /// `type relation has source `: Connects a Type Relation to the Type (that categorizes the<br>            involved Thing(s)).
    typeRelationHasSource, "typeRelationHasSource",
    /// `type relation has target `: Connects a Type Relation to a Thing (that is categorized by the<br>            involved Type).
    typeRelationHasTarget, "typeRelationHasTarget",
    /// `unit of measurement`: A definite magnitude of a quantity, defined and adopted by convention or by<br>            law, that is used as a standard for measurement of the same kind of quantity. Can be<br>            spacial units (cm, m), weight (g, kg), time (s, h), storage (MB, TB) or more informal<br>            units used in the archival context like number of boxes, pages or words.
    unitOfMeasurement, "unitOfMeasurement",
    /// `used from date`: Date at which an Appellation was first used.
    usedFromDate, "usedFromDate",
    /// `used to date`: Date until an Appellation was used.
    usedToDate, "usedToDate",
    /// `was last updated at date `: Connects a Thing to the Date when it was last<br>            modified.
    wasLastUpdatedAtDate, "wasLastUpdatedAtDate",
    /// `was used from date `: Connects an Appellation to the Date from which it was<br>            used.
    wasUsedFromDate, "wasUsedFromDate",
    /// `was used to date `: Connects an Appellation to the Date till when it was<br>            used.
    wasUsedToDate, "wasUsedToDate",
    /// `whole part relation has source `: Connects a Whole Part Relation to the Thing that has some<br>            parts.
    wholePartRelationHasSource, "wholePartRelationHasSource",
    /// `whole part relation has target `: Connects a Whole Part Relation to a Thing that is a<br>            part.
    wholePartRelationHasTarget, "wholePartRelationHasTarget",
    /// `width`: Horizontal dimension of an entity.
    width, "width",
    /// `work relation connects `: Connects a Work Relation to an Agent.
    workRelationConnects, "workRelationConnects"
);
