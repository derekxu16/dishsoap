# dishsoap

- Lexing is done using [Logos](https://github.com/maciejhirsz/logos)
- Parsing is done using a Pratt parser
- The rest is done using [LLVM](https://crates.io/crates/llvm-sys) through [llvm-sys](https://crates.io/crates/llvm-sys)

References:<br>

- [LLVM Kaleidoscope Tutorial](https://llvm.org/docs/tutorial/MyFirstLanguageFrontend/index.html)
- [Using LLVM from Rust to generate WebAssembly binaries](https://medium.com/@jayphelps/using-llvm-from-rust-to-generate-webassembly-93e8c193fdb4) by Jay Phelps
- [Pratt Parsers: Expression Parsing Made Easy](http://journal.stuffwithstuff.com/2011/03/19/pratt-parsers-expression-parsing-made-easy/) by Bob Nystrom
- [LLVM C interface documentation](https://llvm.org/doxygen/group__LLVMCCoreInstructionBuilder.html)
