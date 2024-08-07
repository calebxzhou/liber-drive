import { Component, OnInit, Input, OnDestroy } from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MediaService } from "../media/media.service";
import { Media } from "../media/media";
import { LazyLoadImageModule } from "ng-lazyload-image";
import { LOADING_GIF } from "../const";
@Component({
  selector: "lg-image-tbnl",
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule, LazyLoadImageModule],
  templateUrl: "./image-tbnl.component.html",
  styles: `
  `,
})
// 缩略图
export class ImageTbnlComponent implements OnInit, OnDestroy {
  @Input() media!: Media;
  @Input() galleryName!: string;
  @Input() albumName!: string;
  isVideo = false;
  state: string = "initial";
  videoDuration = "视频";
  progress: number = 0;
  imageUrl: string = LOADING_GIF;
  defaultImageUrl = LOADING_GIF;
  constructor(private ms: MediaService) {}

  ngOnInit(): void {
    //获取缩略图
    this.imageUrl = this.ms.fetchMediaUrl(this.albumName, this.media.name, 1);
    this.isVideo = this.ms.isVideo(this.media);
    setTimeout(() => {
      this.state = "normal";
    }, 0);
    if (this.isVideo && this.media.duration) {
      this.videoDuration = this.ms.formatDuration(this.media.duration);
    }
  }
  ngOnDestroy(): void {}
}
