use regex::Regex;
use std::path::Path;
use std::process::Command;
use std::{env, fs, io};
pub fn resample_audio(input_file: &str, output_file: &str) -> io::Result<()> {
    let status = Command::new("sox")
        .args([input_file, "-r", "16000", "-c", "1", output_file])
        .status()?;

    if status.success() {
        println!("Audio resampled successfully to {}", output_file);
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("SoX command failed with status: {}", status),
        ))
    }
}

pub fn convert_mp4_to_wav(input_path: &str, output_path: &str) -> io::Result<()> {
    let status = Command::new("ffmpeg")
        .args([
            "-i",
            input_path,
            "-vn",
            "-acodec",
            "pcm_s16le",
            "-ar",
            "44100",
            "-ac",
            "2",
            output_path,
        ])
        .status()?;

    if status.success() {
        println!("Conversion successful!");
    } else {
        eprintln!("Conversion failed.");
    }

    Ok(())
}

pub fn transcribe_audio(audio_path: &str, model_path: &str) -> io::Result<String> {
    // Resolve the current working directory

    // Construct the path to the whisper executable relative to the current directory

    let current_dir = std::env::current_dir().expect("Failed to get current directory");

    // Relative path
    let whisper_exec_relative = Path::new("whisper.cpp/build/bin/main");

    // Combine current directory with the relative path
    let whisper_exec = current_dir.join(whisper_exec_relative);

    // Ensure the executable path is valid
    if !whisper_exec.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Executable not found at {:?}", whisper_exec),
        ));
    }
    // Run the transcription command
    let output = Command::new(whisper_exec)
        .args(["-m", model_path, "-f", audio_path])
        .output()?;

    if output.status.success() {
        let transcription = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(transcription)
    } else {
        let error_message = String::from_utf8_lossy(&output.stderr);
        println!("{error_message}");
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Transcription failed: {}", error_message),
        ))
    }
}

pub fn process_transcript(transcript: &str, remove_timestamps: bool) -> String {
    if remove_timestamps {
        extract_text(transcript)
    } else {
        // Return the transcript as is
        transcript.to_string()
    }
}

fn extract_text(transcript: &str) -> String {
    // Regular expression to match lines with timestamps
    let re =
        Regex::new(r"^\[\d{2}:\d{2}:\d{2}\.\d{3} --> \d{2}:\d{2}:\d{2}\.\d{3}\]\s+(.*)$").unwrap();
    let mut extracted_text = String::new();

    for line in transcript.lines() {
        if let Some(caps) = re.captures(line) {
            if let Some(text_match) = caps.get(1) {
                extracted_text.push_str(text_match.as_str());
                extracted_text.push(' ');
            }
        } else {
            // If the line doesn't match, include it as is (handles any additional text)
            extracted_text.push_str(line);
            extracted_text.push(' ');
        }
    }

    extracted_text.trim().to_string()
}

pub fn delete_audio_files(directory: &Path) -> io::Result<()> {
    // Resolve the directory to an absolute path
    let resolved_directory = if directory == Path::new(".") {
        env::current_dir()? // Get the current working directory
    } else {
        directory.to_path_buf()
    };

    // Ensure the path points to a directory
    if !resolved_directory.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory does not exist",
        ));
    }

    // Iterate over files and delete those starting with "audio"
    for entry in fs::read_dir(&resolved_directory)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.starts_with("audio") {
                    fs::remove_file(path)?;
                }
            }
        }
    }
    Ok(())
}
