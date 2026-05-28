# OpenDAW ロードマップ
> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## Phase 10-19: (旧ネイティブegui版での実装完了タスク)
> ※これらのフェーズは完了済みのため省略します。今後は Tauri + Svelte + WASM の新構成へ移行します。

## Phase 20: ピアノロール機能の拡充とMIDIクリップ編集 (完了)
- [x] [1] src-tauri/src/midi/sequence.rs を更新し、ノートの追加・削除・移動（位置・長さ・ベロシティ）を管理するメソッドを実装する (対象: src-tauri/src/midi/sequence.rs)
- [x] [2] src-tauri/src/state/clip.rs に `MidiClip` 構造体を追加し、ノート列（シーケンス）とクリップ長等のメタデータを保持する (対象: src-tauri/src/state/clip.rs, src-tauri/src/state/mod.rs)
- [x] [3] opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートの追加（マウスクリック）および削除のUIインタラクションを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [4] opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートのドラッグでの移動（位置・ピッチ変更）および長さ変更を可能にするWASM上のUIインタラクションを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [5] opendaw-wasm/src/state/track.rs を更新し、Track構造体に `MidiClip` を保持するフィールドを追加する (対象: opendaw-wasm/src/state/track.rs)
- [x] [6] opendaw-wasm/src/app.rs を更新し、WASMのピアノロールからのイベントを受け取り、トラック内の `MidiClip` に反映し再生エンジンと同期する (対象: opendaw-wasm/src/app.rs)

## Phase 21: Tauri + Svelte + egui(WASM) ハイブリッドアーキテクチャ移行 (完了)
> UIをSvelteに、高負荷描画をWASM(egui)に、コアロジックをTauri(Rust)に分離する移行作業。
- [x] [1] TauriプロジェクトのセットアップとSvelte連携基盤の構築 (対象: frontend/src-tauri/Cargo.toml, frontend/src/App.svelte)
- [x] [2] WASMパッケージ(opendaw-wasm)のセットアップとSvelteからのマウント処理の実装 (対象: opendaw-wasm/Cargo.toml, frontend/src/components/TimelineCanvas.svelte)
- [x] [3] frontend/src/components/Transport.svelte を作成し、再生・停止・ループ・BPM設定のUIを実装する (対象: frontend/src/components/Transport.svelte)
- [x] [4] frontend/src-tauri/src/app.rs にトランスポート制御用のTauri Command (play, pause, stop, set_bpm 等) を実装する (対象: frontend/src-tauri/src/app.rs)
- [x] [5] frontend/src/components/Tracks.svelte を作成し、トラックヘッダーのUIを実装する (対象: frontend/src/components/Tracks.svelte)
- [x] [6] frontend/src/components/Mixer.svelte を作成し、ボリュームとマスターフェーダーのUIを実装する (対象: frontend/src/components/Mixer.svelte)
- [x] [7] Svelteから `@tauri-apps/api/core` の `invoke` を用いてTauri Commandを呼び出す処理を実装する (対象: frontend/src/components/Transport.svelte, frontend/src/components/Mixer.svelte, frontend/src/App.svelte)
- [x] [8] frontend/src-tauri/src/engine/mod.rs 等を整備し、Tauriから叩けるエンジン制御APIを構築する (対象: frontend/src-tauri/src/engine/mod.rs)
- [x] [9] オーディオスレッドとメインスレッド間でlock-freeな状態同期（RingBuffer等）を確立する (対象: frontend/src-tauri/src/engine/mod.rs)

## Phase 22: セッションビュー (Ableton Liveライク) の導入 (ハイブリッド版) (完了)
> ⚠️ **ハイブリッド開発**: データ構造と同期ロジック基盤は人間が直接コミットします。AI(Jules)はSvelte UIの繋ぎ込みを担当してください。
- [ ] 人間: `Clip`, `Scene` データ構造の設計とコア基盤の実装 (対象: src-tauri/src/core/session.rs など)
- [x] [1] frontend/src/components/SessionView.svelte を作成し、ダミーデータを用いてセッションビューのUIを実装する
- [x] [2] frontend/src/App.svelte を更新し、メイン画面にセッションビューを統合する
- [x] [3] frontend/src/components/ClipLauncher.svelte を作成し、セッションビュー内のクリップランチャーUIを実装する

## Phase 23: モダン・プラグインホスト (VST3 / CLAP) の導入 (ハイブリッド版) (完了)
> ⚠️ **ハイブリッド開発**: VST3/CLAPのFFIなど複雑な実装は人間が直接コミットします。AIはブラウザやUIを担当してください。
- [ ] 人間: `vst3-sys` 等を用いたプラグインロードの安全なラッパー層の実装 (対象: frontend/src-tauri/src/plugin/host.rs)
- [x] [1] frontend/src/components/PluginBrowser.svelte を作成し、ダミーデータを用いてプラグイン一覧を表示するSvelteUIを実装する
- [x] [2] frontend/src/App.svelte を更新し、メイン画面にプラグインブラウザパネルを統合する

## Phase 24: Svelte UIの改善とMIDI機能の統合 (完了)
> SvelteUIの改善と、TauriバックエンドへのMIDI信号のルーティング。
- [x] [1] frontend/src/components/TrackDetails.svelte を作成し、選択中のトラックの詳細設定UI(ボリューム、パン、MIDIルーティング)を実装する
- [x] [2] frontend/src/App.svelte を更新し、TrackDetailsパネルをメイン画面の右側または下部に統合する
- [x] [3] frontend/src-tauri/src/engine/midi_route.rs を作成し、MIDI入力信号を各トラックにルーティングするロジックを実装する
- [x] [4] frontend/src-tauri/src/app.rs を更新し、MIDIデバイス選択とルーティング設定のためのTauri Commandを追加する

## Phase 25: タイムラインキャンバスのオーディオ波形・MIDIノート描画と連携、およびバックエンドリアルタイム性の改善 (完了)
> Tauriから送信されたプロジェクト状態 (トラック、オーディオクリップ、MIDIシーケンス等) をもとに、WASM(egui)キャンバス上で波形およびノートの描画を行うための機能統合。あわせて、Wardenにより報告されたオーディオスレッドのリアルタイム制約違反を修正する。
- [x] [1] frontend/src-tauri/src/engine/mod.rs と frontend/src-tauri/src/engine/midi_route.rs を更新し、`EngineHandle::midi_router` の同期を `RwLock` から lock-free なデータ構造 (RingBuffer等) へ移行し、オーディオスレッド内でのロックを排除する (対象: frontend/src-tauri/src/engine/mod.rs, frontend/src-tauri/src/engine/midi_route.rs)
- [x] [2] frontend/src-tauri/src/state/mod.rs 等を作成し、Tauriバックエンド用のプロジェクト状態データ構造（Track, Clip等）を定義する (対象: frontend/src-tauri/src/state/*.rs)
- [x] [3] frontend/src-tauri/src/engine/mod.rs を更新し、`EngineHandle` に `Arc<std::sync::RwLock<ProjectState>>` などスレッドセーフでclone可能なプロジェクト状態のフィールドを追加する (対象: frontend/src-tauri/src/engine/mod.rs)
- [x] [4] frontend/src-tauri/src/app.rs の `get_project_state` コマンドを更新し、`state.engine` のプロジェクト状態からトラックやクリップ等のデータをシリアライズしてJSONとして返すようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [5] opendaw-wasm/src/app.rs の `sync_project_state_json` を更新し、Tauriから取得したJSONの `tracks` 配列等をパースしてWASM側の `app.state` を同期するロジックを実装する (対象: opendaw-wasm/src/app.rs)
- [x] [6] opendaw-wasm/src/ui/timeline.rs を更新し、同期された `app.state` をもとにオーディオクリップの矩形と波形を描画する処理を実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [x] [7] opendaw-wasm/src/ui/piano_roll.rs を更新し、同期された `app.state` をもとにMIDIノートをキャンバス上に描画する処理を実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [8] frontend/src/components/TimelineCanvas.svelte を更新し、Tauriからプロジェクト状態を取得してWASMへ渡すポーリングループを実装する (対象: frontend/src/components/TimelineCanvas.svelte)
## Phase 26: Svelte UIからのバックエンド状態変更の結合 (完了)
> 現在 Svelte 側から送られるコマンド (再生、ボリューム変更など) が `EngineHandle` を経由してオーディオスレッドには伝達されているが、JSONとして返される `ProjectState` にはUIからの変更が即座に同期されていない可能性がある。UIからのアクションを `EngineHandle` の `ProjectState` にも反映する仕組みを導入し、WASMの描画などと状態を完全に同期する。
- [x] [1] frontend/src-tauri/src/app.rs を更新し、`play`, `pause`, `stop`, `set_bpm`, `set_master_volume` コマンドで `state.engine.project_state` の該当フィールドも更新するようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [2] frontend/src-tauri/src/app.rs を更新し、`set_track_volume`, `set_track_pan` などのトラック更新コマンドで、`state.engine.project_state` 内の `tracks` の該当トラックの状態を更新するよう実装する (対象: frontend/src-tauri/src/app.rs)
- [x] [3] frontend/src-tauri/src/app.rs に `add_track`, `remove_track` 等のトラック管理コマンドを追加し、`ProjectState` のトラック配列を更新できるようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [4] frontend/src/components/Tracks.svelte 等を更新し、トラック追加・削除などのUIアクションで新規コマンドを呼ぶよう連携する (対象: frontend/src/components/Tracks.svelte)
## Phase 27: オーディオクリップの管理機能 (Tauri & WASM統合) (完了)
> SvelteフロントエンドおよびWASM UIから、オーディオクリップを追加・移動・削除できるよう、TauriバックエンドおよびWASM状態同期ロジックを拡充する。
- [x] [1] frontend/src-tauri/src/app.rs に `add_audio_clip`, `remove_audio_clip`, `move_audio_clip` のTauri Commandを実装し、`ProjectState` の該当トラック内のクリップ配列を更新するようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [2] opendaw-wasm/src/app.rs を更新し、Tauriからの `sync_project_state_json` でオーディオクリップの同期を確実に行えるようパース処理を改善する (対象: opendaw-wasm/src/app.rs)
- [x] [3] opendaw-wasm/src/ui/timeline.rs を更新し、オーディオクリップのドラッグによる移動を実装し、状態変更を Tauri へ通知するイベントフローを構築する (対象: opendaw-wasm/src/ui/timeline.rs)

## Phase 28: MIDIクリップの管理機能とピアノロールの連携 (Tauri & WASM統合) (完了)
> タイムラインからMIDIクリップを追加・削除・移動できるようにし、ピアノロールの編集内容をTauriバックエンドへ同期する仕組みを構築する。
- [x] [1] frontend/src-tauri/src/app.rs に `add_midi_clip`, `remove_midi_clip`, `move_midi_clip` などのTauri Commandを実装し、`ProjectState` を更新できるようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [2] frontend/src-tauri/src/app.rs に `update_midi_clip_notes` のTauri Commandを実装し、ピアノロールから送られたノート情報を更新できるようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [3] opendaw-wasm/src/app.rs を更新し、Tauriからの `sync_project_state_json` でMIDIクリップの同期を確実に行えるようパース処理を改善する (対象: opendaw-wasm/src/app.rs)
- [x] [4] opendaw-wasm/src/ui/timeline.rs を更新し、MIDIクリップのドラッグ移動を実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [x] [5] opendaw-wasm/src/ui/piano_roll.rs を更新し、ノートの追加・削除・移動イベントをTauriへ通知する仕組みを実装する (対象: opendaw-wasm/src/ui/piano_roll.rs)

## Phase 29: プロジェクトファイルの保存・読み込み機能の実装 (完了)
> ユーザーが現在のプロジェクト状態をファイルに保存し、後で読み込めるようにする機能。
- [x] [1] frontend/src-tauri/src/app.rs に save_project と load_project のTauri Commandを実装する (対象: frontend/src-tauri/src/app.rs)
- [x] [2] frontend/src/components/Transport.svelte に @tauri-apps/plugin-dialog を用いた保存・読み込みのUIを実装する (対象: frontend/src/components/Transport.svelte)
## Phase 30: アンドゥ・リドゥ (Undo / Redo) 機能の基盤実装 (完了)
> ユーザーが誤った操作を取り消したり、取り消しをやり直したりできる履歴管理機能（Undo / Redo）をバックエンドとフロントエンドに実装する。
- [x] [1] frontend/src-tauri/src/state/history.rs を作成し、`ProjectState` のスナップショット履歴を管理する `HistoryManager` 構造体（undo/redoスタック）を定義する (対象: frontend/src-tauri/src/state/history.rs)
- [x] [2] frontend/src-tauri/src/state/mod.rs を更新し、`history` モジュールを公開する (対象: frontend/src-tauri/src/state/mod.rs)
- [x] [3] frontend/src-tauri/src/engine/mod.rs または `AppState` 内に履歴管理のフィールドを追加し、各種操作時（クリップ追加など）に状態のスナップショットを保存する処理を組み込む (対象: frontend/src-tauri/src/engine/mod.rs, frontend/src-tauri/src/app.rs)
- [x] [4] frontend/src-tauri/src/app.rs に `undo` と `redo` の Tauri Command を実装し、履歴からプロジェクト状態を復元するようにする (対象: frontend/src-tauri/src/app.rs)
- [x] [5] frontend/src/components/Transport.svelte を更新し、Undo と Redo のUIボタンを追加して、Tauri Command を呼び出す連携処理を実装する (対象: frontend/src/components/Transport.svelte)

## Phase 31: グリッドスナップとクオンタイズ機能の実装 (完了)
> タイムラインやピアノロールでのクリップ・MIDIノートの配置時に、拍や小節などのグリッドにスナップ（吸着）する機能を実装する。
- [x] [1] frontend/src-tauri/src/state/mod.rs を更新し、`ProjectState` にグリッド設定（有効/無効、分解能など）のフィールドを追加する (対象: frontend/src-tauri/src/state/mod.rs)
- [x] [2] frontend/src-tauri/src/app.rs にグリッド設定を更新する Tauri Command (`set_grid_settings`) を追加する (対象: frontend/src-tauri/src/app.rs)
- [x] [3] opendaw-wasm/src/app.rs を更新し、Tauriから同期されたJSONからグリッド設定をパースし、WASM側の状態に反映する (対象: opendaw-wasm/src/app.rs)
- [x] [4] opendaw-wasm/src/ui/timeline.rs を更新し、クリップのドラッグ移動時にグリッド設定に基づくスナップ処理を適用する (対象: opendaw-wasm/src/ui/timeline.rs)
- [x] [5] opendaw-wasm/src/ui/piano_roll.rs を更新し、MIDIノートの追加・移動・長さ変更時にグリッド設定に基づくスナップ処理を適用する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [x] [6] frontend/src/components/Transport.svelte または新しいコントロールパネルを更新し、グリッドの有効/無効や分解能を変更するUIを実装し、Commandを呼び出す (対象: frontend/src/components/Transport.svelte)

## Phase 32: コードのリファクタリングとプラグインホスティング（VST3 / CLAP）の完全統合
> Svelte UIとTauriバックエンド間のAPIモジュール構成を整理し、プラグインホスティングの機能を統合する。
- [x] nova: frontend/src-tauri/src/app.rs の Tauri Command を frontend/src-tauri/src/commands/ モジュール配下へ機能別（project.rs, track.rs, clip.rs, transport.rsなど）に分割し、lib.rsで統合する (対象: frontend/src-tauri/src/app.rs, frontend/src-tauri/src/commands/*, frontend/src-tauri/src/lib.rs)
- [ ] nova: [1] opendaw-wasm/src/app.rs から状態同期やIPCイベント処理を機能ごとに切り分ける (対象: opendaw-wasm/src/app.rs)
- [ ] nova: [2] opendaw-wasm/src/engine/mixer.rs および stream.rs からルーティング・バス処理を切り分ける (対象: opendaw-wasm/src/engine/mixer.rs, opendaw-wasm/src/engine/stream.rs)
- [ ] nova: [3] opendaw-wasm/src/ui/piano_roll.rs を描画コンポーネント（グリッド、ノート等）ごとに整理・分割する (対象: opendaw-wasm/src/ui/piano_roll.rs)
- [ ] nova: [4] opendaw-wasm/src/state/track.rs および mod.rs を機能ドメインごとのモジュールに整理・分割する (対象: opendaw-wasm/src/state/track.rs, opendaw-wasm/src/state/mod.rs)
- [ ] nova: [5] opendaw-wasm/src/engine/synth.rs を波形生成やボイス管理ごとにモジュール化する (対象: opendaw-wasm/src/engine/synth.rs)
- [ ] 人間: `vst3-sys` 等を用いたプラグインのロード、GUI表示、音声バッファのやり取り基盤を確立する (対象: frontend/src-tauri/src/plugin/host.rs)
- [x] [1] frontend/src-tauri/src/state/mod.rs を更新し、Track内にロードされたプラグインのリストを保持するフィールドを追加する (対象: frontend/src-tauri/src/state/mod.rs)
- [x] [2] frontend/src-tauri/src/commands/plugin.rs を作成し、プラグインをトラックにロードするためのTauri Command `load_plugin_to_track` を追加する (対象: frontend/src-tauri/src/commands/plugin.rs, frontend/src-tauri/src/lib.rs)
- [x] [3] frontend/src/components/PluginBrowser.svelte を更新し、プラグインをトラックにロードするためのUI連携（D&Dまたはクリック）を実装する (対象: frontend/src/components/PluginBrowser.svelte, frontend/src/components/Tracks.svelte)
- [x] [4] frontend/src/components/TrackDetails.svelte を更新し、選択されたトラックにロードされているプラグイン一覧を表示し、プラグインGUIを開くボタンを追加する (対象: frontend/src/components/TrackDetails.svelte)

## Phase 33: オートメーション（Automation）編集・再生基盤
> ボリュームやパン、プラグインパラメータなどを時間経過で滑らかに変化させる機能。
- [ ] [1] frontend/src-tauri/src/state/mod.rs を更新し、`ProjectState` の `Track` にオートメーションデータ（パラメータ名とポイントのリスト）を保持するフィールドを追加する (対象: frontend/src-tauri/src/state/mod.rs)
- [ ] [2] frontend/src-tauri/src/commands/track.rs に、オートメーションポイントを追加・削除・更新するTauri Commandを追加する (対象: frontend/src-tauri/src/commands/track.rs)
- [ ] [3] opendaw-wasm/src/app.rs を更新し、Tauriから同期されたJSONからオートメーションデータをパースし、WASM側の状態に反映する (対象: opendaw-wasm/src/app.rs)
- [ ] [4] opendaw-wasm/src/ui/timeline.rs を更新し、各トラックの下部にオートメーションレーンを表示し、カーブの描画とポイントの追加・移動（ドラッグ）のUIを実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [ ] [5] frontend/src/components/Tracks.svelte を更新し、各トラックヘッダーにオートメーションの表示/非表示を切り替えるボタンと対象パラメータを選択するドロップダウンを追加する (対象: frontend/src/components/Tracks.svelte)
- [ ] 人間: オーディオエンジン側に、再生時間に応じたパラメータ補間ロジックを実装し、実際のオーディオ処理に反映させる (対象: frontend/src-tauri/src/engine/mod.rs)

---

## 🚀 今後のマイルストーン（商業DAW水準 ＆ Live / Studio One 融合）

> 以下のフェーズは、商業DAWに迫るクオリティを実現し、Ableton Liveの非線形なクリエイティビティとStudio Oneの直感的なワークフローを融合させるためのアイデアストックです。Architectエージェントはこれらを参考にし、必要に応じて詳細なタスクへ分解してください。（順不同で着手可能）

## Phase 34: サブバス / センド＆リターン・ルーティング
> 複数トラックをグループ化するサブバス、およびリバーブ等の空間系エフェクト用センドトラック。
- [ ] [1] frontend/src-tauri/src/state/mod.rs の Track 構造体を更新し、ルーティング先（`output_routing`）やセンドのリスト（`sends`）を保持するフィールドを追加する (対象: frontend/src-tauri/src/state/mod.rs)
- [ ] [2] frontend/src-tauri/src/commands/track.rs に、トラックのルーティングやセンド量を変更する Tauri Command を追加する (対象: frontend/src-tauri/src/commands/track.rs)
- [ ] [3] frontend/src/components/TrackDetails.svelte を更新し、トラックの出力先バスの選択や、センド量（Sends）を調整するUIを実装する (対象: frontend/src/components/TrackDetails.svelte)
- [ ] 人間: オーディオエンジンのグラフ処理順序をトポロジカルソートし、バス階層に応じたレンダリングを実装する (対象: frontend/src-tauri/src/engine/mod.rs)

## Phase 35: 究極のドラッグ＆ドロップ・ワークフロー (Studio One風)
> プラグイン、エフェクトチェイン、オーディオファイル、MIDIパターンをキャンバスやトラックにD&Dするだけで即座にロード＆ルーティングされる仕組み。
- [ ] [1] opendaw-wasm/src/ui/timeline.rs を更新し、外部ファイルやブラウザからのD&Dイベントを受け取る基盤（TauriからWASMへのイベント通知）を構築する (対象: opendaw-wasm/src/ui/timeline.rs, opendaw-wasm/src/app.rs)
- [ ] [2] frontend/src/components/PluginBrowser.svelte やファイルブラウザ（新規作成）から、アイテムをドラッグ開始した際にTauri経由でWASMへドラッグ中のアイテム情報を送信する仕組みを実装する (対象: frontend/src/components/PluginBrowser.svelte)
- [ ] [3] opendaw-wasm/src/ui/timeline.rs を更新し、ドラッグ中のアイテムがトラック上にホバーされた際のハイライト表示を実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [ ] [4] D&D完了（ドロップ）時に、WASMからTauriの対象コマンド（`load_plugin_to_track`, `add_audio_clip` 等）を呼び出し、処理を完了する連携を実装する (対象: opendaw-wasm/src/ui/timeline.rs)

## Phase 36: インストゥルメント / エフェクト・ラックとマクロコントロール (Ableton Live風)
> 複数のプラグインやエフェクトを一つの「ラック」にまとめ、パラレル処理や単一のマクロノブで複数パラメータを同時制御する機能。
- [ ] [1] frontend/src-tauri/src/state/mod.rs を更新し、Track内に複数のデバイスをグループ化する `DeviceRack` のデータ構造を追加する (対象: frontend/src-tauri/src/state/mod.rs)
- [ ] [2] frontend/src-tauri/src/commands/plugin.rs に、ラック内にデバイスを追加・削除・並べ替えるTauri Commandを追加する (対象: frontend/src-tauri/src/commands/plugin.rs)
- [ ] [3] frontend/src/components/DeviceChain.svelte (新規) を作成し、トラックの下部または別パネルでデバイスチェーン/ラックの内容を水平方向に表示するUIを実装する (対象: frontend/src/components/DeviceChain.svelte, frontend/src/App.svelte)
- [ ] [4] frontend/src/components/DeviceChain.svelte に、複数パラメータを1つのノブで操作できる「マクロコントロール」のUIを実装する (対象: frontend/src/components/DeviceChain.svelte)
- [ ] 人間: オーディオエンジンに並列処理グラフノードを追加し、ラック内のデバイス間の音声・MIDI信号のルーティングを実装する (対象: frontend/src-tauri/src/engine/mod.rs)

## Phase 37: コードトラックとハーモニーの自動追従 (Studio One風)
> 楽曲のコード進行を専用トラックで管理し、それに合わせてMIDIやオーディオのピッチが自動追従（移調）する作曲支援機能。
- [ ] [1] frontend/src-tauri/src/state/mod.rs を更新し、`ProjectState` に `ChordTrack` （時間とコード情報のリスト）のフィールドを追加する (対象: frontend/src-tauri/src/state/mod.rs)
- [ ] [2] frontend/src-tauri/src/commands/project.rs に、コードイベントを追加・編集・削除する Tauri Command を追加する (対象: frontend/src-tauri/src/commands/project.rs)
- [ ] [3] opendaw-wasm/src/ui/timeline.rs を更新し、タイムライン上部に専用のコードトラックレーンを描画し、コード名（例: "Cmaj7"）を表示・編集するUIを実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [ ] 人間: MIDIクリップ再生時に、現在のコードスケールに合わせてノートをリアルタイムに移調するエンジンを実装する (対象: frontend/src-tauri/src/engine/mod.rs)

## Phase 38: スクラッチパッド / アレンジ実験エリア (Studio One風)
> メインのアレンジを壊さずにアイデア（Aメロ別バージョンなど）を試せる機能。
- [ ] [1] frontend/src-tauri/src/state/mod.rs を更新し、`ProjectState` に複数のタイムライン（`Arrangement`）を保持し、現在アクティブなものを切り替えられるデータ構造を実装する (対象: frontend/src-tauri/src/state/mod.rs)
- [ ] [2] frontend/src-tauri/src/commands/project.rs に、アレンジメントの追加・削除・切り替えを行う Tauri Command を追加する (対象: frontend/src-tauri/src/commands/project.rs)
- [ ] [3] frontend/src/components/Transport.svelte または新しいコントロールパネルを更新し、アクティブなアレンジメント/スクラッチパッドを選択するドロップダウンを実装する (対象: frontend/src/components/Transport.svelte)
- [ ] [4] opendaw-wasm/src/ui/timeline.rs を更新し、画面を分割してメインアレンジとスクラッチパッドを並べて表示し、両者間でクリップをD&D移動できるUIを実装する (対象: opendaw-wasm/src/ui/timeline.rs)

## Phase 39: リアルタイム・タイムワープとグルーヴ (Ableton Live風)
> オーディオのタイミングをリアルタイムで伸縮・補正（ワープ）する機能と、MIDIのスウィング感を管理するグルーヴ機能。
- [ ] [1] frontend/src-tauri/src/state/clip.rs を更新し、`AudioClip` にワープマーカー（オーディオ内の時間とタイムライン上の時間のマッピング）のリストを追加する (対象: frontend/src-tauri/src/state/clip.rs)
- [ ] [2] frontend/src-tauri/src/commands/clip.rs に、ワープマーカーを追加・移動・削除する Tauri Command を追加する (対象: frontend/src-tauri/src/commands/clip.rs)
- [ ] [3] opendaw-wasm/src/ui/timeline.rs を更新し、オーディオクリップ上にワープマーカーを描画し、ドラッグでタイミングを調整できるUIを実装する (対象: opendaw-wasm/src/ui/timeline.rs)
- [ ] 人間: `rubato` 等のRustリサンプリングクレートを用いた高品質なタイムストレッチアルゴリズムをオーディオエンジンに統合し、ワープマーカーに従ってリアルタイムに再生速度を可変させる (対象: frontend/src-tauri/src/engine/mod.rs)

## Phase 40: MCP / Max for Live ライクなエージェントAPI統合
> DAW自体がMCPサーバーとしてAPIを公開し、外部AIや自作スクリプトで独自のAIジェネレーターやエフェクトを簡単に組み込めるようにする拡張機能。
- [ ] 人間: TauriコマンドおよびRustのコアロジックをラップするMCPサーバ機能を実装し、外部エージェントからの操作を受け付ける (対象: frontend/src-tauri/src/mcp/)
- [ ] [1] frontend/src-tauri/src/commands/project.rs を更新し、MCPサーバーの起動・停止やポート設定を行う Tauri Command を実装する (対象: frontend/src-tauri/src/commands/project.rs)
- [ ] [2] frontend/src/components/TrackDetails.svelte などの設定画面に、MCPサーバーのステータス表示や設定パネルを追加する (対象: frontend/src/components/TrackDetails.svelte)
