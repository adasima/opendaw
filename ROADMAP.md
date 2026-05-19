# OpenDAW ロードマップ

> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## Phase 1: UI基盤 & アーキテクチャ ✅

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
- [x] @A app.rs から AI Agent パネルのUI描画を src/ui/ai_agent.rs に分離する (対象: src/ui/ai_agent.rs, src/ui/mod.rs, src/app.rs)
- [x] `warden:` main.rs の Tokio ランタイム起動部の unwrap を適切なエラーハンドリングに置換する (対象: src/main.rs)

---

## Phase 2: オーディオエンジン基盤 ✅

### 完了済み
- [x] src/engine/mod.rs の AudioEngine にデバイス選択メソッドを追加する (対象: src/engine/mod.rs)
- [x] Cargo.toml に cpal を追加し、デフォルトオーディオデバイスの取得を実装する (対象: Cargo.toml, src/engine/device.rs)
- [x] ringbuf を使ったUI↔オーディオスレッド間の通信チャンネルを構築する (対象: src/engine/channel.rs)
- [x] hound を使った WAV ファイルの読み込みを実装する (対象: Cargo.toml, src/engine/audio_file.rs)
- [x] cpal のオーディオストリーム（出力）を起動するコールバックのスケルトンを実装する (対象: src/engine/stream.rs)
- [x] 読み込んだ WAV データをオーディオストリームで再生する機能を実装する (対象: src/engine/stream.rs)
- [x] [1] @B src/engine/mod.rs に cpal オーディオストリームのライフサイクル管理とチャンネル受信を追加する (対象: src/engine/mod.rs)
- [x] [2] @A src/app.rs で AudioEngine のインスタンス化とチャンネルの保持を実装する (対象: src/app.rs)
- [x] [3] @A src/ui/transport.rs のトランスポート操作（再生/停止）をチャンネル経由でエンジンに送信する (対象: src/ui/transport.rs)
- [x] [4] `warden:` オーディオコールバック内のヒープアロケーション・Mutex使用を監査・修正する (対象: src/engine/stream.rs, src/engine/channel.rs)
- [x] [5] `warden:` src/engine/mod.rs の doc コメント警告 (`empty_line_after_doc_comments` 等) を修正する (対象: src/engine/mod.rs)
- [x] [6] `warden:` src/state/mod.rs の doc コメント警告を修正する (対象: src/state/mod.rs)
- [x] [7] `warden:` src/midi/mod.rs の doc コメント警告を修正する (対象: src/midi/mod.rs)
- [x] [8] `warden:` src/util/mod.rs の doc コメント警告を修正する (対象: src/util/mod.rs)

---

## Phase 3: マルチトラック & ミキシング ✅

### 完了済み
- [x] [1] @A src/state/track.rs を作成し、Track 構造体を定義する（名前、ボリューム、パン、ミュート、ソロ） (対象: src/state/track.rs, src/state/mod.rs)
- [x] [2] @A src/ui/tracks.rs を更新し、トラック一覧UIを実装し、トラックの追加/削除を可能にする (対象: src/ui/tracks.rs)
- [x] [5] @B src/engine/mixer.rs を作成し、オーディオエンジンでマルチトラックミキシング（合算）を実装する (対象: src/engine/mixer.rs, src/engine/mod.rs)
- [x] [3] nova: @A src/ui/mixer.rs で各トラックのボリュームコントロールUIを実装する (対象: src/ui/mixer.rs)
- [x] [4] nova: @A src/ui/mixer.rs で各トラックのパンコントロールUIとミュート・ソロボタンを実装する (対象: src/ui/mixer.rs)
- [x] [6] nova: @A src/ui/import.rs を作成し、rfdを使ったオーディオファイルのインポートダイアログを実装する (対象: src/ui/import.rs, src/ui/mod.rs, src/app.rs)
- [x] [7] nova: @B src/engine/stream.rs のオーディオコールバックで、engine::mixer::mix_tracksを使用するように実装を更新する (対象: src/engine/stream.rs)

---

## Phase 4: MIDI & シーケンシング ✅

### 完了済み
- [x] @B midir を使った MIDI デバイスの列挙・接続を実装する (対象: Cargo.toml, src/midi/mod.rs, src/midi/device.rs)
- [x] @B MIDI メッセージの受信とパースを実装する (対象: src/midi/message.rs)
- [x] @B MIDI ノートイベントのシーケンスデータ構造を定義する (対象: src/midi/sequence.rs)
- [x] [1] nova: @A src/ui/piano_roll.rs を作成し、ピアノロールUIのスケルトンを定義・表示する (対象: src/ui/piano_roll.rs, src/ui/mod.rs, src/app.rs)
- [x] [2] nova: @B src/state/mod.rs に MIDI シーケンス (Sequence) を保持する機能を追加する (対象: src/state/mod.rs)
- [x] [3] nova: @A src/ui/piano_roll.rs を更新し、マウスクリックによるノートの追加・削除UIを実装する (対象: src/ui/piano_roll.rs)

---

## Phase 5: エフェクト & プロセッシング ✅

### 完了済み
- [x] [1] nova: @A src/engine/effects/mod.rs を作成し、エフェクトチェーンの抽象化（AudioEffect トレイト）を定義する (対象: src/engine/effects/mod.rs)
- [x] [2] nova: @A src/engine/effects/gain.rs を作成し、Gain エフェクトを実装する (対象: src/engine/effects/gain.rs, src/engine/effects/mod.rs)
- [x] [3] nova: @A src/engine/effects/filter.rs を作成し、ローパスフィルター（Biquad）を実装する (対象: src/engine/effects/filter.rs, src/engine/effects/mod.rs)
- [x] [4] nova: @B src/state/track.rs を更新し、トラックごとにエフェクト設定を保持するデータ構造を追加する (対象: src/state/track.rs)
- [x] [5] nova: @A src/engine/mixer.rs を更新し、ミキシング時に各トラックのエフェクトチェーンを適用する処理を追加する (対象: src/engine/mixer.rs)
- [x] [6] nova: @B src/ui/effects.rs を作成し、トラックのエフェクトチェーンを編集するUIを実装する (対象: src/ui/effects.rs, src/ui/mod.rs, src/app.rs)

---

## Phase 6: プロジェクト管理 & エクスポート ✅

- [x] [1] nova: @A src/state/project.rs を作成し、プロジェクト状態の構造体とシリアライズ(serde)を定義する (対象: src/state/project.rs, src/state/mod.rs)
- [x] [2] nova: @A src/state/project.rs に bincode を用いたファイルへの保存(Save)機能を実装する (対象: src/state/project.rs)
- [x] [3] nova: @A src/state/project.rs に bincode を用いたファイルからの読み込み(Load)機能を実装する (対象: src/state/project.rs)
- [x] [4] nova: @B src/engine/export.rs を作成し、ミキサー出力をWAVファイルとして書き出す(hound)オフラインレンダリング機能を実装する (対象: src/engine/export.rs, src/engine/mod.rs)
- [x] [5] nova: @A src/ui/project.rs を作成し、UIからプロジェクトの保存・読み込みを実行する機能を実装する (対象: src/ui/project.rs, src/ui/mod.rs, src/app.rs)

---

## Phase 7: AI統合 & MCP ✅
### 完了済み


- [x] [1] nova: @A src/mcp/mod.rs を作成し、MCP サーバーのスケルトンを Tokio ランタイム上に構築する (対象: src/mcp/mod.rs)
- [x] [2] nova: @A src/mcp/transport.rs を作成し、MCP 経由でトランスポートコントロール（再生・停止など）を操作するハンドラを実装する (対象: src/mcp/transport.rs, src/mcp/mod.rs)
- [x] [3] nova: @A src/mcp/tracks.rs を作成し、MCP 経由でトラックの追加・削除操作を行うハンドラを実装する (対象: src/mcp/tracks.rs, src/mcp/mod.rs)
- [x] [4] nova: @B main.rs の Tokio ランタイムから MCP サーバーを起動するように統合する (対象: src/main.rs)

---

## Phase 8: MCPとUIスレッドの連携実装 (進行中)

- [ ] [1] nova: @A src/mcp/channel.rs を作成し、MCPサーバーからUIへのコマンド送信用に crossbeam-channel を定義する (対象: src/mcp/channel.rs, src/mcp/mod.rs)
- [ ] [2] nova: @A src/mcp/transport.rs を更新し、crossbeam-channel を通じてUIへトランスポートコマンドを送信する (対象: src/mcp/transport.rs)
- [ ] [3] nova: @A src/mcp/tracks.rs を更新し、crossbeam-channel を通じてUIへトラック操作コマンドを送信する (対象: src/mcp/tracks.rs)
- [ ] [4] nova: @B src/app.rs を更新し、MCPチャネルからコマンドを受信して状態を更新する処理を実装する (対象: src/app.rs)
