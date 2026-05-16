export interface MidiMetadata {
    file_name: string;
    track_count: number;
    ppq: number;
    duration: number;
    initial_bpm: number;
    time_signatures: { time: number; numerator: number; denominator: number }[];
    labels: string[];
}

export interface ImportOptions {
    bake_sustain: boolean;
    import_tempo: boolean;
    scale_to_bpm?: number;
}

export interface TimeSignatureInfo {
    time: number;
    numerator: number;
    denominator: number;
}
