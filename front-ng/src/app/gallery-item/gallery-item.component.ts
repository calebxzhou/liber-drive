import { Component, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, GalleryInfo, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common";
import { MatGridList, MatGridTile } from "@angular/material/grid-list";
import { toReadableSize } from "../util";
import { Title } from "@angular/platform-browser";
import { trigger, state, style, transition, animate } from "@angular/animations";

@Component({
  selector: "lg-gallery",
  standalone: true,
  imports: [CommonModule,MatGridList,MatGridTile],
  templateUrl: "./gallery-item.component.html",
  animations: [
    trigger('scaleInOut', [
      state('void', style({
        opacity: 0,
        transform: 'scale(0)'
      })),
      transition('void <=> *', animate(300, style({
        opacity: 1,
        transform: 'scale(1)'
      }))),
    ])
  ],
})
export class GalleryItemComponent implements OnInit {
  @Input() gallery!: GalleryInfo;
  size:string="";
  amount:number=0;
  thumbnailUrl:string="";
  constructor(private mediaService: MediaService,private titleService: Title,) {}
 
  ngOnInit(): void { 
    
    this.size=toReadableSize(this.gallery.size);
    this.amount=this.gallery.media_amount;
    this.thumbnailUrl = this.mediaService.getThumbnailUrl(this.gallery.tbnl_media_id);
  }
}
