import { HttpClient, HttpErrorResponse, HttpEvent } from "@angular/common/http";
import { Injectable } from "@angular/core";
import {
  catchError,
  map,
  Observable,
  of,
  switchMap,
  tap,
  throwError,
} from "rxjs";
import { PageService } from "../page.service";
import { Album, Media, ImageExif } from "./media";
import { Router } from "@angular/router";
import { MatDialog } from "@angular/material/dialog";
import { PasswordDialogComponent } from "../password-dialog/password-dialog.component";

@Injectable({
  providedIn: "root",
})
export class MediaService {
  pwd = "";
  constructor(
    private http: HttpClient,
    private page: PageService,
    private router: Router,
    private dialog: MatDialog
  ) {}
  listAllAlbums() {
    return this.http.get<Record<string, string[]>>(`${this.getUrl()}/`);
  }
  private promptForPassword() {
    const dialogRef = this.dialog.open(PasswordDialogComponent);
    return dialogRef.afterClosed();
  }
  fetchAlbum(albumName: string) {
    return this.http
      .get<Album>(`${this.getUrl()}/album?path=${albumName}`)
      .pipe(
        catchError((error: HttpErrorResponse) => {
          if (error.status === 401) {
            return this.promptForPassword().pipe(
              switchMap((password) => {
                if (password) {
                  return this.http
                    .get<Album>(
                      `${this.getUrl()}/album?path=${albumName}&pwd=${password}`
                    )
                    .pipe(
                      tap(() => {
                        // Set the global password if the request is successful
                        this.pwd = password;
                      }),
                      catchError((innerError: HttpErrorResponse) => {
                        if (innerError.status === 401) {
                          alert("密码错误");
                          this.router.navigate(["/"]);
                          return of(null);
                        }
                        return throwError(innerError);
                      })
                    );
                } else {
                  return throwError(error);
                }
              })
            );
          } else if (error.status !== 200) {
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
    return `${this.getUrl()}/media?path=${albumName}&name=${mediaName}&pwd=${
      this.pwd
    }${tbnl > -1 ? `&tbnl=${tbnl}` : ""}`;
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
  getAlbumTbnlUrls(albumName: string, tbnlNames: string[]): string[] {
    return tbnlNames.map(
      (tbnl) =>
        `${this.getUrl()}/media?path=${albumName}&name=${tbnl}&tbnl=1&pwd=${
          this.pwd
        }`
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
