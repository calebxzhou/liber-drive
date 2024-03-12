import { CommonModule } from "@angular/common";
import {
  Component,
  EventEmitter,
  HostListener,
  Input,
  OnInit,
  Output,
} from "@angular/core";
import { Media, ImageExif, GalleryInfo } from "../media/media";
import { MediaService } from "../media/media.service";
import { toReadableSize } from "../util";
import { Title } from "@angular/platform-browser";
import { Router, ActivatedRoute } from "@angular/router";
import { HttpEvent, HttpEventType } from "@angular/common/http";
import { LOADING_GIF } from "../const";
import { MatSnackBar } from "@angular/material/snack-bar";
import { animate, state, style, transition, trigger } from "@angular/animations";
@Component({
  selector: "lg-media-viewer",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./media-viewer.component.html",
   
})
export class MediaViewerComponent implements OnInit {
  //整个相册
  gallery!: GalleryInfo;
  //刚载入的时候 显示哪个图（ID）
  index = 0;
  //所有图片
  medias: Media[] = [];
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
    private router: Router,
    private route: ActivatedRoute,
    private mediaService: MediaService,
    private snackBar: MatSnackBar
  ) {}
  @HostListener("window:keydown", ["$event"])
  onWindowKeyDown(event: KeyboardEvent) {
    if (event.key == "ArrowLeft") {
      this.prev();
    } else if (event.key == "ArrowRight") {
      this.next();
    }
  }
  //鼠标滚轮放大图片
  scale = 1;
  onWheel(event: WheelEvent): void {
    event.preventDefault();
    this.scale += event.deltaY * -0.01;
    this.scale = Math.min(Math.max(1, this.scale), 8);
    //放大自动载入原图
    if(!this.isOriginalLoaded && this.scale > 4){
      this.loadOriginal();
    }
    const target = event.target as HTMLElement;
    target.style.transform = `scale(${this.scale})`;
  }
  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      let smid = params.get("startMediaId");
      let gid = params.get("galleryId");
      if (smid && gid) {
        /* this.mediaService.fetchGallery(+gid).subscribe((gallery) => {
          this.gallery = gallery;
          this.medias = gallery.medias.sort((a, b) => a.time - b.time);
          this.index = gallery.medias.findIndex((m) => m.id === +(smid ?? "0"));
          this.update();
        }); */
      }
    });
  }
  close() {
    window.history.back();
  }
  update() {
    this.displayingUrl = LOADING_GIF;
    this.isOriginalLoaded = false;
    this.now = this.medias[this.index];
   /*  if (this.isImageNow()) this.displayingUrl = this.preview();
    if (this.isVideoNow()) this.displayingUrl = this.full();
 */
    this.fullImageSize = toReadableSize(this.now.size);
    let title = `${this.now.name}`;
    /* let exif = this.now.exif;
    if (exif) {
      title += `⏰${exif.shot_time}📷${exif.make}🔭${exif.lens}📐${exif.focal_len}mm📸${exif.xp_prog}挡👁️F${exif.av}⏱${exif.tv}s@ISO${exif.iso}
      `;
    } */
    this.title = title;
  }
  prev() {
    if (this.index > 0) this.index--;
    this.update();
  }
  next() {
    if (this.index < this.medias.length - 1) this.index++;
    this.update();
  }
  preview() {
    //return this.mediaService.getPreviewUrl(this.now.id);
  }
  full() {
    //return this.mediaService.getOriginalUrl(this.now.id);
  }
  loadOriginal() {
    /* let t1 = Date.now();
    this.mediaService
      .fetchImage(this.full())
      .subscribe((event: HttpEvent<any>) => {
        if (event.type === HttpEventType.DownloadProgress) {
          this.fullImageSize =
            Math.round((100 * event.loaded) / (event.total ?? 1)) + "%";
        } else if (event.type === HttpEventType.Response) {
          let t2 = Date.now();
          const blob: Blob = event.body;
          this.displayingUrl = URL.createObjectURL(blob);
          this.isOriginalLoaded = true;
          this.snackBar.open(
            `已载入原图(⏰${((t2 - t1) / 1000).toFixed(2)}s)`,
            "x",
            {
              duration: 1000,
            }
          );
        }
      }); */
  }
  isImageNow() {
    return this.mediaService.isImage(this.now);
  }
  isVideoNow() {
    return this.mediaService.isVideo(this.now);
  }
}
