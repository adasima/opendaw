# Metronome

> Generates a click sound synchronized with the project's tempo (BPM) to serve as a guide for recording and performance.

## Overview
The metronome generates a periodic click sound based on the playback position and the configured BPM.
An accent (higher pitch) is played on the first beat of each bar, while a standard click sound is played on the other beats, making it easier to identify the beginning of a measure.

## Basic Usage
In the current version, only the core engine (backend) of the metronome feature is implemented.

## Detailed Settings
The internal specifications of the metronome are as follows:

| Parameter | Description | Value |
|:---|:---|:---|
| Standard Click Frequency | The pitch of the click sound for the 2nd beat and onwards | 1000.0 Hz |
| Accent Frequency | The pitch of the click sound for the 1st beat (start of the bar) | 1500.0 Hz |
| Click Duration | The length of time each click sound plays | 0.05 seconds |
| Beats per Bar | The number of beats in a single measure (currently fixed) | 4 beats |

## Keyboard Shortcuts
In the current version, there are no dedicated keyboard shortcuts for operating the metronome.

## See Also
- [Transport Control](../user-guide/transport.md)

## Notes and Limitations
> In the current version, the metronome feature is only implemented in the backend (audio engine), and the UI (such as an on/off toggle button) is not yet implemented. Therefore, it is not possible to enable the metronome from the screen.
