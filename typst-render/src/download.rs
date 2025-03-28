use std::fmt::Display;
use typst_kit::download::{DownloadState, Downloader, Progress};

/// Prints download progress by writing `downloading {0}` followed by repeatedly
/// updating the last terminal line.
pub struct PrintDownload<T>(pub T);

impl<T: Display> Progress for PrintDownload<T> {
    fn print_start(&mut self) {}

    fn print_progress(&mut self, state: &DownloadState) {}

    fn print_finish(&mut self, state: &DownloadState) {}
}

/// Returns a new downloader.
pub fn downloader() -> Downloader {
    let user_agent = concat!("typst/", env!("CARGO_PKG_VERSION"));
    // There's a chance we need to provide a cert
    // Refer to typst-cli source to see how to do it

    Downloader::new(user_agent)
}
