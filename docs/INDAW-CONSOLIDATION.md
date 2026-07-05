# indaw 統合記録 (DAW 系リポジトリの一本化)

日付: 2026-07-05

## 経緯

DAW 実験が 2 リポジトリに分散していたため、opendaw に一本化した。

- **opendaw**(本リポジトリ): Google Jules 検証で積み上げた成果物。4 体の AI エージェント
  (Architect / Nova / Warden / Scribe)による「盆栽スタイル」自律開発の実験場。
  428+ コミット。Rust + WASM + Svelte フロントエンド。
- **indaw**(旧 `adasima/indaw`、private): Antigravity(agy)検証で積み上げた成果物。
  Tauri 2 + SvelteKit(Svelte 5)+ Tailwind 4 の 2 コミット構成。

どちらも「AI エージェントに DAW を作らせる検証」の副産物(スパゲッティ)であり、
目的が同一のため統合を決定(2026-07-05)。

## 統合方法

- indaw の全履歴を `git subtree` で `experiments/indaw/` に取り込み。
- 旧 `adasima/indaw` リポジトリはアーカイブ。

## それぞれの見どころ(成果の要点)

### opendaw(Jules 検証)
- 複数 AI エージェントのファイル管轄分離による並行自律開発の体制設計
  (ROADMAP 駆動、毎時 PR、CI 通過で自動マージ)
- グラスモーフィズム UI、MCP サーバー機能内蔵という AI ネイティブ DAW の方向性

### indaw(agy 検証) → `experiments/indaw/`
- Tauri 2 + Svelte 5 (runes) + Tailwind 4 というモダン構成の最小 DAW 骨格
- Rust→TS の型バインディング生成(`src/lib/bindings/`、ts-rs 系)
- コンポーネント分割が素直: PianoRoll / Mixer / WaveEditor / ChannelStrip /
  TimelineRuler / SplitPane 等、UI 部品の切り方は再利用価値あり
- i18n(ja/en)とキーバインド設定ストアの小さな実装例

## 今後

- 開発の主軸は opendaw 本体。indaw 側は参照用スナップショットとして凍結。
- indaw の UI 部品や型バインディング手法は必要に応じて opendaw 側へ移植する。
