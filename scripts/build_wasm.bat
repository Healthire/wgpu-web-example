cargo build --target=wasm32-unknown-unknown -p wgpu-web-example

mkdir ".\target\webroot"
copy ".\webroot\index.html" ".\target\webroot\index.html"
wasm-bindgen --target no-modules --no-typescript --out-dir ./target/webroot/ ./target/wasm32-unknown-unknown/debug/wgpu-web-example.wasm