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
- Inheritance _(design)_ - [#15](https://github.com/chikai-io/contract-interface/issues/15)
- Metadata gathering/exporting _(design)_ - [#1](https://github.com/chikai-io/contract-interface/issues/1)

## Examples

All examples are currently _dummy_ and drafty, where all method implementations are `unimplemented!()` but they otherwise are able to compile and showcase some usages of the `#[contract]` attribute.  

It is recommended to run `cargo doc --no-deps --examples --open` in order to see what items were created and their generated documentation - note that you can change the example number in the openned url in your browser.  
If you're visualizing/editing them on vscode with rust-analyzer, to best see the autocompletion hints and documentation of various items, the following settings are recommended:

_settings.json_
```json
"rust-analyzer.procMacro.enable": true,
"rust-analyzer.experimental.procAttrMacros": true,
"rust-analyzer.checkOnSave.allTargets": true,
"rust-analyzer.checkOnSave.command": "clippy",
```

- [example_01](./examples/example_01/lib.rs).  
  Methods that have different cases of `self`.  
  [Generated builder](./examples/example_01/client.rs) for making external calls.  
  For the motivation on this, please see [#4](https://github.com/chikai-io/contract-interface/issues/4) and [#14](https://github.com/chikai-io/contract-interface/issues/14).  
  [Manually created](./examples/example_01/api_manual.rs) `extern "C"` functions for the binary.  
  [Automatically created](./examples/example_01/api.rs) `extern "C"` functions for the binary, 
  that actually get created after calling a generated macro. 
- [example_02](./examples/example_02/lib.rs).  
  Traits that use many types of generics (all dummy usages).  
  [Automatically created](./examples/example_02/api.rs) `extern "C"` functions for the binary, 
  that actually get created after calling a geenrated macro.    
  Note that all generics must be known or defined at that time.
- example_03 (currently empty).  
- [example_04](./examples/example_04/lib.rs).  
  Methods that have some argument diversity,  
  such as receiving references.
- [example_05](./examples/example_05/lib.rs).  
  Methods that use some attributes, such as `init` and such).
