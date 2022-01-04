pub const FEATURE_PREFIX_NS: &str = "ns-";
pub const FEATURE_PREFIX_DATASET: &str = "dataset-";
pub const FEATURE_DATASET: &str = "dataset";

pub fn feature_ns_for(suffix: &str) -> String {
    String::from(FEATURE_PREFIX_NS) + suffix
}

pub fn feature_dataset_for(suffix: &str) -> String {
    String::from(FEATURE_PREFIX_NS) + suffix
}
