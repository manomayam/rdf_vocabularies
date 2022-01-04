//! This module orchestrates code generation using sub modules
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use anyhow::Context;

use crate::{gen_ns, onto_index::OntoIndex};

/// A `CodegenContext` wraps context for code generation
#[derive(Clone)]
pub struct CodegenContext {
    /// index of ontologies, against which namespace and dataset modules should be generated
    pub onto_index: Arc<OntoIndex>,
    /// Base repo directory, where code needs to generated
    pub base_dir_path: Arc<Path>,
}

pub fn generate(base_dir_path: &Path, onto_indices_rel_paths: &[&str]) -> anyhow::Result<()> {
    let onto_indices_paths = onto_indices_rel_paths
        .iter()
        .map(|rel_path| base_dir_path.join(rel_path))
        .collect::<Vec<PathBuf>>();
    let onto_index = OntoIndex::new_from_index_files(&onto_indices_paths, base_dir_path)
        .with_context(|| format!("error in loading ontology index"))?;
    let context = CodegenContext {
        onto_index: Arc::from(onto_index),
        base_dir_path: base_dir_path.into(),
    };
    gen_ns::gen_mods(&context)?;
    Ok(())
}
