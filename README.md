# Tree-sitter WASM Rust Example

クロスプラットフォーム言語処理ツールチェーンの手本実装

## 概要

このプロジェクトは、Tree-sitterを用いた最小構成の言語処理パイプラインです。
和音進行を表すミニ言語を、ブラウザとネイティブの両方で処理できます。

### ミニ言語仕様

- **入力**: 和音進行を `-` 区切りで並べた文字列（例: `C-F-G-C`）
- **トークン**: 音名（C, D, E, F, G, A, B）と区切り（-）
- **出力**: 各音名をCメジャースケールの度数に変換（C=1, D=2, E=3, F=4, G=5, A=6, B=7）
  - 例: `C-F-G-C` → `[1,4,5,1]`

## アーキテクチャ

```
           grammar.js
                |
        tree-sitter generate
                |
        +-------+--------+
        |                |
   parser.wasm        parser.c
        |                |
 web-tree-sitter   tree-sitter (native)
        |                |
      CST            CST
        \              /
         \            /
           AST (JSON)
                |
           Rust AST core
```

### 設計原則

1. **Tree-sitter必須**: すべてのパースはTree-sitterで実行（正規表現直接解析禁止）
2. **JSON AST中間表現**: Tree-sitter CSTをシンプルなJSONに変換してRustに渡す
3. **Rust共通処理**: ブラウザ（WASM）とネイティブで同じAST処理ロジックを使用
4. **最小構成**: 外部スキャナなし、純粋なgrammar.jsのみ

## ディレクトリ構成

```
.
├── grammar.js                    # Tree-sitter文法定義（共通）
├── package.json                  # Node.js依存関係
├── Cargo.toml                    # Rustワークスペース
├── src/
│   ├── rust/                     # Rust AST処理コア（WASM化可能）
│   │   ├── Cargo.toml
│   │   └── src/lib.rs           # AST処理・wasm-bindgen
│   ├── native/                   # ネイティブCLI
│   │   ├── Cargo.toml
│   │   └── src/main.rs          # CLIエントリーポイント
│   └── browser/                  # ブラウザ実装
│       ├── index.html           # UIページ
│       └── main.js              # web-tree-sitter統合
└── README.md
```

## 各コンポーネントの責務

### `grammar.js`
Tree-sitter文法定義。ブラウザ・ネイティブ両方で共通使用。
- `progression`: 1つ以上の音名を `-` で区切ったもの
- `note`: C, D, E, F, G, A, B のいずれか

### `src/rust/src/lib.rs`
Rust AST処理コア。
- `AstNode`: JSON AST構造体定義
- `note_to_degree()`: 音名→度数変換
- `process_ast()`: AST処理ロジック
- `process_chord_progression()`: WASM公開API

### `src/native/src/main.rs`
ネイティブCLI実装。
- Tree-sitterでパース（parser.c使用）
- CST→JSON AST変換
- Rust共通処理を呼び出し

### `src/browser/main.js`
ブラウザ実装。
- web-tree-sitterでパース（parser.wasm使用）
- CST→JSON AST変換
- Rust WASM共通処理を呼び出し

## ビルド方法

### 前提条件

```bash
# Node.js & npm
npm install

# Rust & cargo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# tree-sitter-cli
npm install -g tree-sitter-cli
```

### Tree-sitterパーサ生成

```bash
# パーサ生成（parser.c, parser.wasm）
npm run build:parser
```

### Rust Bindingの作成（ネイティブ用）

Tree-sitterが生成したC言語パーサをRustから呼び出すため、バインディングを作成します：

```bash
# tree-sitter-chordprogディレクトリを作成
mkdir -p tree-sitter-chordprog/src

# Cargo.tomlを作成
cat > tree-sitter-chordprog/Cargo.toml << 'EOF'
[package]
name = "tree-sitter-chordprog"
version = "0.1.0"
edition = "2021"

[lib]
path = "src/lib.rs"

[dependencies]
tree-sitter = "0.20"
cc = "1.0"

[build-dependencies]
cc = "1.0"
EOF

# build.rsを作成
cat > tree-sitter-chordprog/build.rs << 'EOF'
fn main() {
    let src_dir = std::path::Path::new("src");
    let parser_path = src_dir.join("parser.c");
    
    cc::Build::new()
        .include(src_dir)
        .file(&parser_path)
        .compile("tree-sitter-chordprog");
    
    println!("cargo:rerun-if-changed={}", parser_path.display());
}
EOF

# lib.rsを作成
cat > tree-sitter-chordprog/src/lib.rs << 'EOF'
use tree_sitter::Language;

extern "C" {
    fn tree_sitter_chordprog() -> Language;
}

pub fn language() -> Language {
    unsafe { tree_sitter_chordprog() }
}
EOF

# 生成されたparser.cをコピー
cp src/parser.c tree-sitter-chordprog/src/
```

### Rust WASM ビルド

```bash
# Rust WASM（ブラウザ用）
npm run build:wasm
```

### ネイティブCLIビルド

```bash
# ネイティブバイナリ
cargo build --release -p chord-cli
```

## 実行方法

### ブラウザ

```bash
# ローカルサーバで起動（例: Python）
cd src/browser
python3 -m http.server 8000

# ブラウザで http://localhost:8000 を開く
# テキストエリアに "C-F-G-C" を入力してParseボタンをクリック
# コンソールに [1,4,5,1] が出力される
```

### ネイティブ

```bash
# CLIで実行
cargo run --release -p chord-cli -- "C-F-G-C"
# 出力: 1,4,5,1

# 別の例
cargo run --release -p chord-cli -- "C-D-E-F-G-A-B"
# 出力: 1,2,3,4,5,6,7
```

## テスト

```bash
# Rustユニットテスト
cargo test -p chord-processor
```

## なぜTree-sitterを使うのか

このミニ言語は単純ですが、以下の理由でTree-sitterを使用します：

1. **拡張性**: 将来的な文法拡張（コード、マイナー等）に対応しやすい
2. **クロスプラットフォーム**: 同じgrammar.jsからブラウザ・ネイティブ両対応
3. **エラー耐性**: Tree-sitterの増分パース・エラー回復機能を活用可能
4. **設計の手本**: 実用的なツールチェーン構築のパターンを示す

## ライセンス

MIT