# 🌌 opendaw

> **Discordライクな直感性 × グラスモーフィズムUI × AI自律駆動**
> 従来の常識を破壊する、ポータブル＆次世代型のAIネイティブDAW（Digital Audio Workstation）。

---

## 🤖 AI Autonomous Development (自律盆栽エコシステム)

このリポジトリは、人間の大まかなビジョンのもと、**2人のAIエージェントが24時間体制で自律的にコードを書き、テストし、マージを繰り返す「盆栽スタイル」**で開発されています。

* **🚀 Nova (機能追加・UX改善担当):** 毎日AM 1:00〜6:00に1時間おきに出勤。Discordライクな美学を保ちながら、直感的なUIやマイクロ機能の追加、UXのブレークスルーをハントします。
* **🛡️ Warden (コードヘルス・堅牢さ担当):** 4時間ごとに1回巡回。Novaが追加したコードを厳しくチェックし、Rustの安全性を活かしたリファクタリング、速度の最適化（不要クローンの削除など）、安全性の向上を全自動で行います。

---

## ✨ 主な特徴 (Features)

- **🎨 Glassmorphic UI:** 従来の重厚で複雑なミキサー画面を撤廃。Discordにインスパイアされた、半透明の美しいグラスモーフィズムと動的なライティングを採用。
- **⚡ Ultra Lightweight & Portable:** インストール不要。数MBの単一バイナリをダブルクリックするだけで一瞬で起動する圧倒的機動性。
- **🦀 Powered by Rust & egui:** 音響の高速ビジュアライザーやミリ秒以下の超低遅延オーディオ処理のため、圧倒的なパフォーマンスを誇るRustスタックを全面採用。

---

## 🚀 はじめかた (Quick Start)

### 💻 バイナリで今すぐ試す（おすすめ）
1. [Releases](https://github.com/adasima/opendaw/releases) から最新の `opendaw-portable-windows.zip` をダウンロードします。
2. ZIPを解凍し、中にある `opendaw.exe` をダブルクリックするだけで起動します。

### 🛠️ ソースコードからビルドする
Rust環境がお手元にある場合は、以下のコマンドで爆速起動できます。
```bash
git clone [https://github.com/adasima/opendaw.git](https://github.com/adasima/opendaw.git)
cd opendaw
cargo run --release
