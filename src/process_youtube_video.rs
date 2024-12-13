use std::{error::Error, path::Path};

use crate::{
    audio::{
        convert_mp4_to_wav, delete_audio_files, process_transcript, resample_audio,
        transcribe_audio,
    },
    openai_api::summarize_text,
    yt_audio::yt_audio::get_yt_audio,
};

pub async fn process_youtube_video(youtube_url: &str) -> Result<String, Box<dyn Error>> {
    // Resolve the current working directory
    let current_dir = std::env::current_dir()?;

    // Construct dynamic paths relative to the current directory
    let input_mp4 = current_dir.join("audio.mp4");
    let input_wav = current_dir.join("audio.wav");
    let output_wav = current_dir.join("audio16k.wav");
    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    let model_path_relative = Path::new("whisper.cpp/models/ggml-base.en.bin");

    // Combine current directory with the relative paths
    let model_path = current_dir.join(model_path_relative);
    // Step 1: Download YouTube audio
    get_yt_audio(youtube_url);

    // Step 2: Convert MP4 to WAV
    convert_mp4_to_wav(input_mp4.to_str().unwrap(), input_wav.to_str().unwrap())?;

    // Step 3: Resample audio to 16kHz mono
    resample_audio(input_wav.to_str().unwrap(), output_wav.to_str().unwrap())?;

    // Step 4: Transcribe audio
    let transcript = transcribe_audio(output_wav.to_str().unwrap(), model_path.to_str().unwrap())?;

    // Step 5: Process transcript
    let remove_timestamps = true;
    let processed_text = process_transcript(&transcript, remove_timestamps);

    // Step 6: Summarize text
    let summary = summarize_text(&processed_text)?;

    // Step 7: Clean up audio files
    delete_audio_files(&current_dir)?;

    Ok(summary)
}
