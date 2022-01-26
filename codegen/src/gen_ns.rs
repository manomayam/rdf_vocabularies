//! This module handles code generation for `ns/<prefix>.rs` modules, and ns index module

use anyhow::Context;
use serde_json::json;
use std::path::Path;

use crate::{
    gen::CodegenContext,
    helpers::io::write_to_file,
    ns_entity_index::{NSEntity, NSEntityIndex},
    templates::{NS_INDEX_MOD_TEMPLATE_ID, NS_MOD_TEMPLATE_ID, TEMPLATE_REGISTRY},
    vocab_index::{Vocab, VocabIndex},
};

/// This function generates namespace modules for vocabs passed in context
pub fn gen_mods(context: &CodegenContext) -> anyhow::Result<()> {
    for (_, vocab) in &context.vocab_index.index {
        let mod_file_path = &context
            .base_dir_path
            .join(format!("src/ns/{}.rs", vocab.safe_prefix));

        log::info!("generating namespace module for {}", vocab.namespace_base);
        gen_mod(vocab, mod_file_path).with_context(|| {
            format!(
                "error in generating namespace module for vocab {}",
                vocab.namespace_base
            )
        })?;
    }
    Ok(())
}

/// This function generate namespace module for given vocab at given mod file path
pub fn gen_mod(vocab: &Vocab, mod_file_path: &Path) -> anyhow::Result<()> {
    let mod_content = compute_mod_content(vocab)?;
    write_to_file(&mod_content, mod_file_path)
}

/// This function computes and returns content of namespace mod for given vocab
pub fn compute_mod_content(vocab: &Vocab) -> anyhow::Result<String> {
    let vocab_file_path = &vocab.abs_file_path;
    let ns_entity_index = NSEntityIndex::new_from_onto_file(&vocab.namespace_base, vocab_file_path)
        .with_context(|| {
            format!(
                "error in loading namespace entity index for vocab {}",
                vocab.namespace_base
            )
        })?;
    (&TEMPLATE_REGISTRY)
        .render(
            NS_MOD_TEMPLATE_ID,
            &json!({
                "vocab": vocab,
                "entities": &ns_entity_index.index.values().collect::<Vec<&NSEntity>>()
            }),
        )
        .with_context(|| {
            format!(
                "error in rendering namespace mod template for vocab {}",
                vocab.namespace_base
            )
        })
}

/// This function generates index `ns/mod.rs` file which exports sub namespace modules corresponding to each vocab in given context
pub fn gen_index_mod(context: &CodegenContext) -> anyhow::Result<()> {
    let index_mod_file_path = &context.base_dir_path.join("src/ns/mod.rs");
    let mod_content = compute_index_mod_content(&context.vocab_index)?;
    write_to_file(&mod_content, &index_mod_file_path)
}

/// This function computes and returns content of index `ns/mod.rs` file
pub fn compute_index_mod_content(vocab_index: &VocabIndex) -> anyhow::Result<String> {
    (&TEMPLATE_REGISTRY)
        .render(
            NS_INDEX_MOD_TEMPLATE_ID,
            &json!({
                "vocabs": vocab_index.index.values().collect::<Vec<&Vocab>>(),
            }),
        )
        .with_context(|| format!("error in rendering namespace index mod template"))
}
