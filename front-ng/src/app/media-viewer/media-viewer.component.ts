import { CommonModule, PlatformLocation } from "@angular/common";
import {
  AfterViewInit,
  CUSTOM_ELEMENTS_SCHEMA,
  Component,
  EventEmitter,
  HostListener,
  Input,
  OnDestroy,
  OnInit,
  Output,
  Renderer2,
  ViewChild,
} from "@angular/core";
import { Media, ImageExif, GalleryInfo } from "../media/media";
import { MediaService } from "../media/media.service";
import { toReadableSize } from "../util";
import { Location } from "@angular/common";
import { Router, ActivatedRoute } from "@angular/router";
import { LOADING_GIF } from "../const";
import { MatSnackBar } from "@angular/material/snack-bar";
import { HttpEvent, HttpEventType } from "@angular/common/http";
import $ from "jquery";
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
  @Input() index!: number;
  @Input() isDisplayViewer!: boolean;
  @Output() isDisplayViewerChange = new EventEmitter<boolean>();
  swiper!: Swiper;

  galleryName = "";
  albumName = "";
  loadProgress = 0;
  //现在图片
  now!: Media;
  //尺寸（载入原图用）
  fullImageSize = "0.0MB";
  //标题
  title = `载入中....`;
  //显示图片
  displayingUrl = LOADING_GIF;
  //是否原图
  isOriginalLoaded = false;

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
    this.swiper = new Swiper("#swiper", {
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
    });
    this.onSwiperIndexChange(this.swiper, this.medias);
    this.swiper.on("activeIndexChange", (s) =>
      this.onSwiperIndexChange(s, this.medias)
    );
  }
  playVideo(media: Media) {
    $("#div_" + media.size).html(`
    <video
          controls
          crossorigin
          playsinline
          src="${this.mediaUrl(media, true)}"
          class="block mx-auto object-contain h-[95vh]"
          loading="lazy"
        ></video>
    `);
    this.isOriginalLoaded = true;
  }
  //改变图片时
  onSwiperIndexChange(swiper: Swiper, medias: Media[]) {
    this.index = swiper.activeIndex;
    let media = medias[this.index];
    this.title = media.name;
    this.fullImageSize = toReadableSize(media.size * 3);
    if (this.isImage(media)) {
      this.mediaService
        .fetchImageExif(this.galleryName, this.albumName, this.now.name)
        .subscribe((exif) => {
          this.title += `⏰${exif.shot_time}📷${exif.make}🔭${exif.lens}📐${exif.focal_len}mm📸${exif.xp_prog}挡👁️F${exif.av}⏱${exif.tv}s@ISO${exif.iso}`;
        });
    }

    this.isOriginalLoaded = false;
    //暂停视频
    $("video").each(function () {
      let e = this as HTMLVideoElement;
      $(e)[0].pause();
    });
  }
  ngOnInit(): void {
    //禁止滚动条
    this.renderer.setStyle(document.body, "overflow", "hidden");
    this.route.paramMap.subscribe((params) => {
      this.galleryName = params.get("galleryName")!;
      this.albumName = params.get("albumName")!;
      this.now = this.medias[this.index];

      this.update();
    });
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
  update() {
    /* this.displayingUrl = LOADING_GIF;
    this.isOriginalLoaded = false;
    this.now = this.medias[this.index];
    if (this.isImageNow()) {
      this.mediaService
        .fetchMedia(this.galleryName, this.albumName, this.now.name, 0)
        .subscribe((event: HttpEvent<any>) => {
          if (event.type === HttpEventType.DownloadProgress) {
            this.loadProgress = Math.round(
              (100 * event.loaded) / (event.total ?? 1)
            );
          } else if (event.type === HttpEventType.Response) {
            const blob: Blob = event.body;
            this.displayingUrl = URL.createObjectURL(blob);
          }
        });
    } */
  }
  prev() {
    if (this.index > 0) this.index--;
    this.update();
  }
  next() {
    if (this.index < this.medias.length - 1) this.index++;
    this.update();
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
}
