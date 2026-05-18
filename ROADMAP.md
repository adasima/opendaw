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
- [x] main.rs の Tokio ランタイム起動部の unwrap を適切なエラーハンドリングに置換する (対象: src/main.rs)

### 残タスク

- [ ] [1] @A src/ui/tracks.rs を作成し、TracksパネルのUI描画を分離する (対象: src/ui/tracks.rs, src/ui/mod.rs)
- [ ] [2] @A src/ui/ai_agent.rs を作成し、AI AgentパネルのUI描画を分離する (対象: src/ui/ai_agent.rs, src/ui/mod.rs)
- [ ] [3] @B src/state/mod.rs へ状態管理ロジック（is_playing等）を分離する (対象: src/state/mod.rs, src/app.rs)
- [ ] [4] @B Spaceキーでの再生/停止ショートカットを実装する (対象: src/app.rs)
- [ ] `warden:` src/engine/mod.rs の doc コメントの警告 (empty_line_after_doc_comments 等) を修正する (対象: src/engine/mod.rs)
- [ ] `warden:` src/state/mod.rs の doc コメントの警告 (empty_line_after_doc_comments 等) を修正する (対象: src/state/mod.rs)
- [ ] `warden:` src/midi/mod.rs の doc コメントの警告 (empty_line_after_doc_comments 等) を修正する (対象: src/midi/mod.rs)
- [ ] `warden:` src/util/mod.rs の doc コメントの警告 (empty_line_after_doc_comments 等) を修正する (対象: src/util/mod.rs)

---

## Phase 2: オーディオエンジン基盤

- [ ] @B src/engine/mod.rs の AudioEngine にデバイス選択メソッドを追加する (対象: src/engine/mod.rs)
- [ ] @B Cargo.toml に cpal を追加し、デフォルトオーディオデバイスの取得を実装する (対象: Cargo.toml, src/engine/device.rs)
- [ ] @B ringbuf を使ったUI↔オーディオスレッド間の通信チャンネルを構築する (対象: src/engine/channel.rs)
- [ ] @B hound を使った WAV ファイルの読み込みを実装する (対象: Cargo.toml, src/engine/audio_file.rs)
- [ ] @B cpal のオーディオストリーム（出力）を起動するコールバックのスケルトンを実装する (対象: src/engine/stream.rs)
- [ ] @A トランスポートUI の再生/停止をオーディオエンジンに接続する (対象: src/ui/transport.rs, src/engine/mod.rs)
- [ ] @B 読み込んだ WAV データをオーディオストリームで再生する機能を実装する (対象: src/engine/stream.rs)
- [ ] `warden:` オーディオコールバック内のヒープアロケーション・Mutex使用を監査・修正 (対象: src/engine/)

---

## Phase 3: マルチトラック & ミキシング

- [ ] @A Track 構造体を定義する（名前、ボリューム、パン、ミュート、ソロ） (対象: src/state/track.rs)
- [ ] @A トラック一覧UIを実装し、トラックの追加/削除を可能にする (対象: src/ui/tracks.rs)
- [ ] @A 各トラックに個別のボリューム・パンコントロールを追加する (対象: src/ui/mixer.rs)
- [ ] @B オーディオエンジンでマルチトラックミキシング（合算）を実装する (対象: src/engine/mixer.rs)
- [ ] @B ソロ/ミュート機能をエンジンに接続する (対象: src/engine/mixer.rs)
- [ ] @A rfd を使ったオーディオファイルのインポートダイアログを実装する (対象: src/ui/import.rs)

---

## Phase 4: MIDI & シーケンシング

- [ ] @B midir を使った MIDI デバイスの列挙・接続を実装する (対象: Cargo.toml, src/midi/mod.rs)
- [ ] @B MIDI メッセージの受信とパースを実装する (対象: src/midi/message.rs)
- [ ] @A ピアノロール UI のスケルトンを作成する (対象: src/ui/piano_roll.rs)
- [ ] @B MIDI ノートイベントのシーケンスデータ構造を定義する (対象: src/midi/sequence.rs)
- [ ] @A ピアノロール上でノートの追加・削除を可能にする (対象: src/ui/piano_roll.rs)

---

## Phase 5: エフェクト & プロセッシング

- [ ] `nova:` エフェクトチェーンの抽象化（AudioEffect トレイト）を定義する (対象: src/engine/effects/mod.rs)
- [ ] `nova:` Gain エフェクトを実装する（最もシンプルなエフェクトとして） (対象: src/engine/effects/gain.rs)
- [ ] `nova:` ローパスフィルター（Biquad）を実装する (対象: src/engine/effects/filter.rs)
- [ ] `nova:` エフェクトチェーンUIを実装する（追加・削除・並び替え） (対象: src/ui/effects.rs)

---

## Phase 6: プロジェクト管理 & エクスポート

- [ ] `nova:` プロジェクトの保存（serde + bincode）を実装する (対象: src/state/project.rs)
- [ ] `nova:` プロジェクトの読み込みを実装する (対象: src/state/project.rs)
- [ ] `nova:` WAV エクスポート（オフラインレンダリング）を実装する (対象: src/engine/export.rs)

---

## Phase 7: AI統合 & MCP

- [ ] `nova:` MCP サーバーのスケルトンを Tokio ランタイム上に構築する (対象: src/mcp/mod.rs)
- [ ] `nova:` MCP 経由でトランスポートコントロールを操作可能にする (対象: src/mcp/transport.rs)
- [ ] `nova:` MCP 経由でトラックの追加・削除を可能にする (対象: src/mcp/tracks.rs)
