import { Component, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { AlbumInfo } from "../media/media";
import { CommonModule } from "@angular/common";
import { MatGridList, MatGridTile } from "@angular/material/grid-list";
import { toReadableSize } from "../util";
import {
  trigger,
  state,
  style,
  transition,
  animate,
} from "@angular/animations";
import { HttpEvent, HttpEventType } from "@angular/common/http";
import { ActivatedRoute, Router } from "@angular/router";

@Component({
  selector: "lg-album-preview",
  standalone: true,
  imports: [CommonModule, MatGridList, MatGridTile],
  templateUrl: "./album-preview.component.html",

    
})
export class AlbumPreviewComponent implements OnInit {
  @Input() info!: AlbumInfo;
  galleryName="";
  size: string = "";
  amount: number = 0;
  thumbnailUrl: string = "";
  constructor(private router: Router,private route: ActivatedRoute,private ms: MediaService) {}

  ngOnInit(): void {
    this.route.paramMap.subscribe((params) => {
      this.galleryName = params.get("galleryName")!;
      this.size = toReadableSize(this.info.size);
      this.amount = this.info.media_amount;
      this.ms
        .getAlbumTbnl(this.galleryName, this.info.name)
        .subscribe((e: HttpEvent<any>) => {
          if (e.type == HttpEventType.Response) {
            this.thumbnailUrl = URL.createObjectURL(e.body);
          }
        });
    });
    
  }
  goAlbum(){
    this.router.navigate([`/gallery/${this.galleryName}/${this.info.name}`])
  }
}
