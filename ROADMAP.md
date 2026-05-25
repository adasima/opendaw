# OpenDAW ロードマップ
> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## Phase 10-19: (旧ネイティブegui版での実装完了タスク)
> ※これらのフェーズは完了済みのため省略します。今後は Tauri + Svelte + WASM の新構成へ移行します。

## Phase 20: ピアノロール機能の拡充とMIDIクリップ編集 (完了)
- [x] [1] @A src-tauri/src/midi/sequence.rs を更新し、ノートの追加・削除・移動（位置・長さ・ベロシティ）を管理するメソッドを実装する (対象: src-tauri/src/midi/sequence.rs)
- [x] [2] @A src-tauri/src/state/clip.rs に `MidiClip` 構造体を追加し、ノート列（シーケンス）とクリップ長等のメタデータを保持する (対象: src-tauri/src/state/clip.rs, src-tauri/src/state/mod.rs)
- [x] [3] @B opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートの追加（マウスクリック）および削除のUIインタラクションを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [4] @A opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートのドラッグでの移動（位置・ピッチ変更）および長さ変更を可能にするWASM上のUIインタラクションを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [5] @B opendaw-wasm/src/state/track.rs を更新し、Track構造体に `MidiClip` を保持するフィールドを追加する (対象: opendaw-wasm/src/state/track.rs)
- [x] [6] @B opendaw-wasm/src/app.rs を更新し、WASMのピアノロールからのイベントを受け取り、トラック内の `MidiClip` に反映し再生エンジンと同期する (対象: opendaw-wasm/src/app.rs)

## Phase 21: Tauri + Svelte + egui(WASM) ハイブリッドアーキテクチャ移行 (完了)
> UIをSvelteに、高負荷描画をWASM(egui)に、コアロジックをTauri(Rust)に分離する移行作業。
- [x] [1] @A TauriプロジェクトのセットアップとSvelte連携基盤の構築 (対象: frontend/src-tauri/Cargo.toml, frontend/src/App.svelte)
- [x] [2] @B WASMパッケージ(opendaw-wasm)のセットアップとSvelteからのマウント処理の実装 (対象: opendaw-wasm/Cargo.toml, frontend/src/components/TimelineCanvas.svelte)
- [x] [3] @A frontend/src/components/Transport.svelte を作成し、再生・停止・ループ・BPM設定のUIを実装する (対象: frontend/src/components/Transport.svelte)
- [x] [4] @A frontend/src-tauri/src/app.rs にトランスポート制御用のTauri Command (play, pause, stop, set_bpm 等) を実装する (対象: frontend/src-tauri/src/app.rs)
- [x] [5] @B frontend/src/components/Tracks.svelte を作成し、トラックヘッダーのUIを実装する (対象: frontend/src/components/Tracks.svelte)
- [x] [6] @B frontend/src/components/Mixer.svelte を作成し、ボリュームとマスターフェーダーのUIを実装する (対象: frontend/src/components/Mixer.svelte)
- [x] [7] @B Svelteから `@tauri-apps/api/core` の `invoke` を用いてTauri Commandを呼び出す処理を実装する (対象: frontend/src/components/Transport.svelte, frontend/src/components/Mixer.svelte, frontend/src/App.svelte)
- [x] [8] @A frontend/src-tauri/src/engine/mod.rs 等を整備し、Tauriから叩けるエンジン制御APIを構築する (対象: frontend/src-tauri/src/engine/mod.rs)
- [x] [9] @A オーディオスレッドとメインスレッド間でlock-freeな状態同期（RingBuffer等）を確立する (対象: frontend/src-tauri/src/engine/mod.rs)

## Phase 22: セッションビュー (Ableton Liveライク) の導入 (ハイブリッド版) (完了)
> ⚠️ **ハイブリッド開発**: データ構造と同期ロジック基盤は人間が直接コミットします。AI(Jules)はSvelte UIの繋ぎ込みを担当してください。
- [ ] 人間: `Clip`, `Scene` データ構造の設計とコア基盤の実装 (対象: src-tauri/src/core/session.rs など)
- [x] [1] @A frontend/src/components/SessionView.svelte を作成し、ダミーデータを用いてセッションビューのUIを実装する
- [x] [2] @A frontend/src/App.svelte を更新し、メイン画面にセッションビューを統合する
- [x] [3] @B frontend/src/components/ClipLauncher.svelte を作成し、セッションビュー内のクリップランチャーUIを実装する

## Phase 23: モダン・プラグインホスト (VST3 / CLAP) の導入 (ハイブリッド版) (完了)
> ⚠️ **ハイブリッド開発**: VST3/CLAPのFFIなど複雑な実装は人間が直接コミットします。AIはブラウザやUIを担当してください。
- [ ] 人間: `vst3-sys` 等を用いたプラグインロードの安全なラッパー層の実装 (対象: src-tauri/src/plugin/host.rs)
- [x] [1] @A frontend/src/components/PluginBrowser.svelte を作成し、ダミーデータを用いてプラグイン一覧を表示するSvelteUIを実装する
- [x] [2] @B frontend/src/App.svelte を更新し、メイン画面にプラグインブラウザパネルを統合する

## Phase 24: Svelte UIの改善とMIDI機能の統合 (完了)
> SvelteUIの改善と、TauriバックエンドへのMIDI信号のルーティング。
- [x] [1] @A frontend/src/components/TrackDetails.svelte を作成し、選択中のトラックの詳細設定UI(ボリューム、パン、MIDIルーティング)を実装する
- [x] [2] @B frontend/src/App.svelte を更新し、TrackDetailsパネルをメイン画面の右側または下部に統合する
- [x] [3] @A frontend/src-tauri/src/engine/midi_route.rs を作成し、MIDI入力信号を各トラックにルーティングするロジックを実装する
- [x] [4] @B frontend/src-tauri/src/app.rs を更新し、MIDIデバイス選択とルーティング設定のためのTauri Commandを追加する

## Phase 25: タイムラインキャンバスのオーディオ波形・MIDIノート描画と連携、およびバックエンドリアルタイム性の改善 (完了)
> Tauriから送信されたプロジェクト状態 (トラック、オーディオクリップ、MIDIシーケンス等) をもとに、WASM(egui)キャンバス上で波形およびノートの描画を行うための機能統合。あわせて、Wardenにより報告されたオーディオスレッドのリアルタイム制約違反を修正する。
- [x] [1] @A frontend/src-tauri/src/engine/mod.rs と frontend/src-tauri/src/engine/midi_route.rs を更新し、`EngineHandle::midi_router` の同期を `RwLock` から lock-free なデータ構造 (RingBuffer等) へ移行し、オーディオスレッド内でのロックを排除する (対象: frontend/src-tauri/src/engine/mod.rs, frontend/src-tauri/src/engine/midi_route.rs)
- [x] [2] @A frontend/src-tauri/src/state/mod.rs 等を作成し、Tauriバックエンド用のプロジェクト状態データ構造（Track, Clip等）を定義する (対象: frontend/src-tauri/src/state/*.rs)
- [x] [3] @A frontend/src-tauri/src/engine/mod.rs を更新し、`EngineHandle` に `Arc<std::sync::RwLock<ProjectState>>` などスレッドセーフでclone可能なプロジェクト状態のフィールドを追加する (対象: frontend/src-tauri/src/engine/mod.rs)
- [x] [4] @A frontend/src-tauri/src/app.rs の `get_project_state` コマンドを更新し、`state.engine` のプロジェクト状態からトラックやクリップ等のデータをシリアライズしてJSONとして返すようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [5] @B opendaw-wasm/src/app.rs の `sync_project_state_json` を更新し、Tauriから取得したJSONの `tracks` 配列等をパースしてWASM側の `app.state` を同期するロジックを実装する (対象: opendaw-wasm/src/app.rs)
- [x] [6] @B opendaw-wasm/src/ui/timeline.rs を更新し、同期された `app.state` をもとにオーディオクリップの矩形と波形を描画する処理を実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [x] [7] @B opendaw-wasm/src/ui/piano_roll.rs を更新し、同期された `app.state` をもとにMIDIノートをキャンバス上に描画する処理を実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [8] @B frontend/src/components/TimelineCanvas.svelte を更新し、Tauriからプロジェクト状態を取得してWASMへ渡すポーリングループを実装する (対象: frontend/src/components/TimelineCanvas.svelte)
## Phase 26: Svelte UIからのバックエンド状態変更の結合
> 現在 Svelte 側から送られるコマンド (再生、ボリューム変更など) が `EngineHandle` を経由してオーディオスレッドには伝達されているが、JSONとして返される `ProjectState` にはUIからの変更が即座に同期されていない可能性がある。UIからのアクションを `EngineHandle` の `ProjectState` にも反映する仕組みを導入し、WASMの描画などと状態を完全に同期する。
- [ ] [1] @A frontend/src-tauri/src/app.rs を更新し、`play`, `pause`, `stop`, `set_bpm`, `set_master_volume` コマンドで `state.engine.project_state` の該当フィールドも更新するようにする (対象: frontend/src-tauri/src/app.rs)
- [ ] [2] @A frontend/src-tauri/src/app.rs を更新し、`set_track_volume`, `set_track_pan` などのトラック更新コマンドで、`state.engine.project_state` 内の `tracks` の該当トラックの状態を更新するよう実装する (対象: frontend/src-tauri/src/app.rs)
- [ ] [3] @B frontend/src-tauri/src/app.rs に `add_track`, `remove_track` 等のトラック管理コマンドを追加し、`ProjectState` のトラック配列を更新できるようにする (対象: frontend/src-tauri/src/app.rs)
- [ ] [4] @B frontend/src/components/Tracks.svelte 等を更新し、トラック追加・削除などのUIアクションで新規コマンドを呼ぶよう連携する (対象: frontend/src/components/Tracks.svelte)
