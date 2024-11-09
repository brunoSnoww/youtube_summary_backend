use std::io::Error;

use blob_dl::yt_audio::get_yt_audio;
use yt_summarizer_backend::{
    audio::{convert_mp4_to_wav, process_transcript, resample_audio, transcribe_audio},
    openai_api::summarize_text,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let path = "/Users/brunoneves/Desktop/personalProjects/yt_summarizer/rust-lambda-book";
    let input_mp4 = format!("{}/{}", path, "audio.mp4");
    let input_wav = format!("{}/{}", path, "audio.wav");
    let output = format!("{}/{}", path, "audio16k.wav");
    let model_path =
        "/Users/brunoneves/Desktop/personalProjects/yt_summarizer/whisper.cpp/models/ggml-base.bin";
    let audio_path =
        "/Users/brunoneves/Desktop/personalProjects/yt_summarizer/rust-lambda-book/audio16k.wav";
    get_yt_audio("");
    let _ = convert_mp4_to_wav(&input_mp4, &input_wav);
    let _ = resample_audio(&input_wav, &output);
    if let Ok(transcript) = transcribe_audio(audio_path, model_path) {
        // Process the transcript with timestamps removed
        let remove_timestamps = true;
        let processed_text = process_transcript(&transcript, remove_timestamps);

        // Now you can pass `processed_text` to the summarization function
        let summary = summarize_text(&processed_text).unwrap();
        println!("{summary}");
    }
    Ok(())
}
