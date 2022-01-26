use rdf_utils::models::arc::{ArcIri, ArcLiteral};

pub fn literal_without_new_line(literal: ArcLiteral) -> ArcLiteral {
    if !literal.txt().contains("\n") && !literal.txt().contains("\r") {
        literal
    } else {
        ArcLiteral::new_dt(
            literal.txt().replace("\n", " ").replace("\r", ""),
            literal.dt(),
        )
    }
}

/// Helper remote types for serde serialization
pub mod ser {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    #[serde(remote = "ArcIri")]
    pub struct SerdeIri(#[serde(getter = "iri_to_string")] String);

    pub fn iri_to_string(iri: &ArcIri) -> String {
        iri.chars().collect::<String>()
    }

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
