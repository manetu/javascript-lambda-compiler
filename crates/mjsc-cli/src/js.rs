/// Higher-level representation of JavaScript.
///
/// This is intended to be used to derive different representations of source
/// code. For example, as a byte array, a string, QuickJS bytecode, compressed
/// bytes, or attributes of the source code like what it exports.
use std::{
    fs::File,
    io::{Cursor, Read},
    path::Path,
    rc::Rc,
};

use anyhow::{Context, Result};
use brotli::enc::{self, BrotliEncoderParams};

#[derive(Clone, Debug)]
pub struct JS {
    source_code: Rc<String>,
}

impl JS {
    fn from_string(source_code: String) -> JS {
        JS {
            source_code: Rc::new(source_code),
        }
    }

    pub fn from_file(path: &Path) -> Result<JS> {
        let mut input_file = File::open(path)
            .with_context(|| format!("Failed to open input file {}", path.display()))?;
        let mut contents: Vec<u8> = vec![];
        input_file.read_to_end(&mut contents)?;
        Ok(Self::from_string(String::from_utf8(contents)?))
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.source_code.as_bytes()
    }

    pub fn compress(&self) -> Result<Vec<u8>> {
        let mut compressed_source_code: Vec<u8> = vec![];
        enc::BrotliCompress(
            &mut Cursor::new(&self.source_code.as_bytes()),
            &mut compressed_source_code,
            &BrotliEncoderParams {
                quality: 11,
                ..Default::default()
            },
        )?;
        Ok(compressed_source_code)
    }
}

#[cfg(test)]
mod tests {
    use crate::js::JS;

    use anyhow::Result;

    #[test]
    fn parse_no_exports() -> Result<()> {
        let exports = parse("function foo() {}")?;
        assert_eq!(Vec::<&str>::default(), exports);
        Ok(())
    }

    #[test]
    fn parse_invalid_js() -> Result<()> {
        let res = parse("fun foo() {}");
        assert_eq!("Invalid JavaScript", res.err().unwrap().to_string());
        Ok(())
    }

    #[test]
    fn parse_one_func_export() -> Result<()> {
        let exports = parse("export function foo() {}")?;
        assert_eq!(vec!["foo"], exports);
        Ok(())
    }

    #[test]
    fn parse_func_export_with_parameter() -> Result<()> {
        let res = parse("export function foo(bar) {}");
        assert_eq!(
            "Exported functions with parameters are not supported",
            res.err().unwrap().to_string()
        );
        Ok(())
    }

    #[test]
    fn parse_generator_export() -> Result<()> {
        let res = parse("export function *foo() {}");
        assert_eq!(
            "Exported generators are not supported",
            res.err().unwrap().to_string()
        );
        Ok(())
    }

    #[test]
    fn parse_two_func_exports() -> Result<()> {
        let exports = parse("export function foo() {}; export function bar() {};")?;
        assert_eq!(vec!["foo", "bar"], exports);
        Ok(())
    }

    #[test]
    fn parse_const_export() -> Result<()> {
        let exports = parse("export const x = 1;")?;
        let expected_exports: Vec<&str> = vec![];
        assert_eq!(expected_exports, exports);
        Ok(())
    }

    #[test]
    fn parse_const_export_and_func_export() -> Result<()> {
        let exports = parse("export const x = 1; export function foo() {}")?;
        assert_eq!(vec!["foo"], exports);
        Ok(())
    }

    #[test]
    fn parse_named_func_export() -> Result<()> {
        let exports = parse("function foo() {}; export { foo };")?;
        assert_eq!(vec!["foo"], exports);
        Ok(())
    }

    #[test]
    fn parse_named_func_export_with_arg() -> Result<()> {
        let res = parse("function foo(bar) {}; export { foo };");
        assert_eq!(
            "Exported functions with parameters are not supported",
            res.err().unwrap().to_string()
        );
        Ok(())
    }

    #[test]
    fn parse_funcs_with_args() -> Result<()> {
        let exports = parse("function foo(bar) {}")?;
        assert_eq!(Vec::<&str>::default(), exports);
        Ok(())
    }

    #[test]
    fn parse_named_func_export_and_const_export() -> Result<()> {
        let exports = parse("function foo() {}; const bar = 1; export { foo, bar };")?;
        assert_eq!(vec!["foo"], exports);
        Ok(())
    }

    #[test]
    fn parse_func_export_and_named_func_export() -> Result<()> {
        let exports = parse("export function foo() {}; function bar() {}; export { bar };")?;
        assert_eq!(vec!["foo", "bar"], exports);
        Ok(())
    }

    #[test]
    fn parse_renamed_func_export() -> Result<()> {
        let exports = parse("function foo() {}; export { foo as bar };")?;
        assert_eq!(vec!["bar"], exports);
        Ok(())
    }

    #[test]
    fn parse_hoisted_func_export() -> Result<()> {
        let exports = parse("export { foo }; function foo() {}")?;
        assert_eq!(vec!["foo"], exports);
        Ok(())
    }

    #[test]
    fn parse_renamed_hosted_func_export() -> Result<()> {
        let exports = parse("export { foo as bar }; function foo() {}")?;
        assert_eq!(vec!["bar"], exports);
        Ok(())
    }

    #[test]
    fn parse_hoisted_exports_with_func_and_const() -> Result<()> {
        let exports = parse("export { foo, bar }; function foo() {}; const bar = 1;")?;
        assert_eq!(vec!["foo"], exports);
        Ok(())
    }

    #[test]
    fn parse_default_arrow_export() -> Result<()> {
        let exports = parse("export default () => {}")?;
        assert_eq!(vec!["default"], exports);
        Ok(())
    }

    #[test]
    fn parse_default_function_export() -> Result<()> {
        let exports = parse("export default function() {}")?;
        assert_eq!(vec!["default"], exports);
        Ok(())
    }

    fn parse(js: &str) -> Result<Vec<String>> {
        JS::from_string(js.to_string()).exports()
    }
}
