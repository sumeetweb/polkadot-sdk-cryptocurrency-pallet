# Minimal Template

This is a minimal template for creating a blockchain using the Polkadot SDK.

# Docs

You can generate and view the [Rust
Docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html) for this template
with this command:

```sh
cargo doc -p minimal-template --open
```

Building chainspec:
```sh
chain-spec-builder -c chainspec.json create --chain-name Sumeet --chain-id sumeet -r ./target/debug/wbuild/minimal-template-runtime/minimal_template_runtime.wasm default
```

