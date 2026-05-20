# Track Management

> This feature manages the addition, removal, and states of tracks (audio, MIDI, synthesizer, etc.) in your project.

## Overview
The Tracks panel is located on the left side of the screen and displays a list of tracks currently in the project. A track is the fundamental unit for assigning independent audio sources or instruments.

## Basic Usage
1. Click the **+ Add Track** button at the top of the panel to add a new standard track.
2. Click the **+ Add Synth Track** button at the top of the panel to add a new track with the synthesizer enabled.
3. Click the **X** button on each track's row in the list to remove that track.

## Detailed Settings
Internally, each track has the following parameters. (*Note: Some of these items are manipulated from the Mixer panel.*)

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| ID | A unique identifier for the track (auto-generated) | - | 1 and up |
| Name | The name of the track | "Track X" / "Synth X" | - |
| Volume | The individual output volume of the track | 1.0 | 0.0 to unlimited (can be boosted) |
| Pan | Panning (left-right positioning) | 0.0 | -1.0 (Left) to 1.0 (Right) |
| Mute | The mute state for the track | Off | On / Off |
| Solo | The solo state. If On, only tracks with Solo set to On will be played | Off | On / Off |
| Synth | The enabled/disabled state of the synthesizer | Off (On for Synth tracks) | On / Off |
| Freq (Hz) | The frequency of the synthesizer (only when Synth is On) | 440.0 | 20.0 to 20000.0 |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated specifically to track operations.

## Related Items
- [Mixer](../user-guide/mixer.md)
- [Audio Import](../user-guide/audio-import.md)

## Notes and Limitations
> In the current version, only adding and removing tracks can be operated from the Tracks panel (on the left side of the screen). Individual settings for volume, pan, mute, solo, and synthesizer can be adjusted from the Mixer panel at the bottom of the screen.
