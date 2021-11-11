# RustPwnTools
# Early development, changing API

[![Build Status](https://github.com/lehrbaumm/RustPwn/workflows/Rust/badge.svg)](https://github.com/lehrbaumm/RustPwn/actions?query=workflow%3ARust)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/pwnd.svg)](https://crates.io/crates/pwnd)

## Learning Rust by writing a PwnTools Library

I like the Rust programming language with its contraints and restrictions.
But I wanted to start a bigger Project, wo get to know more of the language and use it.

Because I play a lot of CTFs in my free time, I am pretty familiar to PwnTools.
So I decided I could implement to most used features of pwntools I use the most in rust.

This way I could write my exploits in Rust and learn more of the language.
My goal will never be to replace or copy pwntools, because the library is huge and I don't use all the features.

Instead I want to write a library, for how I would ease my CTF rapid exploit development.
I will have a look at what features I use a lot and try to implement them step by step.
I even have some ideas, on where I could improve for my own workflow.

My first primary for a first version, will be implementing tubes.
At first release only the function remote / process / recvuntil / recvline / send / sendline / close.
Because this is the part of pwntools I use the most.


## Features

- [X] Process execution
- [X] Process recvuntil / recvline
- [X] Process send / sendline
- [X] Tubes create_process
- [ ] TCP Socket connect
- [ ] TCP Socket connection status
- [ ] TCP Socket recvuntil / recvline
- [ ] TCP Socket send / sendline
- [ ] Attach GDB with tmux
- [ ] Hexdump output
- [ ] Tubes recv_timeout
- [ ] Tubes interactive
- [ ] Cyclic pattern
- [ ] Linux coredump
- [ ] ELF Header
- [ ] PE Header
- [ ] x86/x64 Assembly/Dissassembly
- [ ] ARM Assembly/Dissassembly
- [ ] Binary Packing for little/big endia
- [ ] Checksec (NE Stack, W/X Protection, Stack Canary, PIE)
- [ ] Shellcodes DB
- [ ] ROP Gadgets
- [ ] HTTP Post/Get