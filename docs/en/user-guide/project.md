# Project Management

> This feature allows you to save the current state of your project to a file and load it later to resume your work.

## Overview
You can save the state of your music project, including BPM, master volume, and added tracks, as a `.aura` project file. By opening the saved file, you can resume editing from where you left off, even after restarting the DAW.

## Basic Usage
### Saving a Project
1. Click the **💾 Save Project** button on the menu bar or at the top of the UI.
2. A file save dialog will open. Enter the destination folder and file name.
3. Execute the save to store the project as an `.aura` format file.

### Loading a Project
1. Click the **📂 Load Project** button on the menu bar or at the top of the UI.
2. A file selection dialog will open. Select a previously saved `.aura` file.
3. The file will be loaded, and the DAW's state (such as BPM and track configuration) will be restored.

## Detailed Settings
Currently, a project file (`.aura`) contains the following information:

| Item | Description |
|:---|:---|
| BPM | The tempo of the project (Beats Per Minute) |
| Master Volume | The overall output volume |
| Track Information | Names and settings of each added track |
| Grid Settings | The enabled/disabled state of the grid and its resolution (e.g., 1/4) |

*Note: Temporary playback states, such as whether playback is active or the playhead position, are not saved and will be reset to their initial state upon loading.

## Keyboard Shortcuts
In the current version, there are no dedicated keyboard shortcuts for saving or loading projects.

## Related Topics
- [Export](../user-guide/export.md)

## Notes and Limitations
> - If an error occurs during loading, the project state will not be updated, and an error log will be output.
> - If you move the project file to another PC and open it, it may not play correctly if the absolute paths to the referenced audio files (such as imported WAV files) within the project have changed.
