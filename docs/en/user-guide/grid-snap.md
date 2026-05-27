# Grid Snap

> A feature that snaps the placement and movement of clips and MIDI notes on the timeline and piano roll to a specified resolution.

## Overview
The Grid Snap feature allows you to place clips and input MIDI notes with precise timing (such as beats and measures). It prevents timing deviations and supports the creation of rhythmic and accurate music.

## Basic Usage
1. Click the **Grid Button** in the transport panel to toggle grid snap on or off.
2. Select the snapping unit (resolution) from the adjacent **Resolution Dropdown** (e.g., `1/4`, `1/8`, `1/16`).
3. When the grid is enabled, the following operations are automatically adjusted (snapped) according to the selected resolution:
   - Dragging clips (Audio/MIDI) on the timeline
   - Adding MIDI notes on the piano roll
   - Moving MIDI notes on the piano roll
   - Changing the length of MIDI notes on the piano roll

## Detailed Settings

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Grid Snap | Enabled/disabled state of the grid snap | On | On / Off |
| Resolution | The unit (resolution) for snapping | 1/4 (Quarter note) | 1/1, 1/2, 1/4, 1/8, 1/16, etc. |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated to grid snap operations.

## See Also
- [Transport Control](../user-guide/transport.md)
- [Timeline & Waveform](../user-guide/timeline.md)
- [Piano Roll Editor](../user-guide/piano-roll.md)

## Notes and Limitations
> - The snap feature is applied when the operation finishes (e.g., when dropping a drag or clicking to add a note).
> - Turning the grid off allows for free placement and movement in very fine increments.
