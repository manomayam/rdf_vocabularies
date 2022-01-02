// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT
//! This module provides terms for `Dataset Usage Vocabulary` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|Dataset Usage Vocabulary|
//! |**Prefix**|duv|
//! |**Namespace base IRI**|[http://www.w3.org/ns/duv#](http://www.w3.org/ns/duv#)|
//! |**Description**|The Dataset Usage Vocabulary (DUV) is used to describe consumer experiences, citations, and feedback about datasets from the human perspective.|
//! |**Is defined by**|[http://www.w3.org/ns/duv#](http://www.w3.org/ns/duv#)|
//!

use crate::namespace;

namespace!(
    "http://www.w3.org/ns/duv#",;
    /// `Rating Feedback`: Predefined criteria used to express a user opinion about a dataset or distribution using a discrete range of values.
    RatingFeedback, "RatingFeedback",
    /// `Usage`: A helpful description of actions that can be performed on a given dataset or distribution.
    Usage, "Usage",
    /// `UsageTool`: A synopsis describing the way a tool can use a dataset or distribution.
    UsageTool, "UsageTool",
    /// `User Feedback`: User feedback on the dataset. Expresses whether the dataset was useful or not, for example.
    UserFeedback, "UserFeedback",
    /// `has distributor`: The distributor is the organization that makes the dataset available for downloading and use.
    hasDistributor, "hasDistributor",
    /// `has dataset feedback`: User feedback associated with Dataset or distribution
    hasFeedback, "hasFeedback",
    /// `has rating`: Rating Feedback has rating opinion
    hasRating, "hasRating",
    /// `has dataset/distribution usage`: Dataset/distribution usage guidance or instructions.
    hasUsage, "hasUsage",
    /// `has usage tool`: Describes the tool that provides the Usage 
    hasUsageTool, "hasUsageTool",
    /// `refers to dataset`: Dataset associated with Usage. 
    refersTo, "refersTo"
);
