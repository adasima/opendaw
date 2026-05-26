# Undo / Redo

> A feature to revert project edits or re-apply reverted actions.

## Overview
If you make a mistake while editing a project (such as moving a clip or accidentally adding/removing a track), you can revert the project to its previous state using the Undo feature. You can also re-apply an action that was undone using the Redo feature. The project state is managed as a history stack, allowing you to go back up to 50 previous actions.

## Basic Usage
1. Perform an editing action on your project (e.g., adding/moving clips, deleting tracks).
2. To revert the action, click the **↩️ (Undo)** button in the transport panel. This will restore the project to its previous state.
3. To re-apply the reverted action, click the **↪️ (Redo)** button. This will cancel the undo and move the project forward to the state before the undo was triggered.

## Detailed Settings

In the current version, there are no user-facing settings for the Undo/Redo feature (such as changing the maximum history size).

| Parameter | Description | Internal Limit |
|:---|:---|:---|
| Max History | Maximum number of states that can be remembered | 50 times |

## Keyboard Shortcuts
In the current version, there are no dedicated keyboard shortcuts for the Undo/Redo operations.

## Related Topics
- [Project Management](../user-guide/project.md)

## Notes & Limitations
> - If you perform a new action after undoing, the future Redo history will be cleared.
> - The current audio playback state and transport playhead position are not included in the Undo/Redo history. Only project data such as BPM and track configurations will be restored.
