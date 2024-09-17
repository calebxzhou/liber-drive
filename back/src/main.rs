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
use axum::routing::post;
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
macro_rules! match_or_400 { 
    ($match:expr) => {
        match  $match {
            Some(item) => item,
            None => return StatusCode::BAD_REQUEST.into_response(),
        }
    };
}
//获取全部影集 
async fn get_albums(
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {

    //每个album名称 和第一个照片
    let mut new_map: HashMap<String, MediaItem> = HashMap::new();

    for (key, album) in &serv.albums {
        if let Some((_, first_media_item)) = album.medias.iter().next() {
            //没有密码的才显示
            if album.pwd == Option::None {
                new_map.insert(album.name.clone(), first_media_item.clone());
            }
        }
    }
    Json(&new_map).into_response()
}
async fn get_album(
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> impl IntoResponse {
    // Check if params contain albums_path
    let albums_path = match params.get("albums_path") {
        Some(path) => path,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let album_names: Vec<&str> = albums_path.split('/').collect();
    let album = match_or_404!(find_album(&serv.albums, &album_names) );

    //请求缩略图，返回4x MediaItem (第1-2-3-4/5)
    if params.contains_key("tbnl") {
        let media_items: Vec<&MediaItem> = album.medias.values().collect();
        let media_count = media_items.len();

        if media_count <= 4 {
            return Json(media_items).into_response();
        } else {
            let first_fifth = media_items[media_count / 5];
            let second_fifth = media_items[2 * media_count / 5];
            let third_fifth = media_items[3 * media_count / 5];
            let fourth_fifth = media_items[4 * media_count / 5];
            let selected_items = vec![first_fifth, second_fifth, third_fifth, fourth_fifth];
            return Json(selected_items).into_response();
        }
    }
    //验证密码正确
    if let Some(album_pwd) = &album.pwd {
        if let Some(query_pwd) = params.get("pwd") {
            if query_pwd == album_pwd {
                return Json(album).into_response();
            }
        }
        return StatusCode::UNAUTHORIZED.into_response();
    }

    Json(album).into_response()
} 

async fn get_media(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<&MainService>,
) -> Response<Body> {
    let albums_path = match_or_400!(params.get("albums_path"));
    let media_name = match_or_400!(params.get("media_name"));
    let album_names: Vec<&str> = albums_path.split('/').collect();

    let album = match_or_404!(find_album(&serv.albums, &album_names) );

    let media = match_or_404!( album.medias.get(media_name) );
    //读取预览
    if let Some(level) = params.get("tbnl") {
        return handle_preview(media, if level == "1" { true } else { false }, &headers).await;
    }
    //验证密码正确
    if let Some(album_pwd) = &album.pwd {
        if let Some(query_pwd) = params.get("pwd") {
            if query_pwd == album_pwd {
                return handle_file(media, &headers).await;
            }
        }
        return StatusCode::UNAUTHORIZED.into_response();
    }
    //读取视频时长

    handle_file(media, &headers).await
}
fn find_album<'a>(albums: &'a HashMap<String, Album>, album_names: &[&str]) -> Option<&'a Album> {
    let mut current_album = albums.get(album_names[0])?;
    for &name in &album_names[1..] {
        current_album = current_album.sub_albums.get(name)?;
    }
    Some(current_album)
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
        .route("/media", get(get_media).with_state(serv))
        //读取影集 
        .route("/album", get(get_album).with_state(serv)) 
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

    // Create the first server
    let server_v4 = axum_server::bind_rustls(
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 7789),
        config.clone(),
    )
        .serve(app.clone().into_make_service());

    // Create the second server
    let server_v6 = axum_server::bind_rustls(
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789),
        config.clone(),
    )
        .serve(app.clone().into_make_service());

    // Use tokio::try_join! to run both servers concurrently
    match tokio::try_join!(server_v4, server_v6) {
        Ok(_) => info!("Servers ran successfully"),
        Err(e) => eprintln!("Server error: {}", e),
    }
}
