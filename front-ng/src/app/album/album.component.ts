import { Component, Input, OnInit } from "@angular/core";
import { ActivatedRoute, Router, RouterModule } from "@angular/router";
import { Album, DefaultAlbum, GalleryInfo, Media } from "../media/media";
import { CommonModule, formatDate } from "@angular/common";
import { MediaService } from "../media/media.service";
import { MatIconModule } from "@angular/material/icon";
import { MatButtonModule } from "@angular/material/button";
import { MatToolbarModule } from "@angular/material/toolbar";
import { MediaViewerComponent } from "../media-viewer/media-viewer.component";
import { MatExpansionModule } from "@angular/material/expansion";
import { readableDate, toReadableSize } from "../util";
import { NavbarComponent } from "../navbar/navbar.component";
import { ImageTbnlComponent } from "../image-tbnl/image-tbnl.component";
import { LOADING_GIF } from "../const";
import { LazyLoadImageModule } from "ng-lazyload-image";
@Component({
  selector: "lg-album",
  standalone: true,
  templateUrl: "./album.component.html",
  styles: `
  .img-grid{
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(128px, 1fr));
    grid-gap: 1px;
  }
  .grid-item{
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    height: 128px;
  }
  `,
  imports: [
    CommonModule,
    MatToolbarModule,
    MatIconModule,
    MatButtonModule,
    MediaViewerComponent,
    RouterModule,
    NavbarComponent,
    MatExpansionModule,
    ImageTbnlComponent,
    LazyLoadImageModule,
  ],
})
export class AlbumComponent implements OnInit {
  galleryName!: string;
  albumName!: string;
  defaultImageUrl = LOADING_GIF;
  title: string = "";
  album: Album = DefaultAlbum;
  reverseOrder = (a: { key: string }, b: { key: string }) => {
    return a.key > b.key ? -1 : b.key > a.key ? 1 : 0;
  };
  //所有图片
  medias: Media[] = [];
  //要看大图的图片（当前日期下）
  mediasBeingViewed: Media[] = [];
  imageAmount = 0;
  videoAmount = 0;
  //日期折叠
  visibleGroups: { [key: string]: boolean } = {};
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
      this.mediaService
        .fetchAlbum(this.galleryName, this.albumName)
        .subscribe((album) => {
          this.album = album;
          let medias = Object.values(this.album.medias);
          this.medias = medias;
          let groups = this.mediaService.groupMediaByDay(medias);
          this.videoAmount = medias.filter((m) => this.isVideo(m)).length;
          this.imageAmount = medias.length - this.videoAmount;
          this.mediaGroups = groups;
          this.visibleGroups[Object.keys(groups)[0]] = true;
          this.title = `${album.name}(${medias.length})`;
        });
    });
  }
  getMediasByDate(date: string) {
    return this.mediaGroups[date]!;
  }
  getImageUrl(media: Media): string {
    return this.mediaService.fetchMediaUrl(
      this.galleryName,
      this.albumName,
      media.name,
      1
    );
  }
  getVideoDuration(media: Media) {
    return this.mediaService.formatDuration(media.duration ?? 0);
  }
  getDateVideoAmount(date: string) {
    return this.getMediasByDate(date).filter((m) => this.isVideo(m)).length;
  }
  getDateImageAmount(date: string) {
    return this.getMediasByDate(date).filter((m) => !this.isVideo(m)).length;
  }
  isVideo(media: Media): boolean {
    return this.mediaService.isVideo(media);
  }
  size(media: Media) {
    return toReadableSize(media.size);
  }
  date(key: string) {
    return readableDate(key);
  }
  isDisplayViewer = false;
  viewerIndex = 0;
  openViewer(date: string, index: number) {
    this.isDisplayViewer = true;
    this.mediasBeingViewed = this.mediaGroups[date];
    this.viewerIndex = index;
  }

  toggleVisibility(key: string): void {
    this.visibleGroups[key] = !this.visibleGroups[key];
  }

  isVisible(key: string): boolean {
    return this.visibleGroups[key];
  }
}
