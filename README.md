# Near Contract Interface

This is a WIP fork of [near-sdk-macros](https://github.com/near/near-sdk-rs/tree/master/near-sdk-macros) that enables generics for [Near contracts](https://docs.near.org/docs/develop/contracts/overview).

## Motivation

Please see [near-sdk-rs#606](https://github.com/near/near-sdk-rs/issues/606) and the features below for more details.

## Features

- Generics for traits, impls and structs _(WIP)_ - [#5](https://github.com/chikai-io/contract-interface/issues/5)
- Public items _(WIP)_ - [#14](https://github.com/chikai-io/contract-interface/issues/14)
- Builder-pattern for making requests _(WIP)_ - [#4](https://github.com/chikai-io/contract-interface/issues/4)
- Wasm extern functions _(WIP)_ - [#12](https://github.com/chikai-io/contract-interface/issues/12)
- Named arguments _(WIP)_ - [#10](https://github.com/chikai-io/contract-interface/issues/13)
- Metadata gathering/exporting _(design)_ - [#1](https://github.com/chikai-io/contract-interface/issues/1)

## Examples

All examples are currently _dummy_ and drafty, where all method implementations are `unimplemented!()` but they otherwise are able to compile and showcase some usages of the `#[contract]` attribute.  

### Documentation

It is recommended to generate the documentation of your projects in order to visualize the generated items:

```console
cargo doc -p contract-interface -p contract-standards  --no-deps --example example_01 --open
```

Then on the bottom-left you'll find the documentation of both `contract-interface` and `contract-standards`. Also, to see an example's documentation, such as of `example_01`:

```console
cargo doc -p contract-standards -p contract-interface --no-deps --example example_01 --open
```

### Rust-Analyzer

If you're visualizing/editing them on vscode with rust-analyzer, to best see the autocompletion hints and documentation of various items, the following settings are recommended:

_settings.json_
```json
"rust-analyzer.procMacro.enable": true,
"rust-analyzer.experimental.procAttrMacros": true,
"rust-analyzer.checkOnSave.allTargets": true,
"rust-analyzer.checkOnSave.command": "clippy",
```

### Example Files

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
