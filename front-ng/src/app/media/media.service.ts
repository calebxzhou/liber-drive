import { HttpClient, HttpEvent, HttpEventType } from "@angular/common/http";
import { Injectable, OnInit } from "@angular/core";
import { BehaviorSubject, Observable, filter, switchMap, tap } from "rxjs";
import { PageService } from "../page.service";
import {
  GalleryInfo,
  Album,
  Media,
  AlbumInfo,
  DefaultGallery,
  DefaultAlbum,
} from "./media";

@Injectable({
  providedIn: "root",
})
export class MediaService {
  constructor(private http: HttpClient, private page: PageService) {}

  fetchGallery(name: string): Observable<GalleryInfo> {
    return this.http.get<GalleryInfo>(`${this.getUrl()}/gallery/${name}`);
  }
  fetchAlbum(galleryName: string, albumName: string) {
    return this.http.get<Album>(
      `${this.getUrl()}/gallery/${galleryName}/${albumName}`
    );
  }
  getUrl(): string {
    return `http://${this.page.getHostName()}:7789`;
  }
  fetchBlob(url: string): Observable<HttpEvent<Blob>> {
    return this.http.get(url, {
      responseType: "blob",
      reportProgress: true,
      observe: "events",
    });
  }
  fetchMediaUrl(
    galleryName: string,
    albumName: string,
    mediaName: string,
    tbnl: number
  ) {
    return `${this.getUrl()}/gallery/${galleryName}/${albumName}/${mediaName}${
      tbnl > -1 ? `?tbnl=${tbnl}` : ""
    }`;
  }
  //tbnl：预览 -1原图 0大图 1小图
  fetchMedia(
    galleryName: string,
    albumName: string,
    mediaName: string,
    tbnl: number
  ): Observable<HttpEvent<Blob>> {
    return this.fetchBlob(
      this.fetchMediaUrl(galleryName, albumName, mediaName, tbnl)
    );
  }
  getAlbumTbnl(
    galleryName: string,
    albumName: string
  ): Observable<HttpEvent<Blob>> {
    return this.http
      .get(`${this.getUrl()}/gallery/${galleryName}/${albumName}?tbnl=1`, {
        responseType: "text",
      })
      .pipe(
        switchMap((tbnlName) => {
          // Assuming tbnlName is the name of the media file you want to fetch
          return this.fetchMedia(galleryName, albumName, tbnlName, 1);
        })
      );
  }
  isVideo(media: Media) {
    return (
      media.name.toLocaleLowerCase().endsWith(".mp4") ||
      media.name.toLocaleLowerCase().endsWith(".mov")
    );
  }
  isImage(media: Media) {
    return (
      media.name.toLocaleLowerCase().endsWith(".jpg") ||
      media.name.toLocaleLowerCase().endsWith(".png") ||
      media.name.toLocaleLowerCase().endsWith(".heic")
    );
  }
  groupMediaByDay(mediaArray: Media[]): Record<string, Media[]> {
    const grouped: Record<string, Media[]> = {};

    mediaArray.sort((a,b)=>b.time-a.time)
    .forEach((media) => {
      // Convert Unix timestamp to a date string (YYYY-MM-DD)
      const date = new Date(media.time * 1000).toISOString().split("T")[0];

      // If the date isn't in the grouped object, initialize it with an empty array
      if (!grouped[date]) {
        grouped[date] = [];
      }

      // Push the current media object to the array for the date
      grouped[date].push(media);
    });
    
    return grouped;
  }
  // In your Angular component
  getVideoDuration(url: string): Promise<number> {
    return new Promise((resolve, reject) => {
      const video = document.createElement("video");
      video.preload = "metadata";

      video.onloadedmetadata = function () {
        window.URL.revokeObjectURL(video.src);
        resolve(video.duration);
      };

      video.onerror = function () {
        reject("Error loading video metadata.");
      };

      video.src = url;
    });
  }
  formatDuration(seconds: number): string {
    const hours: number = Math.floor(seconds / 3600);
    const minutes: number = Math.floor((seconds % 3600) / 60);
    const remainingSeconds: number = seconds % 60;

    const paddedHours = hours.toString().padStart(2, "0");
    const paddedMinutes = minutes.toString().padStart(2, "0");
    const paddedSeconds = remainingSeconds.toString().padStart(2, "0");

    return `${paddedHours}:${paddedMinutes}:${paddedSeconds}`;
  }
}
