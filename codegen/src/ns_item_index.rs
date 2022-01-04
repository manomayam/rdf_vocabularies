use std::path::Path;

use indexmap::IndexMap;
use serde::Serialize;
use sophia_api::{dataset::Dataset, term::SimpleIri};
use sophia_term::ArcTerm;

use crate::helpers::{
    key_words::sanitize_ident,
    rdf_arc_dataset::{get_arc_dataset, get_object_of_functional_statement_with},
    rdf_term::{some_if_iri, some_if_literal},
    rdf_types::{
        ser::{SerdeIri, SerdeOptLiteral},
        ArcDataset, ArcIri, ArcLiteral,
    },
};

pub const NAMESPACE_BASE_TERM_IDENT: &str = "NAMESPACE_BASE";

pub static TERM_PRED_LABEL: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#", Some("label"));
pub static TERM_PRED_COMMENT: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/2000/01/rdf-schema#", Some("comment"));
pub static TERM_PRED_SPEC_STATEMENT: SimpleIri =
    SimpleIri::new_unchecked("http://www.w3.org/ns/spec#", Some("statement"));

#[derive(Serialize)]
pub struct NSEntity {
    #[serde(with = "SerdeIri")]
    pub id: ArcIri,
    pub term_suffix: String,
    pub safe_term_suffix: String,
    #[serde(with = "SerdeOptLiteral")]
    pub label: Option<ArcLiteral>,
    #[serde(with = "SerdeOptLiteral")]
    pub comment: Option<ArcLiteral>,
}

/// Index of items in a given namespace
pub struct NSEntityIndex {
    pub namespace_base: ArcIri,
    pub index: IndexMap<ArcIri, NSEntity>,
}

impl NSEntityIndex {
    /// Given an ontology file and the namespace base, creates a namespace_entity_index and returns it.
    pub fn new_from_onto_file(namespace_base: &ArcIri, onto_file_path: &Path) -> Self {
        let onto_dataset = get_arc_dataset(&[onto_file_path]);
        Self::new(namespace_base, &onto_dataset)
    }

    /// Given an ontology dataset and the namespace base, creates a namespace_entity_index and returns it.
    pub fn new(namespace_base: &ArcIri, onto_dataset: &ArcDataset) -> Self {
        let entity_terms = onto_dataset.iris().unwrap();
        let entity_ids_sorted: Vec<ArcIri> = entity_terms
            .iter()
            .filter_map(|t| Some(some_if_iri(t)?.clone()))
            .collect();

        let mut index: IndexMap<ArcIri, NSEntity> =
            IndexMap::with_capacity(entity_ids_sorted.len() + 1);

        for entity_id in entity_ids_sorted {
            if let Some(entity) = Self::get_entity_info(&entity_id, onto_dataset, namespace_base) {
                index.insert(entity_id, entity);
            }
        }
        Self {
            index,
            namespace_base: namespace_base.clone(),
        }
    }

    fn get_entity_info(
        id: &ArcIri,
        onto_dataset: &ArcDataset,
        namespace_base: &ArcIri,
    ) -> Option<NSEntity> {
        let term_suffix: String = id.match_ns(namespace_base)?.collect();
        let safe_term_suffix = if term_suffix.is_empty() {
            NAMESPACE_BASE_TERM_IDENT.into()
        } else {
            sanitize_ident(&term_suffix)
        };
        let label = if let Some(val) = get_object_of_functional_statement_with(
            onto_dataset,
            id,
            &TERM_PRED_LABEL,
            Option::<&ArcTerm>::None,
        ) {
            Some(some_if_literal(&val)?.clone())
        } else {
            None
        };

        let mut comment = None;
        for pred in [TERM_PRED_COMMENT, TERM_PRED_SPEC_STATEMENT] {
            if let Some(val) = get_object_of_functional_statement_with(
                onto_dataset,
                id,
                &pred,
                Option::<&ArcTerm>::None,
            ) {
                comment = Some(some_if_literal(&val)?.clone());
                break;
            }
        }

        Some(NSEntity {
            id: id.clone(),
            term_suffix,
            safe_term_suffix,
            label,
            comment,
        })
    }
}
