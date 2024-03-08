import { Component, HostListener, OnInit } from "@angular/core";
import { MediaService } from "../media/media.service";
import { Gallery, GalleryInfo, MediaItem } from "../media/media";
import { CommonModule } from "@angular/common"; 
import { GalleryItemComponent } from "../gallery-item/gallery-item.component";
import { RouterModule } from "@angular/router";
import { Title } from "@angular/platform-browser";

@Component({
  selector: "gallery-grid",
  templateUrl: "./gallery-grid.component.html",
  imports: [CommonModule, GalleryItemComponent,RouterModule],
  standalone: true,
})
export class GalleryGridComponent implements OnInit {
  galleries: GalleryInfo[] = [];

  constructor(private galleryService: MediaService,private titleService: Title) {}
 

  ngOnInit() {
    this.titleService.setTitle("嘉乐周的光影世界 2.0");
    this.getGalleries();
  }

  getGalleries(): void {
    this.galleryService.fetchAllGalleries().subscribe((g) => {
      this.galleries=this.galleryService.processGalleries(g);
    });
  }
}
