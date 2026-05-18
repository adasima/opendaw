# Track Management

> This feature manages the addition, removal, and states of tracks (audio, MIDI, etc.) in your project.

## Overview
The Tracks panel is located on the left side of the screen and displays a list of tracks currently in the project. A track is the fundamental unit for assigning independent audio sources or instruments.

## Basic Usage
1. Click the **+ Add Track** button at the top of the panel to add a new track.
2. Click the **X** button on each track's row in the list to remove that track.

## Detailed Settings
Internally, each track has the following parameters. (*Note: Only some of these can currently be manipulated directly from the UI.*)

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| ID | A unique identifier for the track (auto-generated) | - | 1 and up |
| Name | The name of the track | "Track X" | - |
| Volume | The individual output volume of the track | 1.0 | 0.0 to unlimited (can be boosted) |
| Pan | Panning (left-right positioning) | 0.0 | -1.0 (Left) to 1.0 (Right) |
| Mute | The mute state for the track | Off | On / Off |
| Solo | The solo state. If On, only tracks with Solo set to On will be played | Off | On / Off |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated specifically to track operations.

## Related Items
- [Mixer](../user-guide/mixer.md)
- [Audio Import](../user-guide/audio-import.md)

## Notes and Limitations
> In the current version, only adding and removing tracks can be operated from the UI. Individual settings for volume, pan, mute, and solo, as well as changing the track name, are defined as internal states but are not yet implemented in the UI.
