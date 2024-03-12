import { Component, Input, OnInit } from "@angular/core";
import { ActivatedRoute, Router, RouterModule } from "@angular/router";
import { Album, DefaultAlbum, GalleryInfo, Media } from "../media/media";
import { Title } from "@angular/platform-browser";
import { CommonModule } from "@angular/common";
import { MediaService } from "../media/media.service";
import { MatIconModule } from "@angular/material/icon";
import { MatButtonModule } from "@angular/material/button";
import { MatToolbarModule } from "@angular/material/toolbar";
import { MediaViewerComponent } from "../media-viewer/media-viewer.component";
import { toReadableSize } from '../util';
import { NavbarComponent } from "../navbar/navbar.component";
import { ImageGridComponent } from "../image-grid/image-grid.component";
@Component({
  selector: "lg-album",
  standalone: true,
  imports: [
    CommonModule,
    MatToolbarModule,
    MatIconModule,
    MatButtonModule,
    MediaViewerComponent ,
    RouterModule,
    NavbarComponent,
    ImageGridComponent
  ],
  templateUrl: "./album.component.html",
  styles: `
   
 
  `,
})
export class AlbumComponent implements OnInit {
  galleryName!:string;
  albumName!:string;

  title: string = "";
  album : Album = DefaultAlbum; 
  medias: Media[] = [];

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private mediaService: MediaService
  ) {}

  ngOnInit() {
    // get the id from the route parameter
    this.route.paramMap.subscribe((params) => {
      this.galleryName = params.get("galleryName")!;
      this.albumName = params.get("albumName")!;
      
      this.mediaService.fetchAlbum(this.galleryName,this.albumName).subscribe((album) => {
        this.album=album;
        this.medias=Object.values(album.medias).sort((a,b)=>a.time-b.time)
        this.title=album.name +" "+ toReadableSize(album.size);
      });
        this.mediaService.fetchAlbum(this.galleryName,this.albumName);
      
     
     
    });
  }
  isVideo(media: Media): boolean {
    return this.mediaService.isVideo(media);
  }
  back() {
    this.router.navigate(["/"]);
  }
  size(media:Media){return toReadableSize(media.size) }
  imgPreview(media: Media) {
    return this.mediaService.fetchMediaUrl(this.galleryName,this.album.name,media.name,1);
  }
}
