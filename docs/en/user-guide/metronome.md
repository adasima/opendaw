# Metronome

> Generates a click sound synchronized with the project's tempo (BPM) to serve as a guide for recording and performance.

## Overview
The metronome generates a periodic click sound based on the playback position and the configured BPM.
An accent (higher pitch) is played on the first beat of each bar, while a standard click sound is played on the other beats, making it easier to identify the beginning of a measure.

## Basic Usage
1. Click the **⏱** (Metronome) button located in the transport panel at the top of the main area (Timeline/WASM screen).
2. The button display changes to **⏱ (On)**, and the metronome is enabled.
3. Click the play button (**▶**) in the transport panel to start playback. The click sound will play according to the set BPM.
4. To disable the metronome, click the **⏱** button again to change it to **⏱ (Off)**.

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
