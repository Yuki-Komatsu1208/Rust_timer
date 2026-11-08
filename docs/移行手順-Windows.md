# Rustタイマー 移行用ソース

## 開き方

ZIPを展開し、`.vscode`と`rust_timer`が並んでいるフォルダをVS Codeで開いてください。
F5で「Dioxus Desktopを起動」を選択すると起動できます。
ターミナルの場合は、`rust_timer`へ移動して `dx serve --desktop` を実行します。

## 移行先PCに必要な環境（Windows）

- Rust（MSVCツールチェーン）とCargo
- Microsoft C++ Build Toolsの「C++によるデスクトップ開発」
- Microsoft Edge WebView2 Runtime
- Dioxus CLI 0.7.10（移行元で確認したバージョン）

Dioxus CLIの導入コマンド：

```powershell
cargo install dioxus-cli --version 0.7.10 --locked
```

プロジェクトの実行：

```powershell
cd rust_timer
dx serve --desktop
```

初回は依存ライブラリの取得とビルドのためネット接続が必要です。
Windowsの準備：https://dioxuslabs.com/learn/0.7/getting_started/

## 含めたもの

- Rustソース（src）
- Cargo.toml / Cargo.lock（依存バージョン固定）
- Dioxus設定、使用中のmain.css、Tailwindの入力ファイル
- VS CodeのF5起動設定
- AGENTS.md、Clippy設定、.gitignore

## 除外したもの

- target、実行ファイル、ビルドキャッシュ、Git履歴
- 生成済みassets/tailwind.css（必要時にdxが再生成）
- 現在のソースで参照していないひな形画像（favicon.ico、header.svg）

## 現在の実装と学習の続き

- Dioxus Desktop 0.7.10。Cargo.tomlの指定は0.7.1で、Cargo.lockに解決済みバージョンを保存。
- タスクトレイ常駐。×は非表示、「表示」で再表示、「終了」で終了。
- メニューイベントはuse_muda_event_handlerで受け取る。
- Timer構造体にstart / stop / reset / is_running / accumulated / elapsed / check_finishedを実装。
- 現在は5秒の確認用タイマー。時間切れでリセットし、画面とログに完了を表示。
- 作業・休憩時間の入力、フェーズ切替、赤・黒の横長進捗バー、OS通知は今後の作業。

このプロジェクトはRustの学習用です。AIには説明とコード例を依頼し、実装・編集は本人が行う方針です。
ファイルの読み取りによる確認は許可しています。