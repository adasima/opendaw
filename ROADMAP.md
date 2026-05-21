# OpenDAW ロードマップ
> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。
## Phase 10: シンセサイザー (Software Instrument) の基盤実装 (完了)
- [x] [1] @A src/engine/synth.rs を作成し、基本的なオシレータ(サイン波)を実装する (対象: src/engine/synth.rs, src/engine/mod.rs)
- [x] [2] @A src/engine/mixer.rs を更新し、シンセサイザーの出力をミックスできるようにする (対象: src/engine/mixer.rs)
- [x] [3] @B src/state/track.rs を更新し、トラックにシンセサイザー情報を保持できるようにする (対象: src/state/track.rs)
- [x] [4] @B src/state/project.rs を更新し、シンセサイザー状態を含むプロジェクトのシリアライズ・デシリアライズを実装する (対象: src/state/project.rs)
- [x] [5] @B src/ui/tracks.rs を更新し、シンセサイザートラックを追加するUIボタンを実装する (対象: src/ui/tracks.rs)
- [x] [6] @B src/ui/mixer.rs を更新し、各トラックのシンセサイザーパラメータ(周波数など)を調整するUIを実装する (対象: src/ui/mixer.rs)

---

## Phase 11: MIDIノートとシンセサイザーの連携 (community) (進行中)
- [ ] nova: [1] @A src/engine/stream.rs でプレイヘッド位置に基づきアクティブなノートを判定するロジックを実装 (対象: src/engine/stream.rs)
- [ ] nova: [2] @A src/engine/mixer.rs で判定されたノートに基づきシンセサイザーの set_active, set_frequency を呼ぶロジックを追加 (対象: src/engine/mixer.rs)

## Phase 12: ADSRエンベロープと波形選択 (community)
- [ ] nova: [1] @A src/engine/synth.rs に波形(Sine, Square, Sawtooth)の列挙型と ADSR エンベロープの構造体および処理を実装 (対象: src/engine/synth.rs)
- [ ] nova: [2] @B src/state/track.rs に ADSR パラメータと波形選択の設定を保持するフィールドを `SynthSetting` に追加し、デフォルト値を設定 (対象: src/state/track.rs)

## Phase 13: UIからのパラメータ制御 (community)
- [ ] nova: [1] @B src/ui/mixer.rs を更新し、各トラックの波形選択(ComboBox)と ADSR パラメータ(Slider)を調整するUIを実装 (対象: src/ui/mixer.rs)

## Phase 14: セッションビュー (Ableton Liveライク) の導入
> ⚠️ **ハイブリッド開発**: データ構造と同期ロジック基盤は人間が直接コミットします。AI(Jules)はUIの繋ぎ込みを担当してください。
- [ ] 人間: `Clip`, `Scene` データ構造の設計とコア基盤の実装 (対象: src/core/session.rs など)
- [ ] Jules: eguiを用いたセッションビューのクリップグリッドUIの描画 (対象: src/ui/session_view.rs)

## Phase 15: モダン・プラグインホスト (VST3 / CLAP) の導入
> ⚠️ **ハイブリッド開発**: VST3/CLAPのFFIなど複雑な実装は人間が直接コミットします。AIはブラウザやUIを担当してください。
- [ ] 人間: `vst3-sys` 等を用いたプラグインロードの安全なラッパー層の実装 (対象: src/plugin/host.rs)
- [ ] Jules: プラグインをスキャンし一覧表示するブラウザパネルUIの実装 (対象: src/ui/browser.rs)
