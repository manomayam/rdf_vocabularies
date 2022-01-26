//! for a given namespace, if their ontology file is defined by remote file, and it is not made available yet at it's local path, then it de-references and converts the ontology into nquads and persists at it's local path

use std::str::FromStr;

use anyhow::Context;
use mime::Mime;
use rdf_dynsyn::{
    correspondence::Correspondent,
    media_type,
    parser::quads::DynSynQuadParserFactory,
    serializer::quads::DynSynQuadSerializerFactory,
    syntax::{RdfSyntax, N_QUADS},
};
use rdf_utils::models::arc::ArcDataset;
use sophia_api::{
    parser::QuadParser,
    quad::stream::QuadSource,
    serializer::{QuadSerializer, Stringifier},
};
use sophia_term::Term;

use crate::{gen::CodegenContext, helpers::io::write_to_file, vocab_index::Vocab};

pub fn ensure_nq_for_context(context: &CodegenContext) -> anyhow::Result<()> {
    for vocab in context.vocab_index.index.values() {
        ensure_nq_for_vocab(vocab)?;
    }
    Ok(())
}

pub fn ensure_nq_for_vocab(vocab: &Vocab) -> anyhow::Result<bool> {
    // log::info!("onto file path: {:?}, exists: {}", &vocab.abs_file_path, vocab.abs_file_path.exists());
    if !vocab.abs_file_path.exists() {
        log::info!(
            "ontology file does not exist for vocab: {}",
            &vocab.safe_prefix
        );
        load_nq_for_vocab(vocab).and(Ok(true))
    } else {
        Ok(false)
    }
}

pub fn load_nq_for_vocab(vocab: &Vocab) -> anyhow::Result<()> {
    let ontology_uri = vocab.is_defined_by.chars().collect::<String>();
    let response = reqwest::blocking::get(&ontology_uri)?;

    let mt = response.headers().get(http::header::CONTENT_TYPE);
    if let None = mt {
        return Err(anyhow::anyhow!(
            "media type not supplied for ontology document for vocab {}",
            vocab.id
        ));
    }
    let mt = Mime::from_str(
        Mime::from_str(mt.unwrap().to_str()?)
            .with_context(|| "Invalid mime type for ontology document for vocab")?
            .essence_str(),
    )?;

    let mut content = response.text()?;
    if &mt != &*media_type::APPLICATION_N_QUADS {
        let syntax = Correspondent::<RdfSyntax>::try_from(&mt)
            .with_context(|| "Unknown mime type for ontology for given vocab")?
            .value;
        let parser = DynSynQuadParserFactory::new()
            .try_new_parser(
                syntax,
                Some(ontology_uri),
                Some(Term::Iri(vocab.namespace_base.clone())),
            )
            .with_context(|| "Un supported syntax in ontology document")?;
        let dataset: ArcDataset =
            parser
                .parse_str(&content)
                .collect_quads()
                .with_context(|| {
                    format!(
                        "document parsing error, syntax: {}, vocab: {}, content: {}",
                        syntax, vocab.prefix, &content
                    )
                })?;

        let mut nq_serializer =
            DynSynQuadSerializerFactory::new(None).try_new_stringifier(N_QUADS)?;
        content = nq_serializer.serialize_dataset(&dataset)?.to_string();
    }

    write_to_file(&content, &vocab.abs_file_path)?;
    Ok(())
}
