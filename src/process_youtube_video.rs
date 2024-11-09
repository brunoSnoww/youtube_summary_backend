use std::error::Error;

use crate::{
    audio::{convert_mp4_to_wav, process_transcript, resample_audio, transcribe_audio},
    openai_api::summarize_text,
    yt_audio::yt_audio::get_yt_audio,
};

pub async fn process_youtube_video(youtube_url: &str) -> Result<String, Box<dyn Error>> {
    // Replace hardcoded paths with dynamic or configured paths as needed
    let path = "/Users/brunoneves/Desktop/personalProjects/yt_summarizer/youtube_summary_backend"; // Use a temporary directory
    let input_mp4 = format!("{}/{}", path, "audio.mp4");
    let input_wav = format!("{}/{}", path, "audio.wav");
    let output_wav = format!("{}/{}", path, "audio16k.wav");

    // Paths to models and binaries; adjust as needed
    let model_path =
        "/Users/brunoneves/Desktop/personalProjects/yt_summarizer/whisper.cpp/models/ggml-base.bin";
    let audio_path = &output_wav;

    // Step 1: Download YouTube audio
    get_yt_audio(youtube_url);

    // Step 2: Convert MP4 to WAV
    convert_mp4_to_wav(&input_mp4, &input_wav)?;

    // Step 3: Resample audio to 16kHz mono
    resample_audio(&input_wav, &output_wav)?;

    // Step 4: Transcribe audio
    let transcript = transcribe_audio(audio_path, model_path)?;

    // Step 5: Process transcript
    let remove_timestamps = true;
    let processed_text = process_transcript(&transcript, remove_timestamps);

    // Step 6: Summarize text
    let summary = summarize_text(&processed_text)?;

    Ok(summary)
}
