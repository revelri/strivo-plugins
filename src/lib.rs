//! First-party plugins for StriVo.
//!
//! - [`crunchr`] — transcription + analysis (Whisper CLI, Voxtral, Mistral, OpenRouter)
//! - [`archiver`] — recording organization + gallery rendering

#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::type_complexity)]
#![allow(clippy::too_many_arguments)]

pub mod archiver;
pub mod crunchr;
