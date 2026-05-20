# Mixer

> Features to adjust the overall project volume (master volume) and individual track mixing (volume, pan, mute, and solo).

## Overview
The mixer panel is used to manage the final volume level of the audio output and the balance of each track. It is located at the bottom of the screen as the "Mixer & Effects" panel, allowing you to operate the master channel and the mixer controls for each track (Volume, Pan, Mute, Solo).

## Basic Usage
### Master Controls
1. Drag the **Master Volume** slider left or right to adjust the overall output volume.
2. Click the **🔊** / **🔇** button to toggle the mute state of the overall audio output.

### Track Controls
The following operations are available on each track's panel:
1. Drag the **Volume** slider left or right to adjust the track's output volume.
2. Drag the **Pan** slider left or right to adjust the track's panning (left-right positioning).
3. Click the **M** or **M (On)** button to toggle the mute state of the track.
4. Click the **S** or **S (On)** button to toggle the solo state of the track.
5. Click the **FX** button to open the effects settings screen for the track.
6. Click the **Synth** checkbox to toggle the synthesizer enabled/disabled.
7. When Synth is enabled, drag the **Freq (Hz)** slider to adjust the synthesizer frequency.

## Detailed Settings
The mixer panel manipulates the following parameters.

### Master Channel
| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Master Volume | The final output volume of the entire project | 0.8 | 0.0 to 1.0 |
| Mute | The state of whether the overall audio is muted | Off (🔊) | On (🔇) / Off (🔊) |

### Individual Tracks
| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Volume | The individual output volume of the track | 1.0 | 0.0 to 2.0 (can be boosted) |
| Pan | Panning (left-right positioning) | 0.0 | -1.0 (Left) to 1.0 (Right) |
| Mute | The mute state for the track | Off (M) | On (M (On)) / Off (M) |
| Solo | The solo state. If On, only tracks with Solo set to On will be played | Off (S) | On (S (On)) / Off (S) |
| FX | Open the effects settings screen | - | - |
| Synth | The enabled/disabled state of the synthesizer | Off | On / Off |
| Freq (Hz) | The frequency of the synthesizer (only when Synth is On) | 440.0 | 20.0 to 20000.0 |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated to mixer operations.

## See Also
- [Tracks](../user-guide/tracks.md)
- [Effects](../user-guide/effects.md)

## Notes and Limitations
> In the current version, only basic mixing features for the master and individual tracks are supported. Advanced routing, such as effect chains, will be added in future updates.
