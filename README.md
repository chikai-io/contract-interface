# Near Contract Interface

_This is a WIP fork of [near-sdk-macros](https://github.com/near/near-sdk-rs/tree/master/near-sdk-macros) that enables generics for [Near contracts](https://docs.near.org/docs/develop/contracts/overview)._

## Motivation

Please see [near-sdk-rs#606](https://github.com/near/near-sdk-rs/issues/606) and the following features for more details.

## Features

- Generics for traits, impls and structs _(WIP)_ - [#5](https://github.com/chikai-io/contract-interface/issues/5)
- Public items _(WIP)_ - [#14](https://github.com/chikai-io/contract-interface/issues/14)
- Builder-pattern for making requests _(WIP)_ - [#4](https://github.com/chikai-io/contract-interface/issues/4)
- Wasm extern functions _(WIP)_ - [#12](https://github.com/chikai-io/contract-interface/issues/12)
- Named arguments _(WIP)_ - [#10](https://github.com/chikai-io/contract-interface/issues/13)
- Metadata gathering/exporting _(design)_ - [#1](https://github.com/chikai-io/contract-interface/issues/1)

### Dev Hints

#### Documentation

As this library makes heavy usage of generated modules, it is always recommended to see the documentation of your project in order to know about the generated items, such as with:

```console
cargo doc --no-deps --open -p contract-interface -p contract-standards
```

Then on the bottom-left you'll find the docs of both `contract-interface` and `contract-standards`, and also from other packages that you may want to add.  

#### Rust-Analyzer

If you're visualizing/editing them on vscode with rust-analyzer, to best see the autocompletion hints and documentation of various items, the following settings are recommended:

_settings.json_
```json
"rust-analyzer.procMacro.enable": true,
"rust-analyzer.experimental.procAttrMacros": true,
"rust-analyzer.checkOnSave.allTargets": true,
"rust-analyzer.checkOnSave.command": "clippy",
```

## Examples

### Fungible Token

Please check it's [workspace](examples/fungible-token/) to read more about testing it.  
As it has it's own workspace, instead of viewing it from this root directory you should check/experiment on it on a separated workspace editor, such as with `code ./examples/fungible-token/` or similar commands.

### Dummy Examples

All other examples are currently _dummy_ and drafty, where all method implementations are `unimplemented!()` but they otherwise are able to compile and showcase some usages of the `#[contract]` attribute.  
They are also contained in the root workspace, so you don't need to open a separated editor for them.

#### Documentation

Also, to see a dummy example's documentation, such as of `example_01`, you can use:

```console
cargo doc -p contract-standards -p contract-interface --no-deps --example example_01 --open
```

#### Dummy Example Files

- [example_01](./examples/example_01/lib.rs):  
  Has methods that have different cases of `self`.  
  Uses a [generated builder](./examples/example_01/client.rs) for making external calls.  
  For the motivation on this, please check [#4](https://github.com/chikai-io/contract-interface/issues/4) and [#14](https://github.com/chikai-io/contract-interface/issues/14).  
  Shows [manually](./examples/example_01/api_manual.rs) and [automatically](./examples/example_01/api.rs) created `extern "C"` functions for the binary.  
- [example_02](./examples/example_02/lib.rs):  
  Has traits that use many types of generics (all dummy usages).  
  Shows [manually](./examples/example_02/api_manual.rs) and [automatically](./examples/example_02/api.rs) created `extern "C"` functions for the binary.  
  Note that all generics must be known or defined at that time.
- [example_03](./examples/example_03/lib.rs):  
  Has a contract that contains a `FungibleToken` from the standard.
  Shows [manually](./examples/example_03/api_manual.rs) and [automatically](./examples/example_03/api.rs) created `extern "C"` functions for the binary.  
- [example_04](./examples/example_04/lib.rs):  
  Has methods that have some argument diversity,  
  such as receiving references.
  Shows [manually](./examples/example_04/api_manual.rs) and [automatically](./examples/example_04/api.rs) created `extern "C"` functions for the binary.  
- [example_05](./examples/example_05/lib.rs):  
  Has methods that use some attributes, such as `init` and such).
  Shows [manually](./examples/example_05/api_manual.rs) and [automatically](./examples/example_05/api.rs) created `extern "C"` functions for the binary.  
