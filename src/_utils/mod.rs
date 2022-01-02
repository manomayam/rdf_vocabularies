
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
