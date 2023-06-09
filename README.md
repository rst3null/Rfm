
# 任意精度演算ライブラリRFM
このライブラリは、pure rustで任意の精度持つ有理数の演算が可能なライブラリとなっています。

なお、本ライブラリではメモリの限界まで演算ができるように理論上は組まれていますが、
限界を越えた場合、panic!が発生する可能性があります。

高精度な演算を求める場合にお使いいただけますが、
厳密には上限値を規定していないことから予期しない不具合が発生する危険性があることを承知の上でご利用ください。

## 本ライブラリの利用について

本ライブラリはMITライセンスの制限を守っていただければご自由にご利用いただけます。

本ライブラリを利用するにあたっては利用者の責任において用いるものとし、
作者は一切保証しません。

- 本ライブラリは開発中です。
今後も開発を進めていきますが、開発の都合上予期しない破壊的変更を行う可能性があります。
また、多くの数学関数に対応していないため、本ライブラリは簡易的な計算のみにご利用いただける状況です。
上記についてご了承の上ご利用ください。
