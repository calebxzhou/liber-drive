import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { Observable, tap } from 'rxjs';
import { PageService } from '../page.service';
import { Gallery, MediaItem } from './media';

@Injectable({
  providedIn: 'root'
})
export class MediaService{ 
  galleries!: Gallery[];
  constructor(private http: HttpClient, private page: PageService) {

  }
  getUrl(): string{
    return `http://${this.page.getHostName()}:7789`;
  }
  fetchAllGalleries(): Observable<Gallery[]> {
    return this.http.get<Gallery[]>(`${this.getUrl()}/galleries`).pipe(tap(g=>this.galleries=this.processGalleries(g)));
  }

  processGalleries(g:Gallery[]): Gallery[]{
    let gAll = g
        .filter((g) => g.size > 0)
        .sort((g1, g2) => g1.name.localeCompare(g2.name))
        .reverse();
      let gAllPhotos: Gallery = {
        id: -1,
        name: "全部照片",
        size: 0,
        medias: [],
      };

      // Loop through the Gallery array and accumulate the size and medias
      gAll.forEach((gallery) => {
        // Add the size of each gallery to the total size
        gAllPhotos.size += gallery.size;

        // Loop through the medias map of each gallery
        for (let media of gallery.medias) {
          gAllPhotos.medias.push(media);
        }
      }); 
      return [gAllPhotos].concat(gAll);
  }
  getThumbnailUrl(id: number) :string {
    return `${this.getUrl()}/preview/${id}/2`;
  }
  getPreviewUrl(id: number) :string {
    return `${this.getUrl()}/preview/${id}/0`;
  }
  getOriginalUrl(id: number) :string {
    return `${this.getUrl()}/media/${id}`;
  }
  isVideo(media: MediaItem){
    return media.name.toLocaleLowerCase().endsWith(".mp4")
  }
  isImage(media: MediaItem){
    return media.name.toLocaleLowerCase().endsWith(".jpg")
  }
}
