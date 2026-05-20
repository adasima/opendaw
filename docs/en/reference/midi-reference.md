# MIDI Reference

> A reference for MIDI processing, supported messages, and sequence structures in OpenDAW.

## Overview
OpenDAW supports receiving and parsing messages from external MIDI input devices, as well as managing internal sequences used in the piano roll editor. This document describes the implemented MIDI-related specifications and data structures.

## MIDI Input Devices
OpenDAW detects and connects to MIDI devices on the system using the `midir` crate.
- It creates a virtual input port named `OpenDAW MIDI Input` and establishes a connection.
- Received MIDI messages are processed via asynchronous channels.

## Supported MIDI Messages
The MIDI messages (`MidiMessage`) that can currently be parsed and interpreted are listed below. Other messages are treated as `Unknown` and ignored or kept as raw data.

| Message | Overview | Extracted Data |
|:---|:---|:---|
| Note On | Start of a note | Channel, Note Number, Velocity |
| Note Off | End of a note | Channel, Note Number, Velocity |
| Control Change | Control change | Channel, Control Number, Value |
| Pitch Bend | Pitch change | Channel, Value (14-bit) |

> * Note: A `Note On` message with a velocity of `0` is automatically parsed as a `Note Off` message.

## MIDI Sequence Data Structure
MIDI sequences edited in the piano roll are managed internally as a list of `NoteEvent`s. Each note has the following properties:

| Property | Type | Description |
|:---|:---|:---|
| id | `usize` | Unique identifier for the note (auto-incremented) |
| pitch | `u8` | MIDI note number (0-127) |
| velocity | `u8` | Velocity (0-127) |
| start_beat | `f64` | The start position of the note (in beats) |
| duration_beats | `f64` | The length of the note (in beats) |

The overall `Sequence` structure holds the above `NoteEvent`s as a list and provides operations such as adding, removing, and clearing notes.

## Keyboard Shortcuts
Currently, there are no specific keyboard shortcuts for MIDI settings.

## Related Topics
- [Piano Roll Editor](../user-guide/piano-roll.md)

## Notes & Limitations
> - In the current version, MIDI output (sending messages to external hardware) is not supported.
> - System Exclusive (SysEx) messages are treated as `Unknown` and are not parsed.
