# Clips

> Features related to "Clips", which are the fundamental units holding audio or MIDI data.

## Overview
Clips are the actual instances of audio data or MIDI sequences placed on tracks. In the current project, there are two types: "Audio Clips", which hold recorded waveform data, and "MIDI Clips", which hold note data editable in the Piano Roll.

## Basic Usage
1. **Selecting and Moving Clips**: You can change the start position (time or beats) by dragging the clip on the timeline.
2. While dragging, the move results are drawn in real-time on the UI, and the changed state is synchronized to the backend (Tauri) when the drag ends.

## Audio Clip
Represents recorded audio data or imported audio files.
- **ID**: A unique identifier for the clip.
- **Name**: The name of the clip (e.g., "Recorded Clip").
- **Start Position**: The start position on the timeline (e.g., in seconds).
- **Length**: The length of the clip (e.g., in seconds).
- **Waveform Summary**: Summary data used for drawing the waveform.

## MIDI Clip
Represents a collection of MIDI notes created and edited in the Piano Roll editor.
- **ID**: A unique identifier for the clip.
- **Name**: The name of the clip (e.g., "Synth Melody").
- **Start Beat**: The start position on the timeline (e.g., in beats).
- **Length**: The length of the clip (e.g., in beats).
- **Sequence**: Sequence data holding the placement and duration of notes.

## Related
- [Piano Roll](piano-roll.md)
- [Tracks](tracks.md)

## Notes & Limitations
> - In the current version, synchronization between the UI and backend (Tauri) via dragging clips works, but complete management functionality for MIDI clips and piano roll events is still partially under development (Phase 28).
