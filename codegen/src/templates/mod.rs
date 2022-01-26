//! This module exports handlebars template registry for codegen

use handlebars::Handlebars;
use once_cell::sync::Lazy;

pub static NS_MOD_TEMPLATE_ID: &str = "NS_MOD_TEMPLATE";

pub static NS_INDEX_MOD_TEMPLATE_ID: &str = "NS_INDEX_MOD_TEMPLATE";

pub static DATASET_MOD_TEMPLATE_ID: &str = "DATASET_MOD_TEMPLATE";

pub static DATASET_INDEX_MOD_TEMPLATE_ID: &str = "DATASET_INDEX_MOD_TEMPLATE";

pub static TEMPLATE_REGISTRY: Lazy<Handlebars> = Lazy::new(|| {
    let mut reg = Handlebars::new();
    reg.register_template_string(NS_MOD_TEMPLATE_ID, include_str!("ns_mod.hbs"))
        .unwrap();
    reg.register_template_string(NS_INDEX_MOD_TEMPLATE_ID, include_str!("ns_index_mod.hbs"))
        .unwrap();
    reg.register_template_string(DATASET_MOD_TEMPLATE_ID, include_str!("dataset_mod.hbs"))
        .unwrap();
    reg.register_template_string(
        DATASET_INDEX_MOD_TEMPLATE_ID,
        include_str!("dataset_index_mod.hbs"),
    )
    .unwrap();
    reg
});
