use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::{env, fs};

use album::Album;
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::header::CONTENT_TYPE;
use axum_server::tls_rustls::RustlsConfig;
use log::info;

use axum::http::Extensions;
use axum::http::HeaderMap;
use axum::http::HeaderValue;

use axum::http::Response;

use axum::http::StatusCode;
use axum::Json;
use axum::Router;
use axum::{response::IntoResponse, routing::get};
use media_item::MediaItem;
use media_sender::{handle_file, handle_preview};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};

use crate::main_service::MainService;
pub mod album;
pub mod image_exif;
pub mod main_service;
mod media_item;
pub mod media_sender;
mod test;
mod util;

macro_rules! match_or_404 {
    ($match:expr) => {
        match $match {
            Some(item) => item,
            None => return StatusCode::NOT_FOUND.into_response(),
        }
    };
}
//获取影集
async fn get_albums(
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    //每个album名称 和第一个照片
    let mut new_map: HashMap<String, MediaItem> = HashMap::new();

    for (key, album) in &serv.albums {
        if let Some((_, first_media_item)) = album.medias.iter().next() {
            new_map.insert(album.name.clone(), first_media_item.clone());
        }
    }
    Json(&new_map).into_response()
}
async fn get_album(
    Path(album_name): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    let album = match_or_404!(serv.albums.get(&album_name));
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
    Path((album_name, media_name)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> Response<Body> {
    let album = match_or_404!(serv.albums.get(&album_name));
    let media = match_or_404!(album.medias.get(&media_name));

    //读取预览
    if let Some(level) = params.get("tbnl") {
        return handle_preview(media, if level == "1" { true } else { false }, &headers).await;
    }

    //读取视频时长

    return handle_file(media, &headers).await;
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    util::logger_init();
    fs::create_dir_all("cache/exif").unwrap();
    fs::create_dir_all("cache/video").unwrap();
    fs::create_dir_all("cache").expect("创建缓存目录失败");
    let drive_dirs = util::load_drive_dirs().unwrap();
    let compression_layer = CompressionLayer::new().br(true).compress_when(
        |_status: axum::http::StatusCode,
         _version: axum::http::Version,
         headers: &HeaderMap,
         _extensions: &Extensions| {
            //只压缩json和普通文本
            //不压别的
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

    let serv = MainService::new(&drive_dirs);
    let serv_copy = serv.clone();

    use rayon::prelude::*;
    //todo 多线程建立缩略图
    serv_copy.albums.par_iter().for_each(|(album_name, album)| {
        println!("Album: {}", album_name);
        album
            .medias
            .par_iter()
            .for_each(|(media_name, media_item)| {
                if let Err(e) = media_item.create_preview(true) {
                    info!("创建小图错误：{:?}", e);
                }
                if let Err(e) = media_item.create_preview(false) {
                    info!("创建大图错误：{:?}", e);
                }
            });
    });
    let serv = Box::leak(Box::new(serv));
    let app = Router::new()
        //读取照片视频
        .route("/:albumName/:mediaName", get(get_media).with_state(serv))
        //读取影集
        .route("/:albumName", get(get_album).with_state(serv))
        .route("/", get(get_albums).with_state(serv))
        .layer(compression_layer)
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods(tower_http::cors::Any)
                .allow_headers(Any),
        );

    let config =
        RustlsConfig::from_pem_file(PathBuf::from("../../1.crt"), PathBuf::from("../../1.key"))
            .await
            .unwrap();
    info!("ready");
    axum_server::bind_rustls(
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789),
        config.clone(),
    )
    .serve(app.clone().into_make_service())
    .await
    .unwrap();
}
