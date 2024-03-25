mod commands;
mod js;
mod wasm_generator;

use crate::commands::{Command};
use crate::wasm_generator::r#static as static_generator;
use anyhow::{Result};
use js::JS;
use std::fs;
use structopt::StructOpt;

fn main() -> Result<()> {
    let cmd = Command::from_args();

    match &cmd {
        Command::Compile(opts) => {
            let js = JS::from_file(&opts.input)?;
            let wasm = static_generator::generate(&js, opts.no_source_compression)?;
            fs::write(&opts.output, wasm)?;
            Ok(())
        }
    }
}
