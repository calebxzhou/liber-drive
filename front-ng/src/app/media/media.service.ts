import { HttpClient } from '@angular/common/http';
import { Injectable, OnInit } from '@angular/core';
import { Observable, tap } from 'rxjs';
import { PageService } from '../page.service';
import { Gallery, GalleryInfo, MediaItem } from './media';

@Injectable({
  providedIn: 'root'
})
export class MediaService{
  constructor(private http: HttpClient, private page: PageService) {

  }
  getUrl(): string{
    return `http://${this.page.getHostName()}:7789`;
  }
  fetchAllGalleries(): Observable<GalleryInfo[]> {
    return this.http.get<GalleryInfo[]>(`${this.getUrl()}/galleries`)
  }
  fetchGallery(id:number) :Observable< Gallery>{
    return this.http.get<Gallery>(`${this.getUrl()}/gallery/${id}`)
  }
  processGalleries(g:GalleryInfo[]): GalleryInfo[]{
    return g
        .filter((g) => g.size > 0)
        .sort((g1, g2) => g1.name.localeCompare(g2.name))
        .reverse();
      /* let gAllPhotos: GalleryInfo = {
        id: -1,
        name: "全部照片",
        size: 9999999999,
        media_amount: 99999
        tbnl_media_ids: [],
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
      return [gAllPhotos].concat(gAll); */
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
  fetchPreview(id:number): Observable<Blob> {
    return this.http.get(this.getPreviewUrl(id), { responseType: 'blob' });
  }
  isVideo(media: MediaItem){
    return media.name.toLocaleLowerCase().endsWith(".mp4")||media.name.toLocaleLowerCase().endsWith(".mov")
  }
  isImage(media: MediaItem){
    return media.name.toLocaleLowerCase().endsWith(".jpg")||media.name.toLocaleLowerCase().endsWith(".png")||media.name.toLocaleLowerCase().endsWith(".heic")
  }
}
