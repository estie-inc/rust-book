# 書籍「バックエンドエンジニアを目指す人のための Rust」収録ソースコード

[バックエンドエンジニアを目指す人のための Rust (翔泳社)](https://www.shoeisha.co.jp/book/detail/9784798186016)に収録されているソースコードを公開しています。
学習にお役立ていただければ幸いです。

## ソースコードのダウンロード
https://github.com/estie-inc/rust-book/archive/refs/heads/main.zip をダウンロードしてお好きな場所に展開してください。

gitが使える場合は、以下のコマンドを実行してダウンロードすることもできます。
```sh
$ git clone https://github.com/estie-inc/rust-book
```

## 動かし方
このリポジトリは、Cargoの[ワークスペース](https://doc.rust-jp.rs/book-ja/ch14-03-cargo-workspaces.html)という仕組みを使って、複数のパッケージをまとめて管理しています。

それぞれのパッケージを実行するには、`cargo run -p`のあとにパッケージ名を指定して次のように実行してください。

```sh
$ cargo run -p hello_world 
Hello, world!

Hello, Cargo!
```
