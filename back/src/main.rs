use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::{self, Read, Seek, SeekFrom};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::crate_version;
use iron::headers::{
    ContentType, ETag, IfModifiedSince, LastModified,
};
use iron::mime::{Mime, SubLevel, TopLevel};
use iron::status;
use iron::{Chain, Handler, Iron, IronError, IronResult, Request, Response, Set};
use iron_cors::CorsMiddleware;
use lazy_static::lazy_static;
use mime_guess as mime_types;
use mime_types::from_path;
use path_dedot::ParseDot;
use percent_encoding::percent_decode;
use serde::Serialize;
use termcolor::Color;

use color::{build_spec, Printer};
use middlewares::RequestLogger;
use util::{error_io2iron, StringError};
use walkdir::WalkDir;

use crate::media_processing::{get_image_exif, get_media_preview};

mod color;
mod media_processing;
mod middlewares;
mod test;
mod util;
pub mod media_item;
pub mod media_scanner;


fn main() {
    let drive_dir = read_file_to_string("drive_dir.txt").expect("工作目录读取失败");
    println!("工作目录：{}", drive_dir);
    let drive_dir = PathBuf::from(drive_dir).canonicalize().unwrap();

    //扫描

    let mut chain = Chain::new(MainHandler { drive_dir });

    chain.link_around(CorsMiddleware::with_allow_any());

    chain.link_after(RequestLogger {
        printer: Printer::new(),
    });

    let mut server = Iron::new(chain);
    server.threads = 256; //threads as usize;

    /* #[cfg(feature = "native-tls")]
    let rv = if let Some(cert) = cert {
        use hyper_native_tls::NativeTlsServer;
        let ssl = NativeTlsServer::new(cert, certpass.unwrap_or("")).unwrap();
        server.https(
            SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789),
            ssl,
        )
    } else { */
        server.http(SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 7789));
    //};


}

struct MainHandler {
    drive_dir: PathBuf,
}
fn read_file_to_string(file_path: &str) -> std::io::Result<String> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}


impl Handler for MainHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut fs_path = self.drive_dir.clone();
        let path_prefix = req
            .url
            .path()
            .into_iter()
            .filter(|s| !s.is_empty())
            .map(|s| {
                percent_decode(s.as_bytes())
                    .decode_utf8()
                    .map(|path| PathBuf::from(&*path))
                    .map_err(|_err| {
                        IronError::new(
                            StringError(format!("invalid path: {}", s)),
                            status::BadRequest,
                        )
                    })
            })
            .collect::<Result<Vec<PathBuf>, _>>()?
            .into_iter()
            .collect::<PathBuf>();
        fs_path.push(&path_prefix);
        let fs_path = fs_path.parse_dot().unwrap();
        let path_metadata = match fs::metadata(&fs_path) {
            Ok(value) => value,
            Err(err) => {
                let status = match err.kind() {
                    io::ErrorKind::PermissionDenied => status::Forbidden,
                    io::ErrorKind::NotFound => status::NotFound,
                    _ => status::InternalServerError,
                };
                return Err(IronError::new(err, status));
            }
        };
        //所有请求参数
        let params: HashMap<String, String> =
            url::form_urlencoded::parse(req.url.query().unwrap_or("").as_bytes())
                .into_owned()
                .collect();
        //预览级别 0=webp压图 1=512x缩图 2=256x缩图 3=128缩图 4=64缩图 5=32缩图
        if let Some(pre_lvl) = params.get("preview") {
            let tbnl = get_media_preview(&pre_lvl.parse::<u8>().unwrap_or(0), &fs_path).expect("");
            let mut response = Response::with((status::Ok, tbnl));
            response.headers.set(ContentType(Mime(
                TopLevel::Image,
                SubLevel::from_str("webp").unwrap(),
                vec![],
            )));
            return Ok(response);
        }
        if let Some(meta) = params.get("meta") {
            let meta = meta.as_str();
            let response = match meta {
                //元数据：文件尺寸
                "size" => Response::with((
                    status::Ok,
                    fs::metadata(&fs_path).unwrap().len().to_string(),
                )),
                //元数据：exif信息
                "exif" => match get_image_exif(&fs_path) {
                    Ok(o) => Response::with((status::Ok, serde_json::to_string(&o).unwrap())),
                    Err(_e) => Response::with(status::Ok),
                },
                &_ => Response::with(status::BadRequest),
            };
            return Ok(response);
        }

        //发送目录/文件
        if path_metadata.is_dir() {
            self.list_directory_all_files( &fs_path)
        } else {
            self.send_file(req, &fs_path)
        }
    }
}

#[derive(Serialize)]
struct FileItem {
    name: String,
    time: u64,
    size: u64,
}
impl MainHandler {
    //发送目录
    fn list_directory_all_files(&self, fs_path: &Path) -> IronResult<Response> {

        let mut resp = Response::with(status::Ok);
        let fs_path = fs_path.to_owned();
        let read_dir = fs::read_dir(&fs_path).map_err(error_io2iron)?;
        let mut entries = Vec::new();
        for entry_result in read_dir {
            let entry = entry_result.map_err(error_io2iron)?;

            let mut file_name = entry.file_name().into_string().unwrap();
            //如果是目录 文件名最后加个斜杠
            if entry.file_type().unwrap().is_dir() {
                file_name.push('/');
            }
            entries.push(file_name);
        }
        //发送目录下所有文件名
        resp.set_mut(serde_json::to_string(&entries).unwrap());

        Ok(resp)
    }

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
