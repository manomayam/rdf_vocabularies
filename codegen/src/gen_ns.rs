use std::path::Path;

use crate::{gen::CodegenContext, helpers::io::write_to_file, onto_index::{OntoPrefixMapping, OntoIndex}};

pub fn gen_mods(context: CodegenContext) {
    for (_, onto_mapping) in &context.onto_index.index {
        let mod_file_path = &context
            .base_dir_path
            .join(format!("src/ns/{}.rs", onto_mapping.safe_prefix));
        gen_mod(onto_mapping, mod_file_path);
    }
}

pub fn gen_mod(onto_mapping: &OntoPrefixMapping, mod_file_path: &Path) {
    let mod_content = compute_mod_content(onto_mapping);
    write_to_file(&mod_content, mod_file_path);
}

pub fn compute_mod_content(onto_mapping: &OntoPrefixMapping) -> String {
    todo!()
}

pub fn gen_index_mod(context: CodegenContext) {
    let index_mod_file_path = &context
        .base_dir_path
        .join("src/ns/mod.rs");
    let mod_content = compute_index_mod_content(&context.onto_index);
    write_to_file(&mod_content, &index_mod_file_path);
}

pub fn compute_index_mod_content(onto_index: &OntoIndex) -> String {
    todo!()
}


pub mod term {
    use std::sync::Arc;

    /// A struct containing information regarding a term in given namespace
    pub struct NSTerm {
        pub ident: Arc<str>,
        pub term_suffix: Arc<str>,
        pub doc: Arc<str>,
    }
}


pub mod module {

}
