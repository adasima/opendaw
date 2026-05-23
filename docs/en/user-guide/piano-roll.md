# Piano Roll

> A built-in sequencer interface that allows you to intuitively input and edit MIDI notes using your mouse.

## Overview
The Piano Roll editor is a feature for visually creating and editing melodies, chords, and rhythm patterns. It allows you to graphically adjust the placement and duration of notes on a grid-based interface.

## Basic Usage
1. **Add Note**: Left-click on any empty space on the piano roll grid.
2. **Move Note**: Left-click and drag an existing note to change its position or pitch.
3. **Remove Note**: Right-click on an existing note you wish to delete.
4. **Pan/Scroll**: Scroll with your mouse wheel while hovering, or middle-click and drag to move the visible area.
5. **Zoom**: Hold the `Ctrl` key and scroll the mouse wheel to zoom in and out on the timeline.

## Detailed Settings
In the current version, the piano roll interface operates based on the following specifications:

| Parameter | Description | Default | Range |
|:---|:---|:---|:---|
| Pitch | Musical scale (MIDI note number) | Centered around C4 | 0 - 127 |
| Velocity | Strength of the note | N/A (Not Implemented) | Fixed |
| Duration | Note length | 16th note equivalent | Fixed |

## Related
- [MIDI Reference](../reference/midi-reference.md)
- [Tracks](tracks.md)

## Notes & Limitations
> - Currently, when a note is added, its duration is fixed to a 16th note equivalent.
> - Features to change the duration of notes by dragging their edges are planned for future updates.
> - Note velocity is currently not implemented in the piano roll.
