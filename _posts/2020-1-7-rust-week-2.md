---
layout: post
author: der0pa
---

Websock-Main.elm
{% gist 48934aa990c7e0b8e5baa65b33e48660 %}


working thru [wasmtime tutorial.](https://github.com/bytecodealliance/wasmtime/blob/master/docs/WASI-tutorial.md)


installed this : [wasi-sdk_8.0_amd64.deb](https://github.com/CraneStation/wasi-sdk/releases/download/wasi-sdk-8/wasi-sdk_8.0_amd64.deb)

```
395currahee@penguin:~/projects/wasmtime/wasi-tutorial/demo$ cargo build --target wasm32-wasi
   Compiling demo v0.1.0 (/home/395currahee/projects/wasmtime/wasi-tutorial/demo)
    Finished dev [unoptimized + debuginfo] target(s) in 3.34s
```
here is #hello world! in wasmtime  ... possibly take a seat!
```
395currahee@penguin:~/projects/wasmtime/wasi-tutorial$ wasmtime --dir=. --dir=/tmp demo.wasm test.txt /tmp/somewhere.txt
395currahee@penguin:~/projects/wasmtime/wasi-tutorial$ cat /tmp/somewhere.txt 
hello world!
```