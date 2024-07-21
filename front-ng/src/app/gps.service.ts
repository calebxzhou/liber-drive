import { Injectable } from "@angular/core";
import { deg2rad } from "./util";
import { GeolocationService } from "@ng-web-apis/geolocation";

@Injectable({
  providedIn: "root",
})
export class GpsService {
  constructor() {}

  //xx deg xx min xx sec -> 经纬度数字
  parseDegMinSec(input: string): number {
    const parts = input.split(" ");
    const deg = parseFloat(parts[0]);
    const min = parseFloat(parts[2]);
    const sec = parseFloat(parts[4]);

    return deg + min / 60 + sec / 3600;
  }
  calculateDistance(
    lat1: number,
    lon1: number,
    lat2: number,
    lon2: number
  ): number {
    const R = 6371; // Radius of the earth in km
    const dLat = deg2rad(lat2 - lat1);
    const dLon = deg2rad(lon2 - lon1);
    const a =
      Math.sin(dLat / 2) * Math.sin(dLat / 2) +
      Math.cos(deg2rad(lat1)) *
        Math.cos(deg2rad(lat2)) *
        Math.sin(dLon / 2) *
        Math.sin(dLon / 2);
    const c = 2 * Math.atan2(Math.sqrt(a), Math.sqrt(1 - a));
    const distance = R * c; // Distance in km
    return distance;
  }
}
