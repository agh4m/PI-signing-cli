# DiSA CLI tool

This is a command line tool for the DiSA project.
It indexes a directory of files and generates an hash for each file, allowing you to sign the hash and store it in a blockchain, and send the files to a remote server.

## Development

This project is developed in Rust, using the [clap](https://docs.rs/clap/latest/clap/index.html) library for the command line interface.
It also uses an adapter for Autentication.Gov to sign the hashes, written in C++.
The adapter is a shared library that is loaded by the Rust code, and compiled by Cargo via gcc.

It is recomended to have both C++ and Rust tooling for development.

### Building

To build the project, you need to have Rust and Cargo installed. You also need to have the Autentication.Gov adapter compiled and available in its default location.

To build the project, run the following command:

```sh
cargo build
```

This will compile the project and generate the binary in the `target/debug` directory.

### Running

To run the project, you can use the following command:

```sh
cargo run -- <args>
```

This will compile the project and run it with the given arguments.

#### Available arguments

The available arguments are:
```
-p --path: The path to the directory to index (Required)
-s --save_location: The location to save the signed hashes (Optional) (Default: ./)
-c --cmd: Give the choice to use Chave Móvel Digital or Cartão de Cidadão (Optional) (Default: 1 - Cartão de Cidadão)
  -to use Chave Móvel Digital, use 0
-h --help: Show the help message (Exclusive)
-V --version: Show the version of the tool (Exclusive)
```

## Installation

todo!()
