# Synthesizer (Software Instrument)

> An internal software instrument feature that generates sound within the project.

## Overview
The Synthesizer feature generates and outputs audio waveforms directly within the DAW. Currently, it supports the generation of a basic sine wave oscillator. This allows you to start creating sounds within the DAW itself without needing to import external audio files.

## Basic Usage
### Adding a Synth Track
1. Click the **+ Add Synth Track** button at the top of the Tracks panel on the left side of the screen.
2. A new track with the synthesizer enabled (e.g., "Synth 1") will be added.

### Adjusting Volume and Parameters
1. Locate the controls for the added track in the Mixer panel at the bottom of the screen.
2. Adjust the volume using the **Volume** slider.
3. Move the **Freq (Hz)** slider left or right to change the pitch of the generated sound.

## Detailed Settings
Each synthesizer has the following parameters. These settings are saved in the project file (`.aura`).

| Parameter | Description | Default Value | Range |
|:---|:---|:---|:---|
| Synth | The enabled/disabled state of the synthesizer | Off (On when adding a Synth track) | On / Off |
| Freq (Hz) | The frequency of the generated sine wave | 440.0 | 20.0 to 20000.0 |

## Keyboard Shortcuts
In the current version, there are no keyboard shortcuts dedicated specifically to synthesizer operations.

## Related Topics
- [Track Management](../user-guide/tracks.md)
- [Mixer](../user-guide/mixer.md)

## Notes and Limitations
> - In the current version, the synthesizer generates sound when it receives active MIDI notes (e.g., from the piano roll). If there are no active MIDI notes, it remains silent.
> - The only waveform currently available is the Sine Wave. Additional waveforms (like Square or Sawtooth) and ADSR envelopes are planned for future updates.
