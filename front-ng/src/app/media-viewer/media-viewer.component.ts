import { CommonModule, PlatformLocation } from "@angular/common";
import {
  AfterViewInit,
  CUSTOM_ELEMENTS_SCHEMA,
  Component,
  ElementRef,
  EventEmitter,
  HostListener,
  Input,
  OnDestroy,
  OnInit,
  Output,
  QueryList,
  Renderer2,
  ViewChild,
  ViewChildren,
} from "@angular/core";
import { Media, ImageExif, GalleryInfo } from "../media/media";
import { MediaService } from "../media/media.service";
import { toReadableSize } from "../util";
import { Location } from "@angular/common";
import { Router, ActivatedRoute } from "@angular/router";
import { LOADING_GIF } from "../const";
import { MatSnackBar } from "@angular/material/snack-bar";
import { HttpEvent, HttpEventType } from "@angular/common/http";
// import Swiper styles
import "swiper/css";
import "swiper/css/navigation";
import "swiper/css/pagination";
import { Swiper } from "swiper";

@Component({
  selector: "lg-media-viewer",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./media-viewer.component.html",
  schemas: [CUSTOM_ELEMENTS_SCHEMA],
})
export class MediaViewerComponent implements OnInit, OnDestroy, AfterViewInit {
  //所有图片
  @Input() medias!: Media[];
  @Input() index: number = -1;
  @Input() isDisplayViewer!: boolean;
  @Input() galleryName!: string;
  @Input() albumName!: string;
  @Output() isDisplayViewerChange = new EventEmitter<boolean>();
  @ViewChildren("videoPlayer") videoPlayers: QueryList<ElementRef> =
    new QueryList();
  swiper!: Swiper;
  loadProgress = 0;
  //尺寸（载入原图用）
  fullImageSize = "0.0MB";
  //标题
  title = "";
  //显示图片
  displayingUrl = LOADING_GIF;
  //是否原图
  isOriginalLoaded = false;
  //是否载入视频
  isVideoLoaded = false;
  //视频比特率（每秒消耗流量）
  bitrate = "";
  //当前图片缩放比例
  scaleRatio = 1;
  constructor(
    private renderer: Renderer2,
    private router: Router,
    private location: Location,
    private platformLocation: PlatformLocation,
    private route: ActivatedRoute,
    private mediaService: MediaService,
    private snackBar: MatSnackBar
  ) {
    //取消默认返回键逻辑
    history.pushState(null, "", window.location.href); // Prevent the default back action
  }
  ngAfterViewInit() {
    let params = {
      // Optional parameters
      direction: "horizontal",
      loop: false,
      keyboard: {
        enabled: true,
        onlyInViewport: false,
      },
      // If we need pagination
      pagination: {
        el: ".swiper-pagination",
      },
      speed: 400,
      spaceBetween: 0,
      // Navigation arrows
      navigation: {
        nextEl: ".swiper-button-next",
        prevEl: ".swiper-button-prev",
      },
      initialSlide: this.index,
      // And if we need scrollbar
      scrollbar: {
        el: ".swiper-scrollbar",
      },
      autoHeight: true,
      virtual: {
        enabled: true,
      },
      zoom: {
        maxRatio: 3,
      },
      on: {
        zoomChange: (swiper: Swiper, scale: number) => {
          //放得太大 不允许切图
          swiper.allowTouchMove = scale < 2;
          this.scaleRatio = scale;
        },
      },
    };

    let el = document.querySelector("swiper-container")!;
    Object.assign(el, params);
    el.initialize();
    this.onSwiperIndexChange(el.swiper, this.medias);
    this.swiper = el.swiper;
    el.swiper.on("activeIndexChange", (s: Swiper) =>
      this.onSwiperIndexChange(s, this.medias)
    );
  }
  playVideo(media: Media) {
    this.isVideoLoaded = true;
  }
  scaleOut() {
    this.swiper.zoom.out();
  }
  //改变图片时
  onSwiperIndexChange(swiper: Swiper, medias: Media[]) {
    this.index = swiper.activeIndex;
    let media = medias[this.index];
    this.title = media.name;
    this.fullImageSize = toReadableSize(media.size * 3);
    if (media.exif) {
      let exif = media.exif;
      this.title += `⏰${exif.shot_time}📷${exif.make}🔭${exif.lens}📐${exif.focal_len}mm📸${exif.xp_prog}挡👁️F${exif.av}⏱${exif.tv}s@ISO${exif.iso}`;
    }

    this.isOriginalLoaded = false;
    this.isVideoLoaded = false;
    //暂停视频
    this.videoPlayers.forEach((player) => {
      const videoEl: HTMLVideoElement = player.nativeElement;
      videoEl.pause();
    });
    if (this.isVideo(media) && media.duration) {
      this.bitrate = toReadableSize(media.size / media.duration);
    }
  }
  ngOnInit(): void {
    //禁止滚动条
    this.renderer.setStyle(document.body, "overflow", "hidden");

    //返回键逻辑=关闭
    this.platformLocation.onPopState(() => {
      this.close();
    });
  }
  ngOnDestroy() {
    this.renderer.removeStyle(document.body, "overflow");
  }
  close() {
    this.isDisplayViewerChange.emit(false);
  }
  loadOriginal(media: Media) {
    let t1 = Date.now();
    this.mediaService
      .fetchMedia(this.galleryName, this.albumName, media.name, -1)
      .subscribe((event: HttpEvent<any>) => {
        if (event.type === HttpEventType.DownloadProgress) {
          this.fullImageSize =
            Math.round((100 * event.loaded) / (event.total ?? 1)) + "%";
        } else if (event.type === HttpEventType.Response) {
          let t2 = Date.now();
          const blob: Blob = event.body;
          let imageEl: HTMLImageElement = document.getElementById(
            `img_${media.size}`
          ) as HTMLImageElement;
          imageEl.src = URL.createObjectURL(blob);
          this.isOriginalLoaded = true;
          this.snackBar.open(
            `已载入原图(⏰${((t2 - t1) / 1000).toFixed(2)}s)`,
            "x",
            {
              duration: 1000,
            }
          );
        }
      });
  }
  isImage(media: Media) {
    return this.mediaService.isImage(media);
  }
  isVideo(media: Media) {
    return this.mediaService.isVideo(media);
  }
  mediaUrl(media: Media, full: boolean) {
    return this.mediaService.fetchMediaUrl(
      this.galleryName,
      this.albumName,
      media.name,
      full ? -1 : 0
    );
  }
  mediaRecorder: MediaRecorder | undefined;
  private chunks: BlobPart[] = [];

  startRecording() {
    navigator.mediaDevices
      .getUserMedia({ video: true, audio: true })
      .then((stream) => {
        this.mediaRecorder = new MediaRecorder(stream);
        this.mediaRecorder.start();

        this.mediaRecorder.ondataavailable = (e) => {
          this.chunks.push(e.data);
        };
      });
  }

  stopRecording() {
    if (this.mediaRecorder) {
      this.mediaRecorder.stop();

      this.mediaRecorder.onstop = () => {
        const blob = new Blob(this.chunks, { type: "video/mp4" });
        this.chunks = [];

        const videoURL = window.URL.createObjectURL(blob);
        const link = document.createElement("a");
        link.href = videoURL;
        link.download = "RecordedVideo.mp4";
        link.click();
      };
      this.mediaRecorder = undefined;
    }
  }
}
