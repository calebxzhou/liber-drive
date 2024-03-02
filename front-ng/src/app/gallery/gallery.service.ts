import { HttpClient } from '@angular/common/http';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';
import { PageService } from '../page.service';
import { Gallery } from './gallery';

@Injectable({
  providedIn: 'root'
})
export class GalleryService {
  private url = 'http://localhost:7789/galleries';
 
  constructor(private http: HttpClient, private page: PageService) { 
   
  }

  getGalleries(): Observable<Gallery[]> {
    return this.http.get<Gallery[]>(`http://${this.page.getHostName()}:7789/galleries`);
  }
}
