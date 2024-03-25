use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "mjsc", about = "Manetu Javascript Compiler for Lambda Functions")]
pub enum Command {
    /// Compiles JavaScript to WebAssembly.
    Compile(CompileCommandOpts),
}

#[derive(Debug, StructOpt)]
pub struct CompileCommandOpts {
    #[structopt(parse(from_os_str))]
    /// Path of the JavaScript input file.
    pub input: PathBuf,

    #[structopt(short = "o", parse(from_os_str), default_value = "index.wasm")]
    /// Desired path of the WebAssembly output file.
    pub output: PathBuf,

    #[structopt(long = "no-source-compression")]
    /// Disable source code compression, which reduces compile time at the expense of generating larger WebAssembly files.
    pub no_source_compression: bool,
}
