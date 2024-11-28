use url::Url;

use super::error::{BlobResult, BlobdlError};

#[derive(Debug, PartialOrd, PartialEq, Clone)]
pub enum DownloadOption {
    YtVideo(usize),
    YtPlaylist,
}

pub fn analyze_url(command_line_url: &str) -> BlobResult<DownloadOption> {
    return if let Ok(url) = Url::parse(command_line_url) {
        if let Some(domain_name) = url.domain() {
            if domain_name.contains("youtu") {
                inspect_yt_url(url)
            } else {
                Err(BlobdlError::UnsupportedWebsite)
            }
        } else {
            Err(BlobdlError::DomainNotFound)
        }
    } else {
        Err(BlobdlError::UrlParsingError)
    };
}

fn inspect_yt_url(yt_url: Url) -> BlobResult<DownloadOption> {
    if let Some(query) = yt_url.query() {
        if query.contains("&index=")
            && (yt_url.path().contains("playlist") || query.contains("list"))
        {
            return Ok(DownloadOption::YtPlaylist);
        }
        return Ok(DownloadOption::YtVideo(0));
    }

    Err(BlobdlError::QueryCouldNotBeParsed)
}
