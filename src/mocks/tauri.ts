// Mock Tauri API for browser testing
import { writable } from 'svelte/store';

console.log("⚠️ USING MOCK TAURI API ⚠️");

// Mock State
let mockState = {
    is_playing: false,
    playhead_position: 0,
    tempo: 120,
    is_looping: false,
    loop_start: 0,
    loop_end: 4,
    synth: {
        attack: 0.05,
        decay: 0.1,
        sustain: 0.7,
        release: 0.2
    },
    tracks: [
        {
            id: 0,
            name: "Mock Track 1",
            kind: "Midi",
            volume: 1.0,
            pan: 0.0,
            muted: false,
            soloed: false,
            content: {
                Midi: {
                    notes: [
                        { note: 60, start_time: 0, duration: 1, velocity: 100 },
                        { note: 64, start_time: 1, duration: 1, velocity: 100 },
                        { note: 67, start_time: 2, duration: 1, velocity: 100 },
                    ],
                    tempo: 120
                }
            }
        }
    ],
    // Client-side state
    autoScroll: true,
    isPlaying: false,
    selectedTrackId: 0
};

// Simulate backend playback loop
setInterval(() => {
    if (mockState.is_playing) {
        mockState.playhead_position += 0.016; // Approx 60fps
        if (mockState.is_looping && mockState.playhead_position >= mockState.loop_end) {
            mockState.playhead_position = mockState.loop_start;
        }
    }
}, 16);

export async function invoke(cmd: string, args: any = {}) {
    console.log(`[MockInvoke] ${cmd}`, args);

    switch (cmd) {
        case 'get_audio_state':
            return { ...mockState };
        case 'play':
            mockState.is_playing = true;
            return;
        case 'pause':
        case 'stop':
            mockState.is_playing = false;
            if (cmd === 'stop') mockState.playhead_position = 0;
            return;
        case 'seek':
            mockState.playhead_position = args.position;
            return;
        case 'set_looping':
            mockState.is_looping = args.enabled;
            return;
        case 'set_loop_region':
            mockState.loop_start = args.start;
            mockState.loop_end = args.end;
            return;
        case 'update_sequence':
            const track = mockState.tracks.find(t => t.id === args.trackId);
            if (track && track.content && 'Midi' in track.content) {
                track.content.Midi.notes = args.notes;
            }
            return;
        case 'get_midi_info':
            return {
                file_name: args.path,
                track_count: 3,
                ppq: 480,
                duration: 120.0,
                initial_bpm: 128.0,
                time_signatures: [{ time: 0, numerator: 4, denominator: 4 }],
                labels: ["Track 1", "Track 2", "Drums"]
            };
        case 'import_midi':
            // Create dummy tracks
            const startId = mockState.tracks.length;
            const newIds = [];
            for (let i = 0; i < 3; i++) {
                const id = startId + i;
                const notes = [];
                // Generate dummy pattern
                if (i === 0) {
                    // Chords
                    for (let bar = 0; bar < 8; bar++) {
                        notes.push({ note: 60, start_time: bar * 4, duration: 2, velocity: 90 });
                        notes.push({ note: 64, start_time: bar * 4, duration: 2, velocity: 90 });
                        notes.push({ note: 67, start_time: bar * 4, duration: 2, velocity: 90 });
                    }
                } else if (i === 1) {
                    // Arpeggio (16th notes)
                    for (let j = 0; j < 64; j++) {
                        notes.push({
                            note: 72 + (j % 12),
                            start_time: j * 0.25,
                            duration: 0.2,
                            velocity: 80 + (j % 20)
                        });
                    }
                } else {
                    // Drums (Kick every beat)
                    for (let j = 0; j < 32; j++) {
                        notes.push({ note: 36, start_time: j, duration: 0.1, velocity: 120 });
                    }
                }

                mockState.tracks.push({
                    id: id,
                    name: i === 0 ? "Chords" : i === 1 ? "Arpeggio" : "Drums",
                    kind: "Midi",
                    volume: 1.0,
                    pan: i === 1 ? -0.5 : i === 0 ? 0.5 : 0.0,
                    muted: false,
                    soloed: false,
                    content: {
                        Midi: {
                            notes: notes,
                            tempo: args.options.import_tempo ? 135 : 120
                        }
                    }
                });
                newIds.push(id);
            }
            if (args.options.import_tempo) {
                mockState.tempo = 128;
            }
            return newIds;
        case 'new_project':
            mockState.tracks = [];
            mockState.playhead_position = 0;
            mockState.is_playing = false;
            return;
        case 'export_midi':
            return;
        case 'export_project':
            return;
            return;
    }
}

export async function listen(event: string, handler: (event: any) => void) {
    console.log(`[MockListen] ${event}`);
    // Return unlisten function
    return () => console.log(`[MockUnlisten] ${event}`);
}
