---
description: デバッグ・開発用タスク (Antigravity統合)
---
// turbo-all

# INDAW デバッグワークフロー

## 1. 開発サーバー起動
```powershell
cd c:/Users/sonnp/Desktop/indaw
npm run tauri dev
```

## 2. Rust ビルドチェック (ホットリロード前)
```powershell
cd c:/Users/sonnp/Desktop/indaw/src-tauri
cargo check
```

## 3. フロントエンドチェック
```powershell
cd c:/Users/sonnp/Desktop/indaw
npm run check
```

## 4. デバッグパネル確認
アプリ内の **TopBar** に「🐛 Debug」ボタンがある場合:
- クリックでデバッグパネル開閉
- 現在の `AudioState` をJSON表示
- Track一覧とプロパティ確認
- 手動コマンド実行

## 5. コンソールログ確認
- **Rust側**: ターミナル出力 (`println!`, `eprintln!`)
- **フロント側**: DevTools Console (F12)

## 6. よくある問題

### 音が出ない
1. オーディオデバイス確認
2. Track 0 が Mute になっていないか
3. `is_playing` が true になっているか (デバッグパネル)

### UI が更新されない
1. `startSync()` が呼ばれているか確認
2. DevTools で `audioStore` を監視

### パニック
1. Rust 出力でスタックトレース確認
2. 最近の変更箇所を確認
3. `try_read()`/`try_write()` を使用しているか

## 7. ログレベル設定 (Rust)
```rust
// main.rs や lib.rs で環境変数設定
// RUST_LOG=debug cargo run
```
