## 🛡️ Warden 巡回報告 — 2026-05-29

### 発見事項
- ⚠️ `frontend/src-tauri/src/engine/mod.rs`: `EngineHandle` 構造体内の `midi_route_tx` に `Arc<std::sync::Mutex<MidiRouteProducer>>` が使われています。オーディオコールバックやリアルタイム処理に関連する部分での `Mutex` の使用はリアルタイム制約に違反する懸念があります（ロックフリーのデータ構造への変更を推奨）。
- ⚠️ `opendaw-wasm/src/midi/mapping.rs`: `SharedMidiMappingRegistry` で `Arc<Mutex<MidiMappingRegistry>>` が使われています。こちらもオーディオ処理に巻き込まれる場合、同様にリアルタイム制約違反の恐れがあります。
