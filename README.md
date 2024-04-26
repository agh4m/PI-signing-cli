# DiSA ~~CLI tool~~ Desktop Client

~~This is a command line tool for the DiSA project.~~
This repo now hosts a monorepo that includes the cli, a gui and a shared library all in their own crates.
It indexes a directory of files and generates an hash for each file, allowing you to sign the hash and store it in a blockchain, and send the files to a remote server.

## Development

Part of this project is developed in Rust, using the [clap](https://docs.rs/clap/latest/clap/index.html) library for the command line interface.
The gui development is done with [Tauri](https://tauri.app/), which was choosen for its use of web technologies and their necessity when interacting with the Autenticacao.gov auth api.

It also uses an adapter for Autentication.Gov to sign the hashes, written in C++.
The adapter is a shared library that is loaded by the Rust code, and compiled by Cargo via gcc. This is subject to change due to cross compilation [issues with windows](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwallpapercave.com%2Fwp%2Fwp5338276.jpg&f=1&nofb=1&ipt=dd97c7215cf26cab8becadbd60b0e5065d668f6223a2c2dcb6904195f2a2c24b&ipo=images).

It is recomended to have both C++ and Rust tooling for development, as well as Bun or Npm for the gui develpment.

### Dotenv

```
This section only applies to the cli for now.
```

This project uses a `.env` file to store the environment variables.

The following variables are required:

```
RELEASE_MODE=production | development
BASIC_AUTH_USER=<username>
BASIC_AUTH_PASS=<password>
APPLICATION_ID=<application_id>
```

The `RELEASE_MODE` variable is used to determine if a signature is produced or not. If set to `production`, the documents are signed, however, if set to `development`, the signing step is skipped.
`BASIC_AUTH_USER`, `BASIC_AUTH_PASS` and `APPLICATION_ID` are used to authenticate with the CMD server.

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
Similarly to the build command, you can use the `--release` flag, before any other arguments, to run the project in production mode.

#### Available arguments

```
This section only applies to the cli, as should be obvious.
```

The available arguments are:
```
-p --path: The path to the directory to index (Required)
-s --save_location: The location to save the signed hashes (Optional) (Default: ./)
-c --cmd: Give the choice to use Chave Móvel Digital or Cartão de Cidadão (Optional) (Default: CC)
  -to use Chave Móvel Digital, add this flag
-a --archive_files: Send the file to the remote server (Optional) (Default: true)
  -to not send the files to the remote server, add this flag
-t --threads: The number of threads to use (Optional) (Default: half of the available cores)
-b --bearer_token: Pass the autheticaton token.
-h --help: Show the help message (Exclusive)
-V --version: Show the version of the tool (Exclusive)
```

## Testing

```
Only the shared library has tests for now.
```

Whilst there are some tests, these only test certain functions of the project.
Some functions were deemed too simple to test, or out of the scope of what should be tested (e.g. functions that only call other functions, or functions that call libraries).

To run the tests, use the following command:

```sh
cargo test
```

## Installation

Precompiled binaries for Windows and Linux are available in the releases tab, these are outdated however and do not intereact with the server, nor do they produce a valid signature.


## FAQ

> Cannot compile the project

Make sure you have the Autentication.Gov SDK installed and a suitable C++ compiler installed in your system.
On Linux systems, this should be gcc, on windows it should be MSVC. Visual Studio might also be needed.

On Windows some aditional configuration might be required, as the SDK is not installed in a path that is automatically detected by MSVC.

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
