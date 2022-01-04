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


/// Helper remote types for serde serialization
pub mod ser {
    use serde::Serialize;
    use super::*;

    #[derive(Serialize)]
    #[serde(remote = "ArcIri")]
    pub struct SerdeIri(
        #[serde(getter = "ArcIri::to_string")]
        String
    );

    #[derive(Serialize)]
    #[serde(remote = "ArcLiteral")]
    pub struct SerdeLiteral(
        #[serde(getter = "ArcLiteral::to_string")]
        String
    );

    #[derive(Serialize)]
    #[serde(remote = "Option<ArcLiteral>")]
    pub struct SerdeOptLiteral(
        #[serde(getter = "opt_arc_literal_to_opt_string")]
        Option<String>
    );

    pub fn opt_arc_literal_to_opt_string(opt_literal: &Option<ArcLiteral>) -> Option<String> {
        match opt_literal {
            Some(literal) => Some(literal.to_string()),
            None => None,
        }
    }

}
