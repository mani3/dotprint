# DotPrint

BDF フォントを使用してテキストをピクセルアート風に表示するコマンドラインです。

## インストール

```bash
brew tap mani3/dotprint
brew install dotprint
```

## 使用方法

```bash
dotprint -h
Usage: dotprint <TEXT> [PIXEL] [SPACE]

Arguments:
  <TEXT>   Rendering text
  [PIXEL]  Rendering pixel character (default: "＠") [default: ＠]
  [SPACE]  Rendering space character (default: "　") [default: "\u{3000}"]

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## 例

```bash
dotprint "もじ" "⬛" "⬜"
⬜⬜⬛⬜⬜⬜⬜⬜⬛⬜⬛⬜⬛⬜⬜⬜
⬛⬛⬛⬛⬛⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜
⬜⬛⬜⬜⬜⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜
⬛⬛⬛⬛⬛⬜⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜
⬜⬛⬜⬜⬜⬛⬜⬜⬛⬜⬜⬜⬜⬜⬜⬜
⬜⬛⬜⬜⬜⬛⬜⬜⬛⬜⬜⬜⬛⬜⬜⬜
⬜⬜⬛⬛⬛⬜⬜⬜⬜⬛⬛⬛⬜⬜⬜⬜
```
