# Example: Fungible Token (FT)

_This is a fork of [fungible-token-example](https://github.com/near/near-sdk-rs/tree/master/examples/fungible-token) that adapts it to this library's `#[contract]` attribute._

Example implementation of a [Fungible Token] contract which uses [contract-standards] and [simulation] tests.

  [Fungible Token]: https://nomicon.io/Standards/Tokens/FungibleTokenCore.html
  [contract-standards]: ../../contract-standards/
  [simulation]: https://github.com/near/near-sdk-rs/tree/master/near-sdk-sim

NOTES:
 - The maximum balance value is limited by U128 (2**128 - 1).
 - JSON calls should pass U128 as a base-10 string. E.g. "100".
 - This does not include escrow functionality, as `ft_transfer_call` provides a superior approach. An escrow system can, of course, be added as a separate contract.

## Building
To build run:
```bash
./build.sh
```

## Testing
To test run:
```bash
cargo test --all
cargo run --example heavy
```

## Documentation
To see the doc:
```bash
cargo doc --no-deps --open -p contract-interface -p contract-standards -p fungible-token -p defi
```
Then on the bottom-left you can choose the package.

## Changelog

### `1.0.0`

- Switched from using [NEP-21](https://github.com/near/NEPs/pull/21) to [NEP-141](https://github.com/near/NEPs/issues/141).

