use std::path::{Path, PathBuf};

use anyhow::Context;
use indexmap::IndexMap;
use rdf_utils::models::arc::{ArcIri, ArcLiteral, IndexedArcDataset};
use serde::Serialize;
use sophia_api::term::SimpleIri;
use sophia_term::ArcTerm;

use crate::{
    gen_features::{feature_dataset_for, feature_ns_for},
    helpers::{
        ident::sanitize_ident,
        rdf_arc_dataset::{
            get_arc_dataset, get_lang_literal_object_of_statement_with,
            get_object_of_functional_statement_with, get_subjects_of_statements_with, EN_LANG_TAG,
        },
        rdf_term::some_if_iri,
        rdf_types::{
            literal_without_new_line,
            ser::{SerdeIri, SerdeLiteral, SerdeOptLiteral},
        },
    },
};

pub static TERM_PRED_TYPE: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#", Some("type"));
pub static TERM_PRED_PREFIX: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/ns/rdfa#", Some("prefix"));
pub static TERM_PRED_FILE_NAME: SimpleIri =
    SimpleIri::new_unchecked("http://dbpedia.org/ontology/", Some("filename"));
pub static TERM_PRED_TITLE: SimpleIri =
    SimpleIri::new_unchecked("http://purl.org/dc/terms/", Some("title"));
pub static TERM_PRED_DESCRIPTION: SimpleIri =
    SimpleIri::new_unchecked("http://purl.org/dc/terms/", Some("description"));
pub static TERM_PRED_IS_DEFINED_BY: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#", Some("isDefinedBy"));
pub static TERM_PRED_RDFA_URI: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/ns/rdfa#", Some("uri"));
pub static TERM_TYPE_PREFIX_MAPPING: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/ns/rdfa#", Some("PrefixMapping"));

/// This Struct models prefix-mappings as recorded in `ontologies/_index.nq` index file
#[derive(Serialize)]
pub struct Vocab {
    #[serde(with = "SerdeIri")]
    pub id: ArcIri,
    #[serde(with = "SerdeLiteral")]
    pub prefix: ArcLiteral,
    pub safe_prefix: String,
    #[serde(with = "SerdeLiteral")]
    pub file_name: ArcLiteral,
    pub abs_file_path: PathBuf,
    #[serde(with = "SerdeOptLiteral")]
    pub title: Option<ArcLiteral>,
    #[serde(with = "SerdeOptLiteral")]
    pub description: Option<ArcLiteral>,
    #[serde(with = "SerdeIri")]
    pub namespace_base: ArcIri,
    #[serde(with = "SerdeIri")]
    pub is_defined_by: ArcIri,
    pub feature_ns: String,
    pub feature_dataset: String,
}

/// This is index of ontologies, indexed by PrefixMapping Term.
pub struct VocabIndex {
    pub index: IndexMap<ArcIri, Vocab>,
}

impl VocabIndex {
    /// Given index_files, and base directory creates an onto_index and returns it.
    pub fn new_from_index_files(
        index_file_paths: &[PathBuf],
        base_dir_path: &Path,
    ) -> anyhow::Result<Self> {
        let index_dataset = get_arc_dataset(index_file_paths)
            .with_context(|| format!("error in loading ontology indices"))?;
        Ok(Self::new(&index_dataset, base_dir_path))
    }

    /// Given index dataset, and base directory creates an onto_index and returns it.
    pub fn new(index_dataset: &IndexedArcDataset, base_dir_path: &Path) -> Self {
        let mut vocab_ids = get_subjects_of_statements_with(
            &index_dataset,
            &TERM_PRED_TYPE,
            &TERM_TYPE_PREFIX_MAPPING,
            Option::<&ArcTerm>::None,
        );
        vocab_ids.sort();

        let mut index: IndexMap<ArcIri, Vocab> = IndexMap::with_capacity(vocab_ids.len() + 1);

        for id in vocab_ids {
            if let Some(vocab) = Self::get_vocab(&id, &index_dataset, base_dir_path) {
                index.insert(id, vocab);
            }
        }
        Self { index }
    }

    fn get_vocab(id: &ArcIri, dataset: &IndexedArcDataset, base_dir_path: &Path) -> Option<Vocab> {
        let prefix = get_lang_literal_object_of_statement_with(
            dataset,
            id,
            &TERM_PRED_PREFIX,
            Option::<&ArcTerm>::None,
            EN_LANG_TAG,
        )?
        .clone();

        let file_name = get_lang_literal_object_of_statement_with(
            dataset,
            id,
            &TERM_PRED_FILE_NAME,
            Option::<&ArcTerm>::None,
            EN_LANG_TAG,
        )?;

        let title = if let Some(val) = get_lang_literal_object_of_statement_with(
            dataset,
            id,
            &TERM_PRED_TITLE,
            Option::<&ArcTerm>::None,
            EN_LANG_TAG,
        ) {
            Some(literal_without_new_line(val))
        } else {
            None
        };

        let description = if let Some(val) = get_lang_literal_object_of_statement_with(
            dataset,
            id,
            &TERM_PRED_DESCRIPTION,
            Option::<&ArcTerm>::None,
            EN_LANG_TAG,
        ) {
            Some(literal_without_new_line(val))
        } else {
            None
        };

        let is_defined_by = some_if_iri(&get_object_of_functional_statement_with(
            dataset,
            id,
            &TERM_PRED_IS_DEFINED_BY,
            Option::<&ArcTerm>::None,
        )?)?
        .clone();

        let namespace_base = some_if_iri(&get_object_of_functional_statement_with(
            dataset,
            id,
            &TERM_PRED_RDFA_URI,
            Option::<&ArcTerm>::None,
        )?)?
        .clone();

        let safe_prefix = sanitize_ident(prefix.txt());
        let feature_ns = feature_ns_for(&safe_prefix);
        let feature_dataset = feature_dataset_for(&safe_prefix);
        let abs_file_path = base_dir_path.join(file_name.txt().as_ref());

        Some(Vocab {
            id: id.clone(),
            safe_prefix,
            prefix,
            file_name,
            abs_file_path,
            title,
            description,
            is_defined_by,
            namespace_base,
            feature_ns,
            feature_dataset,
        })
    }
}
