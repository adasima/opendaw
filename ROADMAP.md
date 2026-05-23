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

## Phase 11: MIDIノートとシンセサイザーの連携 (community) (完了)
- [x] nova: [1] @A src/engine/channel.rs を更新し、アクティブなMIDIノートの周波数を送受信するメッセージを追加する (対象: src/engine/channel.rs)
- [x] nova: [2] @A src/engine/stream.rs を更新し、受信したノート情報をTrackMixDataに含める処理を実装する (対象: src/engine/stream.rs)
- [x] nova: [3] @A src/engine/mixer.rs を更新し、TrackMixDataのノート情報でシンセサイザーを発音させるロジックを実装する (対象: src/engine/mixer.rs)
- [x] nova: [4] @B src/app.rs を更新し、プレイヘッド位置から現在アクティブなノートを判定してオーディオエンジンに送信するロジックを実装する (対象: src/app.rs)

## Phase 12: ADSRエンベロープと波形選択 (community) (完了)
- [x] [1] @A src/engine/synth.rs に波形(Sine, Square, Sawtooth)の列挙型と ADSR エンベロープの構造体を実装する (対象: src/engine/synth.rs)
- [x] [2] @A src/engine/synth.rs などの関連ファイルを更新し、オシレーターで波形を生成し、エンベロープを適用する処理を実装する (対象: src/engine/synth.rs)
- [x] [3] @B src/state/track.rs に ADSR パラメータと波形選択の設定を保持するフィールドを `SynthSetting` に追加し、デフォルト値を設定する (対象: src/state/track.rs)

## Phase 13: UIからのパラメータ制御 (community) (完了)
- [x] [1] @A src/engine/channel.rs に `UpdateSynthParams` メッセージを追加し、UIとエンジンの通信を拡張する (対象: src/engine/channel.rs)
- [x] [2] @A src/engine/stream.rs を更新し、受信した `UpdateSynthParams` を各トラックのオシレーターに反映させる (対象: src/engine/stream.rs)
- [x] [3] @B src/ui/mixer.rs を更新し、各トラックの波形選択(ComboBox)と ADSR パラメータ(Slider)のUIを実装する (対象: src/ui/mixer.rs)
- [x] [4] @B src/app.rs を更新し、UIで変更されたシンセサイザーのパラメータをポーリングしてオーディオエンジンに送信する (対象: src/app.rs)

## Phase 14: セッションビュー (Ableton Liveライク) の導入 (AI実装完了・人間の実装待ち)
> ⚠️ **ハイブリッド開発**: データ構造と同期ロジック基盤は人間が直接コミットします。AI(Jules)はUIの繋ぎ込みを担当してください。
- [ ] 人間: `Clip`, `Scene` データ構造の設計とコア基盤の実装 (対象: src/core/session.rs など)
- [x] [1] @A src/ui/session_view.rs を作成し、ダミーデータを用いてセッションビューのスケルトンUI（クリップグリッド）を描画する (対象: src/ui/session_view.rs, src/ui/mod.rs)
- [x] [2] @B src/app.rs を更新し、メイン画面にセッションビューを統合する (対象: src/app.rs)

## Phase 15: モダン・プラグインホスト (VST3 / CLAP) の導入 (AI実装完了・人間の実装待ち)
> ⚠️ **ハイブリッド開発**: VST3/CLAPのFFIなど複雑な実装は人間が直接コミットします。AIはブラウザやUIを担当してください。
- [ ] 人間: `vst3-sys` 等を用いたプラグインロードの安全なラッパー層の実装 (対象: src/plugin/host.rs)
- [x] [1] @A src/ui/browser.rs を作成し、ダミーデータを用いてプラグイン一覧を表示するブラウザパネルUIのスケルトンを実装する (対象: src/ui/browser.rs, src/ui/mod.rs)
- [x] [2] @B src/app.rs を更新し、メイン画面または新規ウィンドウとしてプラグインブラウザパネルを統合する (対象: src/app.rs)

## Phase 16: メトロノーム機能の追加 (完了)
- [x] [1] @A src/engine/metronome.rs を作成し、BPMと再生位置に基づいてクリック音を生成する機能を実装する (対象: src/engine/metronome.rs, src/engine/mod.rs)
- [x] [2] @A src/engine/mixer.rs を更新し、メトロノーム音をマスター出力にミックスする処理を追加する (対象: src/engine/mixer.rs)
- [x] [3] @B src/state/mod.rs の `DawState` に `is_metronome_enabled` を追加する (対象: src/state/mod.rs)
- [x] [4] @B src/ui/transport.rs を更新し、メトロノームのオン/オフを切り替えるUIボタンを追加する (対象: src/ui/transport.rs)

## Phase 17: オーディオクリップの録音と波形表示 (完了)
- [x] [1] @A src/engine/recording.rs を作成し、オーディオ入力の録音処理とバッファ管理を実装する (対象: src/engine/recording.rs, src/engine/mod.rs)
- [x] [2] @B src/state/clip.rs を作成し、録音されたオーディオデータのメタデータ(長さ、波形サマリー等)を保持する構造体を実装する (対象: src/state/clip.rs, src/state/mod.rs)
- [x] [3] @B src/ui/timeline.rs を更新し、録音済みオーディオクリップの波形描画を実装する (対象: src/ui/timeline.rs)
- [x] [4] @B src/ui/transport.rs を更新し、録音ボタンの追加と録音状態の切り替えUIを実装する (対象: src/ui/transport.rs)


## Phase 18: 録音済みオーディオクリップの再生 (完了)
- [x] [1] @A src/engine/channel.rs を更新し、録音したデータをエンジンのバッファに送信するメッセージを追加する (対象: src/engine/channel.rs)
- [x] [2] @A src/engine/stream.rs を更新し、録音したオーディオデータの再生処理を実装する (対象: src/engine/stream.rs)
- [x] [3] @B src/app.rs を更新し、録音停止時にキャプチャしたオーディオデータを新しいクリップとしてトラックに追加する処理を実装する (対象: src/app.rs)

## Phase 19: エフェクトプラグイン対応 (完了)
- [x] [1] @A src/engine/effects/mod.rs に新しいエフェクト（例：ディレイ、リバーブ）の構造体とトレイト実装を追加する (対象: src/engine/effects/mod.rs, src/engine/effects/delay.rs)
- [x] [2] @B src/ui/effects.rs を更新し、新しいエフェクトのパラメータを調整するUIコンポーネントを追加する (対象: src/ui/effects.rs)
- [x] [3] @A src/engine/channel.rs にエフェクトパラメータ更新用のメッセージを追加し、UIからの変更をエンジンに伝達できるようにする (対象: src/engine/channel.rs)
- [x] [4] @A src/engine/mixer.rs または src/engine/stream.rs を更新し、エフェクトをオーディオパイプラインに組み込む (対象: src/engine/mixer.rs, src/engine/stream.rs)

## Phase 20: ピアノロール機能の拡充とMIDIクリップ編集 (進行中)
- [ ] [1] @A src/midi/sequence.rs を更新し、ノートの追加・削除・移動（位置・長さ・ベロシティ）を管理するメソッドを実装する (対象: src/midi/sequence.rs)
- [ ] [2] @A src/state/clip.rs に `MidiClip` 構造体を追加し、ノート列（シーケンス）とクリップ長等のメタデータを保持する (対象: src/state/clip.rs, src/state/mod.rs)
- [ ] [3] @B src/ui/piano_roll.rs を更新し、ノートの追加（マウスクリック）、削除、ドラッグでの移動/長さ変更を可能にするUIインタラクションを実装する (対象: src/ui/piano_roll.rs)
- [ ] [4] @B src/app.rs を更新し、ピアノロールUIでの変更を `state` の `MidiClip` に反映し、再生エンジン（アクティブノート）と同期する (対象: src/app.rs)
