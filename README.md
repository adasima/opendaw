# 🌌 OpenDAW

> **Discordライクな直感性 × グラスモーフィズムUI × AI自律駆動**
> 従来の常識を破壊する、ポータブル＆次世代型のAIネイティブDAW（Digital Audio Workstation）。

---

## 🤖 AI Autonomous Development — 自律盆栽エコシステム

このリポジトリは、人間の大まかなビジョンのもと、**4体のAIエージェントが24時間体制で自律的にコードを書き、設計し、テストし、ドキュメントを整備し、マージを繰り返す「盆栽スタイル」**で開発されています。

| エージェント | 役割 | 頻度 | 管轄 |
|:---:|:---|:---:|:---|
| 🏗️ **Architect** | 設計・計画・ロードマップ管理 | 4時間ごと | `ROADMAP.md`, `ARCHITECTURE.md`, `DESIGN.md` |
| 🚀 **Nova** | 新機能の追加・UX改善 | 毎時 | `src/`, `Cargo.toml` |
| 🛡️ **Warden** | コード品質・パフォーマンス・安全性 | 4時間ごと | `src/` |
| 📝 **Scribe** | ユーザーマニュアルの整備（日英対応） | 4時間ごと | `docs/` |

### 仕組み

```
Architect が ROADMAP.md で次の目標を定める
    ↓
Nova が ROADMAP に従い、毎時1つの小さな機能をPR
    ↓
Warden が Nova のコードを巡回し、品質を担保
    ↓
Scribe が実装済み機能のドキュメントを日英で整備
    ↓
全PRは CI通過後に自動マージ（コンフリクト時は安全にキャンセル）
```

> 各エージェントは**ファイルの管轄が分離**されているため、並行実行してもコンフリクトしません。
> 詳しくは [CONTRIBUTING.md](CONTRIBUTING.md) を参照してください。

---

## ✨ 主な特徴 (Features)

- 🎨 **Glassmorphic UI** — Discordにインスパイアされた、半透明の美しいグラスモーフィズムと動的なライティング。従来の重厚なミキサー画面を撤廃。
- ⚡ **Ultra Lightweight & Portable** — インストール不要。数MBの単一バイナリをダブルクリックするだけで一瞬起動。
- 🦀 **Powered by Rust & egui** — ミリ秒以下の超低遅延オーディオ処理のため、Rustスタックを全面採用。
- 🤖 **AI Agent統合** — MCPサーバー内蔵により、AIエージェントからDAWを直接操作可能。
- 📖 **日英ドキュメント完備** — 商用ソフトウェア水準のユーザーマニュアルを自動整備。

---

## 🚀 はじめかた (Quick Start)

### 💻 バイナリで今すぐ試す（おすすめ）
1. [Releases](https://github.com/adasima/opendaw/releases) から最新の `opendaw-portable-windows.zip` をダウンロードします。
2. ZIPを解凍し、中にある `opendaw.exe` をダブルクリックするだけで起動します。

### 🛠️ ソースコードからビルドする
Rust環境がお手元にある場合は、以下のコマンドでビルドできます。
```bash
git clone https://github.com/adasima/opendaw.git
cd opendaw
cargo run --release
```

---

## 📐 プロジェクト構造

| ファイル / ディレクトリ | 説明 |
|:---|:---|
| [`ROADMAP.md`](ROADMAP.md) | 開発ロードマップ（フェーズ・タスク管理） |
| [`ARCHITECTURE.md`](ARCHITECTURE.md) | モジュール構造・スレッドモデル・設計原則 |
| [`DESIGN.md`](DESIGN.md) | UIデザインシステム（美学・レイアウト） |
| [`CONTRIBUTING.md`](CONTRIBUTING.md) | コントリビューションガイド |
| `src/` | ソースコード（Rust） |
| `docs/ja/` | ユーザーマニュアル（日本語・正本） |
| `docs/en/` | ユーザーマニュアル（英語・翻訳版） |

---

## 🗺️ ロードマップ

現在の進捗は [`ROADMAP.md`](ROADMAP.md) で管理されています。

| Phase | 内容 | 状態 |
|:---:|:---|:---:|
| 1 | UI基盤 & アーキテクチャ | 🔄 進行中 |
| 2 | オーディオエンジン基盤 | ⏳ 次 |
| 3 | マルチトラック & ミキシング | ⏳ |
| 4 | MIDI & シーケンシング | ⏳ |
| 5 | エフェクト & プロセッシング | ⏳ |
| 6 | プロジェクト管理 & エクスポート | ⏳ |
| 7 | AI統合 & MCP | ⏳ |

---

## 💬 フィードバック・要望

機能の要望やバグ報告は **Issue** で受け付けています！

👉 [新しいIssueを作成する](https://github.com/adasima/opendaw/issues/new)

Issue に記載いただいた内容は、AIエージェント（Architect）が定期的に確認し、ロードマップに取り込みます。

> ⚠️ このリポジトリはAIエージェントが24時間自律開発しています。
> **外部からのPRは自動的にクローズされます。** 詳しくは [`CONTRIBUTING.md`](CONTRIBUTING.md) を参照してください。

---

## 📜 ライセンス

[MIT License](LICENSE)
