# Changelog

> This document records the update history of OpenDAW.

## Phase 30-32: Foundation Integration for Undo/Redo, Grid Snap, and Plugins

### ✨ New Features
- **Sub Bus / Send & Return Routing**: The ability to change the output destination for each track, and route sends to other tracks (adding send destinations and adjusting send amounts) has been added and can now be controlled from the Svelte UI Track Details panel.
- **Undo / Redo Introduction**: A history management foundation for the project state has been built, allowing for the undoing and redoing of actions such as adding or moving clips.
- **Grid Snap Feature**: A feature has been implemented to snap the placement of clips and MIDI notes on the timeline and piano roll to a specified resolution (such as beats and measures).
- **Frontend Integration for Plugin Hosting**: The operation of loading VST3 / CLAP plugins from the plugin browser onto a track is now coordinated between the Svelte UI (browser and track details panels) and the Tauri backend.

### 🚀 Improvements
- **Backend API Refactoring**: Tauri Commands that were previously concentrated in `app.rs` have been split and organized into function-specific modules such as `project.rs`, `track.rs`, `clip.rs`, `transport.rs`, and `plugin.rs`.

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
