use std::collections::HashMap;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::{env, fs};
use std::sync::Arc;
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
use rayon::prelude::*;
use serde::Deserialize;
use tokio::sync::Mutex;
use crate::main_service::{MainService, SharedService};
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
pub fn collect_media_from_sub_albums<'a>(album: &'a Album, media_ids: &mut Vec<String>) {
    for sub_album in album.sub_albums.values() {
        for media in sub_album.medias.values() {
            media_ids.push(media.get_media_id());
        }
        if media_ids.len() >= 4 {
            return;
        }
        collect_media_from_sub_albums(sub_album, media_ids);
    }
}

fn get_selected_media_items(media_items: Vec<String>) -> Vec<String> {
    if media_items.len() <= 4 {
        media_items
    } else {
        media_items.into_iter().take(4).collect()
    }
}

async fn list_all_albums(
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<SharedService>,
) -> impl IntoResponse {
    let serv = serv.read().await;
    let mut new_map: HashMap<String, Vec<String>> = HashMap::new();

    for (key, album) in &serv.albums {
        let mut media_items: Vec<String> = album.medias.values().map(|item| item.get_media_id().clone()).collect();
        //不够4个 去子相册里拿
        if media_items.len() < 4 {
            collect_media_from_sub_albums(album, &mut media_items);
        }

        let selected_items = get_selected_media_items(media_items);
        new_map.insert(album.name.clone(), selected_items);
    }

    Json(&new_map).into_response()
}

#[axum_macros::debug_handler]

async fn get_album(
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<SharedService>,
) -> impl IntoResponse {
    let serv = serv.read().await;
    let albums_path = match params.get("path") {
        Some(path) => path,
        None => return StatusCode::BAD_REQUEST.into_response(),
    };

    let album_names: Vec<&str> = albums_path.split('/').collect();
    let album = match_or_404!(find_album(&serv.albums, &album_names));
    if params.get("has_pwd").is_some(){
        return if album.pwd.is_some() {
            "true".into_response()
        } else {
            "false".into_response()
        }
    }
    if params.contains_key("tbnl") {
        let media_items: Vec<&MediaItem> = album.medias.values().collect();
        let selected_items = get_selected_media_items(media_items.iter().map(|i|i.get_media_id().clone()).collect());
        return Json(selected_items).into_response();
    }

    if let Some(album_pwd) = &album.pwd {
        if let Some(query_pwd) = params.get("pwd") {
            if query_pwd == album_pwd {
                return Json(album.clone().into_info()).into_response();
            }
        }
        return StatusCode::UNAUTHORIZED.into_response();
    }

    Json(album.clone().into_info()).into_response()
}


async fn get_media(
    headers: HeaderMap,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<SharedService>,
) -> Response<Body> {
    let serv = serv.read().await;
    let albums_path = params.get("path");
    let media_name = params.get("name");
    let media_id = params.get("id");

    let media = if let Some(media_id) = media_id {
        serv.medias.get(media_id)
    } else if let Some((albums_path, media_name)) = albums_path.zip(media_name) {
        let album_names: Vec<&str> = albums_path.split('/').collect();
        let album = match_or_404!(find_album(&serv.albums, &album_names));
        album.medias.get(media_name)
    } else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let media = match_or_404!(media);

    if let Some(media_pwd) = &media.pwd {
        return if params.get("pwd").map_or(false, |query_pwd| query_pwd == media_pwd) {
            handle_media_request(media, &params, &headers).await
        } else {
            StatusCode::UNAUTHORIZED.into_response()
        }
    }

    handle_media_request(media, &params, &headers).await
}

async fn handle_media_request(
    media: &MediaItem,
    params: &HashMap<String, String>,
    headers: &HeaderMap,
) -> Response<Body> {
    if let Some(level) = params.get("tbnl") {
        handle_preview(media, level == "1", headers).await
    } else {
        handle_file(media, headers).await
    }
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
    fs::create_dir_all("cache/service").unwrap();
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
    
    {
        let serv = serv.read().await;
        info!("ok 共{}个相册", serv.albums.len());
    }
    let app = Router::new()

        //读取照片视频
        .route("/media", get(get_media).with_state(serv.clone()))
        //读取影集
        .route("/album", get(get_album).with_state(serv.clone()))
        .route("/", get(list_all_albums).with_state(serv.clone()))
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
