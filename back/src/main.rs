
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io::{Read};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::ops::Deref;
use std::path::PathBuf;

use std::sync::Arc;

use axum::body::Body; 
use axum::extract::State; 

use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};
use axum::http::header::ETAG;
use axum::http::header::IF_MODIFIED_SINCE;
use axum::http::header::LAST_MODIFIED;

use axum::http::Extensions;
use axum::http::HeaderMap;
use axum::http::HeaderValue;

use axum::http::Response;

use axum::http::StatusCode;
use axum::Json;
use axum::Router;
use axum::{response::IntoResponse, routing::get}; 

use chrono::Local;

use env_logger::Builder; 
use gallery::{GalleryInfo};


use log::info;
use log::LevelFilter;
use media_processing::get_media_preview;

use media_sender::handle_file; 
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
        File::open(path).unwrap()
    } else {
        let mut file = File::create(path).unwrap();
        file.write_all(b".");
        file
    };
    let mut contents = String::new();
    let _dir = file.read_to_string(&mut contents).unwrap();
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
    .compress_when(|_status: axum::http::StatusCode, _version: axum::http::Version, headers: &HeaderMap, _extensions: &Extensions| {
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
            "/gallery/:id",
            get(get_gallery).with_state(Arc::clone(&serv)),
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
async fn get_all_galleries(State(serv): State<Arc<MainService>>) -> Json<Vec<GalleryInfo>> { 
    
    Json(serv.galleries_info.deref().to_vec())
}
async fn get_gallery(
    axum::extract::Path(id): axum::extract::Path<u32>,
    State(serv): State<Arc<MainService>>
) -> impl IntoResponse{
    match serv.galleries.get(&id) {
        Some(g) => {
            Json(g).into_response()
        },
        None => StatusCode::NOT_FOUND.into_response(),
    } 
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

    let etag = etag::EntityTag::from_data(image.as_slice());
    let resp = Response::builder()
    .header(CACHE_CONTROL, "public, max-age=604800")
    .header(LAST_MODIFIED, convert_u64_to_http_date(modified).unwrap())
    .header(ETAG, etag.to_string())
    .header(CONTENT_TYPE,"image/webp")
    ;
    if let Some(if_mod)  = headers.get(IF_MODIFIED_SINCE){
        if modified <= convert_http_date_to_u64(if_mod).unwrap() {
            if let Some(h_etag) = headers.get(ETAG){
                if h_etag.to_str().unwrap() == etag.to_string(){
                    return Response::builder().status(304).body(Body::empty()).unwrap();
                }
            }
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