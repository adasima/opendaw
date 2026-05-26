# Changelog

> This document records the update history of OpenDAW.

## Phase 25-29: Feature Integration and Enhancement for Tauri + WASM Architecture

### ✨ New Features
- **Project Save/Load Functionality**: You can now save and restore the project's BPM, master volume, and added track states as `.aura` files.
- **MIDI Clip Management and Piano Roll Integration**: MIDI clips can now be added, removed, and moved from the timeline, and piano roll edits are now synchronized with the Tauri backend.
- **Audio Clip Management Integration**: You can now add, move, and remove audio clips from the Svelte frontend and WASM UI, expanding the state synchronization logic with the Tauri backend.

### 🚀 Improvements
- **State Synchronization from Svelte UI**: Transport operations (play, stop, etc.) and track management (volume, add, remove, etc.) are now instantly reflected in the `ProjectState` of the Tauri backend.
- **Timeline Integration with Waveform and Note Drawing**: Based on the project state sent from Tauri, audio waveforms and MIDI notes are now drawn on the WASM canvas.
- **Real-time Performance Improvements**: To eliminate lock contention in the audio thread, state synchronization for MIDI routing and other features has been migrated to lock-free data structures.

## Upcoming Features
- Full implementation of the Session View (Clip/Scene data structures)
- Implementation of a safe wrapper layer for loading VST3/CLAP plugins
