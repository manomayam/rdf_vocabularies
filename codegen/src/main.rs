use rdf_vocabularies_codegen::gen;
use std::{env, path::PathBuf};

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let args: Vec<String> = env::args().collect();
    gen::generate(
        &PathBuf::from(&args[1]),
        &["ontologies/_index.nq", "ontologies_ext/_index.nq"],
    )
    .unwrap();
}
