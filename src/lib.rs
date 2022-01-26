//! This crate is a distribution of the most commonly used RDF vocabularies. It also provides sophia terms for each vocabulary's terminology.
//!
//! For each vocabulary with prefix `vocabulary_prefix`, it provides a module `rdf_vocabularies::ns::{vocab_prefix}`, that provides sophia terms for entities defined in that vocab's namespace, and a module `rdf_vocabularies::dataset::{vocab_prefix}` that provides ontology data of that vocab as a sophia quad-source or sophia dataset. These modules are behind cargo features `ns-{vocab_prefix}`, `dataset-{vocab_prefix}` respectively for each vocab. This way you can include only required vocabularies.
//! 
//! Please check documentation for included vocabularies, and their terms. These are generated from their ontologies, and also includes doc-comments for quick reference.
//!
//! # Usage
//!
//! Include the crate in your project dependencies, with features corresponding to required ontologies
//! ```toml
//! [dependencies]
//! # includes namespaces for vocabularies `rdf`, `foaf`, `solid`, and ontology datasets for vocabularies `foaf`.
//! rdf_vocabularies = { version = "0.1.0", features=["ns-rdf", "ns-foaf", "ns-solid", "dataset-foaf"] }
//! 
//! ```
//! 
//! And then use them.
//! 
//! ```
//! use rdf_vocabularies::{ns::{rdf, foaf, solid}, dataset::foaf};
//! use sophia_api::term::term_eq;
//! use sophia_term::StaticTerm;
//! 
//! # pub fn try_main() -> Result<(), Box<dyn std::error::Error>> {
//! assert!(term_eq(&foaf::Agent, &StaticTerm::new_iri("http://xmlns.com/foaf/0.1/Agent")?));
//! assert!(term_eq(&rdf::subject, &StaticTerm::new_iri("http://www.w3.org/1999/02/22-rdf-syntax-ns#subject")?));
//! # Ok(())
//! # }
//! # fn main() {try_main().unwrap();}
//! ```
//! 

/// Provides sophia quad-sources/datasets of ontologies for included vocabularies
pub mod dataset;
/// provides sophia term constants for entities in namespace of included vocabularies.
pub mod ns;

mod _utils;
