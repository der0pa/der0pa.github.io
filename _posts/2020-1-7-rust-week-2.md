---
layout: post
author: der0pa
---

* Rust - a language for the future!

# wasmtime tutorial


working thru [wasmtime tutorial.](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md)


installed this : [wasi-sdk_8.0_amd64.deb](https://github.com/CraneStation/wasi-sdk/releases/download/wasi-sdk-8/wasi-sdk_8.0_amd64.deb)

testing C program
{% gist c334b061fe85661c94fcf36e0e5a28ec %}

termial output:
```console
395currahee@penguin:~/projects/wasmtime/wasi-tutorial/demo$ cargo build --target wasm32-wasi
   Compiling demo v0.1.0 (/home/395currahee/projects/wasmtime/wasi-tutorial/demo)
    Finished dev [unoptimized + debuginfo] target(s) in 3.34s
```
here is #hello world! in wasmtime  ... possibly take your seat!

more terminal output:
```console
395currahee@penguin:~/projects/wasmtime/wasi-tutorial$ wasmtime --dir=. --dir=/tmp demo.wasm test.txt /tmp/somewhere.txt
395currahee@penguin:~/projects/wasmtime/wasi-tutorial$ cat /tmp/somewhere.txt 
hello world!
```

and the rust version of this:
{% gist 3fe58f4fafa7bfbd5d10096f3bb6d190 %}

```console
$ wasmtime --dir=. --dir=/tmp demo.wasm test.txt /tmp/somewhere.txt
$ cat /tmp/somewhere.txt
hello world
```
wasmtime uses a security model which is capability-based.
allows sandboxing and file read/write access on specified local directories 

console output: 
```console
395currahee@penguin:~/projects/der0pa.github.io/rust-code/wasmtime/wasi-tutorial$ nano demo.wat
395currahee@penguin:~/projects/der0pa.github.io/rust-code/wasmtime/wasi-tutorial$ wasmtime demo.wat
hello world
 
```

the source code example web assembly **text** format   ... looks clojure like
 
{% gist 58e47279d411ba3bb8be43746186e931 %}

installed [bat](https://github.com/sharkdp/bat/releases/download/v0.12.1/bat_0.12.1_amd64.deb) to attempt syntax highlighting in the .md file.

expose rust syntax highlighting
```console
395currahee@penguin:~/projects/der0pa.github.io/rust-code/wasmtime/wasi-tutorial/demo$ bat src/*.rs
```
above command 
# bat src/*rs #
console output/print

```rust
───────┬──────────────────────────────────────────────────────────
       │ File: src/main.rs
───────┼──────────────────────────────────────────────────────────
   1   │ use std::env;
   2   │ use std::fs;
   3   │ use std::io::{Read, Write};
   4   │ 
   5   │ fn process(input_fname: &str, output_fname: &str) -> Resu
       │ lt<(), String> {
   6   │     let mut input_file =
   7   │         fs::File::open(input_fname).map_err(|err| format!
       │ ("error opening input: {}", err))?;
   8   │     let mut contents = Vec::new();
   9   │     input_file
  10   │         .read_to_end(&mut contents)
  11   │         .map_err(|err| format!("read error: {}", err))?;
  12   │ 
  13   │     let mut output_file = fs::File::create(output_fname)
  14   │         .map_err(|err| format!("error opening output '{}'
       │ : {}", output_fname, err))?;
  15   │     output_file
  16   │         .write_all(&contents)
  17   │         .map_err(|err| format!("write error: {}", err))
  18   │ }
  19   │ 
  20   │ fn main() {
  21   │     let args: Vec<String> = env::args().collect();
  22   │     let program = args[0].clone();
  23   │ 
  24   │     if args.len() < 3 {
  25   │         eprintln!("{} <input_file> <output_file>", progra
       │ m);
  26   │         return;
  27   │     }
  28   │ 
  29   │     if let Err(err) = process(&args[1], &args[2]) {
  30   │         eprintln!("{}", err)
  31   │     }
  32   │ }
```

# Ownership

[video on borrowing](https://youtu.be/vtUMT-GNaYE)


ownership code 
{% gist 6c441e04a750a4c3a1142ac39931e881 %}

more code

{% gist 04fda0b05c8a4a7c31cebda29fda39ba %}

[playground code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=04fda0b05c8a4a7c31cebda29fda39ba)


finished second week up looking at 'rust by example'  
# Tuples 
in particular

code source:
[rust by example](https://doc.rust-lang.org/rust-by-example/primitives/tuples.html)

Tuples  from rust-by-example
{% gist 4629a9209cb73e31fdccc1dd05fffc54 %} 
week three comming next