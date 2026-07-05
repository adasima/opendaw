export function getCurrentWindow() {
    return {
        minimize: () => console.log("[MockWindow] minimize"),
        maximize: () => console.log("[MockWindow] maximize"),
        unmaximize: () => console.log("[MockWindow] unmaximize"),
        close: () => console.log("[MockWindow] close"),
        isMaximized: async () => false
    }
}
