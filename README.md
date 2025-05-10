# vsixHarvester

## VSCode Extension Downloader in Rust

This Rust program reads the `recommendations` array from an `extensions.json` file and downloads the corresponding VSIX packages for Visual Studio Code extensions.

### Features

- Reads a list of extensions from `extensions.json`.
- Downloads the latest version of each extension as a VSIX package.
- Supports proxy configuration.
- Option to force re-download even if the file already exists.
- Provides verbose output for detailed logging.
- Downloads a single extension by its ID.

### Prerequisites

- **Rust** and **Cargo** installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

### Installation

```sh
cargo install vsixHarvester
```

### Usage

```sh
vsixHarvester [OPTIONS]
```

#### Options

- `-i`, `--input <INPUT>`: Path to the `extensions.json` file. Default is `./.vscode/extensions.json`.
- `-d`, `--destination <DESTINATION>`: Destination folder to save the VSIX files. Default is `./.vscode/extensions`.
- `-f`, `--force`: Force re-download even if the extension file already exists.
- `--proxy <PROXY>`: Proxy URL to use for HTTP requests.
- `-v`, `--verbose`: Enable verbose output for detailed logging.
- `-h`, `--help`: Print help information.
- `-a`, `--arch <ARCHITECTURE>`: OS architecture to install the extensions for.
- `-s`, `--single <EXTENSION_ID>`: Download a single extension by its ID (e.g., `publisher.extensionName`). If this option is used, `--input` is ignored.

#### Example

```sh
vsixHarvester \
  --input ./your/path/to/extensions.json \
  --destination ./your/path/to/extensions \
  --force \
  --arch win32-x64 \
  --verbose
```

To download a single extension:

```sh
vsixHarvester \
  --single publisher.extensionName \
  --force \
  --arch win32-x64 \
  --destination ./extensions \
  --verbose
```

##### Architecture options

- `win32-x64`
- `win32-arm64`
- `darwin-x64`
- `darwin-arm64`
- `linux-x64`
- `linux-arm64`
- `alpine-x64`
- `alpine-arm64`
- `win32-ia32`
- `linux-armhf`
- `web`

[Platform-specific](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions)

### extensions.json Format

The `extensions.json` file should have the following structure:

```json
{
  "recommendations": [
    "publisher.extensionName",
    "anotherPublisher.anotherExtensionName",
    // Add more extensions as needed
  ]
}
```

### Thanks

- Inspired from [offvsix](https://github.com/exaluc/offvsix)
