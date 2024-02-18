export const FILE_TYPE_EXTENSION = {
  zip: ["zip", "rar", "7z", "gz"],
  code: ["c", "py", "rs", "java", "sh"],
  excel: ["xls", "xlsx"],
  img: ["jpg", "png", "tiff", "avif", "heif", "bmp", "gif", "webp", "svg"],
  raw: ["cr2", "orf", "rw2"],
  ppt: ["ppt", "pptx"],
  text: ["txt"],
  video: ["mp4", "mkv", "flv", "gif", "avi", "mov","MOV", "wmv"],
  word: ["doc", "docx"],
};
/**
 * 文件项
 */

export default class FileItem {
  /**
   *
   * @param {string} name 文件名
   * @param {string} queryUrl 请求url
   */
  constructor(name,queryUrl) {
    this.name = name;
    this.queryUrl=queryUrl;
  }

  /**
   * 获取文件扩展名
   * @param {string} this.name 文件名
   * @returns {string} 扩展名
   */
  getExtension() {
    return this.name
      .slice(((this.name.lastIndexOf(".") - 1) >>> 0) + 2)
      .toLowerCase();
  }
  getIcon() {
    if(this.isDir()){
      return '📁';
    }
    switch (this.getType()) {
      case 'img':
        return '🖼️';
      case 'video':
        return '🎥';
      default:
        return '📜';
    }
  }
  isImg(){
    return this.getType() === "img" ;
  }
  isVideo(){
    return this.getType() === "video";
  }
  /**
   * 获取文件类型 
   * @returns {string} 文件类型 无法识别则为file
   */
  getType() {
    return Object.entries(FILE_TYPE_EXTENSION).reduce(
      (acc, [key, value]) =>
        value.includes(this.getExtension()) ? key : acc,
      "file"
    );
  }
  /**
   * 文件是否为媒体（图片/视频） 
   * @returns {boolean}
   */
  isMedia() {
    return (
      this.getType() === "img" 
      || this.getType() === "video"
    );
  }
  /**
   * 文件是否目录
   * @returns {boolean}
   */
  isDir() {
    return this.name.endsWith('/');
  }
}
