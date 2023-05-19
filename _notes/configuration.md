# 設定ファイルについて
- 以下のコマンドで、プロジェクト直下にシンボリックリンクを作成した。
- これで、エイリアスを変更することで、オリジナルにも反映されることを確認。
```sh
ln -s ~/Library/Application\ Support/cargo-atcoder.toml ./
```

- `[dependencies]` 以下に依存するパッケージを指定
    - 各コンテストの Cargo.toml にコピーされる。
- `template = ` 以下にテンプレートを記述。

- シンボリックリンクの git 管理がうまくいかなさそうなので，ファイルを開くコマンドを置いておく
code ~/Library/Application\ Support/cargo-atcoder.toml 