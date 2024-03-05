import { Component, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, GalleryInfo, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common";
import { MatGridList, MatGridTile } from "@angular/material/grid-list";
import { toReadableSize } from "../util";
import { Title } from "@angular/platform-browser";

@Component({
  selector: "lg-gallery",
  standalone: true,
  imports: [CommonModule,MatGridList,MatGridTile],
  templateUrl: "./gallery-item.component.html",
  styles: ` 
 

    `,
})
export class GalleryItemComponent implements OnInit {
  @Input() gallery!: GalleryInfo;
  thumbnailUrls: string[] = [];
  size:string="";
  amount:number=0;
  constructor(private mediaService: MediaService,private titleService: Title,) {}
 
  ngOnInit(): void { 
    
    this.size=toReadableSize(this.gallery.size);
    this.amount=this.gallery.media_amount;
    this.thumbnailUrls = this.gallery.tbnl_media_ids.map(id=>this.mediaService.getThumbnailUrl(id))
  }
}