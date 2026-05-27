# Transport Control

> Features to control playback, stopping, recording, looping, and tempo (BPM) of your track.

## Overview
The transport control is necessary to manipulate the playback position (playhead) on the timeline and manage the playback state of the project. It is located in the main panel at the bottom center of the screen.

## Basic Usage
1. Click the **⏺ (Record)** button to toggle recording mode. While recording, the button changes to a red **⏺ (On)**.
2. Click the **▶ (Play)** button to start playback. While playing, the button changes to **⏸ (Pause)**.
3. Click the **⏸ (Pause)** button to pause playback.
4. Click the **⏹ (Stop)** button to stop playback and return the playhead to the beginning (0.0).
5. Click the **🔁 (Loop)** button to toggle loop playback on and off. When loop is on, playback will automatically restart from the beginning when it reaches the end of the timeline.
6. Click the **⏱ (Metronome)** button to toggle the metronome on and off.

## Detailed Settings

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| BPM | The tempo of the project (beats per minute). Affects playback speed. | 120.0 | 20.0 to 300.0 |
| Loop | When turned on, playback continues from the beginning upon reaching the end of the timeline. | On | On / Off |
| Metronome | The enabled/disabled state of the metronome. | Off | On / Off |
| Recording | The enabled/disabled state of recording. | Off | On / Off |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated to transport operations.

## See Also
- [Timeline & Waveform](../user-guide/timeline.md)
- [Metronome](../user-guide/metronome.md)
- [Grid Snap](../user-guide/grid-snap.md)

## Notes and Limitations
> In the current version, changing the BPM simply affects the overall playback speed proportionally. Also, the playhead position is managed as a percentage (0.0 to 100.0), and the virtual time (Time) is calculated and displayed based on the playhead position.

## Grid Settings
A feature that snaps the placement and movement of clips and MIDI notes on the timeline and piano roll to a specified resolution (beats or measures).

1. Click the **Grid Button** in the transport panel to toggle grid snap on or off.
2. Select the snapping unit from the adjacent **Resolution Dropdown** (e.g., `1/4`, `1/8`, `1/16`).
3. When the grid is enabled, dragging clips or adding, moving, and resizing notes in the piano roll will automatically snap to the selected resolution.
