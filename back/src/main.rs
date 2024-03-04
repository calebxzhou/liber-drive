use std::collections::HashMap;
use std::convert::Infallible;
use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{self, Read, Seek, SeekFrom};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use axum::body::Body;
use axum::body::Bytes;
use axum::error_handling::HandleErrorLayer;
use axum::extract::State;
use axum::http::header;
use axum::http::header::ACCESS_CONTROL_MAX_AGE;
use axum::http::header::CACHE_CONTROL;
use axum::http::header::CONTENT_TYPE;
use axum::http::header::ETAG;
use axum::http::header::IF_MODIFIED_SINCE;
use axum::http::header::LAST_MODIFIED;
use axum::http::header::RANGE;
use axum::http::Extensions;
use axum::http::HeaderMap;
use axum::http::HeaderValue;
use axum::http::Request;
use axum::http::Response;

use axum::http::StatusCode;
use axum::Json;
use axum::Router;
use axum::{response::IntoResponse, routing::get}; 
use chrono::DateTime;
use chrono::Local;
use clap::crate_version;
use env_logger::Builder; 
use gallery::Gallery;

use lazy_static::lazy_static;
use log::info;
use log::LevelFilter;
use media_processing::get_media_preview;
use media_processing::ImageExif;
use media_sender::handle_file;
use mime_guess as mime_types;
use serde_json::ser;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use util::convert_http_date_to_u64;
use util::convert_u64_to_http_date; 


use crate::main_service::MainService; 
use tower_http::compression::CompressionLayer; 
mod gallery;
pub mod main_service;
pub mod media_item;
mod media_processing;
pub mod media_scanner; 
mod test;
mod util;
pub mod media_sender;

//载入日志
fn logger_init() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
}
//载入工作目录
fn load_drive_dir() -> PathBuf {
    let path = std::path::Path::new("./drive_dir.txt");
    let mut file = if path.exists() {
        File::open(&path).unwrap()
    } else {
        let mut file = File::create(&path).unwrap();
        file.write_all(b".");
        file
    };
    let mut contents = String::new();
    let dir = file.read_to_string(&mut contents).unwrap();
    contents = contents.trim().to_owned();
    info!("工作目录：{}", contents);
    let dir = PathBuf::from(contents).canonicalize().unwrap();
    info!("工作目录：{}", dir.display().to_string());
    dir
}
#[tokio::main]
async fn main() {
    logger_init();
    fs::create_dir_all("cache").expect("创建缓存目录失败");
    let drive_dir = load_drive_dir();
    let compression_layer = CompressionLayer::new()
    .gzip(true)
    .deflate(true)
    .br(true)
    .compress_when(|status: axum::http::StatusCode, _version: axum::http::Version, headers: &HeaderMap, _extensions: &Extensions| {
        // Get the content type of the response
        let content_type = headers.get(  CONTENT_TYPE);

        // If the content type is an image or video, do not compress
        if let Some(content_type) = content_type {
            let content_type = content_type.to_str().unwrap_or_default();
            if content_type.starts_with("image/") || content_type.starts_with("video/") {
                return false;
            }
        }

        // Otherwise, compress the response
        true
    }); 
    let serv = MainService::new(&drive_dir);
    let serv = Arc::new(serv);
    let app = Router::new()
        .route(
            "/galleries",
            get(get_all_galleries).with_state(Arc::clone(&serv)),
        )
        .route(
            "/preview/:id/:level",
            get(get_preview).with_state(Arc::clone(&serv)),
        )
        .route(
            "/media/:id",
            get(get_media).with_state(Arc::clone(&serv)),
        )
        .route(
            "/exif/:id",
            get(get_exif).with_state(Arc::clone(&serv)),
        )
        .layer(compression_layer)
    .layer(CorsLayer::new()
        .allow_origin("*".parse::<HeaderValue>().unwrap())
        .allow_methods(tower_http::cors::Any)
        .allow_headers(vec![CONTENT_TYPE])
    );

    // run our app with hyper, listening globally on port 3000
    let listener =
        tokio::net::TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
async fn get_all_galleries(State(serv): State<Arc<MainService>>) -> Json<Vec<Gallery>> {
  
    Json(serv.galleries.deref().to_vec())
}
async fn get_exif(
    axum::extract::Path(id): axum::extract::Path<u32>,
    State(serv): State<Arc<MainService>>
) -> impl IntoResponse {
    match serv.medias.get(&id) {
        Some(media) => {
            match &media.exif {
                Some(exif) => Json(exif.clone()).into_response(),
                None => StatusCode::NOT_FOUND.into_response(),
            }
        },
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
async fn get_preview(
    headers: HeaderMap,
    axum::extract::Path((id, level)): axum::extract::Path<(u32, u8)>,
    State(serv): State<Arc<MainService>>,
) -> impl IntoResponse{
    let media= serv.medias.get(&id).expect("media !exists");
    let modified = media.time;

    
    let image = match get_media_preview(&level, &media.path) {
        Ok(o) => {o},
        Err(e) => {
            return Response::builder().status(500).body(Body::from(format!("Preview Err! {}, {}",&media.path.display().to_string(),e))).unwrap();
        },
    };

    let etag = etag::EntityTag::weak(format!(
        "{0:x}-{1:x}",
        media.size,
        media.time
    ).as_str());
    let resp = Response::builder()
    .header(CACHE_CONTROL, "public, max-age=604800")
    .header(LAST_MODIFIED, convert_u64_to_http_date(modified).unwrap())
    .header(ETAG, etag.to_string())
    .header(CONTENT_TYPE,"image/webp")
    ;
    if let Some(if_mod)  = headers.get(IF_MODIFIED_SINCE){
        if modified <= convert_http_date_to_u64(if_mod).unwrap(){
            return Response::builder().status(304).body(Body::empty()).unwrap();
        }
    }
    resp
    .body(Body::from(image))
    .unwrap()
 
} 
async fn get_media( 
    headers: HeaderMap,
     axum::extract::Path(id): axum::extract::Path<u32>,
     State(serv): State<Arc<MainService>>)-> impl IntoResponse {
    let media = serv.medias.get(&id).expect("media !exists");
 
    handle_file(media,&headers).await

}  
 
/* impl MainHandler { 

    fn send_file<P: AsRef<Path>>(&self, req: &Request, path: P) -> IronResult<Response> {
        use filetime::FileTime;
        use iron::headers::{
            AcceptRanges, ByteRangeSpec, ContentLength, ContentRange, ContentRangeSpec,
            ContentType, EntityTag, IfMatch, IfRange, Range, RangeUnit,
        };
        use iron::headers::{CacheControl, CacheDirective, HttpDate};
        use iron::method::Method;

        let path = path.as_ref();
        let metadata = fs::metadata(path).map_err(error_io2iron)?;

        let time = FileTime::from_last_modification_time(&metadata);
        let modified = time::Timespec::new(time.seconds() as i64, 0);
        let etag = EntityTag::weak(format!(
            "{0:x}-{1:x}.{2:x}",
            metadata.len(),
            modified.sec,
            modified.nsec
        ));

        let mut resp = Response::with(status::Ok);

        resp.headers.set(AcceptRanges(vec![RangeUnit::Bytes]));

        match req.method {
            Method::Head => {
                let content_type = req
                    .headers
                    .get::<ContentType>()
                    .cloned()
                    .unwrap_or_else(|| ContentType(Mime(TopLevel::Text, SubLevel::Plain, vec![])));

                resp.headers.set(content_type);
                resp.headers.set(ContentLength(metadata.len()));
            }
            Method::Get => {
                // Set mime type
                let mime = mime_types::from_path(path).first_or_octet_stream();
                resp.headers
                    .set_raw("content-type", vec![mime.to_string().into_bytes()]);

                let mut range = req.headers.get::<Range>();

                if range.is_some() {
                    // [Reference]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/If-Match
                    // Check header::If-Match
                    if let Some(IfMatch::Items(items)) = req.headers.get::<IfMatch>() {
                        if !items.iter().any(|item| item.strong_eq(&etag)) {
                            return Err(IronError::new(
                                StringError("Etag not matched".to_owned()),
                                status::RangeNotSatisfiable,
                            ));
                        }
                    };
                }

                // [Reference]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/If-Range
                let matched_ifrange = match req.headers.get::<IfRange>() {
                    Some(IfRange::EntityTag(etag_ifrange)) => etag.weak_eq(etag_ifrange),
                    Some(IfRange::Date(HttpDate(date_ifrange))) => {
                        time::at(modified) <= *date_ifrange
                    }
                    None => true,
                };
                if !matched_ifrange {
                    range = None;
                }

                match range {
                    Some(Range::Bytes(ranges)) => {
                        if let Some(range) = ranges.get(0) {
                            let (offset, length) = match *range {
                                ByteRangeSpec::FromTo(x, mut y) => {
                                    // "x-y"
                                    if x >= metadata.len() || x > y {
                                        return Err(IronError::new(
                                            StringError(format!("Invalid range(x={}, y={})", x, y)),
                                            status::RangeNotSatisfiable,
                                        ));
                                    }
                                    if y >= metadata.len() {
                                        y = metadata.len() - 1;
                                    }
                                    (x, y - x + 1)
                                }
                                ByteRangeSpec::AllFrom(x) => {
                                    // "x-"
                                    if x >= metadata.len() {
                                        return Err(IronError::new(
                                                StringError(format!(
                                                    "Range::AllFrom to large (x={}), Content-Length: {})",
                                                    x, metadata.len())),
                                                status::RangeNotSatisfiable
                                            ));
                                    }
                                    (x, metadata.len() - x)
                                }
                                ByteRangeSpec::Last(mut x) => {
                                    // "-x"
                                    if x > metadata.len() {
                                        x = metadata.len();
                                    }
                                    (metadata.len() - x, x)
                                }
                            };
                            let mut file = fs::File::open(path).map_err(error_io2iron)?;
                            file.seek(SeekFrom::Start(offset)).map_err(error_io2iron)?;
                            let take = file.take(length);

                            resp.headers.set(ContentLength(length));
                            resp.headers.set(ContentRange(ContentRangeSpec::Bytes {
                                range: Some((offset, offset + length - 1)),
                                instance_length: Some(metadata.len()),
                            }));
                            resp.body = Some(Box::new(Box::new(take) as Box<dyn Read + Send>));
                            resp.set_mut(status::PartialContent);
                        } else {
                            return Err(IronError::new(
                                StringError("Empty range set".to_owned()),
                                status::RangeNotSatisfiable,
                            ));
                        }
                    }
                    Some(_) => {
                        return Err(IronError::new(
                            StringError("Invalid range type".to_owned()),
                            status::RangeNotSatisfiable,
                        ));
                    }
                    _ => {
                        resp.headers.set(ContentLength(metadata.len()));
                        let file = fs::File::open(path).map_err(error_io2iron)?;
                        resp.body = Some(Box::new(file));
                    }
                }
            }
            _ => {
                return Ok(Response::with(status::MethodNotAllowed));
            }
        }

        static SECONDS: u32 = 7 * 24 * 3600; // max-age: 7.days()
        if let Some(&IfModifiedSince(HttpDate(ref if_modified_since))) =
            req.headers.get::<IfModifiedSince>()
        {
            if modified <= if_modified_since.to_timespec() {
                return Ok(Response::with(status::NotModified));
            }
        };
        let cache = vec![CacheDirective::Public, CacheDirective::MaxAge(SECONDS)];
        resp.headers.set(CacheControl(cache));
        resp.headers.set(LastModified(HttpDate(time::at(modified))));
        resp.headers.set(ETag(etag));
        Ok(resp)
    }
}
 */
