# Piano Roll

> A built-in sequencer interface that allows you to intuitively input and edit MIDI notes using your mouse.

## Overview
The Piano Roll editor is a feature for visually creating and editing melodies, chords, and rhythm patterns. It allows you to graphically adjust the placement and duration of notes on a grid-based interface. It also supports displaying ARA2 / SV2 lyrics and pitch bend curves.

## Basic Usage
1. **Add Note**: Left-click on any empty space on the piano roll grid (adds a 16th note by default).
2. **Remove Note**: Right-click on an existing note you wish to delete.
3. **Move Note**: Drag the center of a note to change its position (time) or pitch.
4. **Resize Note**: Drag the right edge of a note to change its duration.

## View Navigation
1. **Scroll (Panning)**: Use the mouse wheel, or Middle-click and drag.
2. **Zoom**: `Ctrl` + Mouse wheel to zoom in and out horizontally.

## Detailed Settings
In the current version, the piano roll interface operates based on the following specifications:

| Parameter | Description | Default | Range |
|:---|:---|:---|:---|
| Pitch | Musical scale (MIDI note number) | C-1 - G9 | 0 - 127 |
| Velocity | Strength of the note | 100 | Fixed |
| Duration | Note length | 16th note | Adjustable |

## Related
- [MIDI Reference](../reference/midi-reference.md)
- [Tracks](tracks.md)
- [Grid Snap](grid-snap.md)

## Notes & Limitations
> - Currently, when a note is added, its velocity (100) is set as a fixed value.

## Grid Snap
When adding, moving, or resizing notes on the piano roll, if the grid setting is enabled in the transport panel, the operation will snap to the selected resolution.
