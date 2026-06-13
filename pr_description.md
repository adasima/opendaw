💡 **What:** Refactored `ProjectState` to use `Vec<Arc<Track>>` instead of `Vec<Track>`, creating a Copy-on-Write (CoW) pattern via `std::sync::Arc::make_mut` for track mutations.
🎯 **Why:** To drastically reduce the performance overhead when creating undo/redo snapshots. Deep cloning an entire large project every action could introduce major UI freezes.
📊 **Measured Improvement:** In a benchmark simulating a project with 100 tracks and 1000 clips per track, the time to clone the `ProjectState` dropped from ~52.8 ms per clone down to ~8.6 µs per clone (more than a 6000x speedup).
