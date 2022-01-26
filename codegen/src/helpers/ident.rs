//! This module provides helpers to handle names of identifiers

use once_cell::sync::Lazy;

const INVALID_CHARS_RE: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(&format!(
        "[{}]",
        regex::escape(r##"!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##)
    ))
    .unwrap()
});

pub fn sanitize_ident(ident: &str) -> String {
    if ident.len() == 0 {
        return ident.to_string();
    }
    let mut ident = INVALID_CHARS_RE.replace_all(ident, "_").to_string();
    if ident.chars().next().unwrap().is_numeric() {
        ident = format!("N_{}", ident);
    }
    if syn::parse_str::<syn::Ident>(&ident).is_err() {
        ident + "_"
    } else {
        ident
    }
}
