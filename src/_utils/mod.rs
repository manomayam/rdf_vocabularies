
/// Create a "namespace module"
/// defining a set of terms within a given IRI space.
///
/// # Tests
/// This macro also create a test module to check that all created IRIs are valid.
///
/// This allows to skip those checks at runtime, keeping the initialization of the namespace fast.
// This is adapted from https://docs.rs/sophia_api/0.7.1/src/sophia_api/ns.rs.html#1-389
#[macro_export]
macro_rules! namespace {
    ($iri_prefix:expr, $($(#[$outer:meta])*$suffix:ident),*; $($(#[$outer1:meta])*$r_id:ident, $r_sf:expr),*) => {
        /// Prefix used in this namespace.
        pub static PREFIX:&'static str = $iri_prefix;
        $(
            $(#[$outer])*
            pub static $suffix: sophia_api::term::SimpleIri =
                sophia_api::term::SimpleIri::new_unchecked($iri_prefix, Some($suffix));
            $crate::ns_iri!($iri_prefix, $suffix);
        )*
        $(
            $(#[$outer1])*
            #[allow(non_upper_case_globals)]
            pub static $r_id: sophia_api::term::SimpleIri =
                sophia_api::term::SimpleIri::new_unchecked($iri_prefix, Some($r_sf));
        )*

        /// Test module for checking tha IRIs are valid
        #[cfg(test)]
        mod test_valid_iri {
            #[test]
            $(
                #[allow(non_snake_case)]
                #[test]
                fn $suffix() {
                    sophia_api::term::SimpleIri::new($iri_prefix, Some(stringify!($suffix))).expect(stringify!($suffix));
                }
            )*
            $(
                #[allow(non_snake_case)]
                #[test]
                fn $r_id() {
                    sophia_api::term::SimpleIri::new($iri_prefix, Some($r_sf)).expect($r_sf);
                }
            )*
        }
    };
    ($iri_prefix:expr, $($suffix:ident),*) => {
        namespace!($iri_prefix, $($suffix),*;);
    };
}


#[macro_export]
macro_rules! dataset_from_path {
    ($dataset_path:expr) => {
        use rio_turtle::{NQuadsParser, TurtleError};
        use rdf_utils::models::arc::IndexedArcDataset;
        use sophia_api::quad::stream::QuadSource;
        use sophia_rio::parser::StrictRioSource;
        use sophia_turtle::parser::nq;

        /// Static str content of ontology dataset, encoded as nquads
        pub static DATASET_CONTENT: &str = include_str!($dataset_path);

        /// It returns an instance of a type that implements [`QuadSource`](QuadSource) trait.
        /// We can collect the quads from this instance to any type that implements [`Dataset`](sophia_api::dataset::Dataset) trait using [`.collect_quads`](QuadSource::collect_quads) method
        pub fn get_quad_source() -> StrictRioSource<NQuadsParser<&'static [u8]>, TurtleError> {
            nq::parse_str(DATASET_CONTENT)
        }

        /// This method returns an instance of [`IndexedArcDataset`](IndexedArcDataset), which implements  [`Dataset`](sophia_api::dataset::Dataset)
        pub fn get_indexed_dataset() -> IndexedArcDataset {
            match get_quad_source().collect_quads::<IndexedArcDataset>() {
                Ok(dataset) => dataset,
                Err(_) => unreachable!("If it is error, included static content must be faulty"),
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;
            use sophia_api::dataset::Dataset;

            fn validate_iris(dataset: IndexedArcDataset) {
                let iris_result = dataset.iris();
                claim::assert_ok!(&iris_result);
            }

            #[test]
            pub fn can_collect_into_dataset() {
                let dataset: IndexedArcDataset = get_indexed_dataset();
                validate_iris(dataset);
            }
        }
    }
}
