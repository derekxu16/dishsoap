# dishsoap-compiler

### Usage
The only target is WebAssembly right now
1. `cargo run file.soap` generates `main.bc`, an LLVM bitcode file<br>
2. `llc -march=wasm32 -filetype=obj main.bc -o main.o` generates an object file
3. `lld -flavor wasm --allow-undefined --no-entry --export-all main.o -o main.wasm` generates a wasm file
