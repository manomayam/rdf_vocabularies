//! This module provides various helpers to handle with sophia rdf datasets
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use sophia_api::dataset::DQuadSource;
use sophia_api::{
    dataset::{Dataset, MutableDataset},
    quad::Quad,
    term::TTerm,
};
use sophia_term::ArcTerm;
use sophia_term::Term;
use sophia_turtle::parser::nq;

use super::rdf_types::{ArcDataset, ArcIri};


/// It take potentially one or more file paths and parses all those files into a single dataset.
pub fn get_arc_dataset(dataset_file_paths: &[&Path]) -> ArcDataset {
    let mut dataset = ArcDataset::new();
    for f_path in dataset_file_paths {
        let f = File::open(f_path).unwrap();
        let f = BufReader::new(f);
        let source = nq::parse_bufread(f);
        dataset.insert_all(source).unwrap();
    }
    dataset
}

/// A function that can be passed to `map` of iterators to get object terms of statements
fn statement_to_object_map_fn(r: <DQuadSource<ArcDataset> as Iterator>::Item) -> Option<ArcTerm> {
    let r = r.unwrap();
    Some(r.o().clone())
}

/// A function that can be passed to `filter_map` of iterators to get subject terms of statements whose whose subject terms are iris
fn statement_to_iri_subject_filter_map_fn(r: <DQuadSource<ArcDataset> as Iterator>::Item) -> Option<ArcIri> {
    let r = r.unwrap();
    if let Term::Iri(sub_iri) = r.s() {
        Some(sub_iri.clone())
    } else {
        None
    }
}

/// Returns subject terms of statements with given predicate, object, graph components, and whose subject terms are iris
pub fn get_subjects_of_statements_with<TP, TO, TG>(
    dataset: &ArcDataset,
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
    dataset: &ArcDataset,
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
    dataset: &ArcDataset,
    s: &TP,
    p: &TO,
    g: Option<&TG>,
) -> Option<ArcTerm>
where
    TP: TTerm + ?Sized,
    TO: TTerm + ?Sized,
    TG: TTerm + ?Sized,
{
    dataset.quads_with_spg(s, p, g).find_map(statement_to_object_map_fn)
}
