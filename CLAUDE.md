# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`makura-kotoba` は Rust 2024 edition で書かれた CLI ツールです。

コンフィグディレクトリの設定ファイル（`~/.config/makura-kotoba/config.toml`）で定義した環境変数を、指定したコマンドの実行時に自動で付与するためのものです。

以下の2つのサブコマンドを持ちます。

- **apply** — 環境変数を付与して任意のコマンドを実行するための、各シェル用ラッパースクリプトを生成する。
- **init** — 各シェルの `preexec` フックに登録するためのシェルスクリプトを生成する。

## Commands

```bash
# ビルド
cargo build

# 実行
cargo run

# テスト実行
cargo test

# 特定のテスト実行
cargo test <test_name>

# リント
cargo clippy

# フォーマット
cargo fmt

# フォーマット確認（変更なし）
cargo fmt --check
```

## Architecture

- `src/main.rs` — エントリーポイント。現在は単一ファイル構成。
- 依存クレートなし（`[dependencies]` は空）。

## 開発ワークフロー

このプロジェクトでは Claude Code プラグイン **`kanban-kit`** を使ってタスク管理・開発を行う。

### タスクの作成

`/add-kanban` スキルで新規タスクを作成する。タスクファイルは `kanban/{xxxx}_{title}/{xxxx}_{title}.md` に置かれ、以下の構造で記述する。

```markdown
# タイトル

## 目的
（なぜこの作業が必要か — 背景・動機・ゴール）

## 要望
（具体的に何をどうしてほしいか）
```

- `## 目的` セクションは**必須**。
- `xxxx` は4桁0パディング連番（例: `0001`）。

### タスクの実行

`/kanban` スキル（または `/kanban <番号>` で指定）でタスクを実行する。

1. **フェーズ1 — 調査・プランニング**: コードを調査し、プランモードで実装計画を提示してユーザーの承認を得る。
2. **フェーズ2 — 実装**: 承認されたプランに従って実装し、`kanban/{xxxx}_{title}/log.md` に段階的に作業ログを記録する。
3. **完了**: kanban ファイルに `## 完了サマリー` を追記してタスクを完了とする。

args 未指定の場合、未完了タスク（`## 完了サマリー` を含まないもの）のうち番号が最大のものが自動選択される。
