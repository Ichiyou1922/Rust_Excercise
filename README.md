# Rust_Excercise

## 環境とエコシステム
- `rustup`: マルチプレクサ．Rustのツールチェーン(コンパイラ`rustc`，パッケージマネージャ`cargo等`)のバージョン管理を行う．
- `cargo`: ビルドシステム兼パッケージマネージャ．依存関係の解決，コンパイル，テスト，ドキュメント生成を一手に担う．C++における`make`+`cmake`+`conan`+`doxygen`を統合した存在であり，プロジェクトの宣言的定義を可能にする．

## 束縛と普遍性
- Rustにおいて`let x = 5`は「代入」ではなく**「束縛(Binding)」**と呼ばれる．
- 数学的な定義x:=5に近く，スコープ内での普遍の事実を宣言する．
- これはデフォルトで「不変（Immutable）」．
- `Default: x -> y`（変更不可）
- `Mutable: let mut x -> v`（変更可能）
- 状態が変わることを特権的な操作として命じさせる（`mut`）ことで，データ競合の発生源を局所化する．

## インストール
- 公式からrustをインストール
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- PATHを通す
```bash
source $HOME/.cargo/env
```


```
```
```
```
