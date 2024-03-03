export interface Gallery {
    id: number;
    name: string;
    size: number;
    medias: Map<number, MediaItem>;
}

export  interface MediaItem {
    id: number;
    path: string; // PathBuf in Rust can be represented as string in TypeScript
    name: string;
    time: number;
    size: number;
    exif?: ImageExif;
}

export interface ImageExif {
    make: string; // 相机
    lens: string; // 镜头
    xp_prog: string; // 档位, char in Rust can be represented as string in TypeScript
    focal_len: string; // 焦距
    aperture: string; // 光圈
    shutter: string; // 快门
    iso: string; // ISO
    shot_time: string; // 拍摄时间
}