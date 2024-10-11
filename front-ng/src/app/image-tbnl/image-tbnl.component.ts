import { Component, OnInit, Input, OnDestroy } from "@angular/core";
import { CommonModule } from "@angular/common";
import { MatProgressSpinnerModule } from "@angular/material/progress-spinner";
import { MediaService } from "../media/media.service";
import { Media } from "../media/media";
import { LazyLoadImageModule } from "ng-lazyload-image";
import { ICON_LOCK, LOADING_GIF } from "../const";
import {
  HttpEventType,
  HttpErrorResponse,
  HttpStatusCode,
} from "@angular/common/http";
@Component({
  selector: "lg-image-tbnl",
  standalone: true,
  imports: [CommonModule, MatProgressSpinnerModule, LazyLoadImageModule],
  templateUrl: "./image-tbnl.component.html",
  styles: `
   img {
    width: 100%;
    height: 100%;
    object-fit: cover; /* Ensures the image covers the area without white borders */
    object-position: center; /* Centers the image */ 
  }
  `,
})
// 缩略图
export class ImageTbnlComponent implements OnInit {
  @Input() path: string | undefined;
  @Input() name: string | undefined;
  @Input() id: string | undefined;
  @Input() borderRadius: string = "";
  imageUrl: string = LOADING_GIF;

  constructor(private ms: MediaService) {}

  ngOnInit(): void {
    let path = this.path;
    let name = this.name;
    let id = this.id;
    if (path && name) {
      this.ms.fetchMediaEvent(path, name, 1).subscribe(
        (event) => {
          if (event.type === HttpEventType.Response) {
            const blob = event.body as Blob;
            const objectUrl = URL.createObjectURL(blob);
            this.imageUrl = objectUrl;
          }
        },
        (error: HttpErrorResponse) => {
          if (error.status === HttpStatusCode.Unauthorized) {
            this.imageUrl = ICON_LOCK;
          }
        }
      );
    } else if (id) {
      this.ms.fetchMediaIdEvent(id, 1).subscribe(
        (event) => {
          if (event.type === HttpEventType.Response) {
            const blob = event.body as Blob;
            const objectUrl = URL.createObjectURL(blob);
            this.imageUrl = objectUrl;
          }
        },
        (error: HttpErrorResponse) => {
          if (error.status === HttpStatusCode.Unauthorized) {
            this.imageUrl = ICON_LOCK;
          }
        }
      );
    }
  }
}
