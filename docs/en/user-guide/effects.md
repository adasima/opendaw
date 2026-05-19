# Effects

> This feature allows you to add and manage effects, such as volume adjustment (Gain) and tone shaping (Filter), for each track.

## Overview
Use the effect chain to process the audio of a track. Currently, two types of effects are supported: Gain and Filter. You can combine multiple effects per track, and they will be applied sequentially.

## Basic Usage
1. From the track list on the main screen, click the **Effects** button of the track you want to apply effects to.
2. In the opened effects window, click the **Add Gain** or **Add Filter** button at the top to add an effect.
3. Added effects are displayed in a list and are applied to the audio signal in order from top to bottom.

## Effect Management
You can perform the following operations for each added effect:

| Operation | Description |
|:---|:---|
| Toggle Enable/Disable | Click the checkbox on the left side of each effect to toggle its enabled/disabled (bypassed) state. |
| Reorder | Click the **↑** or **↓** button on the right side of an effect to change the order in which the effects are applied. |
| Remove | Click the **X** button on the right side of an effect to remove it from the track. |

## Supported Effects

Currently, the following effects are available:

| Effect Name | Description |
|:---|:---|
| Gain | Adjusts the volume of the track. |
| Filter | A low-pass filter (Biquad) that cuts or boosts specific frequency bands. |

> In the current version, the ability to directly edit various parameters of the effects (such as the volume level of Gain or the cutoff frequency of Filter) from the UI is under development.

## Related Topics
- [Track Management](tracks.md)
- [Mixer](mixer.md)
