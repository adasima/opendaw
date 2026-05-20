# MCP API Reference

> The specification for the Model Context Protocol (MCP), which allows AI agents and external applications to control OpenDAW.

## Overview
By utilizing the MCP (Model Context Protocol) server feature, AI agents or external applications can control OpenDAW's transport (play/stop) and track operations. Currently, the MCP server runs asynchronously on the Tokio runtime and interacts with the UI to execute various operations.

## Basic Usage
1. When you start OpenDAW, the MCP server automatically launches in the background.
2. Send commands from an external client via the MCP protocol.
3. Commands sent via the MCP protocol are reflected in the UI through asynchronous channels.

## Detailed Settings (Supported Operations)
A list of currently implemented handlers and supported operations.

| Handler | Operation | Overview | Implementation Status |
|:---|:---|:---|:---|
| Transport | `play` | Starts playback. | UI integrated |
| Transport | `stop` | Stops playback. | UI integrated |
| Transport | `toggle_loop` | Toggles loop playback on/off. | UI integrated |
| Tracks | `add_track` | Adds a new track. | UI integrated |
| Tracks | `remove_track` | Removes the track with the specified ID. | UI integrated |

## Keyboard Shortcuts
Currently, there are no specific keyboard shortcuts for the MCP server.

## Related Topics
- [Transport](../user-guide/transport.md)
- [Tracks](../user-guide/tracks.md)

## Notes and Limitations
> - Specific connection settings, such as TCP/UDP ports and authentication, are planned for a future version.
