use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;

use axum::body::Body;
use axum::extract::State;

use axum::http::header::ETAG;
use axum::http::header::IF_MODIFIED_SINCE;
use axum::http::header::LAST_MODIFIED;
use axum::http::header::{CACHE_CONTROL, CONTENT_TYPE};

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

use log::info;
use log::LevelFilter;
use media_item::GalleryInfo;
use media_sender::handle_file;
use tower_http::cors::CorsLayer;
use util::convert_http_date_to_u64;
use util::convert_u64_to_http_date;

use crate::main_service::MainService;
use tower_http::compression::CompressionLayer;
pub mod main_service;
 mod media_item;
pub mod media_sender;
mod test;
mod util;
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
        let _ = file.write_all(b".");
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
        .br(true)
        .compress_when(
            |_status: axum::http::StatusCode,
             _version: axum::http::Version,
             headers: &HeaderMap,
             _extensions: &Extensions| { 

                //只压缩json和普通文本
                if let Some(content_type) = headers.get(CONTENT_TYPE) {
                    let content_type = content_type.to_str().unwrap_or_default();
                    if content_type == "application/json" {
                        true
                    }else {
                        false
                    }
                }else{
                    false
                }
            },
        );
    let serv = Box::new(MainService::new(&drive_dir));
    let serv: &'static MainService = Box::leak(serv);
    let app = Router::new()
        .route("/galleries", get(get_all_galleries).with_state(serv))
        .route("/gallery/:id", get(get_gallery).with_state(serv))
        .route("/preview/:id/:level", get(get_preview).with_state(serv))
        .route("/media/:id", get(get_media).with_state(serv))
        .route("/exif/:id", get(get_exif).with_state(serv))
        .layer(compression_layer)
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods(tower_http::cors::Any)
                .allow_headers(vec![CONTENT_TYPE]),
        );

    // run our app with hyper, listening globally on port 3000
    let listener =
        tokio::net::TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
//获取所有相册信息
async fn get_all_galleries(State(serv): State<&MainService>) -> Json<Vec<GalleryInfo>> {
    Json(serv.galleries_info.to_vec())
}
//获取单个相册
async fn get_gallery(
    axum::extract::Path(id): axum::extract::Path<u32>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    match serv.galleries.get(&id) {
        Some(g) => Json(g).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
//获取exif信息
async fn get_exif(
    axum::extract::Path(id): axum::extract::Path<u32>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    match serv.medias.get(&id) {
        Some(media) => match &media.exif {
            Some(exif) => Json(exif.clone()).into_response(),
            None => StatusCode::NOT_FOUND.into_response(),
        },
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
//获取webp预览/缩略图
async fn get_preview(
    headers: HeaderMap,
    axum::extract::Path((id, level)): axum::extract::Path<(u32, u8)>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    let media = serv.medias.get(&id).expect("media !exists");
    let modified = media.time;

    let image = match media.get_preview(if level == 2 {true} else { false}) {
        Ok(o) => o,
        Err(e) => {
            return Response::builder()
                .status(500)
                .body(Body::from(format!(
                    "Preview Err! {}, {}",
                    &media.path.display().to_string(),
                    e
                )))
                .unwrap();
        }
    };

    let etag = etag::EntityTag::from_data(image.as_slice());
    let resp = Response::builder()
        .header(CACHE_CONTROL, "public, max-age=3600")
        .header(LAST_MODIFIED, convert_u64_to_http_date(modified).unwrap())
        .header(ETAG, etag.to_string())
        .header(CONTENT_TYPE, "image/webp");
    /* if let Some(if_mod) = headers.get(IF_MODIFIED_SINCE) {
        if modified <= convert_http_date_to_u64(if_mod).unwrap() {
            if let Some(h_etag) = headers.get(ETAG) {
                if h_etag.to_str().unwrap() == etag.to_string() {
                    return resp.status(304).body(Body::from(image)).unwrap();
                }
            }
        }
    } */
    resp.status(200).body(Body::from(image)).unwrap()
}
async fn get_media(
    headers: HeaderMap,
    axum::extract::Path(id): axum::extract::Path<u32>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    let media = serv.medias.get(&id).expect("media !exists");

    handle_file(media, &headers).await
}
 