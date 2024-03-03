import { Component, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common";
import { MatGridList, MatGridTile } from "@angular/material/grid-list";
import { toReadableSize } from "../util";

@Component({
  selector: "lg-gallery",
  standalone: true,
  imports: [CommonModule,MatGridList,MatGridTile],
  templateUrl: "./gallery.component.html",
  styles: ` 
 

    `,
})
export class GalleryComponent implements OnInit {
  @Input() gallery!: Gallery;
  thumbnailUrls: string[] = [];
  size:string="";
  amount:number=0;
  constructor(private mediaService: MediaService) {}
 
  ngOnInit(): void { 
    this.size=toReadableSize(this.gallery.size);
    
    const values = new Map(Object.entries(this.gallery.medias)).values();
    const mediaArray = Array.from(values).map(media=>this.mediaService.getThumbnailUrl(media.id));
    this.amount=mediaArray.length;
    const thirdCount = Math.floor(mediaArray.length / 3);
    this.thumbnailUrls = [
      mediaArray[0], // First image
      mediaArray[thirdCount], // 1/3 count image
      mediaArray[2 * thirdCount], // 2/3 count image
      mediaArray[mediaArray.length - 1] // Last image
    ];
  }
}
