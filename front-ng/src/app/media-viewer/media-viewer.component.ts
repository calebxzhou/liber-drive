import { CommonModule } from "@angular/common";
import { Component, EventEmitter, HostListener, Input, OnInit, Output } from "@angular/core";
import { MediaItem, ImageExif, Gallery } from "../media/media";
import { MediaService } from "../media/media.service";
import { toReadableSize } from "../util";
import { Title } from "@angular/platform-browser";
import { Router, ActivatedRoute } from "@angular/router";

@Component({
  selector: "lg-media-viewer",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./media-viewer.component.html",
  styles: ``,
})
export class MediaViewerComponent implements OnInit {
  gallery!: Gallery;
  //刚载入的时候 显示哪个图（ID）
  index = 0;
  //所有图片
  medias: MediaItem[]=[];
  //现在图片
  now !:MediaItem;
  //尺寸（载入原图用）
  size = "";
  //标题
  title = ``;
  //显示URL
  displayingUrl = "";
  //是否原图
  isOriginalLoaded = false;

  imageCache:Blob[]=[];
  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private mediaService: MediaService,
    private titleService: Title
  ) {}
  @HostListener('window:keydown', ['$event'])
  onWindowKeyDown(event: KeyboardEvent) {
    if (event.key == 'ArrowLeft') {
      this.prev();
    } else if (event.key == 'ArrowRight') {
      this.next();
    }
  }
  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      let smid = params.get("startMediaId");
      let gid = params.get("galleryId");
      if(smid&&gid){
        this.mediaService.fetchGallery(+gid).subscribe((gallery) => {
          this.gallery=gallery;
          this.medias=gallery.medias;
          this.index=gallery.medias.findIndex(m=>m.id===(+(smid??'0')));
          this.update();
        });
      } 
    });
  }
  close() {
    window.history.back();
  }
  update() {
    this.displayingUrl = "";
    this.isOriginalLoaded = false;
    this.now = this.medias[this.index];
    if (this.isImageNow()) this.displayingUrl = this.preview();
    if (this.isVideoNow()) this.displayingUrl = this.full();

    this.size = toReadableSize(this.now.size);
    let title = `${this.now.name}`;
    let exif = this.now.exif;
    if (exif) {
      title += `📷${exif.make}🔭${exif.lens}📐${exif.focal_len}📸${exif.xp_prog}挡👁️${exif.aperture}⏱${exif.shutter} ISO${exif.iso}
      ⏰${exif.shot_time}`;
    }
    this.title = title;
    //this.prefetch5Images();
  }
  prefetch5Images(){
    this.mediaService.fetchPreview(this.medias[this.index-1].id).subscribe(a=>this.imageCache.push(a));
    this.mediaService.fetchPreview(this.medias[this.index-2].id).subscribe(a=>this.imageCache.push(a));
    this.mediaService.fetchPreview(this.medias[this.index+1].id).subscribe(a=>this.imageCache.push(a));
    this.mediaService.fetchPreview(this.medias[this.index+2].id).subscribe(a=>this.imageCache.push(a));
    this.mediaService.fetchPreview(this.medias[this.index+3].id).subscribe(a=>this.imageCache.push(a));
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
    return this.mediaService.getPreviewUrl(this.now.id);
  }
  full() {
    return this.mediaService.getOriginalUrl(this.now.id);
  }
  loadOriginal() {
    this.displayingUrl = this.full();
    this.isOriginalLoaded = true;
  }
  isImageNow() {
    return this.mediaService.isImage(this.now);
  }
  isVideoNow() {
    return this.mediaService.isVideo(this.now);
  }
}
