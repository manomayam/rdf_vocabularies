// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, doc(cfg(feature = "ns-earl")))]
//! This module provides terms for `Evaluation and Report Language (EARL) 1.0 Schema` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Evaluation and Report Language (EARL) 1.0 Schema|
//! |**Prefix**|earl|
//! |**Namespace base IRI**|<https://www.w3.org/ns/earl#>|
//! |**Description**|Formal schema of the Evaluation and Report Language (EARL) 1.0|
//! |**Is defined by**|<https://www.w3.org/ns/earl#>|
//!

use crate::namespace;

namespace!(
    "https://www.w3.org/ns/earl#",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Assertion`: a statement that embodies the results of a test
    Assertion, "Assertion",
    /// `Assertor`: an entity such as a person, a software tool, an organization, or any other grouping that carries out a test collectively
    Assertor, "Assertor",
    /// `Undetermined`: the class of outcomes to denote an undetermined outcome
    CannotTell, "CannotTell",
    /// `Fail`: the class of outcomes to denote failing a test
    Fail, "Fail",
    /// `Not applicable`: the class of outcomes to denote the test is not applicable
    NotApplicable, "NotApplicable",
    /// `Not tested`: the class of outcomes to denote the test has not been carried out
    NotTested, "NotTested",
    /// `Outcome Value`: a discrete value that describes a resulting condition from carrying out the test
    OutcomeValue, "OutcomeValue",
    /// `Pass`: the class of outcomes to denote passing a test
    Pass, "Pass",
    /// `Software`: any piece of software such as an authoring tool, browser, or evaluation tool
    Software, "Software",
    /// `Test Case`: an atomic test, usually one that is a partial test for a requirement
    TestCase, "TestCase",
    /// `Test Criterion`: a testable statement, usually one that can be passed or failed
    TestCriterion, "TestCriterion",
    /// `Test Mode`: describes how a test was carried out
    TestMode, "TestMode",
    /// `Test Requirement`: a higher-level requirement that is tested by executing one or more sub-tests
    TestRequirement, "TestRequirement",
    /// `Test Result`: the actual result of performing the test
    TestResult, "TestResult",
    /// `Test Subject`: the class of things that have been tested against some test criterion
    TestSubject, "TestSubject",
    /// `Asserted By`: assertor of an assertion
    assertedBy, "assertedBy",
    /// `automatic`: 
    automatic, "automatic",
    /// `cantTell`: 
    cantTell, "cantTell",
    /// `failed`: 
    failed, "failed",
    /// `inapplicable`: 
    inapplicable, "inapplicable",
    /// `Info`: additional warnings or error messages in a human-readable form
    info, "info",
    /// `Main Assertor`: assertor that is primarily responsible for performing the test
    mainAssertor, "mainAssertor",
    /// `manual`: 
    manual, "manual",
    /// `Mode`: mode in which the test was performed
    mode, "mode",
    /// `Outcome`: outcome of performing the test
    outcome, "outcome",
    /// `passed`: 
    passed, "passed",
    /// `Pointer`: location within a test subject that are most relevant to a test result
    pointer, "pointer",
    /// `Result`: result of an assertion
    result, "result",
    /// `semiAuto`: 
    semiAuto, "semiAuto",
    /// `Subject`: test subject of an assertion
    subject, "subject",
    /// `Test`: test criterion of an assertion
    test, "test",
    /// `undisclosed`: 
    undisclosed, "undisclosed",
    /// `unknownMode`: 
    unknownMode, "unknownMode",
    /// `untested`: 
    untested, "untested"
);
