# Rust_timer

Rustを学習しながら開発する、Dioxus Desktop製のタイマーです。

2026-11-08に、Downloads内の`rust_timer-source-20261023-180550.zip`から移行しました。元のZIPはDownloadsに残しています。

## 開き方と起動

VS Codeでは、このREADMEがある`Rust_timer`フォルダーを開いてください。F5で「Dioxus Desktopを起動」を選択できます。

ターミナルでは次のコマンドで起動します。

```sh
cd /Users/YukiKomatsu/Development/Learning/Rust_timer
dx serve --desktop
```

ビルド時のエラー確認は、同じフォルダーで実行します。

```sh
cargo check --locked
```

## 開発環境

- macOS / Apple Silicon
- Rust・Cargo 1.99.0（stable / aarch64-apple-darwin）
- Xcodeの開発ツール
- Dioxus CLI 0.7.10（`dx`）

移行元のCargo.tomlとCargo.lockを引き継いでいます。現在のUIはDioxusで、React・TypeScript・Tauriへの変更は行っていません。

## 現在の実装

- 5秒の確認用タイマー。開始・停止・リセットと、経過時間・終了状態の表示。
- タスクトレイの「表示」「終了」メニュー。
- ウィンドウを閉じると非表示になる設定。

これは移行したソースの構成を示す説明です。アプリの実装は学習のため本人が行います。

## AIによる学習支援

AIは質問への回答・説明・調査・レビューのみを行い、アプリの実装・修正は本人が行います。回答内のコード例も、明示的に求めた場合に限ります。

詳細は[AGENTS.md](AGENTS.md)を参照してください。

## 移行元の資料

- [Windows向け移行手順](docs/移行手順-Windows.md)：ZIPに含まれていた原文です。macOSでの起動方法は本READMEを参照してください。
- [Dioxusの元ガイド](docs/Dioxus-AGENTS.original.md)：ZIP内のAGENTS.mdを参考資料として保存しています。
- [Dioxus公式の環境構築手順](https://dioxuslabs.com/learn/0.7/getting_started/)

ソースコードと依存バージョンは元のまま引き継ぎ、VS Codeの起動設定はこのフォルダーを直接開く構成に合わせています。

## 移行時の確認（2026-11-08）

- `cargo check --locked`：成功。既存コードに未使用importなどの警告が8件あります。
- `dx build --desktop --locked`：成功。初回ビルドでTailwind CSSのツールと生成CSSも準備されました。
- macOS用の`target/dx/rust_timer/debug/macos/RustTimer.app`を起動し、プロセスが継続して動くことを確認しました。画面のボタン操作は未確認です。
- `src/`、Cargo.toml、Cargo.lock、既存アセットと保存した移行元資料が、ZIP内の内容と一致することを確認しました。
