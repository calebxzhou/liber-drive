import { Component, HostListener, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common";
import { toReadableSize } from "../util";
import { MatGridListModule } from "@angular/material/grid-list"; 
import { GalleryItemComponent } from "../gallery-item/gallery-item.component";
import { RouterModule } from "@angular/router";

@Component({
  selector: "gallery-grid",
  templateUrl: "./gallery-grid.component.html",
  imports: [CommonModule, GalleryItemComponent,RouterModule],
  standalone: true,
})
export class GalleryGridComponent implements OnInit {
  galleries: Gallery[] = [];

  constructor(private galleryService: MediaService) {}
 

  ngOnInit() {
    this.getGalleries();
  }

  getGalleries(): void {
    this.galleryService.fetchAllGalleries().subscribe((g) => {
      this.galleries=this.galleryService.processGalleries(g);
    });
  }
}
