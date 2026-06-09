# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクト概要

`iso-cubeviz` — 任意サイズ（N×N×N）のルービックキューブ状態をSVGに変換するRust製CLIツール。仕様は `iso-cubeviz-spec-v1.0.md` に確定済み。

## ビルド・実行

```bash
cargo build
cargo run -- -s "size: 3\nU: WWWWWWWWW\nF: GGGGGGGGG\nR: RRRRRRRRR"
cargo run -- --state-file state.txt -o cube.svg
```

## テスト・チェック

```bash
cargo test
cargo clippy
cargo fmt
```

単一テスト実行:
```bash
cargo test <test_name>
```

## アーキテクチャ

実装すべきモジュール構成（仕様書 §7 に基づく）:

- **`main.rs`** — CLI引数パース（`clap`使用）、エントリポイント
- **`model.rs`** — `Color`, `Cube`, `ViewMode`, `BorderStyle`, `BackgroundStyle` の型定義
- **`parser.rs`** — 状態文字列・ファイルのパース（`size:` + U/F/R 3面の色文字列）
- **`svg.rs`** — 等角投影SVG生成（面ごとの平行四辺形描画、奥→手前の順序）

## 状態フォーマット（`--state` / `--state-file`）

```
size: 3
U: WWWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR
```

描画する3面（U/F/R）のみ指定する。面の順序: `0:U, 1:F, 2:R`。各面は左上→右下で N×N 文字。色文字列中の空白文字（スペース・タブ）はパース時に無視する。

## 色コード

| コード | HEX      |
|--------|----------|
| W      | #FFFFFF  |
| Y      | #FFD500  |
| G      | #009B48  |
| B      | #0046AD  |
| R      | #B71234  |
| O      | #FF5800  |
| K      | #1A1A1A  |
| H      | #808080  |

## SVG生成の要点

- ビューモード3種（`balanced` / `top` / `side`）は投影オフセット定数で切り替え
- 表示する3面: U（上）・F（正面）・R（右）
- ステッカーは平行四辺形、`--thickness true` 時はキューブ側面も描画
- 描画順: 奥→手前（奥のステッカーが手前に隠れる）
- SVG crateは不要、文字列構築で生成
