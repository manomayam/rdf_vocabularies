// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Semantic Sensor Network Ontology` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Semantic Sensor Network Ontology|
//! |**Prefix**|ssn|
//! |**Namespace base IRI**|[http://www.w3.org/ns/ssn/](http://www.w3.org/ns/ssn/)|
//! |**Description**|This ontology describes sensors, actuators and observations, and related concepts. It does not describe domain concepts, time, locations, etc. these are intended to be included from other ontologies via OWL imports.|
//! |**Is defined by**|[http://www.w3.org/ns/ssn/](http://www.w3.org/ns/ssn/)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/ssn/",;
    /// `Deployment`: Describes the Deployment of one or more Systems for a particular purpose. Deployment may be done on a Platform.
    Deployment, "Deployment",
    /// `Input`: Any information that is provided to a Procedure for its use.
    Input, "Input",
    /// `Output`: Any information that is reported from a Procedure.
    Output, "Output",
    /// `Property`: A quality of an entity. An aspect of an entity that is intrinsic to and cannot exist without the entity.
    Property, "Property",
    /// `Stimulus`: An event in the real world that 'triggers' the Sensor. The properties associated to the Stimulus may be different to the eventual observed ObservableProperty. It is the event, not the object, that triggers the Sensor.
    Stimulus, "Stimulus",
    /// `System`: System is a unit of abstraction for pieces of infrastructure that implement Procedures. A System may have components, its subsystems, which are other systems.
    System, "System",
    /// `deployed on platform`: Relation between a Deployment and the Platform on which the Systems are deployed.
    deployedOnPlatform, "deployedOnPlatform",
    /// `deployed system`: Relation between a Deployment and a deployed System.
    deployedSystem, "deployedSystem",
    /// `detects`: A relation from a Sensor to the Stimulus that the Sensor detects. The Stimulus itself will be serving as a proxy for some ObservableProperty.
    detects, "detects",
    /// `for property`: A relation between some aspect of an entity and a Property.
    forProperty, "forProperty",
    /// `has deployment`: Relation between a System and a Deployment, recording that the System is deployed in that Deployment.
    hasDeployment, "hasDeployment",
    /// `has input`: Relation between a Procedure and an Input to it.
    hasInput, "hasInput",
    /// `has output`: Relation between a Procedure and an Output of it.
    hasOutput, "hasOutput",
    /// `has property`: Relation between an entity and a Property of that entity.
    hasProperty, "hasProperty",
    /// `has subsystem`: Relation between a System and its component parts.
    hasSubSystem, "hasSubSystem",
    /// `implemented by`: Relation between a Procedure (an algorithm, procedure or method) and an entity that implements that Procedure in some executable way.
    implementedBy, "implementedBy",
    /// `implements`: Relation between an entity that implements a Procedure in some executable way and the Procedure (an algorithm, procedure or method).
    implements, "implements",
    /// `in deployment`: Relation between a Platform and a Deployment, meaning that the deployedSystems of the Deployment are hosted on the Platform.
    inDeployment, "inDeployment",
    /// `is property of`: Relation between a Property and the entity it belongs to.
    isPropertyOf, "isPropertyOf",
    /// `is proxy for`: A relation from a Stimulus to the Property that the Stimulus is serving as a proxy for.
    isProxyFor, "isProxyFor",
    /// `was originated by`: Relation between an Observation and the Stimulus that originated it.
    wasOriginatedBy, "wasOriginatedBy"
);
