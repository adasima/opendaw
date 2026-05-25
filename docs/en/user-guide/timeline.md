# Timeline & Waveform

> Features for displaying the track's waveform, checking the playback position (playhead), performing seek operations, and moving clips.

## Overview
The main timeline is used to visually grasp the overall flow of the project. It displays the playhead indicating the current playback position and audio clips (e.g., recorded waveform data) placed on each track.

## Basic Usage
### Playhead Operation (Seek)
1. **Click** or **drag on an empty space on the timeline** to move (seek) the playhead to that position.
2. The track's playback position is instantly changed based on the clicked position (percentage relative to the width).

### Clip Operation
1. You can change the start position (time) of an **audio clip by dragging it** across the track.
2. While moving a clip, its position is updated in real-time on the UI, and the state is synchronized with the backend (Tauri) at the moment you finish dragging (drop).

## Detailed Settings
There are currently no detailed settings specific to this panel, but its behavior depends on the following parameter.

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Playhead Position | The current position of the playhead (percentage of the entire timeline). | 0.0 | 0.0 to 100.0 |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated to timeline operations.

## See Also
- [Transport Control](../user-guide/transport.md)
- [Recording](../user-guide/recording.md)
- [Clips](../user-guide/clips.md)

## Notes and Limitations
> The playhead position is managed as a percentage (0.0 to 100.0).
