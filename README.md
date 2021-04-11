# Rust Shell

Rust で自作シェルを作ってみたいよ！( ･ㅂ･)و̑ ｸﾞｯ

## Execute

* run

```shell
$ cargo run
```

* build

```shell
$ cargo build
$ ./target/debug/rshell
```

## 現在動くコマンド

* `ls`

カレントディレクトリ直下にあるファイルたちを出力する。

```shell
sudachi@DaiMac.local:/Users/sudachi/projects/rshell $ ls
(debug) ["ls"]
./Cargo.toml
./target
./Cargo.lock
./README.md
./.gitignore
./.git
./src
```

* `cd [dir]`

ワーキングディレクトリを変更する。

```shell
sudachi@DaiMac.local:/Users/sudachi/projects/rshell $ cd /
(debug) ["cd", "/"]
sudachi@DaiMac.local:/ $ ls
```

`cd` コマンドは、引数を複数渡しても最初の引数だけ処理されるっぽい (Macbook の bash で実行してみて検証)
本家に倣って、`cd` の次に来る 1つ目の引数に指定されたディレクトリに移動するようにした。

なお、Tabによる補完機能はないので、全て素手で入力する必要がある
(FIXME..🔥)


* `exit`

シェルから `exit` する。

```shell
sudachi@DaiMac.local:/ $ exit
(debug) ["exit"]
```

`exit` コマンドも、引数を与えても最初の `exit` が実行されるよう..
なので、引数の解析などは行わず、`exit` が最初のコマンドとして渡されたら `return` してプロセスを終了する形にした。


## Links
* [Rustで作った自作Shellの話 (原作)](https://zenn.dev/garebare/articles/a463257c447fa9)
  - [repository](https://github.com/garebareDA/g_shell)

### `std::env`
* [Rustでカレントディレクトリを取得・表示・変更する](https://qiita.com/wildmouse/items/e417a807a93e77c46584)
* [Rustでファイルの一覧を取得してみる](https://note.katsumataryo.com/tech/2019/09/1452.html)

### error
* [Rustのエラー処理](https://qiita.com/fujitayy/items/cafe661415b6aa33d884)

### std::fs
* [Rust でフォルダ内にあるすべてのファイル名を取得するのが遅い（それほど遅くない）](https://qiita.com/benki/items/70ad2ee44cff9efde778)
* [ファイルシステムとのやり取り](https://doc.rust-jp.rs/rust-by-example-ja/std_misc/fs.html)
