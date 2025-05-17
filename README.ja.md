# vsixHarvester

## Rust製のVSCode拡張機能ダウンローダー

このRustプログラムは、Visual Studio Code拡張機能のVSIXパッケージをダウンロードします。`extensions.json`ファイルから拡張機能のリストを読み込んだり、拡張機能IDを指定して単一の拡張機能をダウンロードしたり、ダウンロードせずに拡張機能の情報を表示したりすることができます。

### 特徴

- `extensions.json`からのリスト読み込み、またはID指定による単一拡張機能のダウンロードをサポート。
- サブコマンドが指定されない場合、デフォルトで`extensions.json`またはトップレベルのダウンロードオプションに基づいて拡張機能をダウンロード。
- `info`コマンドを使用して、ダウンロードせずに拡張機能の情報（バージョン、対応プラットフォームなど）を表示。
- 各拡張機能の最新バージョンをVSIXパッケージとしてダウンロード。
- プロキシ設定をサポート。
- ファイルが既に存在していても強制的に再ダウンロードするオプション。
- 詳細なログを表示するオプション。

### 前提条件

- システムに**Rust**と**Cargo**がインストールされていること。[rustup.rs](https://rustup.rs/)からインストールできます。

### インストール

```sh
cargo install vsixHarvest
```

### 使用方法

```sh
vsixHarvest [グローバルオプション] [コマンド] [コマンドオプション]
```

コマンドが指定されない場合、`vsixHarvester`はデフォルトで`download`動作を実行し、トップレベルで提供されたダウンロードオプションを使用します（例：`vsixHarvester --single <ID>`）。

#### グローバルオプション

- `--proxy <PROXY>`：HTTPリクエストに使用するプロキシURL（すべてのコマンドに適用）。
- `-v`, `--verbose`：詳細なログを表示します（すべてのコマンドに適用）。
- `-h`, `--help`：ヘルプ情報を表示。
- `-V`, `--version`：バージョン情報を表示。

#### コマンド

##### `download`

VSIXパッケージをダウンロードします。`download`コマンド自体に特定のダウンロードオプションが指定されていない場合、デフォルトで`./.vscode/extensions.json`を使用します。
コマンドが指定されていない場合、ダウンロードオプションをトップレベルで使用することもできます。

**`download`のオプション（およびトップレベルのデフォルト）：**

- `-i`, `--input <INPUT>`：`extensions.json`ファイルへのパス。デフォルトは`./.vscode/extensions.json`。
- `-d`, `--destination <DESTINATION>`：VSIXファイルを保存するディレクトリ。デフォルトは`./.vscode/extensions`。
- `-f`, `--force`：拡張機能ファイルが既に存在していても再ダウンロードします。
- `-a`, `--arch <ARCHITECTURE>`：拡張機能をインストールする対象OSアーキテクチャ（例：`win32-x64`）。下記の「アーキテクチャオプション」を参照。
- `-s`, `--single <EXTENSION_ID>`：拡張機能ID（例：`publisher.extensionName`）を指定して単一の拡張機能をダウンロードします。このオプションを使用する場合、`--input`は無視されます。

##### `info`

拡張機能の情報（最新バージョン、サポートされているプラットフォームなど）をダウンロードせずに表示します。

**`info`のオプション：**

- `-i`, `--input <INPUT>`：複数の拡張機能の情報を取得するための`extensions.json`ファイルへのパス。デフォルトは`./.vscode/extensions.json`。
- `-s`, `--single <EXTENSION_ID>`：拡張機能ID（例：`publisher.extensionName`）を指定して単一の拡張機能の情報を取得します。このオプションを使用する場合、`--input`は無視されます。

#### 使用例

**デフォルトの動作（`extensions.json`からのダウンロード）：**

```sh
vsixHarvester
```

または明示的に：

```sh
vsixHarvester download
```

**オプション付きで`extensions.json`からダウンロード（サブコマンドなし）：**

```sh
vsixHarvester --input ./path/to/your/extensions.json --destination ./output_dir --force -v
```

**単一の拡張機能をダウンロード（サブコマンドなし）：**

```sh
vsixHarvester -s publisher.extensionName -d ./vsix_files -f -v --arch win32-x64
```

**`download`サブコマンドを明示的に使用：**

```sh
vsixHarvester download -s publisher.extensionName -d ./vsix_files -f -v --arch win32-x64
```

**単一の拡張機能の情報を取得：**

```sh
vsixHarvester info -s publisher.extensionName
```

**`extensions.json`にリストされている拡張機能の情報を取得：**

```sh
vsixHarvester info --input ./path/to/your/extensions.json
```

またはデフォルトの`extensions.json`を使用：

```sh
vsixHarvester info
```

##### アーキテクチャオプション

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

[プラットフォーム固有の拡張機能について](https://code.visualstudio.com/api/working-with-extensions/publishing-extension#platformspecific-extensions)

### extensions.jsonの形式

`extensions.json`ファイルは以下の構造である必要があります：

```json
{
  "recommendations": [
    "publisher.extensionName",
    "anotherPublisher.anotherExtensionName",
    // 必要に応じて拡張機能を追加
  ]
}
```

この形式はVS Codeの[ワークスペース推奨拡張機能](https://code.visualstudio.com/docs/configure/extensions/extension-marketplace#_workspace-recommended-extensions)の仕様に従っています。`recommendations`配列には、ワークスペースを開いた際にVS Codeがインストールを推奨する拡張機能のIDが含まれます。

### 謝辞

- [offvsix](https://github.com/exaluc/offvsix) に影響を受けました。
