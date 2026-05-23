# OpenDAW ロードマップ
> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## Phase 10-19: (旧ネイティブegui版での実装完了タスク)
> ※これらのフェーズは完了済みのため省略します。今後は Tauri + Svelte + WASM の新構成へ移行します。

## Phase 20: ピアノロール機能の拡充とMIDIクリップ編集 (WASM移行作業中)
- [x] [1] @A src-tauri/src/midi/sequence.rs を更新し、ノートの追加・削除・移動（位置・長さ・ベロシティ）を管理するメソッドを実装する (対象: src-tauri/src/midi/sequence.rs)
- [x] [2] @A src-tauri/src/state/clip.rs に `MidiClip` 構造体を追加し、ノート列（シーケンス）とクリップ長等のメタデータを保持する (対象: src-tauri/src/state/clip.rs, src-tauri/src/state/mod.rs)
- [x] [3] @B opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートの追加（マウスクリック）および削除のUIインタラクションを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [ ] [4] @A opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートのドラッグでの移動（位置・ピッチ変更）および長さ変更を可能にするWASM上のUIインタラクションを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [ ] [5] @B src-tauri/src/state/track.rs を更新し、Track構造体に `MidiClip` を保持するフィールドを追加する (対象: src-tauri/src/state/track.rs)
- [ ] [6] @B src-tauri/src/app.rs を更新し、WASMのピアノロールからのイベントをTauri Command/IPC経由で受け取り、トラック内の `MidiClip` に反映し再生エンジンと同期する (対象: src-tauri/src/app.rs)

## Phase 21: Tauri + Svelte + egui(WASM) ハイブリッドアーキテクチャ移行 (新規)
> UIをSvelteに、高負荷描画をWASM(egui)に、コアロジックをTauri(Rust)に分離する移行作業。
- [ ] [1] @A TauriプロジェクトのセットアップとSvelte連携基盤の構築 (対象: src-tauri/Cargo.toml, src/App.svelte)
- [ ] [2] @B WASMパッケージ(opendaw-wasm)のセットアップとSvelteからのマウント処理の実装 (対象: opendaw-wasm/Cargo.toml, src/components/TimelineCanvas.svelte)
- [ ] [3] @A トランスポートコントロール(再生/停止)UIのSvelte実装とTauri Commandの繋ぎ込み (対象: src/components/Transport.svelte, src-tauri/src/app.rs)
- [ ] [4] @B トラック一覧とミキサーUIのSvelte移行 (対象: src/components/Mixer.svelte, src/components/Tracks.svelte)
- [ ] [5] @A オーディオエンジンのTauriバックエンドへの完全移行と状態同期 (対象: src-tauri/src/engine/mod.rs)

## Phase 22: セッションビュー (Ableton Liveライク) の導入 (ハイブリッド版)
> ⚠️ **ハイブリッド開発**: データ構造と同期ロジック基盤は人間が直接コミットします。AI(Jules)はSvelte UIの繋ぎ込みを担当してください。
- [ ] 人間: `Clip`, `Scene` データ構造の設計とコア基盤の実装 (対象: src-tauri/src/core/session.rs など)
- [ ] [1] @A src/components/SessionView.svelte を作成し、ダミーデータを用いてセッションビューのUIを実装する
- [ ] [2] @B src/App.svelte を更新し、メイン画面にセッションビューを統合する

## Phase 23: モダン・プラグインホスト (VST3 / CLAP) の導入 (ハイブリッド版)
> ⚠️ **ハイブリッド開発**: VST3/CLAPのFFIなど複雑な実装は人間が直接コミットします。AIはブラウザやUIを担当してください。
- [ ] 人間: `vst3-sys` 等を用いたプラグインロードの安全なラッパー層の実装 (対象: src-tauri/src/plugin/host.rs)
- [ ] [1] @A src/components/PluginBrowser.svelte を作成し、ダミーデータを用いてプラグイン一覧を表示するSvelteUIを実装する
- [ ] [2] @B src/App.svelte を更新し、メイン画面にプラグインブラウザパネルを統合する
