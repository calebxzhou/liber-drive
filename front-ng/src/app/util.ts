/**
 * 转换unix时间戳到人类可读时间
 */
export function timestampToIsoDate(timestamp: number): string {
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
  return bytes.toFixed(1) + " " + units[i];
}
export function extractNumbers(input: string): number {
  return Number(input.replace(/\D/g, "")); // Replace non-digits with an empty string
}
/**
 * 是否移动端
 */
export function isMobile(): boolean {
  return /Android|webOS|iPhone|iPad|iPod|BlackBerry|IEMobile|Opera Mini/i.test(
    navigator.userAgent
  );
}
//日期转换 e.g. 2024-03-28=今天 03-27=昨天 03-26=前天
export function readableDateTime(input: string): string {
  const inputDate = new Date(input);
  const now = new Date();
  const oneDay = 24 * 60 * 60 * 1000; // milliseconds in one day
  const difference = now.getTime() - inputDate.getTime();

  const timeString = inputDate.toLocaleTimeString("zh-CN", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });

  // For dates beyond yesterday, format as "Mar 27 19:45"
  const dateString = inputDate.toLocaleDateString("zh-CN", {
    year: inputDate.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
    month: "short",
    day: "numeric",
  });
  return `${dateString} ${timeString}`;
}
export function deg2rad(deg: number): number {
  return deg * (Math.PI / 180);
}
export function readableDate(input: string): string {
  const inputDate = new Date(input);
  const now = new Date();

  // Format the date as "Mar 27"
  return inputDate.toLocaleDateString("zh-CN", {
    year: inputDate.getFullYear() !== now.getFullYear() ? "numeric" : undefined,
    month: "short",
    day: "numeric",
  });
}
