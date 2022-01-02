import rdflib
import re
import os
import sys
from typing import Callable, Generator, List
from collections import namedtuple

#  From https://gist.github.com/ritz078/1be714dea593838587c8a5df463a583a
RUST_KEY_WORDS = [
  "as",
  "use",
  "extern crate",
  "break",
  "const",
  "continue",
  "crate",
  "else",
  "if",
  "if let",
  "enum",
  "extern",
  "false",
  "fn",
  "for",
  "if",
  "impl",
  "in",
  "for",
  "let",
  "loop",
  "match",
  "mod",
  "move",
  "mut",
  "pub",
  "impl",
  "ref",
  "return",
  "Self",
  "self",
  "static",
  "struct",
  "super",
  "trait",
  "true",
  "type",
  "unsafe",
  "use",
  "where",
  "while",
  "abstract",
  "alignof",
  "become",
  "box",
  "do",
  "final",
  "macro",
  "offsetof",
  "override",
  "priv",
  "proc",
  "pure",
  "sizeof",
  "typeof",
  "unsized",
  "virtual",
  "yield"
]

GENERATED_NOTICE = "// THIS FILE IS GENERATED. ONE SHOULD NOT MODIFY IT"

NEW_LINE = '\n'

def sanitize_name(name: str):
    s_name = re.sub(r'[- \.]', '_', name)
    return s_name if s_name not in RUST_KEY_WORDS else f"{s_name}_"

OntoInfo = namedtuple("OntoInfo", ["prefix_term", "prefix", "sanitized_prefix", "ns_base_iri", "onto_file_path", "is_defined_by", "title", "description"])

def get_ns_term_names(g: rdflib.Graph, ns_base: str):
    all_nodes = g.all_nodes()
    for node in all_nodes:
        if type(node) != rdflib.URIRef:
            continue
        node: rdflib.URIRef = node
        if not node.startswith(ns_base):
            continue
        name = node.replace(ns_base, "").strip()
        if not name:
            continue
        yield name

def get_doc(info: OntoInfo) -> str:
    return f"""//! This module provides terms for `{info.title or info.prefix}` vocabulary
//!
//! ## Vocabulary:
//!
//! |||
//! |-----|-----|
//! |**Title**|{info.title or ''}|
//! |**Prefix**|{info.prefix}|
//! |**Namespace base IRI**|[{info.ns_base_iri}]({info.ns_base_iri})|
//! |**Description**|{info.description or ''}|
//! |**Is defined by**|{f"[{info.is_defined_by}]({info.is_defined_by})" if info.is_defined_by else ""}|
//!
"""

def get_ns_mod(g: rdflib.Graph, info: OntoInfo) -> str:
    term_names = sorted(get_ns_term_names(g, info.ns_base_iri))
    un_allowed_chars_re = re.compile(r"[-\.]")
    sep = ",\n"
    return f"""{GENERATED_NOTICE}
{get_doc(info)}
use crate::namespace;

namespace!(
    "{info.ns_base_iri}",;
{sep.join([f'    {sanitize_name(tn)}, "{tn}"' for tn in term_names])}
);
"""

def get_ns_mod_from_file(gfp: str, info: OntoInfo, format=None) -> str:
    d = rdflib.Dataset()
    d.parse(str(gfp), format=format)
    return get_ns_mod(d.graph(info.ns_base_iri), info)

def gen_ns_mods(onto_infos: List[OntoInfo], repo_root_dir: str):
    ns_dir = os.path.join(repo_root_dir, "src/ns/")
    for onto_info in onto_infos:
        if not onto_info.prefix_term or not onto_info.sanitized_prefix or not onto_info.ns_base_iri:
            continue
        print(f"Generating ns_mod for prefix: {onto_info.prefix}, sanitized: {onto_info.sanitized_prefix}, ns:{onto_info.ns_base_iri}, fp:{onto_info.onto_file_path}")

        ns_mod_content = get_ns_mod_from_file(onto_info.onto_file_path, onto_info, format='nquads')
        ns_mod_file = os.path.join(ns_dir, f"{onto_info.sanitized_prefix}.rs")
        open(ns_mod_file, 'wb').write(ns_mod_content.encode("utf-8"))


def gen_ns_index_mod(onto_infos: List[OntoInfo], repo_root_dir: str):
    sep = "\n\n"
    get_doc: Callable[[OntoInfo], str] = lambda info: f"""/// This module provides terms for `{info.title or info.prefix}` vocabulary"""
    index_mod_content = f"""{GENERATED_NOTICE}
{sep.join([
    f'{get_doc(info)}{NEW_LINE}#[cfg(feature = "vocab-{info.sanitized_prefix}")]{NEW_LINE}pub mod {info.sanitized_prefix};'
    for info in onto_infos
])}
    """
    index_mod_file = os.path.join(repo_root_dir, "src/ns/mod.rs")
    open(index_mod_file, 'wb').write(index_mod_content.encode('utf-8'))

def gen_dataset_mod_from_file(gfp: str, ns_base: str, format=None) -> str:
    pass

def gen_dataset_mods(onto_infos: List[OntoInfo], repo_root_dir: str):
    pass

def gen_dataset_index_mod(onto_infos: List[OntoInfo], repo_root_dir: str):
    pass

def gen_cargo_features(onto_infos: List[OntoInfo], repo_root_dir: str):
    pass

def get_index_graph(index_file: str) -> rdflib.Graph:
    g = rdflib.Graph()
    g.load(index_file, format='nquads')
    return g

def get_property(graph: rdflib.Graph, sub, property: str):
    return (list(graph.triples((sub, rdflib.URIRef(property), None))) or [[None, None, None]])[0][2]

def get_onto_info_from_onto_file(index_graph: rdflib.Graph, onto_dir: str, onto_fie_name: str):
    pref_triples = list(index_graph.triples((None, rdflib.URIRef("http://dbpedia.org/ontology/filename"), rdflib.Literal(f"ontologies/{onto_fie_name}"))))
    if len(pref_triples) == 0:
        return OntoInfo(None, None, None, None, None, None, None, None)
    
    prefix_term = pref_triples[0][0]
    prefix = str(get_property(index_graph, prefix_term, "http://www.w3.org/ns/rdfa#prefix"))
    sanitized_prefix = sanitize_name(prefix)
    ns_base_iri = get_property(index_graph, prefix_term, "http://www.w3.org/ns/rdfa#uri")
    is_defined_by = get_property(index_graph, prefix_term, "http://www.w3.org/2000/01/rdf-schema#isDefinedBy")
    title =  get_property(index_graph, prefix_term, "http://purl.org/dc/terms/title")
    title = title.replace('\n', ' ') if title else None;
    description = get_property(index_graph, prefix_term, "http://purl.org/dc/terms/description")
    description = description.replace('\n', ' ') if description else None;
    return OntoInfo(prefix_term, prefix, sanitized_prefix, ns_base_iri, os.path.join(onto_dir, onto_fie_name), is_defined_by, title, description)

def gen_vocabs_from_ontologies(repo_root_dir: str):
    onto_dir = os.path.join(repo_root_dir, 'ontologies/')
    dataset_dir = os.path.join(repo_root_dir, 'src/dataset/')
    ns_dir = os.path.join(repo_root_dir, 'src/ns/')

    index_file = os.path.join(onto_dir, "_index.nq")
    if not os.path.exists(index_file):
        sys.exit("no onto index file")
    index_graph = get_index_graph(index_file)

    onto_infos = [
        get_onto_info_from_onto_file(index_graph, onto_dir, onto_file)
        for onto_file in
        filter(
            lambda onto_file: not (onto_file.startswith('_') or not os.path.isfile(os.path.join(onto_dir, onto_file)) or not onto_file.endswith('.nq')),
            os.listdir(onto_dir)
        )
    ]
    onto_infos = sorted(onto_infos, key=lambda o: o.sanitized_prefix)

    gen_ns_mods(onto_infos, repo_root_dir)
    gen_ns_index_mod(onto_infos, repo_root_dir)
    gen_dataset_mods(onto_infos, repo_root_dir)
    gen_dataset_index_mod(onto_infos, repo_root_dir)
    gen_cargo_features(onto_infos, repo_root_dir)


if __name__ == "__main__":
    (scripts_path, script_name) = os.path.split(os.path.abspath(__file__))
    repo_root_dir = os.path.dirname(scripts_path)
    print(f"repo: {repo_root_dir}")
    gen_vocabs_from_ontologies(repo_root_dir)
