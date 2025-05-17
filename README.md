# vsixHarvester

## VSCode Extension Downloader in Rust

This Rust program downloads VSIX packages for Visual Studio Code extensions. It can read a list of extensions from an `extensions.json` file, download a single extension by its ID, or display information about extensions without downloading.

### Features

- Supports downloading single extensions or lists from `extensions.json`.
- If no subcommand is specified, it defaults to downloading extensions based on `extensions.json` or top-level download options.
- Displays extension information (versions, supported platforms) without downloading using the `info` command.
- Downloads the latest version of each extension as a VSIX package.
- Supports proxy configuration.
- Option to force re-download even if the file already exists.
- Provides verbose output for detailed logging.

### Prerequisites

- **Rust** and **Cargo** installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

### Installation

```sh
cargo install vsixHarvester
```

### Usage

```sh
vsixHarvester [GLOBAL_OPTIONS] [COMMAND] [COMMAND_OPTIONS]
```

If no `COMMAND` is specified, `vsixHarvester` defaults to the `download` behavior, using the download options provided at the top level (e.g., `vsixHarvester --single <ID>`).

#### Global Options

- `--proxy <PROXY>`: Proxy URL to use for HTTP requests. (Applies to all commands)
- `-v`, `--verbose`: Enable verbose output for detailed logging. (Applies to all commands)
- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.

#### Commands

##### `download`

Downloads VSIX extension packages. If no specific download options are given with the `download` command itself, it defaults to using `./.vscode/extensions.json`.
The download options can also be used at the top level if no command is specified.

**Options for `download` (and top-level default):**

- `-i`, `--input <INPUT>`: Path to the `extensions.json` file.
  Default: `./.vscode/extensions.json`.
- `-d`, `--destination <DESTINATION>`: Destination folder to save the VSIX files.
  Default: `./.vscode/extensions`.
- `-f`, `--force`: Force re-download even if the extension file already exists.
- `-a`, `--arch <ARCHITECTURE>`: OS architecture to download the extensions for (e.g., `win32-x64`). See "Architecture options" below.
- `-s`, `--single <EXTENSION_ID>`: Download a single extension by its ID (e.g., `publisher.extensionName`). If this option is used, `--input` is ignored.

##### `info`

Displays information about VSIX extensions (latest versions, supported platforms) without downloading them.

**Options for `info`:**

- `-i`, `--input <INPUT>`: Path to the `extensions.json` file to get info for multiple extensions.
  Default: `./.vscode/extensions.json`.
- `-s`, `--single <EXTENSION_ID>`: Get info for a single extension by its ID (e.g., `publisher.extensionName`). If this option is used, `--input` is ignored.

#### Examples

**Default behavior (downloading from `extensions.json`):**

```sh
vsixHarvester
```

Or explicitly:

```sh
vsixHarvester download
```

**Downloading from `extensions.json` with options (no subcommand):**

```sh
vsixHarvester --input ./path/to/your/extensions.json --destination ./output_dir --force -v
```

**Downloading a single extension (no subcommand):**

```sh
vsixHarvester -s publisher.extensionName -d ./vsix_files -f -v --arch win32-x64
```

**Using the `download` subcommand explicitly:**

```sh
vsixHarvester download -s publisher.extensionName -d ./vsix_files -f -v --arch win32-x64
```

**Getting information for a single extension:**

```sh
vsixHarvester info -s publisher.extensionName
```

**Getting information for extensions listed in `extensions.json`:**

```sh
vsixHarvester info --input ./path/to/your/extensions.json
```

Or using the default `extensions.json`:

```sh
vsixHarvester info
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

This format follows the VS Code [Workspace Recommended Extensions](https://code.visualstudio.com/docs/configure/extensions/extension-marketplace#_workspace-recommended-extensions) specification. The `recommendations` array contains extension IDs that VS Code will suggest installing when someone opens the workspace.

### Thanks

- Inspired from [offvsix](https://github.com/exaluc/offvsix)
