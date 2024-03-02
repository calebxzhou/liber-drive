import { Component, Input, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common";

@Component({
  selector: "lg-gallery",
  standalone: true,
  imports: [CommonModule],
  templateUrl: "./gallery.component.html",
  styles: `
  .border {
      border: 5px solid gray;
      border-radius: 20px;
    }
    .rounded-img {
      width: 100%;
      height: 100%;
      object-fit: cover;
      border-radius: 20px;
    }
    `,
})
export class GalleryComponent implements OnInit {
  @Input() gallery!: Gallery;
  thumbnailUrls: string[] = [];
  constructor(private mediaService: MediaService) {}

  ngOnInit(): void { 
    const values = new Map(Object.entries(this.gallery.medias)).values();
    const mediaArray = Array.from(values).map(media=>this.mediaService.getThumbnailUrl(media.id));
    const thirdCount = Math.floor(mediaArray.length / 3);
    this.thumbnailUrls = [
      mediaArray[0], // First image
      mediaArray[thirdCount], // 1/3 count image
      mediaArray[2 * thirdCount], // 2/3 count image
      mediaArray[mediaArray.length - 1] // Last image
    ];
  }
}
