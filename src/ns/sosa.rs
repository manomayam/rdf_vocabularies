// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Sensor, Observation, Sample, and Actuator (SOSA) Ontology` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Sensor, Observation, Sample, and Actuator (SOSA) Ontology|
//! |**Prefix**|sosa|
//! |**Namespace base IRI**|[<http://www.w3.org/ns/sosa/>](<http://www.w3.org/ns/sosa/>)|
//! |**Description**|This ontology is based on the SSN Ontology by the W3C Semantic Sensor Networks Incubator Group (SSN-XG), together with considerations from the W3C/OGC Spatial Data on the Web Working Group.|
//! |**Is defined by**|[<http://www.w3.org/ns/sosa/>](<http://www.w3.org/ns/sosa/>)|
//!

use crate::namespace;

namespace!(
    "<http://www.w3.org/ns/sosa/>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Actuatable Property`: An actuatable quality (property, characteristic) of a FeatureOfInterest.
    ActuatableProperty, "ActuatableProperty",
    /// `Actuation`: An Actuation carries out an (Actuation) Procedure to change the state of the world using an Actuator.
    Actuation, "Actuation",
    /// `Actuator`: A device that is used by, or implements, an (Actuation) Procedure that changes the state of the world.
    Actuator, "Actuator",
    /// `Feature Of Interest`: The thing whose property is being estimated or calculated in the course of an Observation to arrive at a Result or whose property is being manipulated by an Actuator, or which is being sampled or transformed in an act of Sampling.
    FeatureOfInterest, "FeatureOfInterest",
    /// `Observable Property`: An observable quality (property, characteristic) of a FeatureOfInterest.
    ObservableProperty, "ObservableProperty",
    /// `Observation`: Act of carrying out an (Observation) Procedure to estimate or calculate a value of a property of a FeatureOfInterest. Links to a Sensor to describe what made the Observation and how; links to an ObservableProperty to describe what the result is an estimate of, and to a FeatureOfInterest to detail what that property was associated with.
    Observation, "Observation",
    /// `Platform`: A Platform is an entity that hosts other entities, particularly Sensors, Actuators, Samplers, and other Platforms.
    Platform, "Platform",
    /// `Procedure`: A workflow, protocol, plan, algorithm, or computational method specifying how to make an Observation, create a Sample, or make a change to the state of the world (via an Actuator). A Procedure is re-usable, and might be involved in many Observations, Samplings, or Actuations. It explains the steps to be carried out to arrive at reproducible results.
    Procedure, "Procedure",
    /// `Result`: The Result of an Observation, Actuation, or act of Sampling. To store an observation's simple result value one can use the hasSimpleResult property.
    Result, "Result",
    /// `Sample`: A Sample is the result from an act of Sampling.
    Sample, "Sample",
    /// `Sampler`: A device that is used by, or implements, a Sampling Procedure to create or transform one or more samples.
    Sampler, "Sampler",
    /// `Sampling`: An act of Sampling carries out a sampling Procedure to create or transform one or more samples.
    Sampling, "Sampling",
    /// `Sensor`: Device, agent (including humans), or software (simulation) involved in, or implementing, a Procedure. Sensors respond to a stimulus, e.g., a change in the environment, or input data composed from the results of prior Observations, and generate a Result. Sensors can be hosted by Platforms.
    Sensor, "Sensor",
    /// `acts on property`: Relation between an Actuation and the property of a FeatureOfInterest it is acting upon.
    actsOnProperty, "actsOnProperty",
    /// `has feature of interest`: A relation between an Observation and the entity whose quality was observed, or between an Actuation and the entity whose property was modified, or between an act of Sampling and the entity that was sampled.
    hasFeatureOfInterest, "hasFeatureOfInterest",
    /// `has result`: Relation linking an Observation or Actuation or act of Sampling and a Result or Sample.
    hasResult, "hasResult",
    /// `has sample`: Relation between a FeatureOfInterest and the Sample used to represent it.
    hasSample, "hasSample",
    /// `has simple result`: The simple value of an Observation or Actuation or act of Sampling.
    hasSimpleResult, "hasSimpleResult",
    /// `hosts`: Relation between a Platform and a Sensor, Actuator, Sampler, or Platform, hosted or mounted on it.
    hosts, "hosts",
    /// `is acted on by`: Relation between an ActuatableProperty of a FeatureOfInterest and an Actuation changing its state.
    isActedOnBy, "isActedOnBy",
    /// `is feature of interest of`: A relation between a FeatureOfInterest and an Observation about it, an Actuation acting on it, or an act of Sampling that sampled it.
    isFeatureOfInterestOf, "isFeatureOfInterestOf",
    /// `is hosted by`: Relation between a Sensor, Actuator, Sampler, or Platform, and the Platform that it is mounted on or hosted by.
    isHostedBy, "isHostedBy",
    /// `is observed by`: Relation between an ObservableProperty and the Sensor able to observe it.
    isObservedBy, "isObservedBy",
    /// `is result of`: Relation linking a Result to the Observation or Actuation or act of Sampling that created or caused it.
    isResultOf, "isResultOf",
    /// `is sample of`: Relation from a Sample to the FeatureOfInterest that it is intended to be representative of.
    isSampleOf, "isSampleOf",
    /// `made actuation`: Relation between an Actuator and the Actuation it has made.
    madeActuation, "madeActuation",
    /// `made by actuator`: Relation linking an Actuation to the Actuator that made that Actuation.
    madeByActuator, "madeByActuator",
    /// `made by sampler`: Relation linking an act of Sampling to the Sampler (sampling device or entity) that made it.
    madeBySampler, "madeBySampler",
    /// `made by sensor`: Relation between an Observation and the Sensor which made the Observation.
    madeBySensor, "madeBySensor",
    /// `made observation`: Relation between a Sensor and an Observation made by the Sensor.
    madeObservation, "madeObservation",
    /// `made sampling`: Relation between a Sampler (sampling device or entity) and the Sampling act it performed.
    madeSampling, "madeSampling",
    /// `observed property`: Relation linking an Observation to the property that was observed. The ObservableProperty should be a property of the FeatureOfInterest (linked by hasFeatureOfInterest) of this Observation.
    observedProperty, "observedProperty",
    /// `observes`: Relation between a Sensor and an ObservableProperty that it is capable of sensing.
    observes, "observes",
    /// `phenomenon time`: The time that the Result of an Observation, Actuation or Sampling applies to the FeatureOfInterest. Not necessarily the same as the resultTime. May be an Interval or an Instant, or some other compound TemporalEntity.
    phenomenonTime, "phenomenonTime",
    /// `result time`: The result time is the instant of time when the Observation, Actuation or Sampling activity was completed.
    resultTime, "resultTime",
    /// `used procedure`: A relation to link to a re-usable Procedure used in making an Observation, an Actuation, or a Sample, typically through a Sensor, Actuator or Sampler.
    usedProcedure, "usedProcedure"
);
