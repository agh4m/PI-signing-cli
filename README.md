# DiSA Desktop Client

This repo now hosts a monorepo that includes the cli, a gui and a shared library all in their own crates.
It indexes a directory of files and generates an hash for each file, allowing you to sign the hash and store it in a blockchain, and send the files to a remote server.

### Structure

This repo has the following structure
```
cli        Source for the cli crate
sig_lib    Source for the shared library
src-tauri  Source for the tauri rust files
src        Source for the interface
```

To run this project you want to either run from within the cli or the src-tauri folder. Instructions once you are in one of these folders are bellow.

## Development

Part of this project is developed in Rust, using the [clap](https://docs.rs/clap/latest/clap/index.html) library for the command line interface.
The gui development is done with [Tauri](https://tauri.app/), which was chosen for its use of web technologies and their necessity when interacting with the Autenticacao.gov auth api.

It also has an adapter for the [Autentication.Gov](https://github.com/amagovpt/autenticacao.gov) lib used to sign the manifest, written in C++.
This has been tested with both version 3.11 and version 3.12 of that library.

`Note: Despite version 3.12 supporting the new contactless cards, these are not supported by this project`

The adapter is a shared library that is loaded by the Rust code, and compiled by Cargo via gcc. This is subject to change due to cross compilation [issues with windows](https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fwallpapercave.com%2Fwp%2Fwp5338276.jpg&f=1&nofb=1&ipt=dd97c7215cf26cab8becadbd60b0e5065d668f6223a2c2dcb6904195f2a2c24b&ipo=images).

It is needed to have both C++ and Rust tooling for development, as well as Bun or Node.JS for the gui develpment.

### Dotenv

This project uses a `.env` file to store the environment variables.

This file should be placed on the root of either the cli or gui app `(src-tauri)`.

The following variables are required:

```
RELEASE_MODE=production | development
BASIC_AUTH_USER=<username>
BASIC_AUTH_PASS=<password>
APPLICATION_ID=<application_id>
CONTRACT_ADDRESS=<contract_address>
NODE_URL=<node_url for the eth node where the contract is>
PRIVATE_KEY=<private_key for the wallet>
WALLET_ADDRESS=<the wallet addresss>
```

The `RELEASE_MODE` variable is used to determine if a signature is produced or not. If set to `production`, the documents are signed, however, if set to `development`, the signing step is skipped.

`BASIC_AUTH_USER`, `BASIC_AUTH_PASS` and `APPLICATION_ID` are used to authenticate with the CMD server.

The last four variables are used to connect with the blockchain to save the hash of the manifest, providing the proof of the documents existing.

An abi.json file is also required as it describes the contract. This file should be placed on the same folder as the .env.

### CLI

#### Building

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

#### Running

To run the project, you can use the following command:

```sh
cargo run -- <args>
```

This will compile the project and run it with the given arguments.
Similarly to the build command, you can use the `--release` flag, before any other arguments, to run the project in production mode.

##### Available arguments

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
-o --only_blockchain: Special tag used to skip all steps, except the upload to the blockchain.
```

### GUI

#### Building

To build the gui, simply run

```sh
bun tauri build
```

This will ensure that the SvelteKit project is built and then bundled as .deb, .exe, or .dmg.

#### Running

Running this project in development mode is done with

```sh
bun tauri dev
```

as per Tauri's instructions, running the Tauri app and the SvelteKit simultaneosly in one command.

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

> I get a linking with cc failed that mentions pteidlib

Make sure you have the autenticao.gov lib instaled.
The instructions for how to do so are found [here](https://github.com/amagovpt/autenticacao.gov).

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

> The tauri app starts but only shows a blank screen on the development environment

Please add this environment variable

```sh
export WEBKIT_DISABLE_DMABUF_RENDERER=1
```

This fix is discussed in [this thread](https://github.com/tauri-apps/tauri/issues/9304) and mostly seems to affect Nvidia GPU's.

> Cannot compile the tauri app on Arch Linux. (ERROR: Strip call failed)

This is a [known issue](https://github.com/tauri-apps/tauri/issues/8929) with linuxdeploy.

To work around this, run the build command with the following environment variable set:

```sh
NO_STRIP=true bun tauri build
```

> My Lsp crashes when editing this project

If the Lsp runs in node, there is a memory leak caused by cxx compilation artifacts that occurs when it tries to index the target/ forders.

This memory leak shows in the form of a node process using up to 4.7Gb or whatever your limit for a node process is, and higher than expected cpu usage.

To mitigate this, stop any Lsp's, any dev servers and delete any target/ folders cargo might have created (specially inside the cli, sig_lib and src-tauri folders).

If possible, add those same folders to an exclusion list so that the Lsp ignores them.
