use std::{path::Path, sync::Arc};

use crate::onto_index::OntoIndex;

pub const GEN_NOTICE: &str = "// THIS FILE IS GENERATED ONE. ONE SHOULD NOT MODIFY IT MANUALLY";


#[derive(Clone)]
pub struct CodegenContext {
    pub onto_index: Arc<OntoIndex>,
    pub base_dir_path: Arc<Path>,
}
