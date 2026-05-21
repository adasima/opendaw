# Supported Formats

> A list of audio file formats supported by OpenDAW for import and export.

## Overview
OpenDAW utilizes high-precision 32-bit Float for internal audio processing. For external file interactions (import and export), the standard WAV format is supported.

## Supported Formats for Import
For loading (importing) audio files, the following WAV formats are supported. Imported audio data is automatically converted to the internal processing format (32-bit Float, normalized to -1.0 to 1.0).

| Parameter | Supported Specifications | Notes |
|:---|:---|:---|
| Container Format | WAV (`.wav`) | Other formats (MP3, FLAC, OGG, etc.) are not supported |
| Sample Format | Integer (Int) / Floating Point (Float) | |
| Bit Depth (Int) | 8-bit, 16-bit, 24-bit, 32-bit | 8-bit is unsigned, 16-bit and higher are signed |
| Bit Depth (Float) | 32-bit | 64-bit Float is not supported |
| Channels | Mono (1ch) / Stereo (2ch) | |
| Sample Rate | No limitation | Standard rates such as 44.1kHz and 48kHz are supported |

## Supported Formats for Export
The format of files exported from the project is currently fixed to the following specifications:

| Parameter | Setting Value |
|:---|:---|
| Container Format | WAV (`.wav`) |
| Channels | Stereo (2ch) |
| Sample Format | Integer (Int) |
| Bit Depth | 16-bit |

## Related Topics
- [Audio Import](../user-guide/audio-import.md)
- [Export](../user-guide/export.md)

## Notes & Limitations
> - In the current version, reading and writing formats other than WAV (such as AIFF, MP3, FLAC) are not supported.
> - Functionality to change export formats (such as selecting bit depth or sample rate) is planned for future updates.
