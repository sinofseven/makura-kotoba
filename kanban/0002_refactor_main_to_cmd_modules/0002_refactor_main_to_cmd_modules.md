# main.rsをcmdモジュールにリファクタリング

## 目的
main.rsをシンプルにし、各処理をわかりやすくしたい。

## 要望
`src/cmd`ディレクトリを掘って、applyとinitを別ファイルに分けてください

## プラン
1. src/cmd/mod.rs を作成し、ApplyArgs/InitArgs 構造体と pub mod を定義
2. src/cmd/apply.rs を作成し、apply_command 関数を実装
3. src/cmd/init.rs を作成し、init_command 関数を実装
4. src/main.rs を更新し、Commands enum を引数対応にして cmd モジュールを呼び出し

将来的な拡張を考慮し、apply/init それぞれに独立した Args 構造体を用意することで、後から新しいコマンドラインオプションを追加できる設計にした。

## 完了サマリー
2026-05-21T14:15:00+09:00 完了

main.rs をシンプルに（CLI パースとコマンドディスパッチのみ）し、apply と init の実装を独立したファイルに分離。
- src/cmd/mod.rs: モジュール管理と Args 構造体定義
- src/cmd/apply.rs: apply コマンド実装
- src/cmd/init.rs: init コマンド実装

すべてのコマンドが正常に動作確認済み。将来的な引数追加に対応可能な設計。
