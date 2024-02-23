import { FILE_TYPE_EXTENSION } from "./const";

export class FileItem {
  constructor(public name: string, public queryUrl: string) {
  }
  //扩展名
  extension = this.name
    .slice(((this.name.lastIndexOf(".") - 1) >>> 0) + 2)
    .toLowerCase();
  //是否目录
  isDir = this.name.endsWith("/");
  //显示名称 不显示下划线 不显示结尾斜杠
  displayName = this.name
    .replaceAll("_", " ")
    .replaceAll(".", " .")
    .replaceAll("/", "");
  //类型 无法识别则为file
  type = Object.entries(FILE_TYPE_EXTENSION).reduce(
    (acc, [key, value]) => (value.includes(this.extension) ? key : acc),
    "file"
  );
  //是否图片
  isImage = this.type === "img";
  //是否视频
  isVideo =  this.type === "video";
  //是否媒体（图片+视频）
  isMedia =  this.isImage || this. isVideo;

}
