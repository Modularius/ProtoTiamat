# Tiamat


## Development Log

### 04/10/25

When compiling, the error
"This wasm target is unsupported by mio. If using Tokio, disable the net feature."
was encountered.
This was fixed by running
```shell
cargo tree -e features -i mio
```
to see which dependency features were referencing `mio`.

Then I removed `tokio` features `rt-multi-thread` and `signal` from `Cargo.toml` and added
```toml
  "tokio/rt-multi-thread",
  "tokio/signal",
```
to `ssr = [...]` in `communitee/Cargo.toml` etc.

### 04/10/25
When compiling, the error
```
      it looks like the Rust project used to create this Wasm file was linked against
      version of wasm-bindgen that uses a different bindgen format than this binary:

        rust Wasm file schema version: 0.2.104
           this binary schema version: 0.2.100

      Currently the bindgen format is unstable enough that these two schema versions
      must exactly match. You can accomplish this by either updating this binary or
      the wasm-bindgen dependency in the Rust project.

      You should be able to update the wasm-bindgen dependency with:

          cargo update -p wasm-bindgen --precise 0.2.100

      don't forget to recompile your Wasm file! Alternatively, you can update the
      binary with:

          cargo install -f wasm-bindgen-cli --version 0.2.104

      if this warning fails to go away though and you're not sure what to do feel free
      to open an issue at https://github.com/rustwasm/wasm-bindgen/issues!
   2: 
```
was encountered.

This is fixed by fixing the `wasm-bindgen` version in `Cargo.toml` via:
```toml
wasm-bindgen = "=0.2.100"
```