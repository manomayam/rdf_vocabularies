/// Create a "namespace module"
/// defining a set of terms within a given IRI space.
///
/// # Tests
/// This macro also create a test module to check that all created IRIs are valid.
#[macro_export]
macro_rules! namespace {
    (
        base: $base:expr,
        terms: [
            $(
                $(#[$touter:meta])*
                ($tid:ident, $tsuffix:expr)
            ),*
        ]
    ) => {
        /// Namespace base uri str.
        pub static NAMESPACE_BASE: sophia_api::ns::IriRef<&'static str> = sophia_api::ns::IriRef::new_unchecked_const($base);

        $(
            $(#[$touter])*
            #[allow(non_upper_case_globals)]
            pub static $tid: sophia_api::ns::NsTerm = sophia_api::ns::NsTerm::new_unchecked(
                    NAMESPACE_BASE,
                    $tsuffix,
            );
        )*

        /// Tests module
        mod tests {
            $(
                #[allow(non_snake_case)]
                #[test]
                fn $tid() {
                    let iri = sophia_api::ns::NsTerm::new_unchecked(
                        super::NAMESPACE_BASE,
                        $tsuffix,
                    );
                    assert!(sophia_api::ns::IriRef::new(iri.to_string()).is_ok());
                }
            )*
        }
    }
}
