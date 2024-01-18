# OCaml のプロジェクト管理ツール dune を使う
[公式サイト](https://scrapbox.io/ohbarye/Dune)

プロジェクトを作成
```
$ opam init -y
$ opam switch
$ opam switch create 5.0.0 
$ eval `opam config env`
```

dune をインストール
```
opam update
opam install dune
dune init project hello_world
```
