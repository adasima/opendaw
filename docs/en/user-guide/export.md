# Export

> This feature allows you to export (offline render) the mixer output of your project as a WAV file.

## Overview
The export feature enables you to render your created music or audio project into a single WAV file (16-bit, stereo). This allows you to play your track on other media players or share it with others.

## Basic Usage
1. (*Note: Exporting via the UI is currently not implemented. The backend functionality is implemented in `src/engine/export.rs`.*)

## Detailed Settings
In the current version, the export format is fixed to the following settings:

| Parameter | Setting |
|:---|:---|
| Channels | 2 (Stereo) |
| Bit Depth | 16-bit |
| Sample Format | Integer (Int) |
| Sample Rate | Depends on project settings |

## Related Topics
- [Mixer](../user-guide/mixer.md)

## Notes & Limitations
> - In the current version, there are no buttons or menus implemented in the UI to execute an export. The functionality is built into the engine (`src/engine/export.rs`).
> - The only supported file format for exporting is `.wav`.
