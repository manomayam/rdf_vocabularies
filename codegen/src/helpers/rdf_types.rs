use std::sync::Arc;

use sophia_inmem::dataset::{GenericDataset, GspoWrapper, OgpsWrapper};
use sophia_term::blank_node::BlankNode;
use sophia_term::factory::ArcTermFactory;
use sophia_term::iri::Iri;
use sophia_term::literal::Literal;

pub type ArcDataset = OgpsWrapper<GspoWrapper<GenericDataset<u32, ArcTermFactory>>>;
pub type ArcIri = Iri<Arc<str>>;
pub type ArcLiteral = Literal<Arc<str>>;
pub type ArcBNode = BlankNode<Arc<str>>;

pub fn literal_without_new_line(literal: ArcLiteral) -> ArcLiteral {
    if !literal.txt().contains("\n") {
        literal
    } else {
        ArcLiteral::new_dt(literal.txt().replace("\n", " "), literal.dt())
    }
}

/// Helper remote types for serde serialization
pub mod ser {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(remote = "ArcIri")]
    pub struct SerdeIri(#[serde(getter = "ArcIri::to_string")] String);

    #[derive(Serialize)]
    #[serde(remote = "ArcLiteral")]
    pub struct SerdeLiteral(#[serde(getter = "literal_to_string")] String);

    pub fn literal_to_string(literal: &ArcLiteral) -> String {
        literal.txt().to_string()
    }

    #[derive(Serialize)]
    #[serde(remote = "Option<ArcLiteral>")]
    pub struct SerdeOptLiteral(#[serde(getter = "opt_arc_literal_to_opt_string")] Option<String>);

    pub fn opt_arc_literal_to_opt_string(opt_literal: &Option<ArcLiteral>) -> Option<String> {
        match opt_literal {
            Some(literal) => Some(literal.txt().to_string()),
            None => None,
        }
    }
}
