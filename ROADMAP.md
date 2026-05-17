# OpenDAW ロードマップ

> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## Phase 1: UI基盤 & アーキテクチャ ✅ (進行中)

### 完了済み
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

### 残タスク
- [ ] `nova:` app.rs から Mixer & Effects パネルのUI描画を src/ui/mixer.rs に分離する (対象: src/ui/mixer.rs, src/ui/mod.rs, src/app.rs)
- [ ] `nova:` app.rs から Tracks パネルのUI描画を src/ui/tracks.rs に分離する (対象: src/ui/tracks.rs, src/ui/mod.rs, src/app.rs)
- [ ] `nova:` app.rs から AI Agent パネルのUI描画を src/ui/ai_agent.rs に分離する (対象: src/ui/ai_agent.rs, src/ui/mod.rs, src/app.rs)
- [ ] `nova:` app.rs の状態管理ロジック（is_playing等）を src/state/mod.rs に分離する (対象: src/state/mod.rs, src/app.rs)
- [ ] `nova:` キーボードショートカットを実装する: Space=再生/停止 (対象: src/ui/mod.rs)
- [ ] `warden:` main.rs の Tokio ランタイム起動部の unwrap を適切なエラーハンドリングに置換する (対象: src/main.rs)

---

## Phase 2: オーディオエンジン基盤

- [ ] `nova:` src/engine/mod.rs を作成し、AudioEngine 構造体のスケルトンを定義する (対象: src/engine/mod.rs)
- [ ] `nova:` Cargo.toml に cpal を追加し、デフォルトオーディオデバイスの取得を実装する (対象: Cargo.toml, src/engine/device.rs)
- [ ] `nova:` ringbuf を使ったUI↔オーディオスレッド間の通信チャンネルを構築する (対象: src/engine/channel.rs)
- [ ] `nova:` cpal のオーディオストリーム（出力）を起動するコールバックのスケルトンを実装する (対象: src/engine/stream.rs)
- [ ] `nova:` 無音ストリームを出力し、オーディオデバイスが動作することを検証するテストを書く (対象: src/engine/stream.rs)
- [ ] `nova:` hound を使った WAV ファイルの読み込みを実装する (対象: Cargo.toml, src/engine/audio_file.rs)
- [ ] `nova:` 読み込んだ WAV データをオーディオストリームで再生する機能を実装する (対象: src/engine/stream.rs)
- [ ] `nova:` トランスポートUI の再生/停止をオーディオエンジンに接続する (対象: src/ui/transport.rs, src/engine/mod.rs)
- [ ] `warden:` オーディオコールバック内のヒープアロケーション・Mutex使用を監査・修正 (対象: src/engine/)

---

## Phase 3: マルチトラック & ミキシング

- [ ] `nova:` Track 構造体を定義する（名前、ボリューム、パン、ミュート、ソロ） (対象: src/state/track.rs)
- [ ] `nova:` トラック一覧UIを実装し、トラックの追加/削除を可能にする (対象: src/ui/tracks.rs)
- [ ] `nova:` 各トラックに個別のボリューム・パンコントロールを追加する (対象: src/ui/mixer.rs)
- [ ] `nova:` オーディオエンジンでマルチトラックミキシング（合算）を実装する (対象: src/engine/mixer.rs)
- [ ] `nova:` ソロ/ミュート機能をエンジンに接続する (対象: src/engine/mixer.rs)
- [ ] `nova:` rfd を使ったオーディオファイルのインポートダイアログを実装する (対象: src/ui/import.rs)

---

## Phase 4: MIDI & シーケンシング

- [ ] `nova:` midir を使った MIDI デバイスの列挙・接続を実装する (対象: Cargo.toml, src/midi/mod.rs)
- [ ] `nova:` MIDI メッセージの受信とパースを実装する (対象: src/midi/message.rs)
- [ ] `nova:` ピアノロール UI のスケルトンを作成する (対象: src/ui/piano_roll.rs)
- [ ] `nova:` MIDI ノートイベントのシーケンスデータ構造を定義する (対象: src/midi/sequence.rs)
- [ ] `nova:` ピアノロール上でノートの追加・削除を可能にする (対象: src/ui/piano_roll.rs)

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
