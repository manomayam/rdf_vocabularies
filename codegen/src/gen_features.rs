use std::str::FromStr;

use anyhow::Context;
use toml::{
    value::{Array, Table},
    Value,
};

use crate::{
    gen::CodegenContext,
    helpers::io::{read_from_file, write_to_file},
    vocab_index::VocabIndex,
};

pub const FEATURE_PREFIX_NS: &str = "ns-";
pub const FEATURE_PREFIX_DATASET: &str = "dataset-";
pub const FEATURE_DATASET: &str = "dataset";

pub const FEATURES_TABLE_KEY: &str = "features";

pub fn feature_ns_for(vocab_prefix: &str) -> String {
    String::from(FEATURE_PREFIX_NS) + vocab_prefix
}

pub fn feature_dataset_for(vocab_prefix: &str) -> String {
    String::from(FEATURE_PREFIX_DATASET) + vocab_prefix
}

pub fn update_manifest(context: &CodegenContext) -> anyhow::Result<()> {
    let manifest_path = context.base_dir_path.join("Cargo.toml");
    let manifest_content = read_from_file(&manifest_path)?;
    let manifest_val =
        Value::from_str(&manifest_content).with_context(|| "Invalid manifest file")?;
    if let Value::Table(mut manifest_table) = manifest_val {
        update_features_table(&mut manifest_table, &context.vocab_index)?;
        write_to_file(
            &toml::to_string_pretty(&Value::Table(manifest_table))?,
            &manifest_path,
        )
        .with_context(|| "Invalid manifest file")
    } else {
        Err(anyhow::anyhow!("Invalid manifest"))
    }
}

pub fn update_features_table(
    manifest_table: &mut Table,
    vocab_index: &VocabIndex,
) -> anyhow::Result<()> {
    if !manifest_table.contains_key(FEATURES_TABLE_KEY) {
        manifest_table.insert(FEATURES_TABLE_KEY.into(), Value::Table(Table::new()));
    }
    let features_opt = manifest_table.get_mut(FEATURES_TABLE_KEY);
    if let Some(features) = features_opt {
        match features {
            Value::Table(features_table) => {
                let features_table_update = compute_features_table(vocab_index);
                features_table.extend(features_table_update);
                Ok(())
            }
            _ => {
                return Err(anyhow::anyhow!("Invalid features table in manifest"));
            }
        }
    } else {
        return Err(anyhow::anyhow!("Invalid features table in manifest"));
    }
}

pub fn compute_features_table(vocab_index: &VocabIndex) -> Table {
    let mut table = Table::new();
    let ns_dep_features = Value::Array(Array::new());
    let mut dataset_dep_features_list = Array::new();
    dataset_dep_features_list.insert(0, Value::String("dataset".into()));
    let dataset_dep_features = Value::Array(dataset_dep_features_list);
    for vocab in vocab_index.index.values() {
        table.insert(feature_ns_for(&vocab.safe_prefix), ns_dep_features.clone());
        table.insert(
            feature_dataset_for(&vocab.safe_prefix),
            dataset_dep_features.clone(),
        );
    }
    table
}
