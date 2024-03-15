import { Component, OnInit, Input, OnDestroy } from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { HttpEvent, HttpEventType } from "@angular/common/http";
import { Subscription } from "rxjs";
import { MediaService } from "../media/media.service";
import {
  trigger,
  state,
  style,
  animate,
  transition,
} from "@angular/animations";
import { Media } from "../media/media";
@Component({
  selector: "lg-image-tbnl",
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule],
  templateUrl: "./image-tbnl.component.html",
  styles: `
  `,
  animations: [
    trigger("scaleAnimation", [
      state("initial", style({ transform: "scale(0.01)" })),
      state("normal", style({ transform: "scale(1)" })),
      transition("initial => normal", animate("500ms ease-in")),
    ]),
  ],
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
  imageUrl: string | null = null;
  constructor(private ms: MediaService) {}

  ngOnInit(): void {
    this.isVideo = this.ms.isVideo(this.media);
    setTimeout(() => {
      this.state = "normal";
    }, 0);
    if (this.isVideo && this.media.duration) {
      this.videoDuration = this.ms.formatDuration(this.media.duration);
    }
    //获取缩略图
    this.imageUrl = this.ms.fetchMediaUrl(
      this.galleryName,
      this.albumName,
      this.media.name,
      1
    );
  }
  ngOnDestroy(): void {}
}
