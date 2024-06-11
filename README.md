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
chain-spec-builder -c ./chainspec.json create -n custom-runtime -i custom-runtime -r ./target/release/wbuild/minimal-template-runtime/minimal_template_runtime.compact
.wasm -s default
```