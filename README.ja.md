# vsixHarvester

## Rust製のVSCode拡張機能ダウンローダー

このRustプログラムは、`extensions.json`ファイルから`recommendations`配列を読み取り、対応するVisual Studio Code拡張機能のVSIXパッケージをダウンロードします。

### 特徴

- `extensions.json`から拡張機能のリストを読み込む。
- 各拡張機能の最新バージョンをVSIXパッケージとしてダウンロード。
- プロキシ設定をサポート。
- ファイルが既に存在していても再ダウンロード可能。
- 詳細なログを表示するオプション。
- 拡張機能IDを指定して単一の拡張機能をダウンロード。

### 前提条件

- システムに**Rust**と**Cargo**がインストールされていること。[rustup.rs](https://rustup.rs/)からインストールできます。

### インストール

```sh
cargo install vsixHarvest
```

### 使用方法

```sh
vsixHarvest [OPTIONS]
```

#### オプション

- `-i`, `--input <INPUT>`：`extensions.json`ファイルへのパス。デフォルトは`./.vscode/extensions.json`。
- `-d`, `--destination <DESTINATION>`：VSIXファイルを保存するディレクトリ。デフォルトは`./.vscode/extensions`。
- `-f`, `--force`：拡張機能ファイルが既に存在していても再ダウンロードします。
- `--proxy <PROXY>`：HTTPリクエストに使用するプロキシURL。
- `-v`, `--verbose`：詳細なログを表示します。
- `-h`, `--help`：ヘルプ情報を表示。
- `--arch <ARCHITECTURE>`：拡張機能をインストールする対象OSアーキテクチャ。
- `-s`, `--single <EXTENSION_ID>`：拡張機能ID（例: `publisher.extensionName`）を指定して単一の拡張機能をダウンロードします。このオプションを使用する場合、`--input` は無視されます。

#### 使用例

```sh
vsixHarvest \
  --input ./your/path/to/extensions.json \
  --destination ./your/path/to/extensions \
  --force \
  --arch win32-x64 \
  --verbose
```

単一の拡張機能をダウンロードする場合:

```sh
vsixHarvester \
  --single publisher.extensionName \
  --force \
  --arch win32-x64 \
  --destination ./extensions \
  --verbose
```

##### アーキテクチャオプション

- `win32-x64`
- `win32-arm64`
- `darwin-x64`
- `darwin-arm64`
- `linux-x64`
- `linux-arm64`

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

### 謝辞

- [offvsix](https://github.com/exaluc/offvsix) に影響を受けました。
