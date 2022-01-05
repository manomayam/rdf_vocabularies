//! This module provides various helpers to handle with sophia rdf datasets
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::Context;
use rdf_utils::models::arc::{IndexedArcDataset, ArcIri, ArcLiteral};
use sophia_api::dataset::DQuadSource;
use sophia_api::{
    dataset::{Dataset, MutableDataset},
    quad::Quad,
    term::TTerm,
};
use sophia_term::ArcTerm;
use sophia_term::Term;
use sophia_turtle::parser::nq;


/// It take potentially one or more file paths and parses all those files into a single dataset.
pub fn get_arc_dataset(dataset_file_paths: &[PathBuf]) -> anyhow::Result<IndexedArcDataset> {
    let mut dataset = IndexedArcDataset::new();
    for f_path in dataset_file_paths {
        let f = File::open(f_path).with_context(|| {
            format!(
                "Error in opening dataset file: {}",
                f_path.to_string_lossy()
            )
        })?;
        let f = BufReader::new(f);
        let source = nq::parse_bufread(f);
        dataset.insert_all(source).with_context(|| {
            format!(
                "Error in loading dataset from file: {}",
                f_path.to_string_lossy()
            )
        })?;
    }
    Ok(dataset)
}

/// A function that can be passed to `map` of iterators to get object terms of statements
fn statement_to_object_map_fn(r: <DQuadSource<IndexedArcDataset> as Iterator>::Item) -> Option<ArcTerm> {
    let r = r.unwrap();
    Some(r.o().clone())
}

/// A function that can be passed to `filter_map` of iterators to get subject terms of statements whose whose subject terms are iris
fn statement_to_iri_subject_filter_map_fn(
    r: <DQuadSource<IndexedArcDataset> as Iterator>::Item,
) -> Option<ArcIri> {
    let r = r.unwrap();
    if let Term::Iri(sub_iri) = r.s() {
        Some(sub_iri.clone())
    } else {
        None
    }
}

/// Returns subject terms of statements with given predicate, object, graph components, and whose subject terms are iris
pub fn get_subjects_of_statements_with<TP, TO, TG>(
    dataset: &IndexedArcDataset,
    p: &TP,
    o: &TO,
    g: Option<&TG>,
) -> Vec<ArcIri>
where
    TP: TTerm + ?Sized,
    TO: TTerm + ?Sized,
    TG: TTerm + ?Sized,
{
    dataset
        .quads_with_pog(p, o, g)
        .filter_map(statement_to_iri_subject_filter_map_fn)
        .collect()
}

/// Returns object terms of statements with given subject, predicate, graph components
pub fn get_objects_of_statements_with<TP, TO, TG>(
    dataset: &IndexedArcDataset,
    s: &TP,
    p: &TO,
    g: Option<&TG>,
) -> Vec<ArcTerm>
where
    TP: TTerm + ?Sized,
    TO: TTerm + ?Sized,
    TG: TTerm + ?Sized,
{
    dataset
        .quads_with_spg(s, p, g)
        .filter_map(statement_to_object_map_fn)
        .collect()
}

/// Returns object term of functional statement with given subject, predicate, graph components, and whose object term is an iri
pub fn get_object_of_functional_statement_with<TP, TO, TG>(
    dataset: &IndexedArcDataset,
    s: &TP,
    p: &TO,
    g: Option<&TG>,
) -> Option<ArcTerm>
where
    TP: TTerm + ?Sized,
    TO: TTerm + ?Sized,
    TG: TTerm + ?Sized,
{
    dataset
        .quads_with_spg(s, p, g)
        .find_map(statement_to_object_map_fn)
}

pub fn get_lang_literal_object_of_statement_with<TP, TO, TG>(
    dataset: &IndexedArcDataset,
    s: &TP,
    p: &TO,
    g: Option<&TG>,
    pref_lang_tag: &str,
) -> Option<ArcLiteral>
where
    TP: TTerm + ?Sized,
    TO: TTerm + ?Sized,
    TG: TTerm + ?Sized,
{
    let mut opt_literal = None;
    let objects = get_objects_of_statements_with(dataset, s, p, g);
    if objects.len() == 0 {
        return opt_literal;
    }
    for o in objects {
        if let Term::Literal(l) = o {
            if let Some(lang) = l.lang() {
                if lang.as_ref() == pref_lang_tag {
                    opt_literal = Some(l);
                    break;
                }
            }
            if opt_literal == None {
                opt_literal = Some(l);
            }
        }
    }
    opt_literal
}

pub const EN_LANG_TAG: &str = "en";
