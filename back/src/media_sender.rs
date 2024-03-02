use axum::{
    body::Body,
    http::{
        header::{self, ACCEPT_RANGES, CONTENT_LENGTH, CONTENT_TYPE},
        HeaderMap, Response, StatusCode,
    },
    response::IntoResponse,
};
use std::{fs, io::SeekFrom, net::SocketAddr, os::windows::fs::MetadataExt, path::PathBuf};
use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncSeekExt},
};
use tokio_util::io::ReaderStream;

use crate::media_item::MediaItem;

// A struct to hold the range header value
#[derive(Debug)]
struct Range {
    start: u64,
    end: Option<u64>,
}

// A function to parse the range header value
fn parse_range(range: &str) -> Option<Range> {
    // The range header value should be in the format "bytes=start-end"
    if range.starts_with("bytes=") {
        let parts: Vec<&str> = range["bytes=".len()..].split('-').collect();
        if parts.len() == 2 {
            let start = parts[0].parse().ok()?;
            let end = if parts[1].is_empty() {
                None
            } else {
                Some(parts[1].parse().ok()?)
            };
            return Some(Range { start, end });
        }
    }
    None
}

// A function to handle the request
pub async fn handle_file(
    media: &MediaItem,
    range: Option<header::HeaderValue>,
) -> impl IntoResponse {
    let resp = Response::builder();
    let path = &media.path;
    // Try to open the file
    let mut file = File::open(path).await.unwrap();

    // Get the file size
    let file_size = media.size;

    // Parse the range header value
    let range = if let Some(range) = range {
        parse_range(range.to_str().unwrap())
    } else {
        None
    };

    let resp = resp
        .header(ACCEPT_RANGES, "bytes")
        .header(
            CONTENT_TYPE,
            if media.is_image() {
                "image/jpeg"
            } else if media.is_video() {
                "video/mp4"
            } else {
                return Response::builder()
                    .status(400)
                    .body(Body::from("not implemented file type"))
                    .unwrap();
            },
        );

    // If the range is valid, return a partial content response
    if let Some(range) = range {
        // Check if the range is satisfiable
        if range.start >= file_size {
            return Response::builder()
                .status(400)
                .body(Body::from("range start exceeds file size"))
                .unwrap();
        }

        // Seek to the start position of the range
        if let Err(err) = file.seek(SeekFrom::Start(range.start)).await {
            return Response::builder()
                .status(400)
                .body(Body::from("cant Seek to the start position of the range"))
                .unwrap();
        }

        // Calculate the end position and the content length of the range
        let end = range.end.unwrap_or(file_size - 1).min(file_size - 1);
        let len = end - range.start + 1;

        // Convert the file into a stream
        let stream = ReaderStream::new(file);

        // Set the content range and content length headers
        // Return a partial content response with the stream
        resp
        .status(206)
        .header(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", range.start, end, file_size),
        )
        .header(header::CONTENT_LENGTH, len.to_string())
        .body(Body::from_stream(stream))
        .unwrap()
    } else {
        //无range 返回全文件
        let stream = ReaderStream::new(file);
        resp
        .header(
            header::CONTENT_RANGE,
            format!("bytes {}-{}/{}", 0, file_size, file_size),
        )
        .header(header::CONTENT_LENGTH, file_size.to_string())
            .body(Body::from_stream(stream))
            .unwrap()
    }
}
