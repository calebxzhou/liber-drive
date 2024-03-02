import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { PageService } from '../page.service';
import { Gallery } from './media';

@Injectable({
  providedIn: 'root'
})
export class MediaService { 

  constructor(private http: HttpClient, private page: PageService) {

  }
  getUrl(): string{
    return `http://${this.page.getHostName()}:7789`;
  }
  getGalleries(): Observable<Gallery[]> {
    return this.http.get<Gallery[]>(`${this.getUrl()}/galleries`);
  }
  getThumbnailUrl(id: number) :string {
    return `${this.getUrl()}/preview/${id}/2`;
  }
}
