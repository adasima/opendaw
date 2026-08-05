export async function open(options: any) {
    return "mock/file.mid";
}

export async function save(options: any) {
    return "mock/file.mid";
}

export async function ask(message: string, options: any) {
    // For automated testing, we might want to just return true.
    // Or use window.confirm so manual tester sees it.
    return true;
}
