export interface GalleryInfo {
    name: string;
    size: number;
    albums: AlbumInfo[];
}
export interface Album {
    name: string;
    size: number;
    medias: { [key: string]: Media };
}
export interface AlbumInfo {
    name: string;
    size: number;
    media_amount: number;
}
export  interface Media {
    name: string;
    time: number;
    size: number;
}
export const DefaultGallery: GalleryInfo = {
    name: '相册载入中...',
    size: 0,
    albums:[]
  };
  export const DefaultAlbum: Album = {
    name: '影集载入中...',
    size: 0,
    medias: {}
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
}
