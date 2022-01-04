// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `test` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**||
//! |**Prefix**|test|
//! |**Namespace base IRI**|[<http://www.w3.org/2006/03/test-description#>](<http://www.w3.org/2006/03/test-description#>)|
//! |**Description**||
//! |**Is defined by**|[<http://www.w3.org/2006/03/test-description#>](<http://www.w3.org/2006/03/test-description#>)|
//!

use crate::namespace;

namespace!(
    "<http://www.w3.org/2006/03/test-description#>",;
    /// ``: 
    NAMESPACE_BASE, "",
    /// `Status in a review process`: A particular stage in a review process
    ReviewStatus, "ReviewStatus",
    /// `A status for a simple review process containing 6 possible stages`: 
    SimpleReviewStatus, "SimpleReviewStatus",
    /// `A Test Case based on a specification`: A test case which relates to a requirement set in a specification
    SpecificationTestCase, "SpecificationTestCase",
    /// `A Test Case`: A set of test inputs, execution conditions, and expected results developed for a particular objective, such as to exercise a particular program path or to verify compliance with a specific requirement
    TestCase, "TestCase",
    /// `accepted`: the item has gone through a first review, which shows it as valid for further processing
    accepted, "accepted",
    /// `approved`: the item has gone through the review process and was approved
    approved, "approved",
    /// `assigned`: a more specific review of the item has been assigned to someone
    assigned, "assigned",
    /// `expected results`: The results that a conformant implementation is expected to produce when this test is executed
    expectedResults, "expectedResults",
    /// `information resource as input`: Information Resource (e.g. a file) used as input for the test case
    informationResourceInput, "informationResourceInput",
    /// `information resource as expected results`: Information Resource (e.g. a file) that contains the expected results for the test case
    informationResourceResults, "informationResourceResults",
    /// `input`: Parameters or data that are needed for the test execution.
    input, "input",
    /// `on hold`: the item had already gone through the review process, but the results of the review need to be re-assessed due to new input
    onhold, "onhold",
    /// `precondition`: a condition that must be met before the test is executed
    preCondition, "preCondition",
    /// `purpose`: 
    purpose, "purpose",
    /// `rejected`: the item has gone through the review process and was rejected
    rejected, "rejected",
    /// `status of review`: 
    reviewStatus, "reviewStatus",
    /// `reference in specification`: a description or a link of what part of which specification lead to the creation of this test case
    specificationReference, "specificationReference",
    /// `unreviewed`: the item has been proposed, but hasn't been reviewed (e.g. for completeness) yet
    unreviewed, "unreviewed"
);
