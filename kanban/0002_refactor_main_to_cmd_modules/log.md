# タスク 0002 実装ログ

**開始時刻:** 2026-05-21T14:00:00+09:00

## タスク概要
`src/cmd`ディレクトリを掘って、applyとinitを別ファイルに分けてください。
main.rsをシンプルにし、各処理をわかりやすくしたい。

## 調査結果

### 現在の main.rs の構造
- clap で CLI パーサーを定義
- `Cli` 構造体が Commands enum を持つ
- Commands enum: Apply, Init の2つのサブコマンド
- main() 関数でコマンドマッチして apply_command() または init_command() を呼び出し
- 両コマンド関数は現在プレースホルダー（println のみ）

### 依存関係
- clap (version: [dependencies] は空だが Cargo.lock に記載)
- 他に外部依存なし

## 実装プラン

### ディレクトリ構造（目標）
```
src/
  main.rs          - エントリーポイント（CLI パース + コマンド呼び出し）
  cmd/
    mod.rs         - サブモジュール（apply と init を re-export）
    apply.rs       - apply コマンドの実装
    init.rs        - init コマンドの実装
```

### 実装ステップ
1. src/cmd/mod.rs 作成 - 各コマンド用 struct 定義・モジュール宣言
2. src/cmd/apply.rs 作成 - apply_command 実装
3. src/cmd/init.rs 作成 - init_command 実装
4. src/main.rs 更新 - Commands enum を引数構造体対応に変更・モジュール呼び出し

### プランニング経緯
初回提案：シンプルなモジュール分割
ユーザーフィードバック：「将来argsを追加できるようにしてください」
最終プラン：Commands enum を構造体対応にして、将来的な拡張に対応可能な設計に変更

## 実装フェーズ

### ステップ1: src/cmd/mod.rs 作成

実装内容：
- ApplyArgs, InitArgs 構造体を定義（空でも clap Parser 対応）
- apply, init モジュールを pub mod 宣言
- apply_command, init_command を public として再エクスポート

### ステップ2: src/cmd/apply.rs 作成

実装内容：
- pub fn apply_command(args: ApplyArgs) を実装
- プレースホルダー処理を保持

### ステップ3: src/cmd/init.rs 作成

実装内容：
- pub fn init_command(args: InitArgs) を実装
- プレースホルダー処理を保持

### ステップ4: src/main.rs 更新

実装内容：
- mod cmd; 追加
- Commands enum 修正（ApplyArgs, InitArgs 対応）
- main() 関数内のコマンド呼び出しを更新
- 古い apply_command, init_command 関数削除

### コンパイルと動作確認

実行項目：
- `cargo build` でコンパイル成功確認
- `cargo run -- apply` と `cargo run -- init` の動作確認
- `cargo test` 実行（テストが存在すれば）

## 実装詳細

### ステップ1: src/cmd/mod.rs 作成
- ApplyArgs, InitArgs 構造体を定義（clap Parser derive）
- apply, init モジュールを pub mod で宣言
- apply_command, init_command を public 関数として再エクスポート

### ステップ2: src/cmd/apply.rs 作成
- pub fn apply_command(args: ApplyArgs) を実装
- プレースホルダー処理を保持："apply subcommand called" を出力

### ステップ3: src/cmd/init.rs 作成
- pub fn init_command(args: InitArgs) を実装
- プレースホルダー処理を保持："init subcommand called" を出力

### ステップ4: src/main.rs 更新
- `mod cmd;` を追加してモジュール宣言
- cmd から ApplyArgs, InitArgs, apply_command, init_command をインポート
- Commands enum を修正：Apply(ApplyArgs), Init(InitArgs) に変更
- main() 関数内で args をマッチして関数に渡すように変更
- 古い apply_command(), init_command() 関数定義を削除

### 動作確認結果
- `cargo build` - 成功（warning なし、Cargo.toml の license 設定による警告はあるが別件）
- `cargo run -- apply` - 成功："apply subcommand called" を出力
- `cargo run -- init` - 成功："init subcommand called" を出力
- `cargo test` - 成功：0 tests passed（テスト自体は未実装）

## 完了日時
2026-05-21T14:15:00+09:00
