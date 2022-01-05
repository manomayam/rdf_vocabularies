//! This module provides various helpers to handle sophia rdf terms
use sophia_term::{iri::Iri, literal::Literal, Term};
use std::hash::Hash;

/// Given a [`Term`](Term), returns It's value wrapped in [`Some`](Some) if it is of [`Term::Literal`](Term::Literal) variant, else [`None`](None)
pub fn some_if_literal<T>(term: &Term<T>) -> Option<&Literal<T>>
where
    T: AsRef<str> + Clone + Eq + Hash,
{
    if let Term::Literal(val) = term {
        Some(val)
    } else {
        None
    }
}

/// Given a [`Term`](Term), returns It's value wrapped in [`Some`](Some) if it is of [`Term::Iri`](Term::Iri) variant, else [`None`](None)
pub fn some_if_iri<T>(term: &Term<T>) -> Option<&Iri<T>>
where
    T: AsRef<str> + Clone + Eq + Hash,
{
    if let Term::Iri(val) = term {
        Some(val)
    } else {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use claim::{assert_none, assert_some};
    use rdf_utils::models::arc::{ArcIri, ArcLiteral};
    use sophia_term::{literal::Literal, Term};

    use crate::helpers::rdf_term::some_if_iri;

    use super::some_if_literal;

    #[test]
    fn some_if_literal_returns_some_on_literal() {
        let l1: ArcLiteral = Literal::new_lang(Arc::from("some text"), Arc::from("en")).unwrap();
        let l1_term = Term::Literal(l1.clone());

        let l1_val = some_if_literal(&l1_term);
        assert_some!(l1_val);
        assert_eq!(l1_val.unwrap(), &l1);
    }

    #[test]
    fn some_if_literal_returns_none_on_iri() {
        let iri1: ArcIri =
            ArcIri::new_unchecked(Arc::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#term"));
        let iri1_term = Term::Iri(iri1.clone());

        let iri1_val = some_if_literal(&iri1_term);
        assert_none!(iri1_val);
    }

    #[test]
    fn some_if_iri_returns_some_on_iri() {
        let iri1: ArcIri =
            ArcIri::new_unchecked(Arc::from("http://www.w3.org/1999/02/22-rdf-syntax-ns#term"));
        let iri1_term = Term::Iri(iri1.clone());

        let iri1_val = some_if_iri(&iri1_term);
        assert_some!(iri1_val);
        assert_eq!(iri1_val.unwrap(), &iri1);
    }

    #[test]
    fn some_if_iri_returns_none_on_literal() {
        let l1: ArcLiteral = Literal::new_lang(Arc::from("some text"), Arc::from("en")).unwrap();
        let l1_term = Term::Literal(l1.clone());

        let l1_val = some_if_iri(&l1_term);
        assert_none!(l1_val);
    }
}
