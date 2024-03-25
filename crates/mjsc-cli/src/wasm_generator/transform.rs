use std::borrow::Cow;

use anyhow::Result;
use walrus::{CustomSection, IdsToIndices, ModuleConfig, ModuleProducers};

use crate::js::JS;

#[derive(Debug)]
pub struct SourceCodeSection {
    source_code: Vec<u8>,
}

impl SourceCodeSection {
    pub fn compressed(js: &JS) -> Result<SourceCodeSection> {
        Ok(SourceCodeSection {
            source_code: js.compress()?,
        })
    }

    pub fn uncompressed(js: &JS) -> Result<SourceCodeSection> {
        Ok(SourceCodeSection {
            source_code: js.as_bytes().to_vec(),
        })
    }
}

impl CustomSection for SourceCodeSection {
    fn name(&self) -> &str {
        "lambda_source"
    }

    fn data(&self, _ids_to_indices: &IdsToIndices) -> Cow<[u8]> {
        (&self.source_code).into()
    }
}

pub fn module_config() -> ModuleConfig {
    let mut config = ModuleConfig::new();
    config.generate_name_section(false);
    config
}

pub fn add_producers_section(producers: &mut ModuleProducers) {
    producers.clear(); // removes Walrus and Rust
    producers.add_language("JavaScript", "ES2023");
    producers.add_processed_by("mjsc", env!("CARGO_PKG_VERSION"));
}
