use std::collections::HashMap;
use std::convert::Infallible;
use std::io::Write;
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use std::{fs, thread};

#[macro_use]
extern crate rocket;

use album::Album;
use log::{error, info};
use main_service::AlbumList;
use media_item::{is_image, is_video};
use rocket::figment::value::magic::RelativePathBuf;
use rocket::fs::{relative, FileServer, NamedFile};
use rocket::http::Method;
use rocket::State;
use rocket_async_compression::Compression;
use rocket_cache_response::CacheResponse;
use rocket_cors::{AllowedOrigins, CorsOptions};
use tower_http::cors::CorsLayer;

use crate::main_service::MainService;
use tower_http::compression::CompressionLayer;
pub mod album;
pub mod image_exif;
pub mod main_service;
mod media_item;
pub mod media_sender;
mod test;
mod util;
use rocket::serde::json::Json;
use serde::Deserialize;

#[get("/albums")]
fn get_albums(serv: &State<MainService>) -> Json<&AlbumList> {
    Json(&serv.albums)
}
#[get("/media/<album_name>/<media_name>")]
async fn get_media(
    album_name: String,
    media_name: String,
    serv: &State<MainService>,
) -> CacheResponse<Option<NamedFile>> {
    CacheResponse::Public {
        responder: if let Some(album) = serv.albums.get(&album_name) {
            if let Some(media) = album.medias.get(&media_name) {
                NamedFile::open(&media.path).await.ok()
            } else {
                None
            }
        } else {
            None
        },

        max_age: 60 * 60 * 24 * 7, // cached for seconds
        must_revalidate: false,
    }
}
#[get("/album/<album_name>")]
fn get_album(album_name: String, serv: &State<MainService>) -> Option<Json<&Album>> {
    if let Some(album) = serv.albums.get(&album_name) {
        return Some(Json(album));
    }
    None
}

#[launch]
fn rocket() -> _ {
    util::logger_init();
    fs::create_dir_all("cache/exif").unwrap();
    fs::create_dir_all("cache/video").unwrap();
    fs::create_dir_all("cache").expect("创建缓存目录失败");
    let drive_dirs = util::load_drive_dirs().unwrap();
    let config = rocket::Config::figment()
        .merge(("port", 7789))
        .merge(("address", "::0"));
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post, Method::Patch]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true);
    let mut serv = MainService::new(&drive_dirs);
    let serv_copy = serv.clone();

    use rayon::prelude::*;

    serv_copy.albums.par_iter().for_each(|(album_name, album)| {
        println!("Album: {}", album_name);
        album
            .medias
            .par_iter()
            .for_each(|(media_name, media_item)| {
                if let Err(e) = media_item.create_preview(true) {
                    info!("创建小图错误：{:?}", e);
                    if let Err(e) = media_item.create_preview(false) {
                        info!("创建大图错误：{:?}", e);
                    }
                }
            });
    });

    let mut server = rocket::custom(config)
        .manage(serv)
        .mount("/", routes![get_media, get_albums, get_album])
        .mount("/cache", FileServer::from(relative!("cache")))
        .attach(cors.to_cors().unwrap())
        .attach(Compression::fairing());
    server
}

/* #[tokio::main]
async fn main() {
    //todo 多线程建立缩略图 读取exif和视频时长
    let handle = tokio::spawn(async move {
        info!("开始读取exif信息");
        let serv = serv.clone();
        let mut serv = serv.write().unwrap();
        for (album_name, album) in &mut serv.albums {
            println!("Album: {}", album_name);
            for (media_name, media_item) in &mut album.medias {
                if is_image(&media_item.path) {
                    media_item.update_exif_info();
                } else if is_video(&media_item.path) {
                    media_item.update_video_duration();
                }
            }
        }
    });
} */

/* async fn get_media(
    // headers: HeaderMap,
    Path((album_name, media_name)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    State(serv): State<Arc<RwLock<MainService>>>,
) -> Response<Body> {
    let serv = serv.read().unwrap();
    if let Some(album) = serv.albums.get(&album_name) {
        if let Some(media) = album.medias.get(&media_name) {
            //读取预览
            if let Some(level) = params.get("tbnl") {
                return handle_preview(media, if level == "1" { true } else { false }, &headers)
                    .await;
            }
            return handle_file(media, &headers).await;
        }
        return StatusCode::NOT_FOUND.into_response();
    }
    StatusCode::NOT_FOUND.into_response()
}
 */
//获取影集

//请求缩略图，返回第一张的名字
/* if params.contains_key("tbnl") {
    if let Some(first) = album.medias.iter().next() {
        return Ok(first.0.clone().into_response());
    }
} */
