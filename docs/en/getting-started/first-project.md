# First Project

> A tutorial explaining the basic workflow from launching OpenDAW to creating and saving a simple track.

## Overview
On this page, you will experience the workflow of using OpenDAW's basic features to import audio files, place MIDI notes, mix your track, and save your project.

## Basic Usage

### 1. Project Preparation
When you launch OpenDAW, an empty project is opened automatically.
By default, the BPM is set to **120.0**.
You can check basic settings like tempo from the transport controls at the bottom of the screen.

### 2. Adding Tracks
To create a song, you first need to add tracks.
1. Click the **+ Add Track** button in the track panel on the left side of the screen.
2. A new track will be added to the list.

### 3. Importing Audio Files
Load existing WAV files, such as drum loops.
1. Click the **📁 Import Audio** button on the main screen.
2. Select a `.wav` file from your computer using the file selection dialog.
3. A new track with the file name will be added automatically.

### 4. Programming MIDI Notes (Piano Roll)
Use the piano roll editor to create melodies and basslines.
1. Select the track you want to input MIDI data into (Note: Track selection UI is planned for a future update), or simply treat it as a MIDI track.
2. Left-click on the grid in the **Piano Roll** panel in the center of the screen to add a note.
3. If you make a mistake, right-click on the note to delete it.

### 5. Playback and Mixing
Adjust the balance of each track while playing back the phrases you created.
1. Press the **▶ (Play)** button at the bottom of the screen or press the `Space` key to start playback.
2. In the **Mixer & Effects** panel, drag the **Volume** slider for each track to adjust its volume.
3. Adjust the **Pan** slider to create a stereo spread.
4. Adjust the overall volume using the **Master Volume** slider.

### 6. Saving the Project
Save the state of your creation.
1. Click the **💾 Save Project** button at the top of the screen.
2. Specify the save location and save it as an `.aura` format file.
You can now resume your work at any time using the **📂 Load Project** button.

## Detailed Settings
For more details on each feature introduced in this tutorial, please refer to the related topics.

| Step | Related Feature |
|:---|:---|
| Adding Tracks | [Track Management](../user-guide/tracks.md) |
| Importing Audio | [Audio File Import](../user-guide/audio-import.md) |
| Inputting Notes | [Piano Roll Editor](../user-guide/piano-roll.md) |
| Mixing | [Mixer](../user-guide/mixer.md) |
| Saving and Loading | [Project Management](../user-guide/project.md) |

## Keyboard Shortcuts

| Action | Shortcut |
|:---|:---|
| Play/Stop | `Space` |

## Related Topics
- [UI Overview](ui-overview.md)
- [Transport Controls](../user-guide/transport.md)

## Notes and Limitations
> - Currently, the actual playback processing for imported audio files is under development, so only track addition is performed.
> - Notes in the piano roll are fixed at a velocity of 100 and a duration of 1.0 beat.
