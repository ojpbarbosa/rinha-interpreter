![banner](https://github.com/ojpbarbosa/rinha-interpreter/assets/79005271/fb4e70b7-8644-4912-af4d-a3e5d19cc630)

<div align="center">
  <img src="https://img.shields.io/github/languages/top/ojpbarbosa/rinha-interpreter.svg" alt="GitHub top language">
  <img src="https://img.shields.io/github/repo-size/ojpbarbosa/rinha-interpreter.svg" alt="Repository size">
  <a href="https://github.com/ojpbarbosa/rinha-interpreter/commits">
    <img src="https://img.shields.io/github/last-commit/ojpbarbosa/rinha-interpreter.svg" alt="GitHub last commit">
  </a>
  <img src="https://img.shields.io/github/license/ojpbarbosa/rinha-interpreter.svg" alt="GitHub">
</div>

## Description

This project is a Rinha programming language compliant interpreter designed to take part in [@aripiprazole/rinha-de-compiler](https://github.com/aripiprazole/rinha-de-compiler), a "compilers cockfight". I had zero to none experience with Rust before and I decided to build a code interpreter written in it. What a great idea, isn't it? I implemented a [Tree-Walk Interpreter](https://craftinginterpreters.com/a-tree-walk-interpreter.html) that takes advantage of pre-generated [ASTs (abstract syntax trees)](https://wikipedia.org/wiki/Abstract_syntax_tree) through the [rinha crate](https://crates.io/crates/rinha) and evaluates it. It didn't turned out as a bad idea because somehow I got into [Rodrigo Navarro](https://github.com/reu)'s [live coding](https://www.youtube.com/watch?v=FbCdhicY3sk), in which he [implemented a minimal Rinha interpreter in Rust](https://github.com/reu/rinha-compiladores), explaining it along the way. [Henri Borges](https://github.com/hnrbs) also [implemented a great and simple interpreter in Rust](https://github.com/hnrbs/rinha), both of which served as a reference for me to deepen my studies in Rust and create my very own Rinha interpreter.

## Run

By default, the Rinha source code JSON AST is located at `/var/rinha/source.rinha.json`.

```bash
docker build . -t rinha-interpreter
docker run -v /path/to/source.rinha.json:/var/rinha/source.rinha.json --memory=2gb --cpus=2 rinha-interpreter
```

Please note that Docker is _required_ in order to run the interpreter.

## AST

The [rinha crate](https://crates.io/crates/rinha) is required in order to generate the Rinha source code JSON AST.
Once it's installed through `cargo install rinha`, the JSON AST can be generated using the following command.

```bash
rinha /path/to/source.rinha > /path/to/source.rinha.json
```

Please note that Rust is _required_ in order to install the rinha crate.

## References

- [Tree-Walk Interpreter](https://craftinginterpreters.com/a-tree-walk-interpreter.html)
- [AST (abstract syntax tree)](https://wikipedia.org/wiki/Abstract_syntax_tree)
- [rinha crate](https://crates.io/crates/rinha)
- [Rodrigo Navarro](https://github.com/reu)
  - [Live coding](https://www.youtube.com/watch?v=FbCdhicY3sk)
  - [Implementation](https://github.com/reu/rinha-compiladores)
- [Henri Borges](https://github.com/hnrbs)
  - [Implementation](https://github.com/hnrbs/rinha)

## Stack

- [Rust](https://www.rust-lang.org)
  - [Serde](https://serde.rs)
