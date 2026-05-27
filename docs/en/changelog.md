# Changelog

> This document records the update history of OpenDAW.

## Phase 25-32: Feature Integration and Enhancement for Tauri + WASM Architecture

### ✨ New Features
- **Undo/Redo Functionality**: You can now revert and re-apply project editing actions (such as moving, adding, or deleting clips).
- **Grid Snap and Quantization**: You can now snap clips and notes to a specified resolution (such as quarter notes) when placing them on the timeline or piano roll.
- **Project Save/Load Functionality**: You can now save and restore the project's BPM, master volume, and added track states as `.aura` files.
- **MIDI Clip Management and Piano Roll Integration**: MIDI clips can now be added, removed, and moved from the timeline, and piano roll edits are now synchronized with the Tauri backend.
- **Audio Clip Management Integration**: You can now add, move, and remove audio clips from the Svelte frontend and WASM UI, expanding the state synchronization logic with the Tauri backend.

### 🚀 Improvements
- **Backend Command Restructuring**: Tauri commands have been refactored into feature-specific modules (e.g., project.rs, track.rs, clip.rs) for improved maintainability.
- **State Synchronization from Svelte UI**: Transport operations (play, stop, etc.) and track management (volume, add, remove, etc.) are now instantly reflected in the `ProjectState` of the Tauri backend.
- **Timeline Integration with Waveform and Note Drawing**: Based on the project state sent from Tauri, audio waveforms and MIDI notes are now drawn on the WASM canvas.
- **Real-time Performance Improvements**: To eliminate lock contention in the audio thread, state synchronization for MIDI routing and other features has been migrated to lock-free data structures.

## Upcoming Features
- Full implementation of the Session View (Clip/Scene data structures)
- Implementation of a safe wrapper layer for loading VST3/CLAP plugins
