# OpenDAW ロードマップ
> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。
## Phase 10: シンセサイザー (Software Instrument) の基盤実装 (進行中)
- [x] [1] @A src/engine/synth.rs を作成し、基本的なオシレータ(サイン波)を実装する (対象: src/engine/synth.rs, src/engine/mod.rs)
- [x] [2] @A src/engine/mixer.rs を更新し、シンセサイザーの出力をミックスできるようにする (対象: src/engine/mixer.rs)
- [ ] [3] @B src/state/track.rs を更新し、トラックにシンセサイザー情報を保持できるようにする (対象: src/state/track.rs)
- [ ] [4] @B src/state/project.rs を更新し、シンセサイザー状態を含むプロジェクトのシリアライズ・デシリアライズを実装する (対象: src/state/project.rs)
- [ ] [5] @B src/ui/tracks.rs を更新し、シンセサイザートラックを追加するUIボタンを実装する (対象: src/ui/tracks.rs)
- [ ] [6] @B src/ui/mixer.rs を更新し、各トラックのシンセサイザーパラメータ(周波数など)を調整するUIを実装する (対象: src/ui/mixer.rs)

---

## Phase 11: MIDIノートとシンセサイザーの連携 (community)
- [ ] ピアノロールに配置されたMIDIノートのタイミングで、シンセサイザーの `set_active` と `set_frequency` を切り替えるロジックを実装する

## Phase 12: ADSRエンベロープと波形選択 (community)
- [ ] オシレータの音切れを防ぐADSRエンベロープの実装
- [ ] Sine, Square, Sawtooth 波形の選択実装

## Phase 13: UIからのパラメータ制御 (community)
- [ ] トラックUIからADSRや波形を選択・変更できるグラフィカルなUIの実装

---

## Phase 14: セッションビュー (Ableton Liveライク) の導入
> ⚠️ **ハイブリッド開発**: データ構造と同期ロジック基盤は人間が直接コミットします。AI(Jules)はUIの繋ぎ込みを担当してください。
- [ ] 人間: `Clip`, `Scene` データ構造の設計とコア基盤の実装 (対象: src/core/session.rs など)
- [ ] Jules: eguiを用いたセッションビューのクリップグリッドUIの描画 (対象: src/ui/session_view.rs)

## Phase 15: モダン・プラグインホスト (VST3 / CLAP) の導入
> ⚠️ **ハイブリッド開発**: VST3/CLAPのFFIなど複雑な実装は人間が直接コミットします。AIはブラウザやUIを担当してください。
- [ ] 人間: `vst3-sys` 等を用いたプラグインロードの安全なラッパー層の実装 (対象: src/plugin/host.rs)
- [ ] Jules: プラグインをスキャンし一覧表示するブラウザパネルUIの実装 (対象: src/ui/browser.rs)
