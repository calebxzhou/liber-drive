use axum::{
    body::Body,
    http::{
        header::{
            self, ACCEPT_RANGES, CACHE_CONTROL, CONTENT_TYPE, ETAG, IF_MODIFIED_SINCE,
            LAST_MODIFIED,
        },
        HeaderMap, Response, StatusCode,
    },
    response::IntoResponse,
};
use std::io::SeekFrom;
use tokio::{fs::File, io::AsyncSeekExt};
use tokio_util::io::ReaderStream;

use crate::{
    media_item::{is_heif_image, is_jpg_image, is_video, MediaItem},
    util::{convert_http_date_to_u64, convert_u64_to_http_date},
};

#[derive(Debug)]
struct Range {
    start: u64,
    end: Option<u64>,
}

fn parse_range(range: &str) -> Option<Range> {
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

async fn build_response(
    media: &MediaItem,
    headers: &HeaderMap,
    content_type: &str,
    body: Body,
) -> Response<Body> {
    let modified = media.time;
    let etag = etag::EntityTag::weak(format!("{0:x}-{1:x}", media.size, media.time).as_str());
    let mut resp = Response::builder()
        .header(CACHE_CONTROL, "public, max-age=604800")
        .header(LAST_MODIFIED, convert_u64_to_http_date(modified).unwrap())
        .header(ETAG, etag.to_string())
        .header(CONTENT_TYPE, content_type);

    if let Some(if_mod) = headers.get(IF_MODIFIED_SINCE) {
        if modified <= convert_http_date_to_u64(if_mod).unwrap() {
            return Response::builder().status(304).body(Body::empty()).unwrap();
        }
    }

    resp.status(200).body(body).unwrap()
}

pub async fn handle_preview(media: &MediaItem, tbnl: bool, headers: &HeaderMap) -> Response<Body> {
    let image = match media.get_preview(tbnl) {
        Ok(o) => o,
        Err(e) => {
            return Response::builder()
                .status(404)
                .body(Body::from(format!(
                    "Preview Err! {}, {}",
                    &media.path.display().to_string(),
                    e
                )))
                .unwrap();
        }
    };

    build_response(media, headers, "image/webp", Body::from(image)).await
}

pub async fn handle_file(media: &MediaItem, headers: &HeaderMap) -> Response<Body> {
    let range = headers.get(header::RANGE);
    let path = &media.path;
    let mut file = File::open(path).await.unwrap();
    let file_size = media.size;

    let range = range.and_then(|r| parse_range(r.to_str().unwrap()));

    let content_type = if is_heif_image(&media.path) {
        "image/heic"
    } else if is_jpg_image(&media.path) {
        "image/jpeg"
    } else if is_video(&media.path) {
        "video/mp4"
    } else {
        return Response::builder()
            .status(400)
            .body(Body::from("not implemented file type"))
            .unwrap();
    };

    if let Some(range) = range {
        if range.start >= file_size {
            return Response::builder()
                .status(400)
                .body(Body::from("range start exceeds file size"))
                .unwrap();
        }

        if let Err(_err) = file.seek(SeekFrom::Start(range.start)).await {
            return Response::builder()
                .status(400)
                .body(Body::from("cant Seek to the start position of the range"))
                .unwrap();
        }

        let end = range.end.unwrap_or(file_size - 1).min(file_size - 1);
        let len = end - range.start + 1;
        let stream = ReaderStream::new(file);

        return Response::builder()
            .status(206)
            .header(ACCEPT_RANGES, "bytes")
            .header(header::CONTENT_RANGE, format!("bytes {}-{}/{}", range.start, end, file_size))
            .header(header::CONTENT_LENGTH, len.to_string())
            .header(CONTENT_TYPE, content_type)
            .body(Body::from_stream(stream))
            .unwrap();
    }

    let stream = ReaderStream::new(file);
    build_response(media, headers, content_type, Body::from_stream(stream)).await
}
