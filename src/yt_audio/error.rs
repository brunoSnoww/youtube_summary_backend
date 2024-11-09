use std::io::Write;

use colored::Colorize;

pub type BlobResult<T> = Result<T, BlobdlError>;

/// ### The all-encompassing error type used in this project
/// ## Implements From
/// For the Errors std::io::Error and ParseIntError
/// ## Contains
/// Errors for everything that can go wrong in the project
///
/// Useless comments go brr
#[derive(Debug)]
pub enum BlobdlError {
    QueryNotFound,
    UnknownUrl,
    UnsupportedWebsite,
    DomainNotFound,
    UrlParsingError,
    UnknownIssue,
    MissingArgument,
    JsonSerializationError,
    Utf8Error,
    UrlIndexParsingError,
    SerdeError(serde_json::Error),
    IoError(std::io::Error),
    QueryCouldNotBeParsed,
    PlaylistUrlError,

    // This is a 'soft' error: it only means that the ytdlp version could not be checked and should not stop the program. It is handled in main
    CommandNotSpawned,
}

// Things blob-dl regularly tells the user
pub const FFMPEG_UNAVAILABLE_WARNING: &str = "It looks like ffmpeg and ffprobe aren't installed, which means that some of blob-dl's features aren't available!\nPlease install them for a fuller experience";

pub const LONG_ABOUT: &str = "A command line tool used to make downloading youtube videos in various formats easy\nIf you are having problems passing a URL as an argument, try wrapping it in quotes (\"\")!\n\nFor more details check out the github page https://github.com/MicheleCioccarelli/blob-dl\nRecommended yt-dlp version: 2024.10.22";

pub const SHORT_ABOUT: &str = "A command line tool used to make downloading youtube videos in various formats easy\nIf you are having problems passing a URL as an argument, try wrapping it in quotes (\"\")!\n\nFor more details check out the github page https://github.com/MicheleCioccarelli/blob-dl";

pub const YTDLP_NOT_INSTALLED: &str = "blob-dl is a wrapper around yt-dlp and cannot function without it.\nPlease install yt-dlp from the official github page: https://github.com/yt-dlp/yt-dlp";

pub const BEST_QUALITY_PROMPT_PLAYLIST: &str = "Best possible quality for each video";

pub const BEST_QUALITY_PROMPT_SINGLE_VIDEO: &str = "Best possible quality";

pub const SMALLEST_QUALITY_PROMPT_PLAYLIST: &str = "Smallest file size for each video";

pub const SMALLEST_QUALITY_PROMPT_SINGLE_VIDEO: &str = "Smallest file size";

pub const YT_FORMAT_PROMPT_PLAYLIST: &str = "Choose a format to download to every video in (only formats available for all videos are shown)";

pub const YT_FORMAT_PROMPT_SINGLE_VIDEO: &str = "Choose a format to download the video in";

pub const CONVERT_FORMAT_PROMPT_VIDEO_PLAYLIST: &str =
    "Choose a format to recode all the videos to";

pub const CONVERT_FORMAT_PROMPT_VIDEO_SINGLE_VIDEO: &str = "Choose a format to recode the video to";

pub const CONVERT_FORMAT_PROMPT_AUDIO: &str = "Choose an audio format to convert the audios to";

pub const SEE_HELP_PAGE: &str = "Type blob-dl --help for a list of all the available options";

pub const USAGE_MSG: &str = "Usage: blob-dl [OPTIONS] [URL]";

pub const ERROR_RETRY_PROMPT: &str = "The following videos weren't downloaded but retrying might help, choose which videos to re-download [space bar to select]";

pub const UNRECOVERABLE_ERROR_PROMPT: &str =
    "The following videos could not be downloaded due to unrecoverable errors";

pub const DEBUG_REPORT_PROMPT: &str = "By default new errors are flagged as recoverable, if any unrecoverable errors are flagged incorrectly please report them to the github page";

pub const SELECT_ALL: &str = "Select all\n";
pub const SELECT_NOTHING: &str = "Don't re-download anything\n";

pub const WRONG_YTDLP_VERSION: &str = "It looks like you have a yt-dlp version which may not work with blob-dl as expected: you may not be able to fetch formats from youtube.\n\
    To fix this you can update your yt-dlp installation to the correct version with the command: sudo yt-dlp --update-to 2024.10.22";

pub const COMMAND_NOT_SPAWNED: &str = "An instance of ytdlp (used to check which version of the program you have installed) could not be spawned";

// Youtube's error messages
pub const PRIVATE_VIDEO: &str =
    " Private video. Sign in if you've been granted access to this video";

pub const NONEXISTENT_PLAYLIST: &str = " YouTube said: The playlist does not exist.";

pub const HOMEPAGE_REDIRECT: &str =
    " The channel/playlist does not exist and the URL redirected to youtube.com home page";

pub const NETWORK_FAIL: &str = " Unable to download API page: <urlopen error [Errno -3] Temporary failure in name resolution> (caused by URLError(gaierror(-3, 'Temporary failure in name resolution')))";

pub const VIOLENT_VIDEO: &str =
    " This video has been removed for violating YouTube's policy on violent or graphic content";

pub const REMOVED_VIDEO: &str = " Video unavailable. This video has been removed by the uploader";

pub const VIDEO_NOT_FOUND: &str = " not found, unable to continue";

pub const YTDLP_GAVE_UP: &str = " error: HTTP Error 403: Forbidden. Giving up after 10 retries";

pub const NO_API_PAGE: &str = " Unable to download API page: HTTP Error 404: Not Found (caused by <HTTPError 404: 'Not Found'>); please report this issue on https://github.com/yt-dlp/yt-dlp/issues?q= , filling out the appropriate issue template. Confirm you are on the latest version using yt-dlp -U";

pub const ENCODER_STREAM_ERROR: &str = " Postprocessing: Error selecting an encoder for stream 0:1";

pub const NONEXISTENT_VIDEO: &str = "Incomplete data received";

// All copyright error messages begin with this
pub const VIDEO_UNAVAILABLE: &str = " Video unavailable";
// blob-dl custom error messages
pub const BROKEN_URL_ERR: &str =
    "The url provided wasn't recognized, try using a regular youtube url";

pub const UNSUPPORTED_WEBSITE_ERR: &str = "Currently blob-dl only supports downloading youtube videos or playlists, not content from other websites";

pub const UNKNOWN_ISSUE_ERR: &str =
    "Congrats! You ran into an unknown issue, please file a report on blob-dl's github page :)";

pub const MISSING_ARGUMENT_ERR: &str = "You must provide 1 URL";

pub const JSON_SERIALIZATION_ERR: &str =
    "There was a problem serializing this video's format information";

pub const UTF8_ERR: &str = "This video's format information contained non-UTF8 characters and broke the parser, best and worst quality should still work!";

pub const SERDE_ERR: &str =
    "Serde ran into a problem when serializing this video's format information: ";

pub const IO_ERR: &str = "There was an IO error: ";

pub const URL_QUERY_COULD_NOT_BE_PARSED: &str =
    "This url's query could not be parsed, try using a regular youtube url";

pub const URL_INDEX_PARSING_ERR: &str = "The video's index in the playlist couldn't be parsed, please report this issue to the github page";

pub const PLAYLIST_URL_ERROR: &str = "The index/id for the video that you want to download in the playlist could not be parsed.\nTo download just this video try using a url which links directly to it instead of going through a playlist";

impl BlobdlError {
    // Output an error message according to the error at hand
    pub fn report(&self) {
        eprintln!("\n{}\n", USAGE_MSG);
        eprint!("{}: ", "ERROR".red());

        let _ = std::io::stdout().flush();

        match self {
            BlobdlError::QueryNotFound => eprintln!("{}", BROKEN_URL_ERR),

            BlobdlError::UnknownUrl => eprintln!("{}", BROKEN_URL_ERR),

            BlobdlError::UnsupportedWebsite => eprintln!("{}", UNSUPPORTED_WEBSITE_ERR),

            BlobdlError::DomainNotFound => eprintln!("{}", BROKEN_URL_ERR),

            // The link appears to be completely broken
            BlobdlError::UrlParsingError => eprintln!("{}", BROKEN_URL_ERR),

            BlobdlError::UnknownIssue => eprintln!("{}", UNKNOWN_ISSUE_ERR),

            BlobdlError::MissingArgument => eprintln!("{}", MISSING_ARGUMENT_ERR),

            BlobdlError::JsonSerializationError => eprintln!("{}", JSON_SERIALIZATION_ERR),

            BlobdlError::Utf8Error => eprintln!("{}", UTF8_ERR),

            BlobdlError::SerdeError(err) => eprintln!("{} {}", SERDE_ERR, err),

            BlobdlError::IoError(err) => eprintln!("{} {}", IO_ERR, err),

            BlobdlError::QueryCouldNotBeParsed => eprintln!("{}", URL_QUERY_COULD_NOT_BE_PARSED),

            BlobdlError::UrlIndexParsingError => eprintln!("{}", URL_INDEX_PARSING_ERR),

            BlobdlError::PlaylistUrlError => eprintln!("{}", PLAYLIST_URL_ERROR),

            // Early return because this should not be treated as a program-ending error
            BlobdlError::CommandNotSpawned => return,
        }
        eprintln!("{}", SEE_HELP_PAGE);
    }
}

// Implementing conversions and boilerplate
impl std::fmt::Display for BlobdlError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Hi :) I am the BlobdlError default message, I shouldn't show up, if you see me please report me to the github page")
    }
}
impl std::error::Error for BlobdlError {}

impl From<std::io::Error> for BlobdlError {
    fn from(err: std::io::Error) -> Self {
        BlobdlError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for BlobdlError {
    fn from(_: std::str::Utf8Error) -> Self {
        BlobdlError::Utf8Error
    }
}

// Used in run.rs
/// Stores the information found in yt-dlp's error-lines output
#[derive(Debug)]
pub(crate) struct YtdlpError {
    video_id: String,
    error_msg: String,
}

impl YtdlpError {
    pub fn video_id(&self) -> &String {
        &self.video_id
    }

    pub fn error_msg(&self) -> &String {
        &self.error_msg
    }
}

impl std::fmt::Display for YtdlpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result;
        result = format!("{} {}", "yt-video id:", self.video_id);
        result = format!("{}\n   {} {}\n", result, "Reason:", self.error_msg);

        write!(f, "{}", result)
    }
}

impl YtdlpError {
    /// Parses a YtdlpError object from a ytdlp line which contains an error
    pub fn from_error_output(error_line: &str) -> YtdlpError {
        // yt-dlp error line format: ERROR: [...] video_id: reason
        let mut section = error_line.split_whitespace();

        // Skip ERROR:
        section.next().unwrap();

        let mut video_id;

        //  for normal errors this should be [youtube]
        let youtube = section.next().unwrap();

        let is_normal_error = youtube == "[youtube]";
        // todo find a decent way to do this
        let mut strange_err_msg_beginning = "";

        if is_normal_error {
            // This is a usual error, so the video is in the next section
            video_id = section.next().unwrap();
            // Delete the trailing ':'
            video_id = &video_id[..video_id.len() - 1];
        } else {
            // The video doesn't exist, this happens in errors such as NONEXISTENT_VIDEO (see lib.rs)
            strange_err_msg_beginning = youtube;
            video_id = "unavailable";
        }

        // Concatenate together the error message and restore whitespace
        let error_msg = {
            let mut tmp = String::new();
            // I am ashamed
            if !is_normal_error {
                tmp += strange_err_msg_beginning;
            }

            for word in section {
                tmp = tmp + " " + word;
            }
            tmp
        };

        YtdlpError {
            video_id: video_id.to_string(),
            error_msg,
        }
    }
}
