# Timeline & Waveform

> Features for displaying the track's waveform, checking the playback position (playhead), and performing seek operations.

## Overview
The main timeline is used to visually grasp the overall flow of the project. It displays the playhead indicating the current playback position and the audio waveform (currently a mock representation).

## Basic Usage
1. **Click** or **drag on the timeline** to move (seek) the playhead to that position.
2. The track's playback position is instantly changed based on the clicked position (percentage relative to the width).

## Detailed Settings
There are currently no detailed settings specific to this panel, but its behavior depends on the following parameter.

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Playhead Position | The current position of the playhead (percentage of the entire timeline). | 0.0 | 0.0 to 100.0 |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated to timeline operations.

## See Also
- [Transport Control](../user-guide/transport.md)

## Notes and Limitations
> In the current version, the displayed waveform is a placeholder (mock) and not based on actual audio data. The playhead position is managed as a percentage (0.0 to 100.0).
