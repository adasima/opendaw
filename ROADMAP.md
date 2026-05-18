# OpenDAW ロードマップ

> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## Phase 1: UI基盤 & アーキテクチャ ✅ (進行中)

### 完了済み
- [x] app.rs から Mixer & Effects パネルのUI描画を src/ui/mixer.rs に分離する (対象: src/ui/mixer.rs, src/ui/mod.rs, src/app.rs)
- [x] egui/eframe によるウィンドウ表示
- [x] ダークテーマ・グラスモーフィズム基本スタイル
- [x] トランスポートコントロール（再生/停止/ループ）
- [x] マスターボリュームスライダー & ミュートボタン
- [x] パネルレイアウト（トラック/タイムライン/ミキサー/AIエージェント）
- [x] 波形モック描画 & プレイヘッド
- [x] 基本的なユニットテスト
- [x] BPMコントロール & Time表示 (#9)
- [x] タイムラインのシーク機能（クリック・ドラッグ） (#7)
- [x] ミュートボタンとツールチップ (#6)
- [x] @A app.rs の状態管理ロジック（is_playing等）を src/state/mod.rs に分離する (対象: src/state/mod.rs, src/app.rs)
- [x] @A app.rs から Tracks パネルのUI描画を src/ui/tracks.rs に分離する (対象: src/ui/tracks.rs, src/ui/mod.rs, src/app.rs)
- [x] @A キーボードショートカットを実装する: Space=再生/停止 (対象: src/app.rs)
- [x] `warden:` main.rs の Tokio ランタイム起動部の unwrap を適切なエラーハンドリングに置換する (対象: src/main.rs)

### 残タスク

- [ ] @A [1] app.rs から AI Agent パネルのUI描画を src/ui/ai_agent.rs に分離する (対象: src/ui/ai_agent.rs, src/ui/mod.rs, src/app.rs)
- [ ] `warden:` [2] src/engine/mod.rs などの doc コメントの clippy 警告を修正する (対象: src/engine/mod.rs, src/state/mod.rs, src/midi/mod.rs, src/util/mod.rs)

---

## Phase 2: オーディオエンジン基盤

### 完了済み
- [x] @B src/engine/mod.rs の AudioEngine にデバイス選択メソッドを追加する (対象: src/engine/mod.rs)
- [x] @B Cargo.toml に cpal を追加し、デフォルトオーディオデバイスの取得を実装する (対象: Cargo.toml, src/engine/device.rs)

### 残タスク
- [ ] @B [1] src/engine/channel.rs で ringbuf を使ったUI↔オーディオスレッド間の通信チャンネルを構築する (対象: src/engine/channel.rs)
- [ ] @B [2] Cargo.toml に hound を追加し、src/engine/audio_file.rs で WAV ファイルの読み込みを実装する (対象: Cargo.toml, src/engine/audio_file.rs)
- [ ] @B [3] cpal のオーディオストリーム（出力）を起動するコールバックのスケルトンを実装する (対象: src/engine/stream.rs)
- [ ] @B [4] 読み込んだ WAV データをオーディオストリームで再生する機能を実装する (対象: src/engine/stream.rs)
- [ ] @B [5] トランスポートUI の再生/停止をオーディオエンジンに接続する (対象: src/ui/transport.rs, src/engine/mod.rs)
- [ ] `warden:` [6] オーディオコールバック内のヒープアロケーション・Mutex使用を監査・修正 (対象: src/engine/)

---

## Phase 3: マルチトラック & ミキシング

- [ ] @A [1] Track 構造体を定義する（名前、ボリューム、パン、ミュート、ソロ） (対象: src/state/track.rs)
- [ ] @A [2] トラック一覧UIを実装し、トラックの追加/削除を可能にする (対象: src/ui/tracks.rs)
- [ ] @A [3] 各トラックに個別のボリューム・パンコントロールを追加する (対象: src/ui/mixer.rs)
- [ ] @A [4] rfd を使ったオーディオファイルのインポートダイアログを実装する (対象: src/ui/import.rs)
- [ ] @B [5] オーディオエンジンでマルチトラックミキシング（合算）を実装する (対象: src/engine/mixer.rs)
- [ ] @B [6] ソロ/ミュート機能をエンジンに接続する (対象: src/engine/mixer.rs)

---

## Phase 4: MIDI & シーケンシング

- [ ] @B [1] midir を使った MIDI デバイスの列挙・接続を実装する (対象: Cargo.toml, src/midi/mod.rs)
- [ ] @B [2] MIDI メッセージの受信とパースを実装する (対象: src/midi/message.rs)
- [ ] @B [3] MIDI ノートイベントのシーケンスデータ構造を定義する (対象: src/midi/sequence.rs)
- [ ] @A [4] ピアノロール UI のスケルトンを作成する (対象: src/ui/piano_roll.rs)
- [ ] @A [5] ピアノロール上でノートの追加・削除を可能にする (対象: src/ui/piano_roll.rs)

---

## Phase 5: エフェクト & プロセッシング

- [ ] @B [1] エフェクトチェーンの抽象化（AudioEffect トレイト）を定義する (対象: src/engine/effects/mod.rs)
- [ ] @B [2] Gain エフェクトを実装する（最もシンプルなエフェクトとして） (対象: src/engine/effects/gain.rs)
- [ ] @B [3] ローパスフィルター（Biquad）を実装する (対象: src/engine/effects/filter.rs)
- [ ] @A [4] エフェクトチェーンUIを実装する（追加・削除・並び替え） (対象: src/ui/effects.rs)

---

## Phase 6: プロジェクト管理 & エクスポート

- [ ] @A [1] プロジェクトの保存（serde + bincode）を実装する (対象: src/state/project.rs)
- [ ] @A [2] プロジェクトの読み込みを実装する (対象: src/state/project.rs)
- [ ] @B [3] WAV エクスポート（オフラインレンダリング）を実装する (対象: src/engine/export.rs)

---

## Phase 7: AI統合 & MCP

- [ ] @B [1] MCP サーバーのスケルトンを Tokio ランタイム上に構築する (対象: src/mcp/mod.rs)
- [ ] @B [2] MCP 経由でトランスポートコントロールを操作可能にする (対象: src/mcp/transport.rs)
- [ ] @B [3] MCP 経由でトラックの追加・削除を可能にする (対象: src/mcp/tracks.rs)
