# RustPwnTools
# Early development, changing API

[![Build Status](https://github.com/lehrbaumm/RustPwn/workflows/Rust/badge.svg)](https://github.com/lehrbaumm/RustPwn/actions?query=workflow%3ARust)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)
[![Crates.io](https://img.shields.io/crates/v/pwnd.svg)](https://crates.io/crates/pwnd)

## Learning Rust by writing a PwnTools Library

I already have some years of experience with Rust development from multiple small to middle projects.
But in a recent internal project, I relized, my experience and knowledge is still pretty small.

So I was thinking, what could I write in Rust, where I can learn as much as possible.

During CTF competition I always stick to Python, because of the PwnTools library with all its features.
But I want to switch to Rust, because I like the style and execution speed.

So I decided to write a library, that enables me to write CTF exploits purly in Rust.
My goal will never be to replace or copy pwntools, because the library is huge and I don't use all the features.

Instead I want to write a library, for how I would ease my CTF rapid exploit development.
I will have a look at what features I use a lot and try to implement them step by step.
I even have some ideas, on where I could improve for my own workflow.

I also want to share my library during the development in crates.io, to learn how this works.


## Features

- [ ] Process execution
- [ ] TCP Socket
- [ ] UDP Socket
- [ ] Hexdump
- [ ] Pipeline for Sockets and Processes
- [ ] Pipeline recvuntil / recv
- [ ] Pipeline sendline / send
- [ ] Pipeline interactive
- [ ] Cyclic pattern
- [ ] Process coredump on linux
- [ ] gdb remote debugging
- [ ] ELF Header
- [ ] PE Header
- [ ] x86/x64 Assembly/Dissassembly
- [ ] ARM Assembly/Dissassembly
- [ ] Binary Packing for little/big endia
- [ ] Display Binary Protection (NE Stack, W/X Protection, Stack Canary, PIE)
- [ ] Shellcodes DB
- [ ] ROP Gadgets
- [ ] Cryptography
- [ ] HTTP Post/Get

## Interesting Libraries and References

* [tokio](https://crates.io/crates/tokio)
* [nom](https://crates.io/crates/nom)
* [Github Actions](https://github.com/features/actions)
* [Rust API Guidlines](https://rust-lang.github.io/api-guidelines/)