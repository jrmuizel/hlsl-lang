# hlsl-lang-cli

[![Crates.io](https://img.shields.io/crates/v/hlsl-lang-cli)](https://crates.io/crates/hlsl-lang-cli)

[`hlsl-lang`](https://crates.io/crates/hlsl-lang) debugging CLI.

*This is only a prototype for debugging, more options will be added in later updates.*

## Usage

Print HLSL AST to the standard output:
```bash
$ cargo run < source.hlsl
TranslationUnit
  ExternalDeclaration@0:0..45 `Declaration`
    Declaration@0:0..45 `Block`
      [...]
```

## Author

Alixinne <alixinne@pm.me>

## License

BSD-3-Clause
