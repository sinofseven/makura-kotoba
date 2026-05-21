# traitとモジュールスコープの問題修正

## 目的
効率的に書くために、Args用のstructにtraitを設定し、main.rsでtraitをuseしているのに何故かtraitで定義した関数を認識しない。何故認識しないのか教えてください。その上で修正してください。

## 要望
cargo checkでエラーが出たから修正して。

### エラー内容
```
at 17:23:52 ❯ cargo check
    Checking makura-kotoba v0.1.0 (/Users/yuta/space/rust/makura-kotoba)
warning: unused import: `traits::cmd::Cmd`
 --> src/main.rs:8:5
  |
8 | use traits::cmd::Cmd;
  |     ^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

error[E0433]: cannot find module or crate `args` in this scope
  --> src/main.rs:29:34
   |
29 |         Commands::Apply(args) => args::run(),
   |                                  ^^^^ use of unresolved module or unlinked crate `args`
  |
  = help: if you wanted to use a crate named `args`, use `cargo add args` to add it to your `Cargo.toml`

error[E0433]: cannot find module or crate `args` in this scope
  --> src/main.rs:30:33
   |
30 |         Commands::Init(args) => args::run(),
   |                                 ^^^^ use of unresolved module or crate `args`
  |
  = help: if you wanted to use a crate named `args`, use `cargo add args` to add it to your `Cargo.toml`

For more information about this error, try `rustc --explain E0433`.
warning: `makura-kotoba` (bin "makura-kotoba") generated 1 warning
error: could not compile `makura-kotoba` (bin "makura-kotoba") due to 2 previous errors; 1 warning emitted
```

## プラン

### 問題分析
エラーの原因は、main.rs の 29-30行目で `args::run()` と記述していること。

**根本原因の説明：**
- `::` はモジュールスコープを表現する記号。例えば `std::collections::HashMap` のように、モジュール内の関数やアイテムにアクセスします
- 一方で、変数 `args` は ApplyArgs/InitArgs 構造体のインスタンスです
- このインスタンスに対して Cmd トレイトで定義された `run()` メソッドを呼び出したい場合は、ドット記法 `.` を使用します
- 正しくは `args.run()` と記述すべき

**なぜ `Cmd` トレイトが未使用扱いされるのか：**
- `use traits::cmd::Cmd;` でトレイトをimportしていますが、実際のメソッド呼び出しが間違った構文（`args::run()`）であるため、コンパイラはこのトレイトが使用されていないと判定します
- 修正後は `args.run()` でトレイトのメソッドが正しく呼ばれるため、未使用警告も消えます

### 修正内容

**ファイル:** src/main.rs

**変更前**
```rust
match cli.command {
    Commands::Apply(args) => args::run(),
    Commands::Init(args) => args::run(),
}
```

**変更後**
```rust
match cli.command {
    Commands::Apply(args) => args.run(),
    Commands::Init(args) => args.run(),
}
```

変更内容：`::`（モジュールスコープ） → `.`（メソッド呼び出し）

## 完了サマリー

**完了日時:** 2026-05-21T23:00:00+09:00

修正により、すべてのエラーと警告が解決されました：

✅ `cargo check` — エラーと元の `unused import` 警告が消滅
✅ `cargo build` — ビルド成功
✅ `cargo run -- apply --help` — サブコマンド動作確認OK
✅ `cargo run -- init --help` — サブコマンド動作確認OK

このシンプルな構文修正（`::`→`.`）により、Rust のメソッド呼び出しとモジュールスコープの正しい使い分けが実現できました。
