# MCP API Reference

> The specification for the Model Context Protocol (MCP), which allows AI agents and external applications to control OpenDAW.

## Overview
By utilizing the MCP (Model Context Protocol) server feature, AI agents or external applications can control OpenDAW's transport (play/stop) and track operations. Currently, the MCP server is implemented as an asynchronous skeleton running on the Tokio runtime.

## Basic Usage
1. When you start OpenDAW, the MCP server automatically launches in the background.
2. Send commands from an external client via the MCP protocol.
3. (*In the current version, full integration with the UI thread is not yet implemented. Only backend command reception and log output are functional.*)

## Detailed Settings (Supported Operations)
A list of currently implemented handlers and supported operations.

| Handler | Operation | Overview | Implementation Status |
|:---|:---|:---|:---|
| Transport | `play` | Starts playback. | Backend log output only |
| Transport | `stop` | Stops playback. | Backend log output only |
| Transport | `toggle_loop` | Toggles loop playback on/off. | Backend log output only |
| Tracks | `add_track` | Adds a new track. | Backend log output only |
| Tracks | `remove_track` | Removes the track with the specified ID. | Backend log output only |

## Keyboard Shortcuts
Currently, there are no specific keyboard shortcuts for the MCP server.

## Related Topics
- [Transport](../user-guide/transport.md)
- [Tracks](../user-guide/tracks.md)

## Notes and Limitations
> - In the current version, the MCP functionality is limited to a skeleton implementation on the backend. The actual connection with the UI thread (`app.rs`) to update the UI or control audio is incomplete.
> - Specific connection settings, such as TCP/UDP ports and authentication, are planned for a future version.
