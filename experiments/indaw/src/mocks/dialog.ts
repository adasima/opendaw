export async function open(options: any) {
    console.log("[MockDialog] open", options);
    return "mock/file.mid";
}

export async function save(options: any) {
    console.log("[MockDialog] save", options);
    return "mock/file.mid";
}

export async function ask(message: string, options: any) {
    console.log("[MockDialog] ask", message);
    // For automated testing, we might want to just return true.
    // Or use window.confirm so manual tester sees it.
    return true;
}
