# Audio Import

> A feature to load audio files (WAV format) from the local file system into the project.

## Overview
By using the audio import feature, you can bring externally created audio materials or recorded data into your current project. The imported file is automatically added as a new track.

## Basic Usage
1. Click the **📁 Import Audio** button located on the main UI.
2. A file selection dialog will open.
3. Select the `.wav` file you want to load, and click "Open".
4. A new track, named after the file without its extension, will be added to the track list.

## Detailed Settings
In the current version, there are no configurable parameters during import.

| Parameter | Description | Default | Range |
|:---|:---|:---|:---|
| None | None | None | None |

## Keyboard Shortcuts
In the current version, there are no shortcut keys assigned to the import feature.

| Action | Shortcut |
|:---|:---|
| None | None |

## Related Items
- [Track Management](../user-guide/tracks.md)

## Notes and Limitations
> - The currently supported audio format is **WAV** only.
> - In the current implementation (Phase 3), the file loading process (sending data to the audio engine) is incomplete, and only track addition is performed. Actual audio data playback will be supported in a future update.
