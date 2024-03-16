import { CommonModule } from "@angular/common";
import {
  AfterViewInit,
  Component,
  ElementRef,
  EventEmitter,
  HostListener,
  Input,
  OnInit,
  Output,
  ViewChild,
} from "@angular/core";
import { Router, ActivatedRoute } from "@angular/router";
import { MediaService } from "../media/media.service";
import { Media } from "../media/media";
import { toReadableSize } from "../util";
import { Observable, from, map } from "rxjs";
import { ImageTbnlComponent } from "../image-tbnl/image-tbnl.component";
@Component({
  selector: "lg-image-grid",
  standalone: true,
  imports: [CommonModule, ImageTbnlComponent],
  templateUrl: "./image-grid.component.html",
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
})
export class ImageGridComponent {
  @Input() medias!: Media[];
  @Input() galleryName: string = "";
  @Input() albumName: string = "";
  @Output() openViewer: EventEmitter<any> = new EventEmitter();
  constructor(private mediaService: MediaService) {}
  openViewer$(media: Media) {
    this.openViewer.emit(media);
  }
}
