# Piano Roll

> A built-in sequencer interface that allows you to intuitively input and edit MIDI notes using your mouse.

## Overview
The Piano Roll editor is a feature for visually creating and editing melodies, chords, and rhythm patterns. It allows you to graphically adjust the placement and duration of notes on a grid-based interface.

## Basic Usage
1. **Add Note**: Left-click on any empty space on the piano roll grid.
2. **Remove Note**: Right-click on an existing note you wish to delete.

## Detailed Settings
In the current version, the piano roll interface operates based on the following specifications:

| Parameter | Description | Default | Range |
|:---|:---|:---|:---|
| Pitch | Musical scale (MIDI note number) | C3 - C5 | 48 - 72 |
| Velocity | Strength of the note | 100 | Fixed |
| Duration | Note length in beats | 1.0 | Fixed |
| View Range | Visible area in beats | 16.0 | Fixed |

## Related
- [MIDI Reference](../reference/midi-reference.md)
- [Tracks](tracks.md)

## Notes & Limitations
> - Currently, when a note is added, its velocity (100) and duration (1.0 beats) are set as fixed values.
> - Features to move notes (drag and drop) or change their duration are planned for future updates.
> - Scrolling outside the view range is not yet implemented, so you can only edit the first 16 beats within the C3 to C5 range.
