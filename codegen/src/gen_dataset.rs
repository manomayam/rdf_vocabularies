//! This module handles code generation for `dataset/<prefix>.rs` modules, and dataset index module

use anyhow::Context;
use serde_json::json;
use std::path::Path;

use crate::{
    gen::CodegenContext,
    helpers::io::write_to_file,
    templates::{DATASET_MOD_TEMPLATE_ID, TEMPLATE_REGISTRY, DATASET_INDEX_MOD_TEMPLATE_ID},
    vocab_index::{Vocab, VocabIndex},
};

/// This function generates dataset modules for ontologies passed in context
pub fn gen_mods(context: &CodegenContext) -> anyhow::Result<()> {
    for (_, vocab) in &context.vocab_index.index {
        let mod_file_path = &context
            .base_dir_path
            .join(format!("src/dataset/{}.rs", vocab.safe_prefix));

        log::info!("generating dataset module for {}", vocab.namespace_base);
        gen_mod(vocab, mod_file_path).with_context(|| {
            format!(
                "error in generating dataset module for vocab {}",
                vocab.namespace_base
            )
        })?;
    }
    Ok(())
}

/// This function generate dataset module for given vocab at given mod file path
pub fn gen_mod(vocab: &Vocab, mod_file_path: &Path) -> anyhow::Result<()> {
    let mod_content = compute_mod_content(vocab)?;
    write_to_file(&mod_content, mod_file_path)
}

/// This function computes and returns content of dataset mod for given vocab
pub fn compute_mod_content(vocab: &Vocab) -> anyhow::Result<String> {
    (&TEMPLATE_REGISTRY)
        .render(
            DATASET_MOD_TEMPLATE_ID,
            &json!(vocab),
        )
        .with_context(|| {
            format!(
                "error in rendering dataset mod template for vocab {}",
                vocab.namespace_base
            )
        })
}

/// This function generates index `ns/mod.rs` file which exports sub dataset modules corresponding to each vocab in given context
pub fn gen_index_mod(context: &CodegenContext) -> anyhow::Result<()> {
    let index_mod_file_path = &context.base_dir_path.join("src/dataset/mod.rs");
    let mod_content = compute_index_mod_content(&context.vocab_index)?;
    write_to_file(&mod_content, &index_mod_file_path)
}

/// This function computes and returns content of index `ns/mod.rs` file
pub fn compute_index_mod_content(vocab_index: &VocabIndex) -> anyhow::Result<String> {
    (&TEMPLATE_REGISTRY)
        .render(
            DATASET_INDEX_MOD_TEMPLATE_ID,
            &json!({
                "vocabs": vocab_index.index.values().collect::<Vec<&Vocab>>(),
            }),
        )
        .with_context(|| {
            format!(
                "error in rendering dataset index mod template"
            )
        })
}
