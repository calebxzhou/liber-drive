import { CommonModule } from "@angular/common";
import { Component, EventEmitter, Input, OnInit, Output } from "@angular/core";
import { MediaItem, ImageExif } from "../media/media";
import { MediaService } from "../media/media.service";
import { toReadableSize } from "../util";

@Component({
  selector: "lg-media-viewer",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./media-viewer.component.html",
  styles: ``,
})
export class MediaViewerComponent implements OnInit{
  @Input()
  medias: MediaItem[] = []; // the array of media items to display
  @Input() index: number = 0; // the index of the current media item
  @Output() mediaViewerToggled = new EventEmitter<boolean>(); // the event emitter to notify the parent component
  now: MediaItem = this.medias[this.index];
  size = "";
  title = ``;
  displayingUrl = "";
  isOriginalLoaded=false;
  constructor(private mediaService: MediaService) {}
  ngOnInit(): void {
    this.now=this.medias[this.index];
    this.update();
  }
  close() {
    this.mediaViewerToggled.emit(false);
  }
  update() {
    this.displayingUrl = "";
    this.isOriginalLoaded=false;
    this.now=this.medias[this.index];
    if (this.isImageNow()) this.displayingUrl = this.preview();
    if (this.isVideoNow()) this.displayingUrl = this.full();
    
    this.size=toReadableSize(this.now.size);
    let title =`${this.now.name}`;
    let exif = this.now.exif;
    if(exif){
      title += `📷${exif.make}🔭${exif.lens}📐${exif.focal_len}📸${exif.xp_prog}挡👁️${exif.aperture}⏱${exif.shutter} ISO${exif.iso}
      ⏰${exif.shot_time}`
    }
    this.title=title;

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
    this.isOriginalLoaded=true;
  }
  isImageNow() {
    return this.mediaService.isImage(this.now);
  }
  isVideoNow() {
    return this.mediaService.isVideo(this.now);
  }
}
