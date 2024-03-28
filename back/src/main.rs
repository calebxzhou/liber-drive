use std::collections::HashMap;
use std::io::Write;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::sync::{Arc, Mutex};
use std::{fs, thread};

use axum::body::Body;

use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;

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

use log::LevelFilter;
use media_sender::handle_file;
use media_sender::handle_preview;
use tower_http::cors::CorsLayer;

use crate::main_service::MainService;
use tower_http::compression::CompressionLayer;
pub mod image_exif;
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

#[tokio::main]
async fn main() {
    logger_init();
    fs::create_dir_all("cache").expect("创建缓存目录失败");
    let drive_dir = util::load_drive_dir();
    let compression_layer = CompressionLayer::new().br(true).compress_when(
        |_status: axum::http::StatusCode,
         _version: axum::http::Version,
         headers: &HeaderMap,
         _extensions: &Extensions| {
            //只压缩json和普通文本
            if let Some(content_type) = headers.get(CONTENT_TYPE) {
                let content_type = content_type.to_str().unwrap_or_default();
                if content_type == "application/json" {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        },
    );
    let mut serv = MainService::new(&drive_dir);
    serv.read_more_media_infos();
    let serv = Box::leak(Box::new(serv));
    let app = Router::new()
        //读取照片视频
        .route(
            "/gallery/:galleryName/:albumName/:mediaName",
            get(get_media).with_state(serv),
        )
        //读取album
        .route(
            "/gallery/:galleryName/:albumName",
            get(get_album).with_state(serv),
        )
        //读取gallery
        .route("/gallery/:name", get(get_gallery).with_state(serv))
        .layer(compression_layer)
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods(tower_http::cors::Any)
                .allow_headers(vec![CONTENT_TYPE]),
        );

    let listener =
        tokio::net::TcpListener::bind(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789))
            .await
            .unwrap();
    axum::serve(listener, app).await.unwrap();
}
macro_rules! match_or_404 {
    ($match:expr) => {
        match $match {
            Some(item) => item,
            None => return StatusCode::NOT_FOUND.into_response(),
        }
    };
}
//获取相册
async fn get_gallery(
    Path(name): Path<String>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    let g = match_or_404!(serv.galleries_info.get(&name));
    Json(g).into_response()
}
//获取影集
async fn get_album(
    Path((gallery_name, album_name)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    let gallery = match_or_404!(serv.galleries.get(&gallery_name));
    let album = match_or_404!(gallery.albums.get(&album_name));
    //请求缩略图，返回第一张的名字
    if params.contains_key("tbnl") {
        if let Some(first) = album.medias.iter().next() {
            return first.0.clone().into_response();
        }
    }
    Json(album).into_response()
}
async fn get_media(
    headers: HeaderMap,
    Path((gallery_name, album_name, media_name)): Path<(String, String, String)>,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> Response<Body> {
    let gallery = match_or_404!(serv.galleries.get(&gallery_name));
    let album = match_or_404!(gallery.albums.get(&album_name));
    let media = match_or_404!(album.medias.get(&media_name));

    //读取预览
    if let Some(level) = params.get("tbnl") {
        return handle_preview(media, if level == "1" { true } else { false }, &headers).await;
    }

    //读取视频时长

    return handle_file(media, &headers).await;
}
