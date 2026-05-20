# OpenDAW アーキテクチャガイド

> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。

## モジュール構造

```
src/
├── main.rs              # エントリーポイント（eframe起動 + Tokioランタイム）
├── app.rs               # AuraDawApp（eframe::App実装、トップレベル状態）
│
├── ui/                  # UIコンポーネント（egui描画のみ、ロジックを持たない）
│   ├── mod.rs           # UIモジュールのルート、共通スタイル定数
│   ├── transport.rs     # トランスポートコントロール（再生/停止/ループ）
│   ├── timeline.rs      # タイムライン & 波形描画
│   ├── tracks.rs        # トラック一覧パネル
│   ├── mixer.rs         # ミキサーパネル（ボリューム/パン/エフェクト）
│   ├── ai_agent.rs      # AIエージェントパネル
│   ├── piano_roll.rs    # ピアノロールエディタ
│   ├── effects.rs       # エフェクトチェーンUI
│   └── import.rs        # ファイルインポートダイアログ
│
├── engine/              # オーディオエンジン（リアルタイムスレッド）
│   ├── mod.rs           # AudioEngine 構造体、初期化・制御
│   ├── device.rs        # オーディオデバイス列挙・選択 (cpal)
│   ├── stream.rs        # オーディオストリーム管理 (cpal callback)
│   ├── channel.rs       # UI↔オーディオスレッド通信 (ringbuf)
│   ├── audio_file.rs    # オーディオファイル読み込み (hound, symphonia)
│   ├── mixer.rs         # マルチトラックミキシング
│   ├── export.rs        # WAVエクスポート（オフラインレンダリング）
│   └── effects/         # オーディオエフェクト
│       ├── mod.rs       # AudioEffect トレイト定義
│       ├── gain.rs      # ゲインエフェクト
│       └── filter.rs    # Biquadフィルター
│
├── midi/                # MIDI処理
│   ├── mod.rs           # MIDIシステム初期化
│   ├── device.rs        # MIDIデバイス列挙・接続管理 (midir)
│   ├── message.rs       # MIDIメッセージのパース・生成
│   └── sequence.rs      # MIDIシーケンスデータ構造
│
├── state/               # アプリケーション状態（UIとエンジンの橋渡し）
│   ├── mod.rs           # グローバル状態管理
│   ├── project.rs       # プロジェクト保存/読み込み (serde + bincode)
│   └── track.rs         # トラック状態定義
│
├── mcp/                 # MCPサーバー（AI統合）
│   ├── mod.rs           # MCPサーバー起動・ルーティング
│   ├── channel.rs       # MCPとUI間の通信チャンネル (crossbeam-channel)
│   ├── transport.rs     # トランスポート操作ハンドラ
│   └── tracks.rs        # トラック操作ハンドラ
│
└── util/                # 共通ユーティリティ
    └── mod.rs           # 定数、ヘルパー関数
```

## スレッドモデル

```
┌─────────────────────┐
│   UIスレッド (main)  │  ← eframe/egui が占有
│   - 描画             │
│   - ユーザー入力     │
│   - 状態表示         │
└──────────┬──────────┘
           │ ringbuf / atomic
           │ (lock-free)
┌──────────▼──────────┐
│ オーディオスレッド    │  ← cpal callback が占有
│   - サンプル生成     │
│   - ミキシング       │
│   - エフェクト処理   │
│   ⚠️ アロケーション禁止 │
└─────────────────────┘

┌─────────────────────┐
│ Tokioランタイム      │  ← 別スレッドで起動
│   - MCPサーバー      │
│   - 非同期ファイルI/O│
└─────────────────────┘
```

## 重要な設計原則

### 1. UIとエンジンの分離
- `ui/` モジュールは `engine/` を直接呼び出さない
- `state/` を介してデータを受け渡す
- 通信は lock-free な仕組みのみ使用

### 2. リアルタイム安全性
オーディオコールバック内で禁止:
- ヒープアロケーション（`Vec::new()`, `Box::new()`, `String`, `format!()`）
- `Mutex::lock()`, `RwLock`
- ファイルI/O, ネットワークI/O
- `println!()`, ログ出力
- パニック（`.unwrap()` 禁止、`.unwrap_or_default()` を使用）

### 3. モジュールの責務
- 1ファイル = 1つの明確な責務
- 300行を超えたら分割を検討
- public API には日本語 doc コメントを付与
