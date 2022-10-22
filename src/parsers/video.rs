use reqwest_impersonate::Url;

/// A video hosted on a website
struct Video {
    /// The url to the video
    url: Url,
    /// The format of the video.
    /// If it is a "Container" format, the quality may be included as a number representing `{quality}p`
    format: VideoFormat,
    /// The size of the video, if available
    size: Option<usize>,
}

/// The format of the video
enum VideoFormat {
    Container(Option<u32>),
    Stream,
}
