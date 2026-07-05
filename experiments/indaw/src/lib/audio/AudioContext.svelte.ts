import { invoke, listen } from '../api';
import type {
    AudioStateSnapshot,
    SynthParams,
    TrackSnapshot,
    NoteEvent
} from '../bindings';
import type { OscillatorType } from '../bindings/OscillatorType';



/**
 * Core Audio Context managed with Svelte 5 Runes.
 * Replaces the legacy writable store.
 */
export class AudioContext {
    // --- Backend Synced State ---
    isPlaying = $state(false);
    playheadPosition = $state(0);
    tempo = $state(120);
    isLooping = $state(false);
    loopStart = $state(0);
    loopEnd = $state(4);
    masterVolume = $state(1.0);

    // Explicitly typed complex objects
    synthParams = $state<SynthParams>({
        attack: 0.05,
        decay: 0.1,
        sustain: 0.7,
        release: 0.2,
        oscillator_type: "Sine" as OscillatorType
    });

    tracks = $state<TrackSnapshot[]>([]);

    // --- Audio Configuration ---
    outputDevices = $state<{ name: string; is_default: boolean }[]>([]);
    selectedDeviceName = $state("");

    // --- Client-side State ---
    selectedTrackId = $state(0);
    autoScroll = $state(true);

    // Sync handles
    private stateIntervalId: number | null = null;
    private unlistenPlayhead: (() => void) | null = null;

    constructor() {
        // Automatically start sync on creation? 
        // Better to explicit start/stop in component mount/unmount or root layout.
    }

    /**
     * Start the sync loop with the backend (or mock)
     */
    async startSync() {
        if (this.stateIntervalId) return;

        // Listen for playhead updates (60fps from backend)
        this.unlistenPlayhead = await listen('playhead-update', (event: any) => {
            this.playheadPosition = event.payload as number;
        });

        // Sync other state at lower frequency (5fps is enough for tracks, tempo, etc.)
        const syncState = async () => {
            try {
                const snapshot = await invoke('get_audio_state') as AudioStateSnapshot;
                if (snapshot) {
                    this.applySnapshot(snapshot);
                }
            } catch (e) {
                console.error("Sync error:", e);
            }
        };

        syncState(); // Initial sync
        this.stateIntervalId = setInterval(syncState, 200) as unknown as number; // 5fps
    }

    stopSync() {
        if (this.stateIntervalId) {
            clearInterval(this.stateIntervalId);
            this.stateIntervalId = null;
        }
        if (this.unlistenPlayhead) {
            this.unlistenPlayhead();
            this.unlistenPlayhead = null;
        }
    }

    private applySnapshot(snapshot: AudioStateSnapshot) {
        this.isPlaying = snapshot.is_playing;
        // playheadPosition is now updated via events, skip here to avoid jitter
        this.tempo = snapshot.tempo;
        this.isLooping = snapshot.is_looping;
        this.loopStart = snapshot.loop_start;
        this.loopEnd = snapshot.loop_end;
        this.synthParams = snapshot.synth;
        this.tracks = snapshot.tracks;
        if (snapshot.master_volume !== undefined) this.masterVolume = snapshot.master_volume;
    }

    // --- Actions ---

    async togglePlayback() {
        if (this.isPlaying) {
            await invoke('pause');
            // Optimistic update
            this.isPlaying = false;
        } else {
            await invoke('play');
            this.isPlaying = true;
        }
    }

    async stop() {
        await invoke('stop');
        this.isPlaying = false;
        this.playheadPosition = 0;
    }

    async seek(position: number) {
        await invoke('seek', { position });
        this.playheadPosition = position;
    }

    async toggleLoop() {
        const newVal = !this.isLooping;
        this.isLooping = newVal; // Optimistic
        await invoke('set_looping', { enabled: newVal });
    }

    async setLoopRegion(start: number, end: number) {
        this.loopStart = start;
        this.loopEnd = end;
        await invoke('set_loop_region', { start, end });
    }

    toggleAutoScroll() {
        this.autoScroll = !this.autoScroll;
    }

    selectTrack(id: number) {
        this.selectedTrackId = id;
    }

    // --- Track Controls ---

    async setTrackVolume(id: number, volume: number) {
        // Optimistic
        const track = this.tracks.find(t => t.id === id);
        if (track) track.volume = volume;
        await invoke('set_track_volume', { trackId: id, volume });
    }

    async setMasterVolume(volume: number) {
        this.masterVolume = volume;
        await invoke('set_master_volume', { volume });
    }

    async setTrackPan(id: number, pan: number) {
        const track = this.tracks.find(t => t.id === id);
        if (track) track.pan = pan;
        await invoke('set_track_pan', { trackId: id, pan });
    }

    async toggleTrackMute(id: number) {
        const track = this.tracks.find(t => t.id === id);
        if (track) {
            track.muted = !track.muted;
            await invoke('set_track_mute', { trackId: id, muted: track.muted });
        }
    }

    async toggleTrackSolo(id: number) {
        const track = this.tracks.find(t => t.id === id);
        if (track) {
            track.soloed = !track.soloed;
            await invoke('set_track_solo', { trackId: id, soloed: track.soloed });
        }
    }

    async addAudioTrack(path: string, name: string) {
        await invoke('add_audio_track', { path, name });
    }

    // --- Sequence Editing ---

    async updateSequence(newNotes: NoteEvent[]) {
        // Find selected track to optimistically update (if MIDI)
        const track = this.tracks.find(t => t.id === this.selectedTrackId);
        if (track && track.kind === "Midi" && track.content && "Midi" in track.content) {
            track.content.Midi.notes = newNotes;
        }

        await invoke('update_sequence', { trackId: this.selectedTrackId, notes: newNotes });
    }

    // --- MIDI Import/Export ---

    async importMidi(path: string, options?: any) {
        const opts = options || { bake_sustain: true, import_tempo: false };
        const newIds = await invoke('import_midi', { path, options: opts }) as number[];
        if (Array.isArray(newIds) && newIds.length > 0) {
            this.selectedTrackId = newIds[0];
        }
    }

    async exportMidi(trackId: number, path: string) {
        await invoke('export_midi', { trackId, path });
    }

    // --- Audio Configuration Methods ---

    async loadOutputDevices() {
        try {
            const devices = await invoke('get_output_devices') as { name: string; is_default: boolean }[];
            this.outputDevices = devices;
            if (!this.selectedDeviceName && devices.length > 0) {
                const def = devices.find(d => d.is_default);
                this.selectedDeviceName = def ? def.name : devices[0].name;
            }
        } catch (e) {
            console.error("Failed to load output devices:", e);
        }
    }

    async setOutputDevice(name: string) {
        // Allow re-selection to force restart engine
        this.selectedDeviceName = name;
        await invoke('set_output_device', { name });
    }

    async refreshTracks() {
        try {
            const state = await invoke('get_audio_state') as AudioStateSnapshot;
            this.tracks = state.tracks;
        } catch (e) { console.error("Failed to refresh tracks:", e); }
    }

    // --- Recording ---
    isRecording = $state(false);
    currentRecordingPath = "";

    async toggleRecording() {
        if (this.isRecording) {
            await this.stopRecording();
        } else {
            await this.startRecording();
        }
    }

    async startRecording() {
        if (this.isRecording) return;
        try {
            const { appLocalDataDir, join } = await import('@tauri-apps/api/path');
            const dir = await appLocalDataDir();
            const filename = `recording_${Date.now()}.wav`;
            const path = await join(dir, filename);

            await invoke('start_recording', { path });
            this.currentRecordingPath = path;
            this.isRecording = true;
        } catch (e) {
            console.error("Failed to start recording:", e);
        }
    }

    async stopRecording() {
        if (!this.isRecording) return;
        try {
            await invoke('stop_recording');
            this.isRecording = false;

            // Add to track
            if (this.currentRecordingPath) {
                await invoke('add_audio_track', { path: this.currentRecordingPath });
                await this.refreshTracks();
            }
        } catch (e) {
            console.error("Failed to stop recording:", e);
        }
    }

    async newProject() {
        await invoke('new_project');
        // Reset local state
        this.tracks = [];
        this.playheadPosition = 0;
        this.selectedTrackId = 0;
        this.isPlaying = false;
        this.synthParams = { attack: 0.05, decay: 0.1, sustain: 0.7, release: 0.2, oscillator_type: "Sine" as OscillatorType };
    }

    async setOscillatorType(type: OscillatorType) {
        this.synthParams.oscillator_type = type;
        await invoke('set_oscillator_type', { oscillator_type: type });
    }

    async setAdsr(attack: number, decay: number, sustain: number, release: number) {
        // Optimistic
        this.synthParams = { ...this.synthParams, attack, decay, sustain, release };
        await invoke('set_adsr', { attack, decay, sustain, release });
    }
}

// Global Singleton Instance
export const audioContext = new AudioContext();
