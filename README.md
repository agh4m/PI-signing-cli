# DiSA CLI tool

This is a command line tool for the DiSA project.
It indexes a directory of files and generates an hash for each file, allowing you to sign the hash and store it in a blockchain, and send the files to a remote server.

## Development

This project is developed in Rust, using the [clap](https://docs.rs/clap/latest/clap/index.html) library for the command line interface.
It also uses an adapter for Autentication.Gov to sign the hashes, written in C++.
The adapter is a shared library that is loaded by the Rust code, and compiled by Cargo via gcc.

It is recomended to have both C++ and Rust tooling for development.

### Building

To build the project, you need to have Rust and Cargo installed. You also need to have the Autentication.Gov skd installed and a suitable C++ compiler installed in your system.

To build the project in dev mode, run the following command:

```sh
cargo build
```

Alternatively, run the following to build in production optimized mode:

```sh
cargo build --release
```

This will compile the project and generate the binary in the `target/debug` or `target/release` directory respectively.

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

## Testing

todo!()

## Installation

Precompiled binaries for Windows and Linux are available in the releases tab.


## FAQ

> Cannot compile the project

Make sure you have the Autentication.Gov SDK installed and a suitable C++ compiler installed in your system.
On Linux systems, this should be gcc, on windows it should be MSVC. Visual Studio might be needed.
On Windows some configuration might be needed, as the SDK is not installed in a path that is automatically detected by the MSVC.

> Cannot detect card readers

On Linux, make sure you have the `pcscd` service running.
This can be done by running the following command:

```sh
sudo systemctl start pcscd
```

You might also want to enable the service to start on boot:

```sh
sudo systemctl enable pcscd
```
