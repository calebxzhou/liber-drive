import { v4 as uuidv4 } from "uuid";

export interface Album {
  name: string;
  size: number;
  medias: { [key: string]: Media };
}
export interface AlbumInfo {
  name: string;
  tbnl_url: string;
  latest_media_time: number;
}
export interface Media {
  name: string;
  time: number; //unix timestamp
  size: number;
  exif?: ImageExif;
  duration?: number;
}
export function getMediaId(media: Media) {
  return `${media.time}_${media.size}`;
}
export const DefaultAlbum: Album = {
  name: "影集载入中...",
  size: 0,
  medias: {},
};
export interface ImageExif {
  make: string; // 相机
  lens: string; // 镜头
  xp_prog: string; // 档位
  focal_len: string; // 焦距
  av: string; // 光圈
  tv: string; // 快门
  iso: string; // ISO
  shot_time: string; // 拍摄时间
  meter_mode: string;
  exp_cp: string;
  flash: string;
  loca: GPSLocation;
}
export interface GPSLocation {
  alt: string;
  lng: string;
  lat: string;
}
