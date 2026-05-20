# OpenDAW ロードマップ
> このファイルは Architect エージェントが管理します。Nova / Warden は変更しないでください。
## Phase 10: シンセサイザー (Software Instrument) の基盤実装 (進行中)
- [x] [1] @A src/engine/synth.rs を作成し、基本的なオシレータ(サイン波)を実装する (対象: src/engine/synth.rs, src/engine/mod.rs)
- [x] [2] @A src/engine/mixer.rs を更新し、シンセサイザーの出力をミックスできるようにする (対象: src/engine/mixer.rs)
- [ ] [3] @B src/state/track.rs を更新し、トラックにシンセサイザー情報を保持できるようにする (対象: src/state/track.rs)
- [ ] [4] @B src/state/project.rs を更新し、シンセサイザー状態を含むプロジェクトのシリアライズ・デシリアライズを実装する (対象: src/state/project.rs)
- [ ] [5] @B src/ui/tracks.rs を更新し、シンセサイザートラックを追加するUIボタンを実装する (対象: src/ui/tracks.rs)
- [ ] [6] @B src/ui/mixer.rs を更新し、各トラックのシンセサイザーパラメータ(周波数など)を調整するUIを実装する (対象: src/ui/mixer.rs)
