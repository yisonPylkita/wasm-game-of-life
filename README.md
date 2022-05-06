# Game of Life in Rust WASM
Conway's Game of Life written in Rust WASM

## 🚴 Usage
### Prerequisites
#### Install Rustup
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
#### Install `wasm-pack`
```bash
cargo install wasm-pack
```
### 🛠️ Building
```bash
wasm-pack build --target web
```

### Using
```bash
python3 -m http.server 8000

# Now go to localhost:8000 in your browser
```

### 🔬 Test in Headless Browsers
> WARNING: No tests yet!
```bash
wasm-pack test --headless --chrome
```
