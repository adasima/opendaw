# Audio Settings

> Detailed settings related to OpenDAW's audio input/output, such as audio interfaces and buffer sizes.

## Overview
OpenDAW automatically detects your system's default audio device (e.g., WASAPI on Windows, CoreAudio on macOS, ALSA on Linux) and starts audio output with optimal settings. Currently, advanced routing or explicit device switching is not supported.

## Basic Behavior
1. Upon launching OpenDAW, the default audio output device configured in your system is automatically selected.
2. The sample rate and buffer size will follow the OS-level device settings.
3. If an error occurs (e.g., no device found), an error will be logged, and audio processing will be bypassed.

## Related Topics
- [Troubleshooting](troubleshooting.md)
- [System Requirements](../getting-started/system-requirements.md)

## Notes and Limitations
> - In the current version, the ability to manually select an audio device from the UI is not implemented. If you wish to change the device, please change the default output destination in your OS's sound settings and restart OpenDAW.
> - Similarly, there is no feature provided to adjust latency by changing the buffer size.
