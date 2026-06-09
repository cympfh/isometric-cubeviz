## [x] Bug: svg が想定通りではない [2026-06-09 14:33 完了]

```
size: 3
U: RRR RRR RRR
F: BBB BBB BBB
R: GGG GGG GGG
```

の出力結果が /tmp/center.png です。
/tmp/center.png をよく見てみて。

## [x] README を書く [2026-06-09 14:45 完了]

最低限の使い方を簡単に書いて。
絵文字禁止。過剰な宣伝禁止。

## [x] --thickness false でOK [2026-06-09 14:48 完了]

--thickness というオプションを消す。
常に今で言うとこの false 扱いの挙動で固定する

## [x] state file をもっと自由に書けるよう [2026-06-09 15:54 完了]

今一行で書かないといけない

```
size: 3
U: RRR
RRR
RRR
F: BBBBBBBBB
R:
GGG
GGG
GGG
```

自由に改行させてほしい
