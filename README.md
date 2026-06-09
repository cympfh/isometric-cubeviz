# iso-cubeviz

N×N×N のルービックキューブ状態を等角投影 SVG に変換する CLI ツール。

## インストール

```
cargo build --release
```

`target/release/iso-cubeviz` が生成される。

## 使い方

```
iso-cubeviz [OPTIONS]
```

### オプション

| オプション | 説明 | デフォルト |
|---|---|---|
| `-s`, `--state <STATE>` | 状態文字列を直接指定 | |
| `--state-file <FILE>` | 状態ファイルを指定 | |
| `-o`, `--output <FILE>` | 出力先 SVG ファイル（省略時は stdout） | |
| `--view <VIEW>` | ビューモード: `balanced` / `top` / `side` | `balanced` |
| `--border <BORDER>` | ボーダー太さ: `thin` / `normal` / `thick` | `normal` |
| `--thickness <BOOL>` | キューブ側面の厚みを描画するか | `true` |
| `--background <BG>` | 背景: `transparent` / `light` / `white` | `transparent` |

## 状態フォーマット

表示する 3 面（U: 上、F: 正面、R: 右）を指定する。

```
size: 3
U: WWWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR
```

- `size` は 1 以上の整数
- 各面は左上から右下に向かって N×N 個の色コードを列挙
- 空白・タブは無視される

### 色コード

| コード | 色 | HEX |
|---|---|---|
| W | 白 | #FFFFFF |
| Y | 黄 | #FFD500 |
| G | 緑 | #009B48 |
| B | 青 | #0046AD |
| R | 赤 | #B71234 |
| O | 橙 | #FF5800 |
| K | 黒 | #1A1A1A |
| H | 灰 | #808080 |

## 例

```sh
# 引数で直接指定
iso-cubeviz -s "size: 3
U: WWWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR" -o cube.svg

# ファイルから読み込み
iso-cubeviz --state-file state.txt -o cube.svg

# ビューモードを変更
iso-cubeviz --state-file state.txt --view top -o cube_top.svg
```
