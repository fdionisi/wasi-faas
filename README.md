<div align="center">
  <h1>WASI faas</h1>
  <p>
    <b>
      An experiment server running WebAssembly System Interface functions.
    </b>
  </p>
  <sub>
    Built on top of
    <a href="https://github.com/hyperium/hyper" target="_blank">Hyper</a> and
    <a href="https://github.com/bytecodealliance/wasmtime" target="_blank">Wasmtime</a>.
  </sub>
</div>

## Abstract
In the past few yeras, it became quite common to hear buzz words such as WASM and WASI,
function-as-a-service, and relatives.

With the advent of [Deno](https://github.com/denoland/deno), their
[Deploy](https://deno.com/deploy) service, the run to _edge-function_ by
[Netlify](https://www.netlify.com/products/#netlify-edge-functions), 
[Superbase](https://supabase.com/edge-functions), and
[Vercel](https://vercel.com/features/edge-functions), aside of the well enstablished
[Cloudfront Workers](https://workers.cloudflare.com/), I wanted to join the trend with
a small experiment.

After the intersting reading about the imminent
[Wasimtime 1.0 release](https://bytecodealliance.org/articles/wasmtime-10-performance),
I decided to try building a very basic function-as-a-service platform to run
WebAssembly System Interface executables in the cloud.

Similarly as for [fast-cli](https://github.com/fdionisi/fast-cli) and
[flagger](https://github.com/fdionisi/flagger), this project is also an opportunity to
improve my Rust, in this case, diving deeper in the subject of `proc_macro`.

The project is developed with [Zed](https://zed.dev).

## Run the example

Ensure to have `wasm32-wasi` target.

```sh
rustup target add wasm32-wasi
```

Build the example with the above target.

```sh
cargo build --bin wasi-faas-example --target wasm32-wasi --release
```

And then run the server.

```sh
cargo run --release --bin wasi-faas -- --module-path ./target/wasm32-wasi/release/wasi-faas-example.wasm
```

By default, the server will listen publicly on port 8888. Any combination of HTTP
methods and endpoints will run the target module.

```sh
curl --location --request POST 'http://localhost:8888/test-endpoint' \
  --header 'Content-Type: text/plain' \
  --data-raw 'Asd'
```

## License

_WASI faas_ is distributed under the terms of the MIT license.

See [LICENSE](LICENSE) for details.