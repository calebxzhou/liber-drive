import { Component, Input, OnInit } from "@angular/core";
import { ActivatedRoute, Router, RouterModule } from "@angular/router";
import { Album, DefaultAlbum, GalleryInfo, Media } from "../media/media";
import { CommonModule } from "@angular/common";
import { MediaService } from "../media/media.service";
import { MatIconModule } from "@angular/material/icon";
import { MatButtonModule } from "@angular/material/button";
import { MatToolbarModule } from "@angular/material/toolbar";
import { MediaViewerComponent } from "../media-viewer/media-viewer.component";
import { toReadableSize } from "../util";
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
    MediaViewerComponent,
    RouterModule,
    NavbarComponent,
    ImageGridComponent,
  ],
  templateUrl: "./album.component.html",
  styles: `
  `,
})
export class AlbumComponent implements OnInit {
  galleryName!: string;
  albumName!: string;

  title: string = "";
  album: Album = DefaultAlbum;
  reverseOrder = (a: { key: string }, b: { key: string }) => {
    return a.key > b.key ? -1 : b.key > a.key ? 1 : 0;
  };
  //所有图片
  medias: Media[] = [];
  //日期分组图片
  mediaGroups: Record<string, Media[]> = {};
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private mediaService: MediaService
  ) {}

  ngOnInit() {
    this.route.paramMap.subscribe((params) => {
      this.galleryName = params.get("galleryName")!;
      this.albumName = params.get("albumName")!;
      let mediaName = params.get("mediaName");
      this.mediaService
        .fetchAlbum(this.galleryName, this.albumName)
        .subscribe((album) => {
          this.album = album;
          let medias = Object.values(this.album.medias);
          this.medias = medias;
          let groups = this.mediaService.groupMediaByDay(medias);
          this.mediaGroups = groups;
          this.title = album.name + " " + toReadableSize(album.size);
          if (mediaName) {
            let media = medias.find((m) => m.name === mediaName);
            if (media) {
              this.openViewer(media);
            }
          }
        });
    });
  }
  getMediasByDate(date: string) {
    return this.mediaGroups[date]!;
  }
  isVideo(media: Media): boolean {
    return this.mediaService.isVideo(media);
  }
  size(media: Media) {
    return toReadableSize(media.size);
  }

  isDisplayViewer = false;
  viewerIndex = 0;
  openViewer(media: Media) {
    this.isDisplayViewer = true;
    this.viewerIndex = this.medias.findIndex((m) => m === media);
  }
}
