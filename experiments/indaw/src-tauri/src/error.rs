use serde::Serialize;
use thiserror::Error;

/// Application-level error type that can be serialized to JSON for frontend communication.
#[derive(Debug, Error, Serialize)]
#[serde(tag = "type", content = "details")]
pub enum AppError {
    #[error("Audio device error: {message}")]
    AudioDevice {
        message: String,
    },
    
    #[error("File I/O error: {message}")]
    FileIO {
        message: String,
        path: Option<String>,
    },
    
    #[error("MIDI parsing error: {message}")]
    MidiParse {
        message: String,
    },
    
    #[error("Invalid operation: {message}")]
    InvalidOperation {
        message: String,
    },
    
    #[error("Internal error: {message}")]
    Internal {
        message: String,
    },
}

impl AppError {
    pub fn audio_device(msg: impl Into<String>) -> Self {
        Self::AudioDevice { message: msg.into() }
    }
    
    pub fn file_io(msg: impl Into<String>, path: Option<String>) -> Self {
        Self::FileIO { message: msg.into(), path }
    }
    
    pub fn midi_parse(msg: impl Into<String>) -> Self {
        Self::MidiParse { message: msg.into() }
    }
    
    pub fn invalid_operation(msg: impl Into<String>) -> Self {
        Self::InvalidOperation { message: msg.into() }
    }
    
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal { message: msg.into() }
    }
}

// Convert from anyhow::Error for easier migration
impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        Self::Internal { message: err.to_string() }
    }
}

