# iso-cubeviz 仕様書 v1.0（最終確定版）

**作成日**: 2026-06-09  
**目的**: Webサイト埋め込み用のルービックキューブ画像を、任意の状態からSVGに変換するコンパイラ型CLIツール  
**バイナリ名**: `iso-cubeviz`

---

## 1. 概要

`iso-cubeviz` は、**任意サイズ（N×N×N）のルービックキューブ**の状態を直接指定し、**3つの面が見える angled view** で表現した高品質な **SVG** を出力するRust製CLIツールです。

回転記号の適用機能はv1.0では持ちません。状態は直接指定する方式を採用します。

---

## 2. 機能要件（v1.0で確定）

- 任意のN（3以上）に対応
- 状態の直接指定（6色＋黒＋灰色の計8色）
- 3種類のビューアングルから選択可能
- SVG出力（stdout / ファイル）
- 境界線・厚み効果・背景のスタイルをオプションで選択可能

---

## 3. コマンドラインインターフェース

```bash
iso-cubeviz [OPTIONS]
```

### オプション一覧（全て確定）

| オプション                    | 説明                                           | 必須 | デフォルト     |
|-------------------------------|------------------------------------------------|------|----------------|
| `-s, --state <STATE>`         | 状態を直接指定（後述のフォーマット）           | 必須 | -              |
| `--state-file <PATH>`         | 状態ファイルを指定（--stateと排他）            | -    | -              |
| `--view <MODE>`               | ビューアングル（balanced / top / side）        | -    | `balanced`     |
| `--border <STYLE>`            | 境界線の太さ（thin / normal / thick）          | -    | `normal`       |
| `--thickness <BOOL>`          | キューブに厚みを持たせる（true / false）       | -    | `true`         |
| `--background <STYLE>`        | 背景（transparent / light / white）            | -    | `transparent`  |
| `-o, --output <FILE>`         | 出力ファイル（未指定時はstdout）               | -    | stdout         |
| `-h, --help`                  | ヘルプ                                         | -    | -              |
| `-V, --version`               | バージョン                                     | -    | -              |

### 使用例

```bash
# 基本（3x3、balancedビュー）
iso-cubeviz -s "size: 3\nU: WWWWWWWWW\nF: GGGGGGGGG\nR: RRRRRRRRR" -o cube.svg

# 5x5 + topビュー + 厚みなし + 細い境界線
iso-cubeviz --state-file state.txt --view top --thickness false --border thin -o big.svg

# パイプ出力
iso-cubeviz --state-file state.txt | cat
```

---

## 4. 状態指定フォーマット（v1.0確定）

### 基本形式（テキスト）

```
size: <N>
U: <色N×N文字>
F: <色N×N文字>
R: <色N×N文字>
```

描画する3面（U/F/R）のみ指定する。省略した面は灰色（H）で初期化される。

### 色コード（8色）

| コード | 色     | HEXコード   | 用途             |
|--------|--------|-------------|------------------|
| W      | White  | `#FFFFFF`   | 上面             |
| Y      | Yellow | `#FFD500`   | 下面             |
| G      | Green  | `#009B48`   | 正面             |
| B      | Blue   | `#0046AD`   | 背面             |
| R      | Red    | `#B71234`   | 右面             |
| O      | Orange | `#FF5800`   | 左面             |
| K      | Black  | `#1A1A1A`   | 任意             |
| H      | Gray   | `#808080`   | 任意             |

### 例（3x3）

```
size: 3
U: WWWWWWWWW
F: GGGGGGGGG
R: RRRRRRRRR
```

### 例（4x4）

```
size: 4
U: WWWWWWWWWWWWWWWW
F: GGGGGGGGGGGGGGGG
R: RRRRRRRRRRRRRRRR
```

**補足**:
- 各面の色文字列は **左上から右下** へ1行で記述
- 読みやすさのために色文字列中に空白文字（スペース・タブ）を入れてもよい（パース時に無視する）
- `--state` で直接渡すか、`--state-file` でファイルから読み込む
- 不正な文字・長さが合わない場合はエラー終了

---

## 5. ビューアングル（3パターン確定）

| モード     | 名前          | 特徴                                   | 用途例             |
|------------|---------------|----------------------------------------|--------------------|
| `balanced` | Balanced      | 上面・正面・右面がバランスよく見える   | デフォルト・汎用   |
| `top`      | Top-heavy     | 上面をより大きく・強調して見せる       | 上面の模様重視     |
| `side`     | Side-heavy    | 正面と右面をより大きく見せる           | 側面の模様重視     |

- 細かい角度調整はv1.0では行わない
- 各モードは内部で異なる投影オフセットを使用
- どのモードでも**必ず3つの面**が見える構成とする

---

## 6. SVG出力仕様（確定）

### 基本要件
- 自己完結したSVG（`viewBox`付き）
- ステッカーは**平行四辺形**で描画
- 3つの面（U / F / R）を表示
- ファイルサイズはできるだけコンパクトに

### スタイルオプションの反映

- `--border`:
  - `thin`: 細い境界線
  - `normal`: 標準的な境界線
  - `thick`: はっきり目立つ境界線

- `--thickness`:
  - `true`: キューブに厚みを持たせ、側面も軽く描画（立体感を出す）
  - `false`: ステッカーのみ（フラット）

- `--background`:
  - `transparent`: 背景透明
  - `light`: 薄いグレー背景
  - `white`: 白背景

### その他の描画ルール
- ステッカー間の隙間は最小限に
- 各ステッカーは単色塗りつぶし＋境界線
- 面の重なり順は適切に描画（奥→手前）
- `<title>` と `<desc>` に簡単な説明を入れる

---

## 7. データモデル（実装向け）

```rust
pub enum Color { W, Y, G, B, R, O, K, H }

pub struct Cube {
    pub size: usize,
    pub faces: [Vec<Vec<Color>>; 3], // 0:U, 1:F, 2:R
}

pub enum ViewMode { Balanced, Top, Side }
pub enum BorderStyle { Thin, Normal, Thick }
pub enum BackgroundStyle { Transparent, Light, White }
```

---

## 8. エラーハンドリング（確定）

- 不正なサイズ（2以下など）→ エラーメッセージを出して終了
- 状態文字列の長さが合わない → 明確なエラー
- 不明な色コード → エラー
- ファイル読み込み失敗 → 標準的なエラーメッセージ

---

## 9. 非機能要件

- Rustで実装
- 依存は最小限に抑える（`clap` は使用可）
- N=10程度までは快適に動作
- SVGはモダンブラウザで問題なく表示される品質

---

## 10. 実装に関する推奨（参考）

- 状態パースはシンプルに自前で実装可能
- SVG生成は文字列構築で十分（`svg` crateは必須ではない）
- 投影計算は各ビューごとに定数でオフセットを切り替える形が最も簡単
- テストは「状態→SVGの整合性」ではなく「パースの正確さ」と「出力SVGの基本構造」で十分

---

**この仕様書はv1.0として完全に確定**しています。