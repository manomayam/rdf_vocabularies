//! This module handles code generation for `ns/<prefix>.rs` modules, and ns index module

use anyhow::Context;
use serde_json::json;
use std::path::Path;

use crate::{
    gen::CodegenContext,
    helpers::io::write_to_file,
    ns_entity_index::{NSEntity, NSEntityIndex},
    onto_index::{OntoIndex, OntoPrefixMapping},
    templates::{NS_MOD_TEMPLATE_ID, TEMPLATE_REGISTRY},
};

/// This function generates namespace modules for ontologies passed in context
pub fn gen_mods(context: &CodegenContext) -> anyhow::Result<()> {
    for (_, onto_mapping) in &context.onto_index.index {
        let mod_file_path = &context
            .base_dir_path
            .join(format!("src/ns/{}.rs", onto_mapping.safe_prefix));

        log::info!(
            "generating namespace module for {}",
            onto_mapping.namespace_base
        );
        gen_mod(onto_mapping, mod_file_path).with_context(|| {
            format!(
                "error in generating namespace module for namespace {}",
                onto_mapping.namespace_base
            )
        })?;
    }
    Ok(())
}

/// This function generate namespace module for given ontology at given mod file path
pub fn gen_mod(onto_mapping: &OntoPrefixMapping, mod_file_path: &Path) -> anyhow::Result<()> {
    let mod_content = compute_mod_content(onto_mapping)?;
    write_to_file(&mod_content, mod_file_path)
}

/// This function computes and returns content of namespace mod for given ontology
pub fn compute_mod_content(onto_mapping: &OntoPrefixMapping) -> anyhow::Result<String> {
    let onto_file_path = &onto_mapping.abs_file_path;
    let ns_entity_index =
        NSEntityIndex::new_from_onto_file(&onto_mapping.namespace_base, onto_file_path)
            .with_context(|| {
                format!(
                    "error in loading namespace entity index for namespace {}",
                    onto_mapping.namespace_base
                )
            })?;
    (&TEMPLATE_REGISTRY)
        .render(
            NS_MOD_TEMPLATE_ID,
            &json!({
                "onto_mapping": onto_mapping,
                "entities": &ns_entity_index.index.values().collect::<Vec<&NSEntity>>()
            }),
        )
        .with_context(|| {
            format!(
                "error in rendering namespace mod template for namespace {}",
                onto_mapping.namespace_base
            )
        })
}

/// This function generates index `ns/mod.rs` file which exports sub namespace modules corresponding to each ontology in given context
pub fn gen_index_mod(context: CodegenContext) -> anyhow::Result<()> {
    let index_mod_file_path = &context.base_dir_path.join("src/ns/mod.rs");
    let mod_content = compute_index_mod_content(&context.onto_index);
    write_to_file(&mod_content, &index_mod_file_path)
}

/// This function computes and returns content of index `ns/mod.rs` file
pub fn compute_index_mod_content(_onto_index: &OntoIndex) -> String {
    todo!()
}
