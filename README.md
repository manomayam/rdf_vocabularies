# rdf_vocabularies

This crate is a distribution of few most commonly used RDF vocabularies. It also provides sophia terms for each vocabulary's terminology.

For each vocabulary with prefix `vocab_prefix`, it provides a module `rdf_vocabularies::ns::{vocab_prefix}`, that provides sophia terms for entities defined in that vocab's namespace, and a module `rdf_vocabularies::dataset::{vocab_prefix}` that provides ontology data of that vocab as a sophia quad-source or sophia dataset. These modules are behind cargo features `ns-{vocab_prefix}`, `dataset-{vocab_prefix}` respectively for each vocab. This way you can include only required vocabularies.

Please check documentation for included vocabularies, and their terms. These are generated from their ontologies, and also includes doc-comments for quick reference.

## Usage

Include the crate in your project dependencies, with features corresponding to required ontologies
```toml
[dependencies]
# includes namespaces for vocabularies `rdf`, `foaf`, `solid`, and ontology datasets for vocabularies `foaf`.
rdf_vocabularies = { version = "0.1.7", features=["ns-rdf", "ns-foaf", "ns-solid", "dataset-foaf"] }

```

And then use them.

```rust
use rdf_vocabularies::{ns::{rdf, foaf, solid}};
use sophia_api::term::{term_eq, SimpleIri};

assert!(term_eq(&foaf::Agent, &SimpleIri::new_unchecked("http://xmlns.com/foaf/0.1/", Some("Agent"))));
assert!(term_eq(&rdf::subject, &SimpleIri::new_unchecked("http://www.w3.org/1999/02/22-rdf-syntax-ns#", Some("subject"))));
```


License: MIT OR Apache-2.0
