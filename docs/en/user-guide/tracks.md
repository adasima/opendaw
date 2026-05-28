# Track Management

> This feature manages the addition, selection, and detailed settings of tracks (audio, MIDI, synthesizer, etc.) in your project.

## Overview
The Tracks panel is located on the left side of the screen and displays a list of tracks currently in the project. A track is the fundamental unit for assigning independent audio sources or instruments. Selecting a track opens the **Track Details** panel on the right side, allowing for more advanced configuration.

## Basic Usage
### Adding, Selecting, and Deleting Tracks
1. Click the **+** button at the top of the panel to add a new track. (Track type selection will be added in a future update.)
2. Click any track in the track list to make it active (selected), which will display the **Track Details** panel on the right side.
3. Hover over a track and click the **✕** button (Delete Track) that appears to delete the track.

### Track Controls
The following buttons are arranged on the track header, allowing you to quickly toggle states.

- **M** (Mute): Mutes the audio for this track.
- **S** (Solo): Sets this track to the solo state. Only tracks with Solo enabled will be played.
- **R** (Record): Toggles the record standby (record arm) state for this track.

## Track Details (Advanced Settings)
The **Track Details** panel, displayed when a track is selected, allows for the following settings.

### Mixer
| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Volume | The individual output volume of the track | 0.8 (80%) | 0.0 to 1.0 (0% to 100%) |
| Pan | Panning (left-right positioning) | 0.0 (0%) | -1.0 (Left) to 1.0 (Right) |

### MIDI Routing
| Parameter | Description |
|:---|:---|
| Input Device | Selects the input device to receive MIDI signals from |
| Channel | Specifies the MIDI channel to receive (0=All, 1 to 16) |

### Plugins
Displays a list of plugins (such as VST3 / CLAP) loaded onto the track.
You can open the dedicated UI for each plugin by clicking the **Open GUI** button next to it (functionality currently being integrated).
Plugins can be added using the [Plugin Browser](../user-guide/browser.md).

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated specifically to track operations, but you can make a track active by pressing the `Enter` key while it is focused.

## Related Items
- [Mixer](../user-guide/mixer.md)
- [Audio Import](../user-guide/audio-import.md)

## Notes and Limitations
> - MIDI Routing settings are only functional if a MIDI device is connected.
