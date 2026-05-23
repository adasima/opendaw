# Recording

> A feature to record audio from a microphone or external input.

## Overview
OpenDAW can record input from a microphone or audio interface connected to the system and place it as an audio clip on the timeline. The recorded waveform is drawn upon stopping the recording, allowing for visual confirmation.

## Basic Usage
1. Click the **⏺ (Record)** button on the transport panel at the bottom center of the main screen.
2. Recording mode is enabled, and the button changes to a red **⏺ (On)**.
3. Click the play button (**▶**) on the transport panel to start playback and recording simultaneously.
4. To stop recording, click the **⏺** button again to change it to **⏺ (Off)**, or stop playback (**⏹** or **⏸**).
5. When recording stops, a recorded audio clip is added to the first track of the project (if no track exists, one will be created automatically), and its waveform is displayed.

## Detailed Settings
Currently, detailed setting screens for the recording feature (such as device selection or buffer size) are not implemented in the UI.
Recording automatically uses the system's default input device.

| Parameter | Description | Default Value |
|:---|:---|:---|
| Recording Device | Audio input device to use | System Default |
| Sample Format | Audio format processed internally | 32-bit Float or 16-bit Int |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated to recording operations.

## See Also
- [Transport Control](../user-guide/transport.md)
- [Timeline & Waveform](../user-guide/timeline.md)

## Notes and Limitations
> - In the current version, you cannot manually select the input device from the UI. The system's default input device is used.
> - Recorded clips are always added to the first track (Track ID 0). Routing settings to arbitrary tracks will be supported in a future update.
> - Actual playback of recorded audio clips is currently under development. The waveform can be viewed, but no sound will be heard during playback.
