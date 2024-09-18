import { HttpClient, HttpErrorResponse, HttpEvent } from "@angular/common/http";
import { Injectable } from "@angular/core";
import { catchError, map, Observable, of, switchMap } from "rxjs";
import { PageService } from "../page.service";
import { Album, Media, ImageExif } from "./media";
import { Router } from "@angular/router";

@Injectable({
  providedIn: "root",
})
export class MediaService {
  constructor(
    private http: HttpClient,
    private page: PageService,
    private router: Router
  ) {}
  fetchAlbumList() {
    return this.http.get<Record<string, Media>>(`${this.getUrl()}/`);
  }
  isAlbumHasPassword(albumName: string): Observable<boolean> {
    return this.http
      .get<boolean>(`${this.getUrl()}/${albumName}?has_pwd`)
      .pipe(map((response) => response));
  }
  checkAlbumPassword(albumName: string): Observable<boolean> {
    return this.http
      .get<boolean>(`${this.getUrl()}/${albumName}?has_pwd`)
      .pipe(map((response) => response));
  }
  fetchAlbum(albumName: string) {
    return this.http.get<Album>(`${this.getUrl()}/${albumName}`).pipe(
      catchError((error: HttpErrorResponse) => {
        if (error.status !== 200) {
          alert(`相册${albumName}不存在`);
          this.router.navigate(["/"]);
        }
        return of(null);
      })
    );
  }
  getUrl(): string {
    return `https://${this.page.getHostName()}:7789`;
  }
  fetchBlob(url: string): Observable<HttpEvent<Blob>> {
    return this.http.get(url, {
      responseType: "blob",
      reportProgress: true,
      observe: "events",
    });
  }
  fetchMediaUrl(albumName: string, mediaName: string, tbnl: number) {
    return `${this.getUrl()}/${albumName}/${mediaName}${
      tbnl > -1 ? `?tbnl=${tbnl}` : ""
    }`;
  }
  fetchImageExif(albumName: string, mediaName: string): Observable<ImageExif> {
    return this.http.get<ImageExif>(
      `${this.getUrl()}/${albumName}/${mediaName}?exif=1`
    );
  }
  //tbnl：预览 -1原图 0大图 1小图
  fetchMedia(
    albumName: string,
    mediaName: string,
    tbnl: number
  ): Observable<HttpEvent<Blob>> {
    return this.fetchBlob(this.fetchMediaUrl(albumName, mediaName, tbnl));
  }
  getAlbumTbnlUrl(albumName: string, firstMedia: Media): string {
    return `${this.getUrl()}/${albumName}/${firstMedia.name}?tbnl=1`;
  }
  getAlbumTbnl(albumName: string): Observable<HttpEvent<Blob>> {
    return this.http
      .get(`${this.getUrl()}/${albumName}?tbnl=1`, {
        responseType: "text",
      })
      .pipe(
        switchMap((tbnlName) => {
          // Assuming tbnlName is the name of the media file you want to fetch
          return this.fetchMedia(albumName, tbnlName, 1);
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
      media.name.toLocaleLowerCase().endsWith(".heic")
    );
  }
  groupMediaByDay(mediaArray: Media[]): Record<string, Media[]> {
    const grouped: Record<string, Media[]> = {};
    mediaArray
      .sort((a, b) => b.time - a.time)
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
    const remainingSeconds: number = Math.round(seconds % 60);

    const paddedHours = hours.toString().padStart(2, "0");
    const paddedMinutes = minutes.toString().padStart(2, "0");
    const paddedSeconds = remainingSeconds.toString().padStart(2, "0");

    return `${paddedHours}:${paddedMinutes}:${paddedSeconds}`;
  }
}
