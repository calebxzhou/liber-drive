/**
 * 转换unix时间戳到人类可读时间
 */
export function toReadableTime(timestamp: number): string {
    const date = new Date(timestamp * 1000);
    const year = date.getFullYear();
    // JavaScript months are 0-indexed, so we add 1 to get the correct month number
    const month = ("0" + (date.getMonth() + 1)).slice(-2);
    const day = ("0" + date.getDate()).slice(-2);
    const hours = ("0" + date.getHours()).slice(-2);
    const minutes = ("0" + date.getMinutes()).slice(-2);
    const seconds = ("0" + date.getSeconds()).slice(-2);
  
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  }
  /**
   * 字节数转可读文件尺寸 eg 1024768 -> 1MB
   * @param {number} byte
   * @returns {string}
   */
  export function toReadableSize(byte: number): string {
    let bytes = Number(byte);
    if (bytes === 0) return "0B";
    const units = ["B", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let i = 0;
    while (bytes >= 1024) {
      bytes /= 1024;
      i++;
    }
    return bytes.toFixed(2) + " " + units[i];
  }
  export function extractNumbers(input:string):number {
    return Number(input.replace(/\D/g, "")); // Replace non-digits with an empty string
  }
  /**
   * 是否移动端
   */
  export function isMobile():boolean {
    return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
      navigator.userAgent
    );
  }